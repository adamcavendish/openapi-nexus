//! Error types for the core orchestration

use snafu::Snafu;

#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum Error {
    #[snafu(display("Failed to parse OpenAPI specification: {}", source))]
    Parse {
        source: openapi_nexus_parser::error::Error,
    },

    #[snafu(display("Failed to transform OpenAPI specification: {}", source))]
    Transform {
        source: openapi_nexus_transforms::TransformError,
    },

    #[snafu(display("Failed to generate code: {}", source))]
    Generate {
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[snafu(display("Unsupported language: {}", language))]
    UnsupportedLanguage { language: String },

    #[snafu(display("Failed to write output to '{}': {}", path, source))]
    WriteOutput {
        path: String,
        source: std::io::Error,
    },

    #[snafu(display("Generator not found for language: {}", language))]
    GeneratorNotFound { language: String },
}
