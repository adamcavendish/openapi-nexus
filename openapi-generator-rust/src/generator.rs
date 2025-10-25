//! Rust code generator

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

use crate::ast::*;
use crate::emitter::RustEmitter;

/// Error type for Rust generation
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GeneratorError {
    #[snafu(display("Generator error: {}", message))]
    Generic { message: String },
}

/// Rust code generator
pub struct RustGenerator {
    emitter: RustEmitter,
}

impl RustGenerator {
    /// Create a new Rust generator
    pub fn new() -> Self {
        Self {
            emitter: RustEmitter,
        }
    }

    /// Generate Rust code from OpenAPI specification
    pub fn generate(&self, openapi: &OpenApi) -> Result<String, GeneratorError> {
        // TODO: Convert OpenAPI to Rust AST
        let nodes = Vec::new();

        // Emit the code
        self.emitter
            .emit(&nodes)
            .map_err(|e| GeneratorError::Generic {
                message: e.to_string(),
            })
    }
}
