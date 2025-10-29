//! TypeScript class definitions for template rendering
//!
//! This module provides simplified class data structures optimized for template rendering.
//! No RcDoc emission - these are pure data structures for Jinja2 templates.

use serde::{Deserialize, Serialize};

use crate::ast::{Generic, Parameter, TypeExpression, Visibility};

/// TypeScript class definition for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassDefinition {
    pub name: String,
    pub properties: Vec<ClassProperty>,
    pub methods: Vec<ClassMethod>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub generics: Vec<Generic>,
    pub is_export: bool,
    pub documentation: Option<String>,
    pub imports: Vec<ImportStatement>,
}

/// TypeScript class property for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassProperty {
    pub name: String,
    pub type_expr: TypeExpression,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_readonly: bool,
    pub optional: bool,
    pub default_value: Option<String>,
    pub documentation: Option<String>,
}

/// TypeScript class method for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClassMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub visibility: Visibility,
    pub is_static: bool,
    pub is_async: bool,
    pub is_abstract: bool,
    pub documentation: Option<String>,
    pub body_template: Option<String>,
    pub body_data: Option<serde_json::Value>,
}

/// Import statement for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportStatement {
    pub module_path: String,
    pub imports: Vec<ImportSpecifier>,
    pub is_type_only: bool,
}

/// Import specifier for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
    pub is_type: bool,
}

