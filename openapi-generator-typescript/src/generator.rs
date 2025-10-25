//! TypeScript code generator

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

use crate::ast::*;
use crate::emitter::TypeScriptEmitter;

/// Error type for TypeScript generation
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GeneratorError {
    #[snafu(display("Generator error: {}", message))]
    Generic { message: String },
}

/// TypeScript code generator
pub struct TypeScriptGenerator {
    emitter: TypeScriptEmitter,
}

impl TypeScriptGenerator {
    /// Create a new TypeScript generator
    pub fn new() -> Self {
        Self {
            emitter: TypeScriptEmitter,
        }
    }

    /// Generate TypeScript code from OpenAPI specification
    pub fn generate(&self, openapi: &OpenApi) -> Result<String, GeneratorError> {
        // TODO: Convert OpenAPI to TypeScript AST
        let nodes = Vec::new();

        // Emit the code
        self.emitter
            .emit(&nodes)
            .map_err(|e| GeneratorError::Generic {
                message: e.to_string(),
            })
    }
}
