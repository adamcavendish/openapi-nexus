//! Hardcoded Rust runtime source files.
//!
//! The runtime provides a reqwest-based `Client`, `Error` enum, and `Auth`
//! trait. These ship as-is in every generated SDK.

use crate::codegen::traits::file_writer::FileInfo;
use crate::generators::rust::common::runtime::render_api_call_error;
use sigil_stitch::type_name::TypeName;

const CLIENT_RS: &str = include_str!("runtime/client.rs.txt");
const ERROR_RS: &str = include_str!("runtime/error.rs.txt");
const AUTH_RS: &str = include_str!("runtime/auth.rs.txt");
const MOD_RS: &str = include_str!("runtime/mod.rs.txt");
const UPLOAD_FILE_RS: &str = include_str!("runtime/upload_file.rs.txt");

/// Returns runtime files ready to write.
pub fn runtime_files(
    header: &str,
    include_api_call_error: bool,
    include_upload_file: bool,
) -> Vec<FileInfo> {
    let mut mod_rs = MOD_RS.to_string();
    let mut error_rs = ERROR_RS.to_string();
    if include_api_call_error {
        error_rs.push('\n');
        error_rs.push_str(
            &render_api_call_error(TypeName::qualified("reqwest::header", "HeaderMap"))
                .expect("reqwest ApiCallError runtime renders"),
        );
    }
    let mut files = vec![
        FileInfo::runtime("client.rs".to_string(), with_header(header, CLIENT_RS)),
        FileInfo::runtime("error.rs".to_string(), with_header(header, &error_rs)),
        FileInfo::runtime("auth.rs".to_string(), with_header(header, AUTH_RS)),
    ];
    if include_upload_file {
        mod_rs.push_str(
            "mod upload_file;\npub use upload_file::{multipart_header_value, UploadFile};\n",
        );
        files.push(FileInfo::runtime(
            "upload_file.rs".to_string(),
            with_header(header, UPLOAD_FILE_RS),
        ));
    }
    files.push(FileInfo::runtime(
        "mod.rs".to_string(),
        with_header(header, &mod_rs),
    ));
    files
}

fn with_header(header: &str, body: &str) -> String {
    let mut out = String::with_capacity(header.len() + body.len());
    out.push_str(header);
    out.push_str(body);
    out
}
