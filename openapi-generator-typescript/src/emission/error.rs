//! Error types for TypeScript emission

use snafu::Snafu;

/// Error type for TypeScript emission
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum EmitError {
    #[snafu(display("Emit error: {}", message))]
    Generic { message: String },
}
