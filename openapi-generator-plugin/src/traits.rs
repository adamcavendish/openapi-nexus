//! Plugin system traits for OpenAPI code generation

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

/// Error type for plugin operations
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum PluginError {
    #[snafu(display("Plugin error: {}", message))]
    Generic { message: String },
}

/// Trait for language generators
pub trait LanguageGenerator {
    /// Generate code for the given OpenAPI specification
    fn generate(&self, openapi: &OpenApi) -> Result<Vec<GeneratedFile>, PluginError>;
}

/// Trait for transformation passes
pub trait TransformPass {
    /// Apply the transformation to the OpenAPI specification
    fn transform(&self, openapi: &mut OpenApi) -> Result<(), PluginError>;
}

/// Trait for code emitters
pub trait Emitter {
    /// Emit the generated files
    fn emit(&self, files: &[GeneratedFile]) -> Result<(), PluginError>;
}

/// Represents a generated file
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub path: String,
    pub content: String,
}
