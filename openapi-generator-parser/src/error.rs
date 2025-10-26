//! Error types for OpenAPI parsing

use snafu::prelude::*;
use std::path::PathBuf;

/// Source location information for error reporting
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub openapi_path: Option<String>,
}

impl SourceLocation {
    pub fn new() -> Self {
        Self {
            file_path: None,
            line: None,
            column: None,
            openapi_path: None,
        }
    }

    pub fn with_file_path(mut self, path: PathBuf) -> Self {
        self.file_path = Some(path);
        self
    }

    pub fn with_line_column(mut self, line: u32, column: u32) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }

    pub fn with_openapi_path(mut self, path: String) -> Self {
        self.openapi_path = Some(path);
        self
    }
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse warning for non-fatal issues
#[derive(Debug, Clone)]
pub struct ParseWarning {
    pub message: String,
    pub location: SourceLocation,
}

impl ParseWarning {
    pub fn new(message: String, location: SourceLocation) -> Self {
        Self { message, location }
    }
}

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to read file '{}': {}", path, source))]
    FileRead {
        path: String,
        source: std::io::Error,
    },

    #[snafu(display("Failed to parse JSON: {}", source))]
    JsonParse { source: serde_json::Error },

    #[snafu(display("Failed to parse YAML: {}", source))]
    YamlParse { source: serde_norway::Error },

    #[snafu(display("Unsupported file format: {}", format))]
    UnsupportedFormat { format: String },

    #[snafu(display("Invalid OpenAPI specification: {}", message))]
    InvalidSpec { message: String },

    #[snafu(display("Validation error: {}", message))]
    ValidationError { message: String },

    #[snafu(display(
        "Unsupported OpenAPI version: {}. Only OpenAPI 3.1.x is supported",
        version
    ))]
    UnsupportedVersion { version: String },

    #[snafu(display("Circular reference detected: {}", reference))]
    CircularReference { reference: String },

    #[snafu(display("External reference not supported: {}", reference))]
    ExternalReference { reference: String },

    #[snafu(display("Missing required field: {}", field))]
    MissingRequiredField { field: String },
}
