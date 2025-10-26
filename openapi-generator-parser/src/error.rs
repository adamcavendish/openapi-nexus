//! Error types for OpenAPI parsing

pub use openapi_generator_common::{ParseWarning, SourceLocation};
use snafu::prelude::*;

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
