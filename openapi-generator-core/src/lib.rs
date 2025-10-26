//! Core orchestration for OpenAPI code generation

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

use openapi_generator_parser::parse_file_with_validation;
use openapi_generator_rust::RustGenerator;
use openapi_generator_transforms::{
    TransformPipeline,
    passes::{NamingConvention, NamingConventionPass, ReferenceResolutionPass, ValidationPass},
};
use openapi_generator_typescript::TypeScriptGenerator;

pub mod error;
pub mod generator;

/// Main code generation orchestrator
pub struct CodeGenerator {
    transform_pipeline: TransformPipeline,
    typescript_generator: TypeScriptGenerator,
    rust_generator: RustGenerator,
}

impl CodeGenerator {
    /// Create a new code generator with default configuration
    pub fn new() -> Self {
        let mut pipeline = TransformPipeline::new()
            .add_pass(ValidationPass::new())
            .add_pass(ReferenceResolutionPass::new())
            .add_pass(NamingConventionPass {
                target_case: NamingConvention::CamelCase,
            });

        Self {
            transform_pipeline: pipeline,
            typescript_generator: TypeScriptGenerator::new(),
            rust_generator: RustGenerator::new(),
        }
    }

    /// Generate code from an OpenAPI specification file
    pub fn generate_from_file<P: AsRef<std::path::Path>>(
        &self,
        input_path: P,
        output_dir: P,
        languages: &[String],
    ) -> Result<(), error::Error> {
        tracing::info!(
            "Parsing OpenAPI specification from: {:?}",
            input_path.as_ref()
        );
        let mut openapi = parse_file_with_validation(input_path).context(error::ParseSnafu)?;

        tracing::info!("Applying transformations");
        self.transform_pipeline
            .transform(&mut openapi)
            .context(error::TransformSnafu)?;

        for language in languages {
            match language.as_str() {
                "typescript" | "ts" => {
                    tracing::info!("Generating TypeScript code");
                    let code = self.typescript_generator.generate(&openapi).map_err(|e| {
                        error::Error::Generate {
                            source: Box::new(e),
                        }
                    })?;
                    self.write_output(&output_dir, "typescript", &code)?;
                }
                "rust" => {
                    tracing::info!("Generating Rust code");
                    let code = self.rust_generator.generate(&openapi).map_err(|e| {
                        error::Error::Generate {
                            source: Box::new(e),
                        }
                    })?;
                    self.write_output(&output_dir, "rust", &code)?;
                }
                _ => {
                    return Err(error::Error::UnsupportedLanguage {
                        language: language.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    fn write_output<P: AsRef<std::path::Path>>(
        &self,
        output_dir: P,
        language: &str,
        code: &str,
    ) -> Result<(), error::Error> {
        let output_path = output_dir.as_ref().join(format!("{}.generated", language));
        std::fs::write(&output_path, code).context(error::WriteOutputSnafu {
            path: output_path.to_string_lossy().to_string(),
        })?;
        tracing::info!("Generated code written to: {:?}", output_path);
        Ok(())
    }
}

impl Default for CodeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
