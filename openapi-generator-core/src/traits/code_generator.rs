//! Trait for language-specific code generators

use crate::traits::file_writer::FileInfo;
use utoipa::openapi::OpenApi;

/// Trait for language-specific code generators
pub trait LanguageCodeGenerator {
    /// Generate files from an OpenAPI specification
    fn generate(
        &self,
        openapi: &OpenApi,
    ) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + Send + Sync>>;
}
