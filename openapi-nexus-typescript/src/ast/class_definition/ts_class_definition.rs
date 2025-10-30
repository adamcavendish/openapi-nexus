use serde::{Deserialize, Serialize};

use super::{TsClassMethod, TsClassProperty, TsImportStatement};
use crate::ast::{TsDocComment, TsGeneric};

/// TypeScript class definition for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsClassDefinition {
    pub name: String,
    pub properties: Vec<TsClassProperty>,
    pub methods: Vec<TsClassMethod>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub generics: Vec<TsGeneric>,
    pub is_export: bool,
    pub documentation: Option<TsDocComment>,
    pub imports: Vec<TsImportStatement>,
}

impl TsClassDefinition {
    /// Create a new class definition
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: Vec::new(),
            methods: Vec::new(),
            extends: None,
            implements: Vec::new(),
            generics: Vec::new(),
            is_export: true,
            documentation: None,
            imports: Vec::new(),
        }
    }

    /// Add a property
    pub fn with_property(mut self, property: TsClassProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Add multiple properties
    pub fn with_properties(mut self, properties: Vec<TsClassProperty>) -> Self {
        self.properties.extend(properties);
        self
    }

    /// Add a method
    pub fn with_method(mut self, method: TsClassMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Add multiple methods
    pub fn with_methods(mut self, methods: Vec<TsClassMethod>) -> Self {
        self.methods.extend(methods);
        self
    }

    /// Set extends clause
    pub fn with_extends(mut self, extends: String) -> Self {
        self.extends = Some(extends);
        self
    }

    /// Add implements clause
    pub fn with_implements(mut self, implements: Vec<String>) -> Self {
        self.implements = implements;
        self
    }

    /// Add generics
    pub fn with_generics(mut self, generics: Vec<TsGeneric>) -> Self {
        self.generics = generics;
        self
    }

    /// Set export flag
    pub fn with_export(mut self, is_export: bool) -> Self {
        self.is_export = is_export;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: TsDocComment) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Add import
    pub fn with_import(mut self, import: TsImportStatement) -> Self {
        self.imports.push(import);
        self
    }

    /// Add multiple imports
    pub fn with_imports(mut self, imports: Vec<TsImportStatement>) -> Self {
        self.imports.extend(imports);
        self
    }

    /// Get template data for rendering
    pub fn to_template_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or(serde_json::Value::Null)
    }
}
