use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use openapi_nexus::generators::rust::reqwest::RustReqwestCodeGenerator;
use openapi_nexus::generators::rust::ureq::RustUreqCodeGenerator;
use openapi_nexus::test_utils::{generate_files, read_fixture};

fn empty_config() -> toml::value::Table {
    toml::value::Table::new()
}

struct TempDir {
    path: PathBuf,
}

impl TempDir {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after UNIX_EPOCH")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "openapi-nexus-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("temporary crate directory should be created");
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_generated_crate(files: &HashMap<String, String>, root: &Path) {
    for (relative_path, contents) in files {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("generated parent directory should be created");
        }
        fs::write(path, contents).expect("generated file should be written");
    }
}

const RUST_TEST_SERVER: &str = r##"
use std::io::{Read, Write};
use std::net::TcpListener;
use std::thread;

fn spawn_one_response_server(status: &str, body: &'static str) -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("test server should bind");
    let addr = listener.local_addr().expect("test server address should be readable");
    let status = status.to_string();
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("test server should accept one request");
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request);
        let response = format!(
            "HTTP/1.1 {status}\r\nContent-Type: application/json\r\nX-Trace-Id: typed-error-test\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream
            .write_all(response.as_bytes())
            .expect("test response should be written");
    });
    format!("http://{addr}")
}

fn spawn_incomplete_success_body_server() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("test server should bind");
    let addr = listener.local_addr().expect("test server address should be readable");
    thread::spawn(move || {
        let (mut stream, _) = listener.accept().expect("test server should accept one request");
        let mut request = [0_u8; 4096];
        let _ = stream.read(&mut request);
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: 999999\r\nConnection: close\r\n\r\n";
        stream
            .write_all(response.as_bytes())
            .expect("test response headers should be written");
    });
    format!("http://{addr}")
}
"##;

fn add_reqwest_runtime_test(root: &Path) {
    let cargo_toml = root.join("Cargo.toml");
    let mut manifest =
        fs::read_to_string(&cargo_toml).expect("generated Cargo.toml should be readable");
    manifest.push_str(
        r#"

[dev-dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
"#,
    );
    fs::write(cargo_toml, manifest).expect("generated Cargo.toml should be updated");

    let tests_dir = root.join("tests");
    fs::create_dir_all(&tests_dir).expect("generated tests directory should be created");
    fs::write(
        tests_dir.join("http_error.rs"),
        [
            RUST_TEST_SERVER,
            r##"
use typed_error_responses_api::apis::{
    CreateResourceError, CreateResourceWithWildcardSuccessError, ResourcesApi,
};
use typed_error_responses_api::models::CreateResourceRequest;
use typed_error_responses_api::runtime::client::Client;
use typed_error_responses_api::runtime::error::Error;

#[tokio::test]
async fn http_503_returns_operation_error_with_typed_body() {
    let body = r#"{"message":"temporarily unavailable","retryable":true}"#;
    let base_url = spawn_one_response_server("503 Service Unavailable", body);
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);
    let request = CreateResourceRequest {
        name: "resource".to_string(),
    };

    let err = api
        .create_resource(&request)
        .await
        .expect_err("HTTP 503 must not be returned as success");

    match err {
        CreateResourceError::ServerError(api_error) => {
            assert_eq!(api_error.status_code(), 503);
            assert_eq!(api_error.raw_body(), body.as_bytes());
            assert!(api_error.headers().iter().any(|(name, value)| {
                name.eq_ignore_ascii_case("x-trace-id") && value == "typed-error-test"
            }));
            let typed_body = api_error
                .body()
                .expect("documented 5XX error body should decode lazily");
            assert_eq!(typed_body.message, "temporarily unavailable");
            assert!(typed_body.retryable);
        }
        other => panic!("expected ServerError variant, got {other:?}"),
    }
}

#[tokio::test]
async fn invalid_error_body_keeps_http_error_and_raw_body() {
    let body = r#"{"message":42,"retryable":true}"#;
    let base_url = spawn_one_response_server("503 Service Unavailable", body);
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);
    let request = CreateResourceRequest {
        name: "resource".to_string(),
    };

    let err = api
        .create_resource(&request)
        .await
        .expect_err("HTTP 503 must remain an operation error even when its body is invalid");

    match err {
        CreateResourceError::ServerError(api_error) => {
            assert_eq!(api_error.status_code(), 503);
            assert_eq!(api_error.raw_body(), body.as_bytes());
            assert!(matches!(api_error.body(), Err(Error::Deserialize(_))));
        }
        other => panic!("expected ServerError variant, got {other:?}"),
    }
}

#[tokio::test]
async fn exact_success_status_wins_over_2xx_wildcard() {
    let body = r#"{"id":"exact-created"}"#;
    let base_url = spawn_one_response_server("201 Created", body);
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);
    let request = CreateResourceRequest {
        name: "resource".to_string(),
    };

    let response = api
        .create_resource_with_wildcard_success(&request)
        .await
        .expect("HTTP 201 should be a successful exact response");

    let created = response
        .created
        .expect("201 must match the exact branch before 2XX wildcard");
    assert_eq!(created.id, "exact-created");
    assert!(response.status_2xx.is_none());
}

#[tokio::test]
async fn unexpected_non_success_status_uses_unexpected_detail() {
    let body = r#"{"message":"teapot"}"#;
    let base_url = spawn_one_response_server("418 I'm a teapot", body);
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);
    let request = CreateResourceRequest {
        name: "resource".to_string(),
    };

    let err = api
        .create_resource_with_wildcard_success(&request)
        .await
        .expect_err("undocumented HTTP 418 must not be returned as success");

    match err {
        CreateResourceWithWildcardSuccessError::Unexpected(api_error) => {
            assert_eq!(api_error.status_code(), 418);
            assert_eq!(api_error.raw_body(), body.as_bytes());
        }
        other => panic!("expected Unexpected variant, got {other:?}"),
    }
}

