//! Dependency analysis for TypeScript AST nodes

use std::collections::HashSet;

use crate::ast::{TsNode, TypeExpression};
use crate::utils::typescript_types::{is_primitive_type, is_runtime_type};

/// Analyzes TypeScript AST nodes to extract type dependencies
pub struct DependencyAnalyzer;

impl DependencyAnalyzer {
    /// Create a new dependency analyzer
    pub fn new() -> Self {
        Self
    }

    /// Extract all type dependencies from a collection of AST nodes
    pub fn analyze_dependencies(&self, nodes: &[TsNode]) -> DependencySet {
        let mut dependencies = DependencySet::new();

        for node in nodes {
            self.extract_node_dependencies(node, &mut dependencies);
        }

        dependencies
    }

    /// Extract dependencies from a single AST node
    fn extract_node_dependencies(&self, node: &TsNode, dependencies: &mut DependencySet) {
        match node {
            TsNode::Interface(interface) => {
                // Extract dependencies from interface properties
                for property in &interface.properties {
                    Self::extract_type_dependencies(&property.type_expr, dependencies);
                }

                // Extract dependencies from extends clause
                for extend in &interface.extends {
                    dependencies.add_model_dependency(extend.clone());
                }
            }
            TsNode::TypeAlias(type_alias) => {
                // Extract dependencies from type alias definition
                Self::extract_type_dependencies(&type_alias.type_expr, dependencies);
            }
            TsNode::Class(class) => {
                // Extract dependencies from class properties
                for property in &class.properties {
                    Self::extract_type_dependencies(&property.type_expr, dependencies);
                }

                // Extract dependencies from method signatures
                for method in &class.methods {
                    // Method parameters
                    for param in &method.parameters {
                        if let Some(type_expr) = &param.type_expr {
                            Self::extract_type_dependencies(type_expr, dependencies);
                        }
                    }

                    // Method return type
                    if let Some(return_type) = &method.return_type {
                        Self::extract_type_dependencies(return_type, dependencies);
                    }
                }

                // Extract dependencies from extends clause
                if let Some(extends) = &class.extends {
                    // Check if it's a runtime dependency
                    if is_runtime_type(extends) {
                        dependencies.add_runtime_dependency(extends.clone());
                    } else {
                        dependencies.add_model_dependency(extends.clone());
                    }
                }

                // Extract dependencies from implements clause
                for implement in &class.implements {
                    dependencies.add_model_dependency(implement.clone());
                }
            }
            TsNode::Function(function) => {
                // Extract dependencies from function parameters
                for param in &function.parameters {
                    if let Some(type_expr) = &param.type_expr {
                        Self::extract_type_dependencies(type_expr, dependencies);
                    }
                }

                // Extract dependencies from return type
                if let Some(return_type) = &function.return_type {
                    Self::extract_type_dependencies(return_type, dependencies);
                }
            }
            TsNode::Enum(_) => {
                // Enums typically don't have dependencies
            }
            TsNode::Import(_) | TsNode::Export(_) => {
                // These are already import/export statements, skip analysis
            }
        }
    }

    /// Extract dependencies from a type expression recursively
    fn extract_type_dependencies(type_expr: &TypeExpression, dependencies: &mut DependencySet) {
        match type_expr {
            TypeExpression::Reference(type_name) => {
                // Handle generic types like Promise<T>, Array<T>, etc.
                if type_name.contains('<') && type_name.contains('>') {
                    // Extract inner types from generic type strings
                    let inner_types = Self::extract_generic_types(type_name);
                    for inner_type in inner_types {
                        if !is_primitive_type(&inner_type) {
                            if is_runtime_type(&inner_type) {
                                dependencies.add_runtime_dependency(inner_type);
                            } else {
                                dependencies.add_model_dependency(inner_type);
                            }
                        }
                    }
                } else {
                    // Only add non-primitive types as dependencies
                    if !is_primitive_type(type_name) {
                        if is_runtime_type(type_name) {
                            dependencies.add_runtime_dependency(type_name.clone());
                        } else {
                            dependencies.add_model_dependency(type_name.clone());
                        }
                    }
                }
            }
            TypeExpression::Array(item_type) => {
                Self::extract_type_dependencies(item_type, dependencies);
            }
            TypeExpression::Union(types) => {
                for type_expr in types {
                    Self::extract_type_dependencies(type_expr, dependencies);
                }
            }
            TypeExpression::Intersection(types) => {
                for type_expr in types {
                    Self::extract_type_dependencies(type_expr, dependencies);
                }
            }
            TypeExpression::Object(properties) => {
                for type_expr in properties.values() {
                    Self::extract_type_dependencies(type_expr, dependencies);
                }
            }
            TypeExpression::Function(func_sig) => {
                // Extract dependencies from function signature parameters
                for param in &func_sig.parameters {
                    if let Some(type_expr) = &param.type_expr {
                        Self::extract_type_dependencies(type_expr, dependencies);
                    }
                }

                // Extract dependencies from return type
                if let Some(return_type) = &func_sig.return_type {
                    Self::extract_type_dependencies(return_type, dependencies);
                }
            }
            TypeExpression::IndexSignature(_, value_type) => {
                Self::extract_type_dependencies(value_type, dependencies);
            }
            TypeExpression::Tuple(types) => {
                for type_expr in types {
                    Self::extract_type_dependencies(type_expr, dependencies);
                }
            }
            TypeExpression::Generic(_)
            | TypeExpression::Literal(_)
            | TypeExpression::Primitive(_) => {
                // These don't have dependencies to extract
            }
        }
    }

