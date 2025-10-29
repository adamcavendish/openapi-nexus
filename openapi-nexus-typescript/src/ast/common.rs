//! Common TypeScript AST types
//!
//! This module contains commonly used types that were previously split across multiple files.
//! Consolidates Parameter, Property, Generic, EnumVariant, and Visibility into one location.

use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;

/// TypeScript visibility modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}

impl Default for Visibility {
    fn default() -> Self {
        Self::Public
    }
}

/// TypeScript parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_expr: Option<TypeExpression>,
    pub optional: bool,
    pub default_value: Option<String>,
}

impl Parameter {
    /// Create a new parameter
    pub fn new(name: String) -> Self {
        Self {
            name,
            type_expr: None,
            optional: false,
            default_value: None,
        }
    }

    /// Create a parameter with type
    pub fn with_type(name: String, type_expr: TypeExpression) -> Self {
        Self {
            name,
            type_expr: Some(type_expr),
            optional: false,
            default_value: None,
        }
    }

    /// Create an optional parameter
    pub fn optional(name: String, type_expr: Option<TypeExpression>) -> Self {
        Self {
            name,
            type_expr,
            optional: true,
            default_value: None,
        }
    }

    /// Set default value
    pub fn with_default(mut self, default_value: String) -> Self {
        self.default_value = Some(default_value);
        self
    }

    /// Format parameter as TypeScript string (for templates)
    pub fn to_typescript_string(&self) -> String {
        let mut result = self.name.clone();

        if self.optional {
            result.push('?');
        }

        if let Some(type_expr) = &self.type_expr {
            result.push_str(": ");
            result.push_str(&type_expr.to_typescript_string());
        }

        if let Some(default_value) = &self.default_value {
            result.push_str(" = ");
            result.push_str(default_value);
        }

        result
    }
}

/// TypeScript property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub documentation: Option<String>,
}

impl Property {
    /// Create a new property
    pub fn new(name: String, type_expr: TypeExpression) -> Self {
        Self {
            name,
            type_expr,
            optional: false,
            documentation: None,
        }
    }

    /// Create an optional property
    pub fn optional(name: String, type_expr: TypeExpression) -> Self {
        Self {
            name,
            type_expr,
            optional: true,
            documentation: None,
        }
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Format property as TypeScript string (for templates)
    pub fn to_typescript_string(&self) -> String {
        let mut result = self.name.clone();

        if self.optional {
            result.push('?');
        }

        result.push_str(": ");
        result.push_str(&self.type_expr.to_typescript_string());

        result
    }
}

/// TypeScript generic parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub constraint: Option<String>,
    pub default: Option<String>,
}

impl Generic {
    /// Create a new generic parameter
    pub fn new(name: String) -> Self {
        Self {
            name,
            constraint: None,
            default: None,
        }
    }

    /// Add constraint (extends clause)
    pub fn with_constraint(mut self, constraint: String) -> Self {
        self.constraint = Some(constraint);
        self
    }

    /// Add default type
    pub fn with_default(mut self, default: String) -> Self {
        self.default = Some(default);
        self
    }

    /// Format generic as TypeScript string (for templates)
    pub fn to_typescript_string(&self) -> String {
        let mut result = self.name.clone();

        if let Some(constraint) = &self.constraint {
            result.push_str(" extends ");
            result.push_str(constraint);
        }

        if let Some(default) = &self.default {
            result.push_str(" = ");
            result.push_str(default);
        }

        result
    }
}

/// TypeScript enum variant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
    pub documentation: Option<String>,
}

impl EnumVariant {
    /// Create a new enum variant
    pub fn new(name: String) -> Self {
        Self {
            name,
            value: None,
            documentation: None,
        }
    }

    /// Create an enum variant with explicit value
    pub fn with_value(name: String, value: String) -> Self {
        Self {
            name,
            value: Some(value),
            documentation: None,
        }
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }

    /// Format enum variant as TypeScript string (for templates)
    pub fn to_typescript_string(&self) -> String {
        let mut result = self.name.clone();

        if let Some(value) = &self.value {
            result.push_str(" = ");
            result.push_str(value);
        }

        result
    }
}

/// Helper functions for formatting collections
impl Parameter {
    /// Format a list of parameters as TypeScript parameter list string
    pub fn format_parameter_list(parameters: &[Parameter]) -> String {
        if parameters.is_empty() {
            "()".to_string()
        } else {
            let param_strings: Vec<String> = parameters
                .iter()
                .map(|p| p.to_typescript_string())
                .collect();

            if parameters.len() > 3 {
                // Multi-line format for long parameter lists
                format!("(\n  {}\n)", param_strings.join(",\n  "))
            } else {
                // Single line format
                format!("({})", param_strings.join(", "))
            }
        }
    }
}

impl Generic {
    /// Format a list of generics as TypeScript generic list string
    pub fn format_generic_list(generics: &[Generic]) -> String {
        if generics.is_empty() {
            String::new()
        } else {
            let generic_strings: Vec<String> =
                generics.iter().map(|g| g.to_typescript_string()).collect();
            format!("<{}>", generic_strings.join(", "))
        }
    }
}

impl EnumVariant {
    /// Format a list of enum variants as TypeScript enum body string
    pub fn format_enum_body(variants: &[EnumVariant]) -> String {
        if variants.is_empty() {
            "{}".to_string()
        } else {
            let variant_strings: Vec<String> =
                variants.iter().map(|v| v.to_typescript_string()).collect();

            if variants.len() > 2 {
                // Multi-line format
                format!("{{\n  {}\n}}", variant_strings.join(",\n  "))
            } else {
                // Single line format
                format!("{{ {} }}", variant_strings.join(", "))
            }
        }
    }
}
