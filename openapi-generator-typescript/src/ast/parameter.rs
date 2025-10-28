//! TypeScript parameter definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript parameter definition
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_expr: Option<TypeExpression>,
    pub optional: bool,
    pub default_value: Option<String>,
}

impl ToRcDocWithContext for Parameter {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if self.optional {
            doc = doc.append(RcDoc::text("?"));
        }

        if let Some(type_expr) = &self.type_expr {
            let type_emitter = TypeExpressionEmitter;
            let type_doc = type_emitter.emit_type_expression_doc(type_expr)?;
            doc = doc.append(RcDoc::text(": ")).append(type_doc);
        }

        if let Some(default_value) = &self.default_value {
            doc = doc
                .append(RcDoc::text(" = "))
                .append(RcDoc::text(default_value.clone()));
        }

        Ok(doc)
    }
}
