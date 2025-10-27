//! Main TypeScript code generator

use std::collections::{HashMap, HashSet};

use utoipa::openapi::OpenApi;
use utoipa::openapi::path::Operation;

use super::api_class_generator::ApiClassGenerator;
use super::runtime_generator::RuntimeGenerator;
use super::schema_generator::SchemaGenerator;
use crate::config::GeneratorConfig;
use crate::core::GeneratorError;
use crate::emission::TypeScriptFileGenerator;
use crate::generator::schema_context::SchemaContext;
use openapi_generator_core::generator_registry::LanguageGenerator;
use openapi_generator_core::traits::code_generator::LanguageCodeGenerator;
use openapi_generator_core::traits::file_writer::{FileCategory, FileInfo, FileWriter};

/// Main TypeScript code generator
pub struct TypeScriptGenerator {
    schema_generator: SchemaGenerator,
    api_class_generator: ApiClassGenerator,
    runtime_generator: RuntimeGenerator,
    config: GeneratorConfig,
}

impl TypeScriptGenerator {
    /// Create a new TypeScript generator with default configuration
    pub fn new() -> Self {
        Self {
            schema_generator: SchemaGenerator::new(),
            api_class_generator: ApiClassGenerator::new(),
            runtime_generator: RuntimeGenerator::new(),
            config: GeneratorConfig::default(),
        }
    }

    /// Create a new TypeScript generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self {
            schema_generator: SchemaGenerator::new(),
            api_class_generator: ApiClassGenerator::new(),
            runtime_generator: RuntimeGenerator::new(),
            config,
        }
    }

    /// Generate multiple TypeScript files from OpenAPI specification
    pub fn generate_files(&self, openapi: &OpenApi) -> Result<Vec<FileInfo>, GeneratorError> {
        let mut file_infos = Vec::new();
        let mut schemas = HashMap::new();

        // Generate interfaces and types from schemas
        if let Some(components) = &openapi.components {
            // Create schema context for reference resolution
            let mut visited = HashSet::new();
            let mut context = SchemaContext::new(&components.schemas, &mut visited);

            for (name, schema_ref) in &components.schemas {
                match self
                    .schema_generator
                    .schema_to_ts_node(name, schema_ref, &mut context)
                {
                    Ok(node) => {
                        schemas.insert(name.clone(), node);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to convert schema {}: {}", name, e);
                    }
                }
            }
        }

        // Generate API classes per tag
        let tag_operations = self.collect_operations_by_tag(openapi);

        // Generate API class for each tag
        for (tag, operations) in tag_operations {
            let api_class =
                self.api_class_generator
                    .generate_api_class(&tag, &operations, openapi)?;
            let class_name = format!("{}Api", self.to_pascal_case(&tag));
            schemas.insert(class_name, api_class);
        }

        // Generate files using file generator
        let file_generator = TypeScriptFileGenerator::with_package_config(
            self.config.file_config.clone(),
            self.config.package_config.clone(),
        );
        let generated_files = file_generator
            .generate_files(&schemas, openapi)
            .map_err(|e| GeneratorError::Generic {
                message: format!("File generation error: {}", e),
            })?;

        // Convert GeneratedFile to FileInfo with proper categories
        for file in generated_files {
            let file_info = FileInfo::new(
                file.filename,
                file.content,
                FileCategory::from(file.file_category),
            );
            file_infos.push(file_info);
        }

        // Generate runtime files
        let runtime_files = self.runtime_generator.generate_runtime_files()?;
        for file in runtime_files {
            // Convert GeneratedFile to FileInfo
            let file_info = FileInfo::new(
                file.filename,
                file.content,
                FileCategory::from(file.file_category),
            );
            file_infos.push(file_info);
        }

        Ok(file_infos)
    }

    /// Collect all operations grouped by their tags
    fn collect_operations_by_tag(
        &self,
        openapi: &OpenApi,
    ) -> HashMap<String, Vec<(String, String, Operation)>> {
        let mut tag_operations = HashMap::new();
        let default_tags = vec!["default".to_string()];

        for (path, path_item) in &openapi.paths.paths {
            // Define HTTP methods and their corresponding operations
            let methods = [
                ("GET", path_item.get.as_ref()),
                ("POST", path_item.post.as_ref()),
                ("PUT", path_item.put.as_ref()),
                ("DELETE", path_item.delete.as_ref()),
                ("PATCH", path_item.patch.as_ref()),
                ("OPTIONS", path_item.options.as_ref()),
                ("HEAD", path_item.head.as_ref()),
            ];

            for (method, operation_opt) in methods {
                if let Some(operation) = operation_opt {
                    let tags = operation.tags.as_ref().unwrap_or(&default_tags);
                    for tag in tags {
                        tag_operations
                            .entry(tag.clone())
                            .or_insert_with(Vec::new)
                            .push((path.clone(), method.to_string(), operation.clone()));
                    }
                }
            }
        }

        tag_operations
    }

    /// Convert to PascalCase efficiently
    fn to_pascal_case(&self, s: &str) -> String {
        if s.is_empty() {
            return String::new();
        }

        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars();

        // Handle first character
        if let Some(first) = chars.next() {
            result.push(first.to_uppercase().next().unwrap());
        }

        // Handle remaining characters
        for c in chars {
            if c.is_alphanumeric() {
                result.push(c.to_lowercase().next().unwrap());
            }
        }

        result
    }
}

impl Default for TypeScriptGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl LanguageGenerator for TypeScriptGenerator {}

impl LanguageCodeGenerator for TypeScriptGenerator {
    fn generate(
        &self,
        openapi: &OpenApi,
    ) -> Result<Vec<FileInfo>, Box<dyn std::error::Error + Send + Sync>> {
        self.generate_files(openapi)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)
    }
}

impl FileWriter for TypeScriptGenerator {
    fn write_files(
        &self,
        output_dir: &std::path::Path,
        files: &[FileInfo],
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Use the default implementation that organizes files by category
        self.write_files_by_category(output_dir, files)
    }
}
