//! TypeScript import handling
//!
//! This module consolidates all import-related functionality including import statements,
//! import collections, import resolution, and dependency analysis.

use std::collections::{HashMap, HashSet};
use std::path;

use heck::ToKebabCase as _;
use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::emission::error::EmitError;
use crate::emission::ts_dependency_analyzer::DependencySet;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript import statement
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Import {
    pub module_path: String,
    pub specifiers: Vec<ImportSpecifier>,
    pub is_type_only: bool,
}

/// TypeScript import specifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
    pub is_type: bool,
}

/// A collection of imports for a TypeScript file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportCollection {
    /// All imports in this collection
    imports: Vec<Import>,
}

/// Import resolver that handles dependency analysis and import generation
#[derive(Debug, Clone)]
pub struct ImportResolver {
    /// Map of schema names to file paths
    schema_to_file_map: HashMap<String, String>,
    /// Current file being processed
    current_file: String,
}

impl Import {
    /// Create a new import
    pub fn new(module_path: impl Into<String>) -> Self {
        Self {
            module_path: module_path.into(),
            specifiers: Vec::new(),
            is_type_only: false,
        }
    }

    /// Create a type-only import
    pub fn new_type_only(module_path: impl Into<String>) -> Self {
        Self {
            module_path: module_path.into(),
            specifiers: Vec::new(),
            is_type_only: true,
        }
    }

    /// Add specifiers
    pub fn with_specifiers(mut self, specifiers: Vec<ImportSpecifier>) -> Self {
        self.specifiers = specifiers;
        self
    }

    /// Add named imports
    pub fn with_named_imports(
        mut self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.specifiers = names
            .into_iter()
            .map(|name| ImportSpecifier::new(name.into()))
            .collect();
        self
    }

    /// Add type imports
    pub fn with_type_imports(mut self, names: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.specifiers = names
            .into_iter()
            .map(|name| ImportSpecifier::new_type(name.into()))
            .collect();
        self
    }

    /// Add a single specifier
    pub fn with_specifier(mut self, specifier: ImportSpecifier) -> Self {
        self.specifiers.push(specifier);
        self
    }

    /// Make this import type-only
    pub fn with_type_only(mut self) -> Self {
        self.is_type_only = true;
        self
    }

    /// Format import as TypeScript string
    pub fn to_typescript_string(&self) -> String {
        if self.specifiers.is_empty() {
            return format!("import '{}';", self.module_path);
        }

        let mut parts = Vec::new();
        parts.push("import".to_string());

        if self.is_type_only {
            parts.push("type".to_string());
        }

        // Format specifiers
        let specifier_strings: Vec<String> = self
            .specifiers
            .iter()
            .map(|spec| {
                let mut s = String::new();
                if spec.is_type && !self.is_type_only {
                    s.push_str("type ");
                }
                s.push_str(&spec.name);
                if let Some(alias) = &spec.alias {
                    s.push_str(" as ");
                    s.push_str(alias);
                }
                s
            })
            .collect();

        if specifier_strings.len() == 1 {
            parts.push(format!("{{ {} }}", specifier_strings[0]));
        } else {
            parts.push(format!("{{ {} }}", specifier_strings.join(", ")));
        }

        parts.push("from".to_string());
        parts.push(format!("'{}'", self.module_path));

        format!("{};", parts.join(" "))
    }
}

impl ToRcDocWithContext for Import {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        Ok(RcDoc::text(self.to_typescript_string()))
    }
}

impl ImportSpecifier {
    /// Create a new import specifier
    pub fn new(name: String) -> Self {
        Self {
            name,
            alias: None,
            is_type: false,
        }
    }

    /// Create a type import specifier
    pub fn new_type(name: String) -> Self {
        Self {
            name,
            alias: None,
            is_type: true,
        }
    }

    /// Set alias
    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }

    /// Make this a type import
    pub fn with_type(mut self) -> Self {
        self.is_type = true;
        self
    }
}

impl ImportCollection {
    /// Create a new empty import collection
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    /// Add an import to the collection
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    /// Add a runtime import (non-type-only)
    pub fn add_runtime_import(
        &mut self,
        module: impl Into<String>,
        specifiers: Vec<ImportSpecifier>,
    ) {
        self.imports
            .push(Import::new(module).with_specifiers(specifiers));
    }

    /// Add a runtime import with named imports
    pub fn add_runtime_named_imports(
        &mut self,
        module: impl Into<String>,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.imports
            .push(Import::new(module).with_named_imports(names));
    }

    /// Add a model import (type-only)
    pub fn add_model_import(
        &mut self,
        module: impl Into<String>,
        specifiers: Vec<ImportSpecifier>,
    ) {
        self.imports
            .push(Import::new_type_only(module).with_specifiers(specifiers));
    }

