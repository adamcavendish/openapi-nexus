//! Error types for TypeScript emission

use snafu::Snafu;

/// Error type for TypeScript emission
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum EmitError {
    #[snafu(display("Emit error: {}", message))]
    Generic { message: String },
    #[snafu(display("Template error: {}", message))]
    TemplateError { message: String },
    #[snafu(display("Import resolution error: {}", message))]
    ImportResolution { message: String },
}