    /// Extract inner types from generic type strings like "Promise<ApiResponse>"
    fn extract_generic_types(type_name: &str) -> Vec<String> {
        let mut inner_types = Vec::new();
        
        // Find the content between < and >
        if let Some(start) = type_name.find('<') {
            if let Some(end) = type_name.rfind('>') {
                if start < end {
                    let inner_content = &type_name[start + 1..end];
                    
                    // Handle nested generics and unions
                    let mut depth = 0;
                    let mut current_type = String::new();
                    
                    for ch in inner_content.chars() {
                        match ch {
                            '<' => {
                                depth += 1;
                                current_type.push(ch);
                            }
                            '>' => {
                                depth -= 1;
                                current_type.push(ch);
                            }
                            '|' if depth == 0 => {
                                // Union separator at top level
                                if !current_type.trim().is_empty() {
                                    inner_types.push(current_type.trim().to_string());
                                }
                                current_type.clear();
                            }
                            ',' if depth == 0 => {
                                // Generic parameter separator at top level
                                if !current_type.trim().is_empty() {
                                    inner_types.push(current_type.trim().to_string());
                                }
                                current_type.clear();
                            }
                            _ => {
                                current_type.push(ch);
                            }
                        }
                    }
                    
                    // Add the last type
                    if !current_type.trim().is_empty() {
                        inner_types.push(current_type.trim().to_string());
                    }
                }
            }
        }
        
        inner_types
    }
}

impl Default for DependencyAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

/// Set of dependencies categorized by type
#[derive(Debug, Clone)]
pub struct DependencySet {
    /// Dependencies on other generated model types
    pub model_dependencies: HashSet<String>,
    /// Dependencies on runtime library types
    pub runtime_dependencies: HashSet<String>,
    /// Dependencies on external library types
    pub external_dependencies: HashSet<String>,
}

impl DependencySet {
    /// Create a new empty dependency set
    pub fn new() -> Self {
        Self {
            model_dependencies: HashSet::new(),
            runtime_dependencies: HashSet::new(),
            external_dependencies: HashSet::new(),
        }
    }

    /// Add a model dependency
    pub fn add_model_dependency(&mut self, type_name: String) {
        self.model_dependencies.insert(type_name);
    }

    /// Add a runtime dependency
    pub fn add_runtime_dependency(&mut self, type_name: String) {
        self.runtime_dependencies.insert(type_name);
    }

    /// Add an external dependency
    pub fn add_external_dependency(&mut self, type_name: String) {
        self.external_dependencies.insert(type_name);
    }

    /// Check if there are any dependencies
    pub fn is_empty(&self) -> bool {
        self.model_dependencies.is_empty()
            && self.runtime_dependencies.is_empty()
            && self.external_dependencies.is_empty()
    }

    /// Get all dependencies as a single set
    pub fn all_dependencies(&self) -> HashSet<String> {
        let mut all = HashSet::new();
        all.extend(self.model_dependencies.iter().cloned());
        all.extend(self.runtime_dependencies.iter().cloned());
        all.extend(self.external_dependencies.iter().cloned());
        all
    }
}

impl Default for DependencySet {
    fn default() -> Self {
        Self::new()
    }
}
