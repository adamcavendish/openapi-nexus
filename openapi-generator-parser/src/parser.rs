//! OpenAPI parser implementation

use snafu::prelude::*;
use std::path::Path;
use utoipa::openapi::OpenApi;

use crate::error::{Error, FileReadSnafu, JsonParseSnafu, YamlParseSnafu};

/// Parse an OpenAPI specification from a file
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<OpenApi, Error> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path)
        .context(FileReadSnafu { path: path.to_string_lossy().to_string() })?;

    parse_content(&content, path.extension().and_then(|ext| ext.to_str()))
}

/// Parse OpenAPI content from a string
pub fn parse_content(content: &str, file_extension: Option<&str>) -> Result<OpenApi, Error> {
    match file_extension {
        Some("json") => parse_json(content),
        Some("yaml") | Some("yml") => parse_yaml(content),
        Some(ext) => Err(Error::UnsupportedFormat { format: ext.to_string() }),
        None => {
            // Try JSON first, then YAML
            parse_json(content).or_else(|_| parse_yaml(content))
        }
    }
}

fn parse_json(content: &str) -> Result<OpenApi, Error> {
    serde_json::from_str(content)
        .context(JsonParseSnafu)
}

fn parse_yaml(content: &str) -> Result<OpenApi, Error> {
    serde_yaml::from_str(content)
        .context(YamlParseSnafu)
}
