use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{TsDocComment, TsExpression};
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsProperty {
    pub name: String,
    pub type_expr: TsExpression,
    pub optional: bool,
    pub documentation: Option<TsDocComment>,
}

impl TsProperty {
    /// Create a new property
    pub fn new(name: String, type_expr: TsExpression) -> Self {
        Self {
            name,
            type_expr,
            optional: false,
            documentation: None,
        }
    }

    /// Create an optional property
    pub fn optional(name: String, type_expr: TsExpression) -> Self {
        Self {
            name,
            type_expr,
            optional: true,
            documentation: None,
        }
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: TsDocComment) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl ToRcDocWithContext for TsProperty {
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
