//! TypeScript generic type parameter definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript generic type parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub constraint: Option<TypeExpression>,
    pub default: Option<TypeExpression>,
}

impl ToRcDocWithContext for Generic {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if let Some(constraint) = &self.constraint {
            let type_emitter = TypeExpressionEmitter;
            let constraint_doc = type_emitter.emit_type_expression_doc(constraint)?;
            doc = doc.append(RcDoc::text(" extends ")).append(constraint_doc);
        }

        if let Some(default) = &self.default {
            let type_emitter = TypeExpressionEmitter;
            let default_doc = type_emitter.emit_type_expression_doc(default)?;
            doc = doc.append(RcDoc::text(" = ")).append(default_doc);
        }

        Ok(doc)
    }
}
