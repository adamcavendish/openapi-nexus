//! Runtime module generator for TypeScript using template-based approach

use crate::core::GeneratorError;
use crate::emission::TsFileCategory;
use crate::generator::file_generator::GeneratedFile;
use crate::templating::TemplatingEmitter;
use utoipa::openapi::OpenApi;

/// Runtime module generator using template-based approach
#[derive(Debug, Clone)]
pub struct RuntimeGenerator {
    templating: TemplatingEmitter,
}

impl RuntimeGenerator {
    /// Create a new runtime generator
    pub fn new(max_line_width: usize) -> Self {
        Self {
            templating: TemplatingEmitter::new(max_line_width),
        }
    }

    /// Generate runtime files using template-based approach
    pub fn generate_runtime_files(
        &self,
        openapi: &OpenApi,
    ) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();

        // Generate the main runtime file using template
        let runtime_content =
            self.templating
                .emit_runtime_file(openapi)
                .map_err(|e| GeneratorError::Generic {
                    message: format!("Failed to generate runtime file: {}", e),
                })?;

        files.push(GeneratedFile {
            filename: "runtime.ts".to_string(),
            content: runtime_content,
            file_category: TsFileCategory::Runtime,
        });

        Ok(files)
    }
}
