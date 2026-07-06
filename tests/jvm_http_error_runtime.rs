use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use openapi_nexus::generators::java::okhttp::JavaOkhttpCodeGenerator;
use openapi_nexus::generators::kotlin::okhttp::KotlinOkhttpCodeGenerator;
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
        fs::create_dir_all(&path).expect("temporary project directory should be created");
        Self { path }
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

struct LockDir {
    path: PathBuf,
}

impl LockDir {
    fn acquire(name: &str) -> Self {
        let path = std::env::temp_dir().join(format!("openapi-nexus-{name}.lock"));
        let deadline = Instant::now() + Duration::from_secs(300);
        loop {
            match fs::create_dir(&path) {
                Ok(()) => return Self { path },
                Err(err) if err.kind() == ErrorKind::AlreadyExists && Instant::now() < deadline => {
                    thread::sleep(Duration::from_millis(250));
                }
                Err(err) => panic!("failed to acquire {name} lock at {}: {err}", path.display()),
            }
        }
    }
}

impl Drop for LockDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn write_generated_project(files: &HashMap<String, String>, root: &Path) {
    for (relative_path, contents) in files {
        let path = root.join(relative_path);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("generated parent directory should be created");
        }
        fs::write(path, contents).expect("generated file should be written");
    }
}

const CHARSET_SMOKE_JAVA: &str = r#"
import com.example.sdk.apis.CreateResourceServerError;
import java.nio.charset.StandardCharsets;
import okhttp3.Headers;

public final class CharsetSmoke {
    public static void main(String[] args) {
        byte[] raw = "{\"message\":\"caf\u00e9\",\"retryable\":true}".getBytes(StandardCharsets.ISO_8859_1);
        Headers headers = Headers.of("Content-Type", "application/json; charset=iso-8859-1");
        CreateResourceServerError detail = new CreateResourceServerError(503, headers, raw);
        String message = detail.body().getMessage();
        if (!"caf\u00e9".equals(message)) {
            throw new AssertionError("expected charset-aware message, got: " + message);
        }
    }
}
"#;

fn add_java_charset_smoke(root: &Path) {
    let smoke_dir = root.join("smoke");
    fs::create_dir_all(&smoke_dir).expect("smoke source directory should be created");
    fs::write(smoke_dir.join("CharsetSmoke.java"), CHARSET_SMOKE_JAVA)
        .expect("Java smoke test should be written");

    let build_gradle = root.join("build.gradle");
    let mut build = fs::read_to_string(&build_gradle).expect("build.gradle should be readable");
    build.push_str(
        r#"

sourceSets {
    smoke {
        java {
            srcDirs("smoke")
        }
        compileClasspath += sourceSets.main.output + sourceSets.main.compileClasspath
        runtimeClasspath += output + compileClasspath
    }
}

tasks.register("charsetSmoke", JavaExec) {
    dependsOn("compileSmokeJava")
    classpath = sourceSets.smoke.runtimeClasspath
    mainClass = "CharsetSmoke"
}
"#,
    );
    fs::write(build_gradle, build).expect("build.gradle should be updated");
}

fn add_kotlin_charset_smoke(root: &Path) {
    let smoke_dir = root.join("smoke");
    fs::create_dir_all(&smoke_dir).expect("smoke source directory should be created");
    fs::write(smoke_dir.join("CharsetSmoke.java"), CHARSET_SMOKE_JAVA)
        .expect("Kotlin smoke test should be written");

    let build_gradle = root.join("build.gradle.kts");
    let mut build = fs::read_to_string(&build_gradle).expect("build.gradle.kts should be readable");
    build.push_str(
        r#"

sourceSets {
    create("smoke") {
        java.srcDirs("smoke")
        compileClasspath += sourceSets["main"].output + sourceSets["main"].compileClasspath
        runtimeClasspath += output + compileClasspath
    }
}

tasks.register<JavaExec>("charsetSmoke") {
    dependsOn("compileSmokeJava")
    classpath = sourceSets["smoke"].runtimeClasspath
    mainClass.set("CharsetSmoke")
}
"#,
    );
    fs::write(build_gradle, build).expect("build.gradle.kts should be updated");
}

fn gradle_charset_smoke(root: &Path) -> std::process::Output {
    let _lock = LockDir::acquire("gradle-charset-smoke");
    let mut cmd = Command::new("gradle");
    let homebrew_jdk21 =
        Path::new("/opt/homebrew/Cellar/openjdk@21/21.0.11/libexec/openjdk.jdk/Contents/Home");
    if homebrew_jdk21.exists() {
        cmd.env("JAVA_HOME", homebrew_jdk21);
    }
    cmd.arg("--no-daemon")
        .arg("--max-workers=1")
        .arg("--quiet")
        .arg("charsetSmoke")
        .env("GRADLE_OPTS", "-Dorg.gradle.daemon=false")
        .current_dir(root)
        .output()
        .expect("generated JVM charset smoke should run")
}

#[test]
fn java_error_detail_body_honors_response_charset() {
    let fixture = read_fixture("valid/typed-error-responses.yaml");
    let files = generate_files(&JavaOkhttpCodeGenerator::new(empty_config()), &fixture)
        .expect("typed error fixture should generate Java");
    let temp = TempDir::new("typed-http-error-java");
    write_generated_project(&files, &temp.path);
    add_java_charset_smoke(&temp.path);

    let output = gradle_charset_smoke(&temp.path);
    assert!(
        output.status.success(),
        "generated Java charset smoke failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn kotlin_error_detail_body_honors_response_charset() {
    let fixture = read_fixture("valid/typed-error-responses.yaml");
    let files = generate_files(&KotlinOkhttpCodeGenerator::new(empty_config()), &fixture)
        .expect("typed error fixture should generate Kotlin");
    let temp = TempDir::new("typed-http-error-kotlin");
    write_generated_project(&files, &temp.path);
    add_kotlin_charset_smoke(&temp.path);

    let output = gradle_charset_smoke(&temp.path);
    assert!(
        output.status.success(),
        "generated Kotlin charset smoke failed\nstatus: {}\nstdout:\n{}\nstderr:\n{}",
        output.status,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
