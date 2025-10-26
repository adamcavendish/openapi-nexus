//! Import generation for TypeScript files

use std::collections::{HashMap, HashSet};

/// Import generator for TypeScript files
pub struct ImportGenerator;

impl ImportGenerator {
    /// Create a new import generator
    pub fn new() -> Self {
        Self
    }

    /// Generate import statements for a file based on its dependencies
    pub fn generate_imports(
        &self,
        filename: &str,
        dependencies: &HashSet<String>,
        all_schemas: &HashMap<String, String>, // schema_name -> filename mapping
    ) -> String {
        if dependencies.is_empty() {
            return String::new();
        }

        let mut imports = Vec::new();
        
        // Sort dependencies for consistent ordering
        let mut sorted_deps: Vec<_> = dependencies.iter().collect();
        sorted_deps.sort();
        
        for dep in sorted_deps {
            if let Some(dep_filename) = all_schemas.get(dep) {
                if dep_filename != filename {
                    // Generate import statement
                    let import_statement = self.generate_import_statement(dep, dep_filename);
                    imports.push(import_statement);
                }
            }
        }

        if imports.is_empty() {
            String::new()
        } else {
            imports.join("\n") + "\n"
        }
    }

    /// Generate a single import statement
    fn generate_import_statement(&self, type_name: &str, filename: &str) -> String {
        // Remove .ts extension from filename for import
        let import_filename = filename.trim_end_matches(".ts");
        format!("import {{ {} }} from './{}';", type_name, import_filename)
    }

    /// Extract dependencies from a TypeScript AST node
    pub fn extract_dependencies(&self, node: &crate::ast::TsNode) -> HashSet<String> {
        let mut dependencies = HashSet::new();
        
        match node {
            crate::ast::TsNode::Interface(interface) => {
                for prop in &interface.properties {
                    self.extract_dependencies_from_type_expr(&prop.type_expr, &mut dependencies);
                }
            }
            crate::ast::TsNode::TypeAlias(type_alias) => {
                self.extract_dependencies_from_type_expr(&type_alias.type_expr, &mut dependencies);
            }
            crate::ast::TsNode::Class(class) => {
                for prop in &class.properties {
                    self.extract_dependencies_from_type_expr(&prop.type_expr, &mut dependencies);
                }
                for method in &class.methods {
                    if let Some(return_type) = &method.return_type {
                        self.extract_dependencies_from_type_expr(return_type, &mut dependencies);
                    }
                    for param in &method.parameters {
                        if let Some(type_expr) = &param.type_expr {
                            self.extract_dependencies_from_type_expr(type_expr, &mut dependencies);
                        }
                    }
                }
            }
            _ => {}
        }
        
        dependencies
    }

    /// Extract dependencies from a type expression
    fn extract_dependencies_from_type_expr(
        &self,
        type_expr: &crate::ast::TypeExpression,
        dependencies: &mut HashSet<String>,
    ) {
        match type_expr {
            crate::ast::TypeExpression::Reference(name) => {
                // Only add if it's not a primitive type
                if !self.is_primitive_type(name) {
                    dependencies.insert(name.clone());
                }
            }
            crate::ast::TypeExpression::Array(item_type) => {
                self.extract_dependencies_from_type_expr(item_type, dependencies);
            }
            crate::ast::TypeExpression::Union(types) => {
                for t in types {
                    self.extract_dependencies_from_type_expr(t, dependencies);
                }
            }
            crate::ast::TypeExpression::Intersection(types) => {
                for t in types {
                    self.extract_dependencies_from_type_expr(t, dependencies);
                }
            }
            crate::ast::TypeExpression::Object(properties) => {
                for (_key, type_expr) in properties {
                    self.extract_dependencies_from_type_expr(type_expr, dependencies);
                }
            }
            crate::ast::TypeExpression::Function(func) => {
                for param in &func.parameters {
                    if let Some(type_expr) = &param.type_expr {
                        self.extract_dependencies_from_type_expr(type_expr, dependencies);
                    }
                }
                if let Some(return_type) = &func.return_type {
                    self.extract_dependencies_from_type_expr(return_type, dependencies);
                }
            }
            crate::ast::TypeExpression::Generic(_generic_name) => {
                // Generic types don't have dependencies to extract
            }
            _ => {}
        }
    }

    /// Check if a type name is a primitive TypeScript type
    fn is_primitive_type(&self, name: &str) -> bool {
        matches!(
            name,
            "string" | "number" | "boolean" | "any" | "unknown" | "null" | "undefined" | "void" | "object"
        )
    }
}

impl Default for ImportGenerator {
    fn default() -> Self {
        Self::new()
    }
}
