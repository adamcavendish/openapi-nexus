//! File generation and organization for TypeScript code

use std::collections::HashMap;
use std::slice;

use heck::{ToKebabCase as _, ToLowerCamelCase as _, ToPascalCase as _, ToSnakeCase as _};
use utoipa::openapi::OpenApi;

use crate::ast::TsNode;
use crate::config::{FileConfig, NamingConvention, PackageConfig};
use crate::emission::ts_file_category::TsFileCategory;
use crate::emission::ts_language_emitter::TsLanguageEmitter;
use crate::generator::package_files_generator::PackageFilesGenerator;
use openapi_nexus_core::traits::EmissionContext;

/// Error type for file generation
#[derive(Debug)]
pub enum FileGeneratorError {
    EmitError { filename: String, source: String },
    DirectoryError { path: String, source: String },
    WriteError { path: String, source: String },
}

impl std::fmt::Display for FileGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileGeneratorError::EmitError { filename, source } => {
                write!(f, "Failed to emit file {}: {}", filename, source)
            }
            FileGeneratorError::DirectoryError { path, source } => {
                write!(f, "Failed to create directory {}: {}", path, source)
            }
            FileGeneratorError::WriteError { path, source } => {
                write!(f, "Failed to write file {}: {}", path, source)
            }
        }
    }
}

impl std::error::Error for FileGeneratorError {}

/// File generator for TypeScript code
#[derive(Debug, Clone)]
pub struct TypeScriptFileGenerator {
    emitter: TsLanguageEmitter,
    file_config: FileConfig,
    package_config: PackageConfig,
}

impl TypeScriptFileGenerator {
    /// Create a new file generator
    pub fn new(file_config: FileConfig, package_config: PackageConfig) -> Self {
        Self {
            emitter: TsLanguageEmitter::new(file_config.max_line_width),
            file_config,
            package_config,
        }
    }

    /// Create a new file generator with package configuration
    pub fn with_package_config(config: FileConfig, package_config: PackageConfig) -> Self {
        Self {
            emitter: TsLanguageEmitter::new(config.max_line_width),
            file_config: config,
            package_config,
        }
    }

    /// Create an emission context with the configured max line width
    fn create_emission_context(&self) -> EmissionContext {
        EmissionContext {
            indent_level: 0,
            max_line_width: self.file_config.max_line_width,
        }
    }

    /// Generate package files (package.json, tsconfig.json, etc.)
    pub fn generate_package_files(
        &self,
        openapi: &OpenApi,
    ) -> Result<Vec<GeneratedFile>, FileGeneratorError> {
        let mut files = Vec::new();

        if self.package_config.generate_package {
            let package_generator = PackageFilesGenerator::new(self.package_config.clone());

            // Generate package.json
            files.push(package_generator.generate_package_json(openapi));

            // Generate tsconfig.json
            files.push(package_generator.generate_tsconfig(openapi));

            // Generate tsconfig.esm.json if configured
            if self.package_config.generate_esm_config {
                files.push(package_generator.generate_tsconfig_esm(openapi));
            }

            // Generate README.md
            files.push(package_generator.generate_readme(openapi));
        }

        Ok(files)
    }