impl ClassDefinition {
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
    pub fn with_property(mut self, property: ClassProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Add multiple properties
    pub fn with_properties(mut self, properties: Vec<ClassProperty>) -> Self {
        self.properties.extend(properties);
        self
    }

    /// Add a method
    pub fn with_method(mut self, method: ClassMethod) -> Self {
        self.methods.push(method);
        self
    }

    /// Add multiple methods
    pub fn with_methods(mut self, methods: Vec<ClassMethod>) -> Self {
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
    pub fn with_generics(mut self, generics: Vec<Generic>) -> Self {
        self.generics = generics;
        self
    }

    /// Set export flag
    pub fn with_export(mut self, is_export: bool) -> Self {
        self.is_export = is_export;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Add import
    pub fn with_import(mut self, import: ImportStatement) -> Self {
        self.imports.push(import);
        self
    }

    /// Add multiple imports
    pub fn with_imports(mut self, imports: Vec<ImportStatement>) -> Self {
        self.imports.extend(imports);
        self
    }

    /// Get template data for rendering
    pub fn to_template_data(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap_or_else(|_| serde_json::Value::Null)
    }
}

impl ClassProperty {
    /// Create a new class property
    pub fn new(name: String, type_expr: TypeExpression) -> Self {
        Self {
            name,
            type_expr,
            visibility: Visibility::Public,
            is_static: false,
            is_readonly: false,
            optional: false,
            default_value: None,
            documentation: None,
        }
    }

    /// Set visibility
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Make static
    pub fn with_static(mut self) -> Self {
        self.is_static = true;
        self
    }

    /// Make readonly
    pub fn with_readonly(mut self) -> Self {
        self.is_readonly = true;
        self
    }

    /// Make optional
    pub fn with_optional(mut self) -> Self {
        self.optional = true;
        self
    }

    /// Set default value
    pub fn with_default(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Format property for template rendering
    pub fn to_typescript_string(&self) -> String {
        let mut parts = Vec::new();

        // Visibility
        match self.visibility {
            Visibility::Private => parts.push("private".to_string()),
            Visibility::Protected => parts.push("protected".to_string()),
            Visibility::Public => {} // Default, no keyword needed
        }

        // Static
        if self.is_static {
            parts.push("static".to_string());
        }

        // Readonly
        if self.is_readonly {
            parts.push("readonly".to_string());
        }

        // Property name and type
        let mut name_type = self.name.clone();
        if self.optional {
            name_type.push('?');
        }
        name_type.push_str(": ");
        name_type.push_str(&self.type_expr.to_typescript_string());

        parts.push(name_type);

        // Default value
        if let Some(default_value) = &self.default_value {
            parts.push(format!("= {}", default_value));
        }

        parts.join(" ")
    }
}

impl ClassMethod {
    /// Create a new class method
    pub fn new(name: String) -> Self {
        Self {
            name,
            parameters: Vec::new(),
            return_type: None,
            visibility: Visibility::Public,
            is_static: false,
            is_async: false,
            is_abstract: false,
            documentation: None,
            body_template: None,
            body_data: None,
        }
    }

    /// Add parameters
    pub fn with_parameters(mut self, parameters: Vec<Parameter>) -> Self {
        self.parameters = parameters;
        self
    }

    /// Set return type
    pub fn with_return_type(mut self, return_type: TypeExpression) -> Self {
        self.return_type = Some(return_type);
        self
    }

    /// Set visibility
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = visibility;
        self
    }

    /// Make static
    pub fn with_static(mut self) -> Self {
        self.is_static = true;
        self
    }

    /// Make async
    pub fn with_async(mut self) -> Self {
        self.is_async = true;
        self
    }

    /// Make abstract
    pub fn with_abstract(mut self) -> Self {
        self.is_abstract = true;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Set body template
    pub fn with_body_template(mut self, template: String, data: Option<serde_json::Value>) -> Self {
        self.body_template = Some(template);
        self.body_data = data;
        self
    }

    /// Format method signature for template rendering
    pub fn to_signature_string(&self) -> String {
        let mut parts = Vec::new();

        // Visibility
        match self.visibility {
            Visibility::Private => parts.push("private".to_string()),
            Visibility::Protected => parts.push("protected".to_string()),
            Visibility::Public => {} // Default, no keyword needed
        }

        // Static
        if self.is_static {
            parts.push("static".to_string());
        }

        // Abstract
        if self.is_abstract {
            parts.push("abstract".to_string());
        }

        // Async
        if self.is_async {
            parts.push("async".to_string());
        }

        // Method name and parameters
        let mut signature = self.name.clone();
        signature.push_str(&Parameter::format_parameter_list(&self.parameters));

        // Return type
        if let Some(return_type) = &self.return_type {
            signature.push_str(": ");
            signature.push_str(&return_type.to_typescript_string());
        }

        parts.push(signature);

        parts.join(" ")
    }
}

impl ImportStatement {
    /// Create a new import statement
    pub fn new(module_path: String) -> Self {
        Self {
            module_path,
            imports: Vec::new(),
            is_type_only: false,
        }
    }

    /// Add import specifier
    pub fn with_import(mut self, name: String, alias: Option<String>) -> Self {
        self.imports.push(ImportSpecifier {
            name,
            alias,
            is_type: false,
        });
        self
    }

    /// Add type import specifier
    pub fn with_type_import(mut self, name: String, alias: Option<String>) -> Self {
        self.imports.push(ImportSpecifier {
            name,
            alias,
            is_type: true,
        });
        self
    }

    /// Make type-only import
    pub fn with_type_only(mut self) -> Self {
        self.is_type_only = true;
        self
    }

    /// Format import statement for template rendering
    pub fn to_typescript_string(&self) -> String {
        if self.imports.is_empty() {
            return format!("import '{}';", self.module_path);
        }

        let mut import_parts = Vec::new();

        // Type-only imports
        if self.is_type_only {
            import_parts.push("type".to_string());
        }

        // Import specifiers
        let specifiers: Vec<String> = self.imports
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

        if specifiers.len() == 1 {
            import_parts.push(format!("{{ {} }}", specifiers[0]));
        } else {
            import_parts.push(format!("{{ {} }}", specifiers.join(", ")));
        }

        import_parts.push("from".to_string());
        import_parts.push(format!("'{}'", self.module_path));

        format!("import {};", import_parts.join(" "))
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
}