#[tokio::test]
async fn success_without_documented_body_does_not_drain_body() {
    let base_url = spawn_incomplete_success_body_server();
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);

    let response = api
        .check_no_success_body()
        .await
        .expect("success without a documented body must not read an incomplete body");

    assert_eq!(response.status_code, 200);
}
"##,
        ]
        .join("\n"),
    )
    .expect("generated runtime test should be written");
}

fn add_ureq_runtime_test(root: &Path) {
    let tests_dir = root.join("tests");
    fs::create_dir_all(&tests_dir).expect("generated tests directory should be created");
    fs::write(
        tests_dir.join("http_error.rs"),
        [
            RUST_TEST_SERVER,
            r##"
use typed_error_responses_api::apis::{CreateResourceError, ResourcesApi};
use typed_error_responses_api::models::CreateResourceRequest;
use typed_error_responses_api::runtime::client::Client;

#[test]
fn http_503_returns_operation_error_with_typed_body() {
    let body = r#"{"message":"temporarily unavailable","retryable":true}"#;
    let base_url = spawn_one_response_server("503 Service Unavailable", body);
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);
    let request = CreateResourceRequest {
        name: "resource".to_string(),
    };

    let err = api
        .create_resource(&request)
        .expect_err("HTTP 503 must not be returned as transport or success");

    match err {
        CreateResourceError::ServerError(api_error) => {
            assert_eq!(api_error.status_code(), 503);
            assert_eq!(api_error.raw_body(), body.as_bytes());
            assert!(api_error.headers().iter().any(|(name, value)| {
                name.eq_ignore_ascii_case("x-trace-id") && value == "typed-error-test"
            }));
            let typed_body = api_error
                .body()
                .expect("documented 5XX error body should decode");
            assert_eq!(typed_body.message, "temporarily unavailable");
            assert!(typed_body.retryable);
        }
        other => panic!("expected ServerError variant, got {other:?}"),
    }
}

#[test]
fn success_without_documented_body_does_not_drain_body() {
    let base_url = spawn_incomplete_success_body_server();
    let client = Client::new(&base_url);
    let api = ResourcesApi::new(&client);

    let response = api
        .check_no_success_body()
        .expect("success without a documented body must not read an incomplete body");

    assert_eq!(response.status_code, 200);
}
"##,
        ]
        .join("\n"),
    )
    .expect("generated runtime test should be written");
}

#[test]
fn reqwest_http_503_is_not_success_and_preserves_error_detail() {
    let fixture = read_fixture("valid/typed-error-responses.yaml");
    let files = generate_files(&RustReqwestCodeGenerator::new(empty_config()), &fixture)
        .expect("typed error fixture should generate");
    let temp = TempDir::new("typed-http-error-reqwest");
    write_generated_crate(&files, &temp.path);
    add_reqwest_runtime_test(&temp.path);

    let output = Command::new("cargo")
        .arg("test")
        .arg("--quiet")
        .current_dir(&temp.path)
        .output()
        .expect("generated crate cargo test should run");

    assert!(
        output.status.success(),
        "generated reqwest runtime test failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn ureq_http_503_is_not_transport_and_preserves_error_detail() {
    let fixture = read_fixture("valid/typed-error-responses.yaml");
    let files = generate_files(&RustUreqCodeGenerator::new(empty_config()), &fixture)
        .expect("typed error fixture should generate");
    let temp = TempDir::new("typed-http-error-ureq");
    write_generated_crate(&files, &temp.path);
    add_ureq_runtime_test(&temp.path);

    let output = Command::new("cargo")
        .arg("test")
        .arg("--quiet")
        .current_dir(&temp.path)
        .output()
        .expect("generated crate cargo test should run");

    assert!(
        output.status.success(),
        "generated ureq runtime test failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
