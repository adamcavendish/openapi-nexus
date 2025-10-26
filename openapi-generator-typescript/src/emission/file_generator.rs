//! File generation and organization for TypeScript code

use std::collections::HashMap;
use std::path::PathBuf;

use crate::ast::TsNode;
use crate::emission::emitter::TypeScriptEmitter;
use crate::imports::ImportGenerator;

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

/// Configuration for file organization
#[derive(Debug, Clone)]
pub struct FileConfig {
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Whether to use granular files (one per schema) or grouped files
    pub granular_files: bool,
    /// File naming convention
    pub naming_convention: NamingConvention,
}

/// File naming conventions
#[derive(Debug, Clone)]
pub enum NamingConvention {
    /// Use camelCase for file names
    CamelCase,
    /// Use kebab-case for file names
    KebabCase,
    /// Use snake_case for file names
    SnakeCase,
}

/// File generator for TypeScript code
pub struct FileGenerator {
    emitter: TypeScriptEmitter,
    import_generator: ImportGenerator,
    config: FileConfig,
}

impl FileGenerator {
    /// Create a new file generator
    pub fn new(config: FileConfig) -> Self {
        Self {
            emitter: TypeScriptEmitter,
            import_generator: ImportGenerator::new(),
            config,
        }
    }

    /// Generate files for all schemas
    pub fn generate_files(
        &self,
        schemas: &HashMap<String, TsNode>,
    ) -> Result<Vec<GeneratedFile>, FileGeneratorError> {
        let mut files = Vec::new();

        if self.config.granular_files {
            // Create schema name to filename mapping
            let schema_to_filename: HashMap<String, String> = schemas
                .keys()
                .map(|name| {
                    // Convert to PascalCase for the mapping
                    let pascal_name = self.to_pascal_case(name);
                    (pascal_name, self.generate_filename(name))
                })
                .collect();

            // Generate one file per schema
            for (name, node) in schemas {
                let filename = self.generate_filename(name);
                
                // Extract dependencies for this node
                let dependencies = self.import_generator.extract_dependencies(node);
                
                // Generate import statements
                let imports = self.import_generator.generate_imports(
                    &filename,
                    &dependencies,
                    &schema_to_filename,
                );
                
                // Generate the main content
                let main_content = self.emitter.emit(std::slice::from_ref(node)).map_err(|e| {
                    FileGeneratorError::EmitError {
                        filename: filename.clone(),
                        source: format!("{}", e),
                    }
                })?;
                
                // Combine imports and main content
                let content = if imports.is_empty() {
                    main_content
                } else {
                    format!("{}\n{}", imports.trim_end(), main_content)
                };

                files.push(GeneratedFile {
                    filename,
                    content,
                    file_type: FileType::Schema,
                });
            }
        } else {
            // Generate grouped files
            files.extend(self.generate_grouped_files(schemas)?);
        }

        // Generate index file
        files.push(self.generate_index_file(schemas)?);

        Ok(files)
    }

    /// Generate grouped files (models.ts, api.ts, etc.)
    fn generate_grouped_files(
        &self,
        schemas: &HashMap<String, TsNode>,
    ) -> Result<Vec<GeneratedFile>, FileGeneratorError> {
        let mut files = Vec::new();

        // Separate schemas and API client
        let mut schema_nodes = Vec::new();
        let mut api_client_node = None;

        for node in schemas.values() {
            match node {
                TsNode::Interface(_) | TsNode::TypeAlias(_) | TsNode::Enum(_) => {
                    schema_nodes.push(node.clone());
                }
                TsNode::Class(_) => {
                    api_client_node = Some(node.clone());
                }
                _ => {}
            }
        }

        // Generate models.ts
        if !schema_nodes.is_empty() {
            let content =
                self.emitter
                    .emit(&schema_nodes)
                    .map_err(|e| FileGeneratorError::EmitError {
                        filename: "models.ts".to_string(),
                        source: format!("{}", e),
                    })?;

            files.push(GeneratedFile {
                filename: "models.ts".to_string(),
                content,
                file_type: FileType::Models,
            });
        }

        // Generate api.ts
        if let Some(api_client) = api_client_node {
            let content =
                self.emitter
                    .emit(&[api_client])
                    .map_err(|e| FileGeneratorError::EmitError {
                        filename: "api.ts".to_string(),
                        source: format!("{}", e),
                    })?;

            files.push(GeneratedFile {
                filename: "api.ts".to_string(),
                content,
                file_type: FileType::Api,
            });
        }

        Ok(files)
    }

    /// Generate index file with exports
    fn generate_index_file(
        &self,
        schemas: &HashMap<String, TsNode>,
    ) -> Result<GeneratedFile, FileGeneratorError> {
        let mut exports = Vec::new();

        if self.config.granular_files {
            // Export all individual schema files
            // Sort keys to ensure deterministic output
            let mut sorted_names: Vec<&String> = schemas.keys().collect();
            sorted_names.sort();
            
            for name in sorted_names {
                let filename = self.generate_filename(name);
                // Remove .ts extension for imports
                let import_name = filename.trim_end_matches(".ts");
                exports.push(format!("export * from './{}';", import_name));
            }
        } else {
            // Export grouped files
            exports.push("export * from './models';".to_string());
            exports.push("export * from './api';".to_string());
        }

        let content = exports.join("\n");

        Ok(GeneratedFile {
            filename: "index.ts".to_string(),
            content,
            file_type: FileType::Index,
        })
    }

    /// Generate filename based on naming convention
    fn generate_filename(&self, name: &str) -> String {
        let base_name = match self.config.naming_convention {
            NamingConvention::CamelCase => name.to_string(),
            NamingConvention::KebabCase => self.to_kebab_case(name),
            NamingConvention::SnakeCase => self.to_snake_case(name),
        };

        format!("{}.ts", base_name)
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
        if s.chars().next().map_or(false, |c| c.is_uppercase()) {
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
    pub file_type: FileType,
}

/// Type of generated file
#[derive(Debug, Clone)]
pub enum FileType {
    Schema,
    Models,
    Api,
    Index,
    Utility,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("generated"),
            granular_files: true,
            naming_convention: NamingConvention::KebabCase,
        }
    }
}
