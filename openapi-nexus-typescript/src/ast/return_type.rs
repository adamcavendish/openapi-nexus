//! TypeScript return type annotation

use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;
use pretty::RcDoc;

/// TypeScript return type annotation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnType(pub Option<TypeExpression>);

impl ReturnType {
    /// Create a new return type annotation
    pub fn new(type_expr: Option<TypeExpression>) -> Self {
        Self(type_expr)
    }

    /// Create a return type with no annotation
    pub fn none() -> Self {
        Self(None)
    }
}

impl ToRcDocWithContext for ReturnType {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match &self.0 {
            Some(type_expr) => {
                let type_emitter = TypeExpressionEmitter;
                let type_doc = type_emitter.emit_type_expression_doc(type_expr)?;
                Ok(RcDoc::text(": ").append(type_doc))
            }
            None => Ok(RcDoc::nil()),
        }
    }
}
