//! Error types for OpenAPI parsing

use snafu::prelude::*;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to read file '{}': {}", path, source))]
    FileRead { path: String, source: std::io::Error },

    #[snafu(display("Failed to parse JSON: {}", source))]
    JsonParse { source: serde_json::Error },

    #[snafu(display("Failed to parse YAML: {}", source))]
    YamlParse { source: serde_yaml::Error },

    #[snafu(display("Unsupported file format: {}", format))]
    UnsupportedFormat { format: String },

    #[snafu(display("Invalid OpenAPI specification: {}", message))]
    InvalidSpec { message: String },
}
