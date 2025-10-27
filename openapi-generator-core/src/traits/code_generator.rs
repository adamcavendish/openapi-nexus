//! Trait for language-specific code generators

use utoipa::openapi::OpenApi;
use crate::traits::file_writer::FileInfo;

/// Trait for language-specific code generators
pub trait LanguageCodeGenerator {
    /// Generate files from an OpenAPI specification
    fn generate(&self, openapi: &OpenApi) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + Send + Sync>>;
}
