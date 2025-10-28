//! TypeScript type alias definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, Generic, GenericList, TypeExpression};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript type alias definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub type_expr: TypeExpression,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for TypeAlias {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let type_emitter = TypeExpressionEmitter;

        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text("type"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        let generic_list = GenericList::new(self.generics.clone());
        doc = doc.append(generic_list.to_rcdoc_with_context(context)?);

        // Add type expression
        let type_doc = type_emitter.emit_type_expression_doc(&self.type_expr)?;
        doc = doc.append(RcDoc::text(" = ")).append(type_doc);

        // Add documentation if present and enabled
        if context.include_docs
            && let Some(docs) = &self.documentation
        {
            let doc_comment = DocComment::new(docs.clone());
            doc = doc_comment
                .to_rcdoc_with_context(context)?
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }
}
