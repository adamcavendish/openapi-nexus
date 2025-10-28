//! TypeScript enum variant definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::DocComment;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// TypeScript enum variant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for EnumVariant {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut variant_doc = RcDoc::text(self.name.clone());
        if let Some(value) = &self.value {
            variant_doc = variant_doc
                .append(RcDoc::text(" = "))
                .append(RcDoc::text(format!("\"{}\"", value)));
        }

        // Add documentation if present and enabled
        if context.include_docs
            && let Some(docs) = &self.documentation
        {
            let doc_comment = DocComment::new(docs.clone());
            return Ok(doc_comment
                .to_rcdoc_with_context(context)?
                .append(RcDoc::line())
                .append(variant_doc));
        }

        Ok(variant_doc)
    }
}