    /// Generate files for all schemas with proper directory structure
    pub fn generate_files(
        &self,
        schemas: &HashMap<String, TsNode>,
        openapi: &OpenApi,
    ) -> Result<Vec<GeneratedFile>, FileGeneratorError> {
        let mut files = Vec::new();

        // Separate API classes from other schemas
        let mut api_classes = HashMap::new();
        let mut other_schemas = HashMap::new();

        for (name, node) in schemas {
            if name.ends_with("Api") {
                api_classes.insert(name.clone(), node.clone());
            } else {
                other_schemas.insert(name.clone(), node.clone());
            }
        }

        // Create schema-to-file mapping for import resolution
        let _schema_to_file_map = self.create_schema_to_file_map(&api_classes, &other_schemas);

        // Generate models files (no directory prefix - handled by core)
        for (name, node) in &other_schemas {
            let filename = self.generate_filename(name);
            let content = self
                .emitter
                .emit_with_context(slice::from_ref(node), &self.create_emission_context())
                .map_err(|e| FileGeneratorError::EmitError {
                    filename: filename.clone(),
                    source: format!("{}", e),
                })?;

            files.push(GeneratedFile {
                filename,
                content,
                file_category: TsFileCategory::Models,
            });
        }

        // Generate API classes (no directory prefix - handled by core)
        for (name, node) in &api_classes {
            let filename = self.generate_filename(name);

            // Use the new emit_with_context method for automatic import generation
            let content = self
                .emitter
                .emit_with_context(slice::from_ref(node), &self.create_emission_context())
                .map_err(|e| FileGeneratorError::EmitError {
                    filename: filename.clone(),
                    source: format!("{}", e),
                })?;

            files.push(GeneratedFile {
                filename,
                content,
                file_category: TsFileCategory::Api,
            });
        }

        // Generate main index.ts
        files.push(self.generate_main_index_file(&api_classes, &other_schemas)?);

        // Generate package files if configured
        if self.package_config.generate_package {
            let package_files = self.generate_package_files(openapi)?;
            files.extend(package_files);
        }

        Ok(files)
    }

    /// Generate main index file
    fn generate_main_index_file(
        &self,
        api_classes: &HashMap<String, TsNode>,
        schemas: &HashMap<String, TsNode>,
    ) -> Result<GeneratedFile, FileGeneratorError> {
        let mut exports = Vec::new();

        // Export runtime files from runtime directory
        exports.push("export * from './runtime/core';".to_string());
        exports.push("export * from './runtime/config';".to_string());
        exports.push("export * from './runtime/api';".to_string());

        // Export all models from models directory
        let mut sorted_names: Vec<&String> = schemas.keys().collect();
        sorted_names.sort();
        for name in sorted_names {
            let filename = self.generate_filename(name);
            let import_name = filename.trim_end_matches(".ts");
            exports.push(format!("export * from './models/{}';", import_name));
        }

        // Export all API classes from apis directory
        let mut sorted_api_names: Vec<&String> = api_classes.keys().collect();
        sorted_api_names.sort();
        for name in sorted_api_names {
            let filename = self.generate_filename(name);
            let import_name = filename.trim_end_matches(".ts");
            exports.push(format!("export * from './apis/{}';", import_name));
        }

        let content = exports.join("\n");

        Ok(GeneratedFile {
            filename: "index.ts".to_string(),
            content,
            file_category: TsFileCategory::Index,
        })
    }

    /// Generate filename based on naming convention
    fn generate_filename(&self, name: &str) -> String {
        let base_name = match self.file_config.naming_convention {
            NamingConvention::CamelCase => name.to_lower_camel_case(),
            NamingConvention::KebabCase => name.to_kebab_case(),
            NamingConvention::SnakeCase => name.to_snake_case(),
            NamingConvention::PascalCase => name.to_pascal_case(),
        };

        format!("{}.ts", base_name)
    }

    /// Create a mapping from schema names to their corresponding filenames
    fn create_schema_to_file_map(
        &self,
        api_classes: &HashMap<String, TsNode>,
        other_schemas: &HashMap<String, TsNode>,
    ) -> HashMap<String, String> {
        let mut map = HashMap::new();

        // Add API classes
        for name in api_classes.keys() {
            let filename = self.generate_filename(name);
            map.insert(name.clone(), filename);
        }

        // Add other schemas
        for name in other_schemas.keys() {
            let filename = self.generate_filename(name);
            map.insert(name.clone(), filename);
        }

        map
    }
}

/// Generated file information
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub filename: String,
    pub content: String,
    pub file_category: TsFileCategory,
}
