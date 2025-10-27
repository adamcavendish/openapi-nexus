//! Unified import management for TypeScript code generation

use std::collections::{HashMap, HashSet};

use pretty::RcDoc;

use crate::ast::{Import, ImportSpecifier, TsNode};
use crate::emission::dependency_analyzer::{DependencyAnalyzer, DependencySet};
use crate::emission::error::EmitError;
use crate::emission::import_emitter::ImportEmitter;

/// Manages import generation and formatting for TypeScript files
pub struct ImportManager {
    dependency_analyzer: DependencyAnalyzer,
    import_emitter: ImportEmitter,
}

impl ImportManager {
    /// Create a new import manager
    pub fn new() -> Self {
        Self {
            dependency_analyzer: DependencyAnalyzer::new(),
            import_emitter: ImportEmitter::new(),
        }
    }

    /// Generate all imports for a file containing the given nodes
    pub fn generate_imports_for_file(
        &self,
        nodes: &[TsNode],
        current_file: &str,
        schema_to_file_map: &HashMap<String, String>,
    ) -> Result<Vec<Import>, EmitError> {
        // Analyze dependencies in the nodes
        let dependencies = self.dependency_analyzer.analyze_dependencies(nodes);

        // Generate imports based on dependencies
        self.generate_imports_from_dependencies(&dependencies, current_file, schema_to_file_map)
    }

    /// Generate Import AST nodes from a dependency set
    fn generate_imports_from_dependencies(
        &self,
        dependencies: &DependencySet,
        current_file: &str,
        schema_to_file_map: &HashMap<String, String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();

        // Generate runtime imports
        if !dependencies.runtime_dependencies.is_empty() {
            let runtime_imports =
                self.generate_runtime_imports(&dependencies.runtime_dependencies)?;
            imports.extend(runtime_imports);
        }

        // Generate model imports
        if !dependencies.model_dependencies.is_empty() {
            let model_imports = self.generate_model_imports(
                &dependencies.model_dependencies,
                current_file,
                schema_to_file_map,
            )?;
            imports.extend(model_imports);
        }

        // Generate external imports
        if !dependencies.external_dependencies.is_empty() {
            let external_imports =
                self.generate_external_imports(&dependencies.external_dependencies)?;
            imports.extend(external_imports);
        }

        // Sort imports for consistent output
        self.sort_imports(&mut imports);

        Ok(imports)
    }

    /// Generate runtime library imports
    fn generate_runtime_imports(
        &self,
        runtime_deps: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();

        // Group runtime dependencies by their source module
        let mut api_imports = Vec::new();
        let mut config_imports = Vec::new();

        for dep in runtime_deps {
            match dep.as_str() {
                "BaseAPI" | "RequestContext" => {
                    api_imports.push(ImportSpecifier {
                        name: dep.clone(),
                        alias: None,
                    });
                }
                "Configuration" => {
                    config_imports.push(ImportSpecifier {
                        name: dep.clone(),
                        alias: None,
                    });
                }
                _ => {
                    // Default to api module for unknown runtime types
                    api_imports.push(ImportSpecifier {
                        name: dep.clone(),
                        alias: None,
                    });
                }
            }
        }

        // Create import statements
        if !api_imports.is_empty() {
            imports.push(Import {
                module: "../runtime/api".to_string(),
                imports: api_imports,
                is_type_only: false,
            });
        }

        if !config_imports.is_empty() {
            imports.push(Import {
                module: "../runtime/config".to_string(),
                imports: config_imports,
                is_type_only: false,
            });
        }

        Ok(imports)
    }

    /// Generate model imports for other generated types
    fn generate_model_imports(
        &self,
        model_deps: &HashSet<String>,
        current_file: &str,
        schema_to_file_map: &HashMap<String, String>,
    ) -> Result<Vec<Import>, EmitError> {
        let mut imports = Vec::new();

        // Group dependencies by their target file
        let mut file_to_types: HashMap<String, Vec<String>> = HashMap::new();

        for dep in model_deps {
            if let Some(target_file) = schema_to_file_map.get(dep) {
                // Skip self-references
                if target_file != current_file {
                    file_to_types
                        .entry(target_file.clone())
                        .or_default()
                        .push(dep.clone());
                }
            } else {
                // If we can't find the file mapping, assume it's in the same directory
                tracing::warn!("No file mapping found for type: {}", dep);
                let assumed_file = format!("{}.ts", self.to_kebab_case(dep));
                if assumed_file != current_file {
                    file_to_types
                        .entry(assumed_file)
                        .or_default()
                        .push(dep.clone());
                }
            }
        }

        // Generate import statements for each target file
        for (target_file, types) in file_to_types {
            let import_path = self.generate_relative_import_path(current_file, &target_file);
            let import_specifiers: Vec<ImportSpecifier> = types
                .into_iter()
                .map(|type_name| ImportSpecifier {
                    name: type_name,
                    alias: None,
                })
                .collect();

            imports.push(Import {
                module: import_path,
                imports: import_specifiers,
                is_type_only: true, // Model imports are typically type-only
            });
        }

        Ok(imports)
    }

    /// Generate external library imports
    fn generate_external_imports(
        &self,
        _external_deps: &HashSet<String>,
    ) -> Result<Vec<Import>, EmitError> {
        // For now, we don't have external dependencies to handle
        // This can be extended in the future for third-party libraries
        Ok(Vec::new())
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
            _ => format!("./{}", to_file_base), // Default to same directory
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

    /// Emit imports as formatted TypeScript code
    pub fn emit_imports(&self, imports: &[Import]) -> Result<String, EmitError> {
        if imports.is_empty() {
            return Ok(String::new());
        }

        let mut import_docs = Vec::new();

        for import in imports {
            let doc = self.import_emitter.emit_import_doc(import)?;
            import_docs.push(doc);
        }

        // Add spacing between imports and following code
        import_docs.push(RcDoc::line());

        let combined = RcDoc::intersperse(import_docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    /// Convert string to kebab-case for file naming
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
}

impl Default for ImportManager {
    fn default() -> Self {
        Self::new()
    }
}
