//! TypeScript file representation
//!
//! This module provides high-level file structures for organizing TypeScript code generation.

use serde::{Deserialize, Serialize};

use crate::ast::{TsClassDefinition, TsImport, TsTypeDefinition};

/// TypeScript file representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsFile {
    pub file_path: String,
    pub imports: Vec<TsImport>,
    pub content: TsFileContent,
    pub header: Option<String>,
}

/// Content types that can be in a TypeScript file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsFileContent {
    /// Single API class file
    ApiClass(TsClassDefinition),
    /// Single type definition file
    TypeDefinition(TsTypeDefinition),
    /// Multiple type definitions in one file
    TypeDefinitions(Vec<TsTypeDefinition>),
    /// Mixed content (classes and types)
    Mixed {
    classes: Vec<TsClassDefinition>,
    types: Vec<TsTypeDefinition>,
    },
    /// Raw TypeScript content (for runtime files, etc.)
    Raw(String),
}

/// File category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TsFileCategory {
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

impl TsFile {
    /// Create a new TypeScript file
    pub fn new(file_path: String, content: TsFileContent) -> Self {
        Self {
            file_path,
            imports: Vec::new(),
            content,
            header: None,
        }
    }

    /// Create an API class file
    pub fn api_class(file_path: String, class: TsClassDefinition) -> Self {
        Self::new(file_path, TsFileContent::ApiClass(class))
    }

    /// Create a type definition file
    pub fn type_definition(file_path: String, type_def: TsTypeDefinition) -> Self {
        Self::new(file_path, TsFileContent::TypeDefinition(type_def))
    }

    /// Create a file with multiple type definitions
    pub fn type_definitions(file_path: String, type_defs: Vec<TsTypeDefinition>) -> Self {
        Self::new(file_path, TsFileContent::TypeDefinitions(type_defs))
    }

    /// Create a mixed content file
    pub fn mixed(
        file_path: String,
        classes: Vec<TsClassDefinition>,
        types: Vec<TsTypeDefinition>,
    ) -> Self {
        Self::new(file_path, TsFileContent::Mixed { classes, types })
    }

    /// Create a raw content file
    pub fn raw(file_path: String, content: String) -> Self {
        Self::new(file_path, TsFileContent::Raw(content))
    }

    /// Add imports
    pub fn with_imports(mut self, imports: Vec<TsImport>) -> Self {
        self.imports = imports;
        self
    }

    /// Add a single import
    pub fn with_import(mut self, import: TsImport) -> Self {
        self.imports.push(import);
        self
    }

    /// Set file header
    pub fn with_header(mut self, header: String) -> Self {
        self.header = Some(header);
        self
    }

    /// Get file category based on path
    pub fn get_category(&self) -> TsFileCategory {
        if self.file_path.contains("/apis/") {
            TsFileCategory::Api
        } else if self.file_path.contains("/models/") {
            TsFileCategory::Models
        } else if self.file_path.contains("/runtime/") {
            TsFileCategory::Runtime
        } else if self.file_path.contains("/config") {
            TsFileCategory::Config
        } else {
            TsFileCategory::Utils
        }
    }

    /// Check if this is an API class file
    pub fn is_api_class(&self) -> bool {
        matches!(self.content, TsFileContent::ApiClass(_))
    }

    /// Check if this is a type definition file
    pub fn is_type_definition(&self) -> bool {
        matches!(
            self.content,
            TsFileContent::TypeDefinition(_) | TsFileContent::TypeDefinitions(_)
        )
    }

    /// Check if this file needs template rendering
    pub fn needs_template_rendering(&self) -> bool {
        matches!(self.content, TsFileContent::ApiClass(_))
    }

    /// Get template data for rendering (if applicable)
    pub fn get_template_data(&self) -> Option<serde_json::Value> {
        match &self.content {
            TsFileContent::ApiClass(class) => Some(serde_json::json!({
                "class": class,
                "imports": self.imports
            })),
            _ => None,
        }
    }

    /// Get the main class definition (if this is an API class file)
    pub fn get_class(&self) -> Option<&TsClassDefinition> {
        match &self.content {
            TsFileContent::ApiClass(class) => Some(class),
            TsFileContent::Mixed { classes, .. } if classes.len() == 1 => classes.first(),
            _ => None,
        }
    }

    /// Get all type definitions in this file
    pub fn get_type_definitions(&self) -> Vec<&TsTypeDefinition> {
        match &self.content {
            TsFileContent::TypeDefinition(type_def) => vec![type_def],
            TsFileContent::TypeDefinitions(type_defs) => type_defs.iter().collect(),
            TsFileContent::Mixed { types, .. } => types.iter().collect(),
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

// Note: The project-level `TypeScriptProject` struct was removed as it's unused.
