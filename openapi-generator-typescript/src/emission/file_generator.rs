//! File generation and organization for TypeScript code

use std::collections::HashMap;

use utoipa::openapi::OpenApi;

use crate::ast::TsNode;
use crate::config::{FileConfig, NamingConvention, PackageConfig};
use crate::emission::emitter::TypeScriptEmitter;
use crate::emission::file_category::TypeScriptFileCategory;
use crate::generator::package_files_generator::PackageFilesGenerator;

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
pub struct TypeScriptFileGenerator {
    emitter: TypeScriptEmitter,
    config: FileConfig,
    package_config: PackageConfig,
}

impl TypeScriptFileGenerator {
    /// Create a new file generator
    pub fn new(config: FileConfig) -> Self {
        Self {
            emitter: TypeScriptEmitter::new(),
            config,
            package_config: PackageConfig::default(),
        }
    }

    /// Create a new file generator with package configuration
    pub fn with_package_config(config: FileConfig, package_config: PackageConfig) -> Self {
        Self {
            emitter: TypeScriptEmitter::new(),
            config,
            package_config,
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

        // Generate models files (no directory prefix - handled by core)
        for (name, node) in &other_schemas {
            let filename = self.generate_filename(name);
            let content = self.emitter.emit(std::slice::from_ref(node)).map_err(|e| {
                FileGeneratorError::EmitError {
                    filename: filename.clone(),
                    source: format!("{}", e),
                }
            })?;

            files.push(GeneratedFile {
                filename,
                content,
                file_category: TypeScriptFileCategory::Models,
            });
        }

        // Generate API classes (no directory prefix - handled by core)
        for (name, node) in &api_classes {
            let filename = self.generate_filename(name);
            
            // The main emitter will handle runtime imports automatically
            let content = self.emitter.emit(&[node.clone()]).map_err(|e| {
                FileGeneratorError::EmitError {
                    filename: filename.clone(),
                    source: format!("{}", e),
                }
            })?;

            files.push(GeneratedFile {
                filename,
                content,
                file_category: TypeScriptFileCategory::Api,
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
            file_category: TypeScriptFileCategory::Index,
        })
    }

    /// Generate filename based on naming convention
    fn generate_filename(&self, name: &str) -> String {
        let base_name = match self.config.naming_convention {
            NamingConvention::CamelCase => self.to_camel_case(name),
            NamingConvention::KebabCase => self.to_kebab_case(name),
            NamingConvention::SnakeCase => self.to_snake_case(name),
            NamingConvention::PascalCase => self.to_pascal_case(name),
        };

        format!("{}.ts", base_name)
    }

    /// Convert to camelCase
    fn to_camel_case(&self, s: &str) -> String {
        let pascal = self.to_pascal_case(s);
        if pascal.is_empty() {
            return pascal;
        }

        let mut chars = pascal.chars();
        let first = chars.next().unwrap().to_lowercase().next().unwrap();
        format!("{}{}", first, chars.as_str())
    }

    /// Convert to kebab-case
    fn to_kebab_case(&self, s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
        }
        result
    }

    /// Convert to snake_case
    fn to_snake_case(&self, s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        }
        result
    }

    /// Convert a string to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        // If the string is already PascalCase, return it as-is
        if s.chars().next().is_some_and(|c| c.is_uppercase()) {
            return s.to_string();
        }

        // Convert first character to uppercase
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
}

/// Generated file information
#[derive(Debug, Clone)]
pub struct GeneratedFile {
    pub filename: String,
    pub content: String,
    pub file_category: TypeScriptFileCategory,
}
