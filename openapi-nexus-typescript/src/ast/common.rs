//! Common TypeScript AST types
//!
//! This module contains commonly used types that were previously split across multiple files.
//! Consolidates Parameter, Property, Generic, EnumVariant, and Visibility into one location.

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

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
}

// ToRcDocWithContext implementations

impl ToRcDocWithContext for Parameter {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if self.optional {
            doc = doc.append(RcDoc::text("?"));
        }

        if let Some(type_expr) = &self.type_expr {
            doc = doc
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(type_expr.to_rcdoc_with_context(context)?);
        }

        if let Some(default_value) = &self.default_value {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(RcDoc::text(default_value.clone()));
        }

        Ok(doc)
    }
}

impl ToRcDocWithContext for Property {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if self.optional {
            doc = doc.append(RcDoc::text("?"));
        }

        doc = doc
            .append(RcDoc::text(":"))
            .append(RcDoc::space())
            .append(self.type_expr.to_rcdoc_with_context(context)?);

        Ok(doc)
    }
}

impl ToRcDocWithContext for Generic {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if let Some(constraint) = &self.constraint {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(constraint.clone()));
        }

        if let Some(default) = &self.default {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(RcDoc::text(default.clone()));
        }

        Ok(doc)
    }
}

impl ToRcDocWithContext for EnumVariant {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if let Some(value) = &self.value {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(RcDoc::text(value.clone()));
        }

        Ok(doc)
    }
}
