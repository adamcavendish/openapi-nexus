//! OpenAPI parser implementation

use snafu::prelude::*;
use std::path::Path;
use utoipa::openapi::OpenApi;

use crate::error::{Error, FileReadSnafu, JsonParseSnafu, YamlParseSnafu};

/// Parse an OpenAPI specification from a file
pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<OpenApi, Error> {
    let path = path.as_ref();
    let content = std::fs::read_to_string(path).context(FileReadSnafu {
        path: path.to_string_lossy().to_string(),
    })?;

    parse_content(&content, path.extension().and_then(|ext| ext.to_str()))
}

/// Parse OpenAPI content from a string
pub fn parse_content(content: &str, file_extension: Option<&str>) -> Result<OpenApi, Error> {
    match file_extension {
        Some("json") => parse_json(content),
        Some("yaml") | Some("yml") => parse_yaml(content),
        Some(ext) => Err(Error::UnsupportedFormat {
            format: ext.to_string(),
        }),
        None => {
            // Try JSON first, then YAML
            parse_json(content).or_else(|_| parse_yaml(content))
        }
    }
}

fn parse_json(content: &str) -> Result<OpenApi, Error> {
    serde_json::from_str(content).context(JsonParseSnafu)
}

fn parse_yaml(content: &str) -> Result<OpenApi, Error> {
    serde_norway::from_str(content).context(YamlParseSnafu)
}

/// Parse an OpenAPI specification from a file with validation
pub fn parse_file_with_validation<P: AsRef<Path>>(path: P) -> Result<OpenApi, Error> {
    let openapi = parse_file(path)?;
    validate_openapi(&openapi)?;
    Ok(openapi)
}

/// Parse OpenAPI content from a string with validation
pub fn parse_content_with_validation(content: &str, file_extension: Option<&str>) -> Result<OpenApi, Error> {
    let openapi = parse_content(content, file_extension)?;
    validate_openapi(&openapi)?;
    Ok(openapi)
}

/// Validate an OpenAPI specification
pub fn validate_openapi(openapi: &OpenApi) -> Result<(), Error> {
    // Basic validation checks
    if openapi.info.title.is_empty() {
        return Err(Error::ValidationError {
            message: "OpenAPI info.title is required".to_string(),
        });
    }

    if openapi.info.version.is_empty() {
        return Err(Error::ValidationError {
            message: "OpenAPI info.version is required".to_string(),
        });
    }

    // Check if there are any paths defined
    if openapi.paths.paths.is_empty() {
        return Err(Error::ValidationError {
            message: "OpenAPI must have at least one path defined".to_string(),
        });
    }

    Ok(())
}
