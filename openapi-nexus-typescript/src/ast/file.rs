//! TypeScript file representation
//!
//! This module provides high-level file structures for organizing TypeScript code generation.

use serde::{Deserialize, Serialize};

use crate::ast::{ClassDefinition, Import, TypeDefinition};

/// TypeScript file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptFile {
    pub file_path: String,
    pub imports: Vec<Import>,
    pub content: FileContent,
    pub header: Option<String>,
}

/// Content types that can be in a TypeScript file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileContent {
    /// Single API class file
    ApiClass(ClassDefinition),
    /// Single type definition file
    TypeDefinition(TypeDefinition),
    /// Multiple type definitions in one file
    TypeDefinitions(Vec<TypeDefinition>),
    /// Mixed content (classes and types)
    Mixed {
        classes: Vec<ClassDefinition>,
        types: Vec<TypeDefinition>,
    },
    /// Raw TypeScript content (for runtime files, etc.)
    Raw(String),
}

/// File category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FileCategory {
    /// API client classes
    Api,
    /// Type definitions and interfaces
    Models,
    /// Runtime support files
    Runtime,
    /// Configuration files
    Config,
    /// Utility files
    Utils,
}

impl TypeScriptFile {
    /// Create a new TypeScript file
    pub fn new(file_path: String, content: FileContent) -> Self {
        Self {
            file_path,
            imports: Vec::new(),
            content,
            header: None,
        }
    }

    /// Create an API class file
    pub fn api_class(file_path: String, class: ClassDefinition) -> Self {
        Self::new(file_path, FileContent::ApiClass(class))
    }

    /// Create a type definition file
    pub fn type_definition(file_path: String, type_def: TypeDefinition) -> Self {
        Self::new(file_path, FileContent::TypeDefinition(type_def))
    }

    /// Create a file with multiple type definitions
    pub fn type_definitions(file_path: String, type_defs: Vec<TypeDefinition>) -> Self {
        Self::new(file_path, FileContent::TypeDefinitions(type_defs))
    }

    /// Create a mixed content file
    pub fn mixed(
        file_path: String,
        classes: Vec<ClassDefinition>,
        types: Vec<TypeDefinition>,
    ) -> Self {
        Self::new(file_path, FileContent::Mixed { classes, types })
    }

    /// Create a raw content file
    pub fn raw(file_path: String, content: String) -> Self {
        Self::new(file_path, FileContent::Raw(content))
    }

    /// Add imports
    pub fn with_imports(mut self, imports: Vec<Import>) -> Self {
        self.imports = imports;
        self
    }

    /// Add a single import
    pub fn with_import(mut self, import: Import) -> Self {
        self.imports.push(import);
        self
    }

    /// Set file header
    pub fn with_header(mut self, header: String) -> Self {
        self.header = Some(header);
        self
    }

    /// Get file category based on path
    pub fn get_category(&self) -> FileCategory {
        if self.file_path.contains("/apis/") {
            FileCategory::Api
        } else if self.file_path.contains("/models/") {
            FileCategory::Models
        } else if self.file_path.contains("/runtime/") {
            FileCategory::Runtime
        } else if self.file_path.contains("/config") {
            FileCategory::Config
        } else {
            FileCategory::Utils
        }
    }

    /// Check if this is an API class file
    pub fn is_api_class(&self) -> bool {
        matches!(self.content, FileContent::ApiClass(_))
    }

    /// Check if this is a type definition file
    pub fn is_type_definition(&self) -> bool {
        matches!(
            self.content,
            FileContent::TypeDefinition(_) | FileContent::TypeDefinitions(_)
        )
    }

    /// Check if this file needs template rendering
    pub fn needs_template_rendering(&self) -> bool {
        matches!(self.content, FileContent::ApiClass(_))
    }

    /// Get template data for rendering (if applicable)
    pub fn get_template_data(&self) -> Option<serde_json::Value> {
        match &self.content {
            FileContent::ApiClass(class) => Some(serde_json::json!({
                "class": class,
                "imports": self.imports
            })),
            _ => None,
        }
    }

    /// Get the main class definition (if this is an API class file)
    pub fn get_class(&self) -> Option<&ClassDefinition> {
        match &self.content {
            FileContent::ApiClass(class) => Some(class),
            FileContent::Mixed { classes, .. } if classes.len() == 1 => classes.first(),
            _ => None,
        }
    }

    /// Get all type definitions in this file
    pub fn get_type_definitions(&self) -> Vec<&TypeDefinition> {
        match &self.content {
            FileContent::TypeDefinition(type_def) => vec![type_def],
            FileContent::TypeDefinitions(type_defs) => type_defs.iter().collect(),
            FileContent::Mixed { types, .. } => types.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// Get file extension
    pub fn get_extension(&self) -> &str {
        if self.file_path.ends_with(".ts") {
            ".ts"
        } else if self.file_path.ends_with(".js") {
            ".js"
        } else if self.file_path.ends_with(".d.ts") {
            ".d.ts"
        } else {
            ""
        }
    }

    /// Get file name without extension
    pub fn get_name_without_extension(&self) -> String {
        let path = std::path::Path::new(&self.file_path);
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string()
    }

    /// Get directory path
    pub fn get_directory(&self) -> String {
        let path = std::path::Path::new(&self.file_path);
        path.parent()
            .and_then(|p| p.to_str())
            .unwrap_or("")
            .to_string()
    }
}

/// Collection of TypeScript files for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScriptProject {
    pub files: Vec<TypeScriptFile>,
    pub package_name: String,
    pub version: String,
    pub description: Option<String>,
}

impl TypeScriptProject {
    /// Create a new TypeScript project
    pub fn new(package_name: String, version: String) -> Self {
        Self {
            files: Vec::new(),
            package_name,
            version,
            description: None,
        }
    }

    /// Add a file to the project
    pub fn add_file(mut self, file: TypeScriptFile) -> Self {
        self.files.push(file);
        self
    }

    /// Add multiple files to the project
    pub fn add_files(mut self, files: Vec<TypeScriptFile>) -> Self {
        self.files.extend(files);
        self
    }

    /// Set description
    pub fn with_description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// Get files by category
    pub fn get_files_by_category(&self, category: FileCategory) -> Vec<&TypeScriptFile> {
        self.files
            .iter()
            .filter(|file| file.get_category() == category)
            .collect()
    }

    /// Get API class files
    pub fn get_api_files(&self) -> Vec<&TypeScriptFile> {
        self.files
            .iter()
            .filter(|file| file.is_api_class())
            .collect()
    }

    /// Get type definition files
    pub fn get_model_files(&self) -> Vec<&TypeScriptFile> {
        self.files
            .iter()
            .filter(|file| file.is_type_definition())
            .collect()
    }

    /// Get files that need template rendering
    pub fn get_template_files(&self) -> Vec<&TypeScriptFile> {
        self.files
            .iter()
            .filter(|file| file.needs_template_rendering())
            .collect()
    }

    /// Get total number of files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Check if project is empty
    pub fn is_empty(&self) -> bool {
        self.files.is_empty()
    }
}
