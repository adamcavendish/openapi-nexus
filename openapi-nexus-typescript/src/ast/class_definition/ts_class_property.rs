use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TsDocComment;
use crate::ast::{TsExpression, TsVisibility};
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript class property for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsClassProperty {
    pub name: String,
    pub type_expr: TsExpression,
    pub visibility: TsVisibility,
    pub is_static: bool,
    pub is_readonly: bool,
    pub optional: bool,
    pub default_value: Option<String>,
    pub documentation: Option<TsDocComment>,
}

impl TsClassProperty {
    /// Create a new class property
    pub fn new(name: String, type_expr: TsExpression) -> Self {
        Self {
            name,
            type_expr,
            visibility: TsVisibility::Public,
            is_static: false,
            is_readonly: false,
            optional: false,
            default_value: None,
            documentation: None,
        }
    }

    /// Set visibility
    pub fn with_visibility(mut self, visibility: TsVisibility) -> Self {
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
    pub fn with_docs(mut self, documentation: TsDocComment) -> Self {
        // Changed to TsDocComment
        self.documentation = Some(documentation); // Updated to use TsDocComment
        self
    }
}

// ToRcDocWithContext implementations
impl ToRcDocWithContext for TsClassProperty {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut parts = Vec::new();

        // Visibility
        match self.visibility {
            TsVisibility::Private => parts.push(RcDoc::text("private")),
            TsVisibility::Protected => parts.push(RcDoc::text("protected")),
            TsVisibility::Public => {}
        }

        // Static
        if self.is_static {
            parts.push(RcDoc::text("static"));
        }

        // Readonly
        if self.is_readonly {
            parts.push(RcDoc::text("readonly"));
        }

        // Property name and type
        let mut name_doc = RcDoc::text(self.name.clone());
        if self.optional {
            name_doc = name_doc.append(RcDoc::text("?"));
        }
        name_doc = name_doc
            .append(RcDoc::text(":"))
            .append(RcDoc::space())
            .append(self.type_expr.to_rcdoc_with_context(context)?);

        parts.push(name_doc);

        // Default value
        if let Some(default_value) = &self.default_value {
            parts.push(
                RcDoc::text("=")
                    .append(RcDoc::space())
                    .append(RcDoc::text(default_value.clone())),
            );
        }

        Ok(RcDoc::intersperse(parts, RcDoc::space()))
    }
}
