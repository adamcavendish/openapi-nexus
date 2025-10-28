//! TypeScript property definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, TypeExpression};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for Property {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let type_emitter = TypeExpressionEmitter;

        let mut property_line = RcDoc::text(self.name.clone());

        if self.optional {
            property_line = property_line.append(RcDoc::text("?"));
        }

        let type_doc = type_emitter.emit_type_expression_doc(&self.type_expr)?;
        property_line = property_line.append(RcDoc::text(": ")).append(type_doc);

        // Add documentation if present and enabled
        if context.include_docs {
            if let Some(docs) = &self.documentation {
                let doc_comment = DocComment::new(docs.clone());
                return Ok(doc_comment
                    .to_rcdoc_with_context(context)?
                    .append(RcDoc::line())
                    .append(property_line));
            }
        }

        Ok(property_line)
    }
}
