//! TypeScript import resolver for dependency analysis and import generation

use std::collections::{HashMap, HashSet};
use std::path;

use heck::ToKebabCase as _;

use crate::ast::{Import, ImportSpecifier};
use crate::emission::dependency_analyzer::DependencySet;
use crate::emission::error::EmitError;

/// Import resolver that handles dependency analysis and import generation
#[derive(Debug, Clone)]
pub struct ImportResolver {
    /// Map of schema names to file paths
    schema_to_file_map: HashMap<String, String>,
    /// Current file being processed
    current_file: String,
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

        // Resolve external dependencies
        if !dependencies.external_dependencies.is_empty() {
            let external_imports =
                self.resolve_external_dependencies(&dependencies.external_dependencies)?;
            imports.extend(external_imports);
        }

        // Sort imports for consistent output
        self.sort_imports(&mut imports);

        Ok(imports)
    }

    /// Resolve runtime dependencies to imports
    fn resolve_runtime_dependencies(
        &self,
        runtime_deps: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();
        let mut api_imports = Vec::new();
        let mut config_imports = Vec::new();

        for dep in runtime_deps {
            match dep.as_str() {
                "BaseAPI" | "RequestContext" => {
                    api_imports.push(ImportSpecifier::new(dep));
                }
                "Configuration" => {
                    config_imports.push(ImportSpecifier::new(dep));
                }
                _ => {
                    // Add to API imports by default
                    api_imports.push(ImportSpecifier::new(dep));
                }
            }
        }

        if !api_imports.is_empty() {
            imports.push(Import::new("../runtime/api").with_specifiers(api_imports));
        }

        if !config_imports.is_empty() {
            imports.push(Import::new("../runtime/config").with_specifiers(config_imports));
        }

        Ok(imports)
    }

    /// Resolve model dependencies to imports
    fn resolve_model_dependencies(
        &self,
        model_deps: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();
        let mut file_to_types: HashMap<String, Vec<String>> = HashMap::new();

        for dep in model_deps {
            if let Some(target_file) = self.schema_to_file_map.get(dep) {
                if target_file != &self.current_file {
                    file_to_types
                        .entry(target_file.clone())
                        .or_default()
                        .push(dep.clone());
                }
            } else {
                // Assume it's in the same directory
                let assumed_file = format!("{}.ts", dep.to_kebab_case());
                if assumed_file != self.current_file {
                    file_to_types
                        .entry(assumed_file)
                        .or_default()
                        .push(dep.clone());
                }
            }
        }

        for (file_path, types) in file_to_types {
            let relative_path = self.generate_relative_import_path(&self.current_file, &file_path);
            let import_specifiers: Vec<ImportSpecifier> = types
                .into_iter()
                .map(|type_name| ImportSpecifier::new(type_name))
                .collect();

            imports.push(Import::type_only(relative_path).with_specifiers(import_specifiers));
        }

        Ok(imports)
    }

    /// Resolve external dependencies to imports
    fn resolve_external_dependencies(
        &self,
        external_deps: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();

        for dep in external_deps {
            imports.push(Import::new(dep).with_named_import(dep.clone()));
        }

        Ok(imports)
    }

    /// Generate relative import path between two files
    fn generate_relative_import_path(&self, from_file: &str, to_file: &str) -> String {
        // Determine the directory structure based on file categories
        let from_dir = self.get_file_directory(from_file);
        let to_dir = self.get_file_directory(to_file);

        // Remove .ts extension from target file
        let to_file_base = to_file.trim_end_matches(".ts");

        match (from_dir.as_str(), to_dir.as_str()) {
            ("apis", "models") => format!("../models/{}", to_file_base),
            ("models", "models") => format!("./{}", to_file_base),
            ("apis", "apis") => format!("./{}", to_file_base),
            ("models", "apis") => format!("../apis/{}", to_file_base),
            _ => {
                // Fallback to simple path resolution
                let from_dir = path::Path::new(from_file)
                    .parent()
                    .unwrap_or(path::Path::new(""));
                let to_path = path::Path::new(to_file);

                if let Ok(relative) = to_path.strip_prefix(from_dir) {
                    format!("./{}", relative.to_string_lossy().trim_end_matches(".ts"))
                } else {
                    format!(
                        "./{}",
                        to_path.file_stem().unwrap_or_default().to_string_lossy()
                    )
                }
            }
        }
    }

    /// Determine the directory for a file based on naming conventions
    fn get_file_directory(&self, filename: &str) -> String {
        if filename.ends_with("Api.ts") || filename.contains("api") {
            "apis".to_string()
        } else {
            "models".to_string()
        }
    }

    /// Sort imports for consistent output
    fn sort_imports(&self, imports: &mut [Import]) {
        imports.sort_by(|a, b| {
            // Sort by import type first (runtime, then models, then external)
            let a_priority = self.get_import_priority(&a.module);
            let b_priority = self.get_import_priority(&b.module);

            match a_priority.cmp(&b_priority) {
                std::cmp::Ordering::Equal => {
                    // If same priority, sort alphabetically by module
                    a.module.cmp(&b.module)
                }
                other => other,
            }
        });
    }

    /// Get priority for import sorting (lower number = higher priority)
    fn get_import_priority(&self, module: &str) -> u8 {
        if module.starts_with("../runtime/") {
            1 // Runtime imports first
        } else if module.starts_with("./")
            || module.starts_with("../models/")
            || module.starts_with("../apis/")
        {
            2 // Local imports second
        } else {
            3 // External imports last
        }
    }
}
