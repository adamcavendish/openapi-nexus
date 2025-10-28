//! TypeScript enum definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, EnumVariant};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// TypeScript enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_const: bool,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for Enum {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text(if self.is_const {
                "const enum"
            } else {
                "enum"
            }))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add enum body
        if self.variants.is_empty() {
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let variant_docs: Result<Vec<_>, _> = self
                .variants
                .iter()
                .map(|variant| variant.to_rcdoc_with_context(context))
                .collect();
            let variants = variant_docs?;

            let force_multiline = context.force_multiline || self.variants.len() > 2;

            let body_content = if force_multiline {
                RcDoc::intersperse(variants, RcDoc::text(",").append(RcDoc::line()))
            } else {
                RcDoc::intersperse(variants, RcDoc::text(", "))
            };

            doc = doc.append(RcDoc::space()).append(
                RcDoc::text("{")
                    .append(RcDoc::line())
                    .append(body_content)
                    .append(RcDoc::line())
                    .append(RcDoc::text("}")),
            );
        }

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