    /// Add a model import with type names
    pub fn add_model_type_imports(
        &mut self,
        module: impl Into<String>,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.imports
            .push(Import::new_type_only(module).with_type_imports(names));
    }

    /// Get all imports
    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    /// Check if collection is empty
    pub fn is_empty(&self) -> bool {
        self.imports.is_empty()
    }

    /// Get number of imports
    pub fn len(&self) -> usize {
        self.imports.len()
    }

    /// Format all imports as TypeScript strings
    pub fn to_typescript_strings(&self) -> Vec<String> {
        self.imports
            .iter()
            .map(|import| import.to_typescript_string())
            .collect()
    }

    /// Format all imports as a single TypeScript string
    pub fn to_typescript_string(&self) -> String {
        self.to_typescript_strings().join("\n")
    }
}

impl ImportResolver {
    /// Create a new import resolver
    pub fn new(schema_to_file_map: HashMap<String, String>, current_file: String) -> Self {
        Self {
            schema_to_file_map,
            current_file,
        }
    }

    /// Resolve dependencies and create import statements
    pub fn resolve_dependencies(
        &self,
        dependencies: &DependencySet,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();

        // Resolve runtime dependencies
        if !dependencies.runtime_dependencies.is_empty() {
            let runtime_imports =
                self.resolve_runtime_dependencies(&dependencies.runtime_dependencies)?;
            imports.extend(runtime_imports);
        }

        // Resolve model dependencies
        if !dependencies.model_dependencies.is_empty() {
            let model_imports =
                self.resolve_model_dependencies(&dependencies.model_dependencies)?;
            imports.extend(model_imports);
        }

        Ok(imports)
    }

    /// Resolve runtime dependencies (classes, functions, etc.)
    fn resolve_runtime_dependencies(
        &self,
        dependencies: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();
        let mut runtime_imports = HashMap::new();

        for dependency in dependencies {
            let import_path = self.resolve_runtime_import_path(dependency)?;
            runtime_imports
                .entry(import_path)
                .or_insert_with(Vec::new)
                .push(ImportSpecifier::new(dependency.clone()));
        }

        for (path, specifiers) in runtime_imports {
            imports.push(Import::new(path).with_specifiers(specifiers));
        }

        Ok(imports)
    }

    /// Resolve model dependencies (types, interfaces)
    fn resolve_model_dependencies(
        &self,
        dependencies: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();
        let mut model_imports = HashMap::new();

        for dependency in dependencies {
            if let Some(file_path) = self.schema_to_file_map.get(dependency) {
                let import_path = self.calculate_relative_import_path(file_path)?;
                model_imports
                    .entry(import_path)
                    .or_insert_with(Vec::new)
                    .push(ImportSpecifier::new_type(dependency.clone()));
            }
        }

        for (path, specifiers) in model_imports {
            imports.push(Import::new_type_only(path).with_specifiers(specifiers));
        }

        Ok(imports)
    }

    /// Resolve runtime import path
    fn resolve_runtime_import_path(&self, dependency: &str) -> Result<String, EmitError> {
        // Map common runtime dependencies to their import paths
        match dependency {
            "BaseAPI" => Ok("../runtime/base_api".to_string()),
            "Configuration" => Ok("../runtime/configuration".to_string()),
            "JSONApiResponse" => Ok("../runtime/classes/json_api_response".to_string()),
            "ResponseError" => Ok("../runtime/classes/response_error".to_string()),
            "RequiredError" => Ok("../runtime/classes/required_error".to_string()),
            _ => {
                // For unknown dependencies, try to infer from naming conventions
                if dependency.ends_with("ApiResponse") {
                    Ok(format!("../runtime/classes/{}", dependency.to_kebab_case()))
                } else if dependency.ends_with("Error") {
                    Ok(format!("../runtime/classes/{}", dependency.to_kebab_case()))
                } else {
                    Ok(format!("../runtime/{}", dependency.to_kebab_case()))
                }
            }
        }
    }

    /// Calculate relative import path from current file to target file
    fn calculate_relative_import_path(&self, target_file: &str) -> Result<String, EmitError> {
        let current_path = path::Path::new(&self.current_file);
        let target_path = path::Path::new(target_file);

        // Get the parent directory of the current file
        let current_dir = current_path
            .parent()
            .ok_or_else(|| EmitError::ImportResolution {
                message: format!("Cannot get parent directory of {}", self.current_file),
            })?;

        // Calculate relative path
        let relative_path = target_path.strip_prefix(current_dir).unwrap_or(target_path);

        // Convert to import path (remove .ts extension, use forward slashes)
        let import_path = relative_path
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");

        // Add ./ prefix for relative imports in same directory
        if !import_path.starts_with("../") && !import_path.starts_with("./") {
            Ok(format!("./{}", import_path))
        } else {
            Ok(import_path)
        }
    }
}

impl Default for ImportCollection {
    fn default() -> Self {
        Self::new()
    }
}
