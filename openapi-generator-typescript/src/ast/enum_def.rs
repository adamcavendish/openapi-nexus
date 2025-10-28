//! TypeScript enum definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, EnumVariant};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

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
        let utils = TypeScriptPrettyUtils::new();

        let mut doc = utils
            .export_prefix()
            .append(RcDoc::text(if self.is_const {
                "const enum"
            } else {
                "enum"
            }))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add enum body
        if self.variants.is_empty() {
            doc = doc.append(RcDoc::space()).append(utils.empty_block());
        } else {
            let variant_docs: Result<Vec<_>, _> = self
                .variants
                .iter()
                .map(|variant| variant.to_rcdoc_with_context(context))
                .collect();
            let variants = variant_docs?;

            let force_multiline = context.force_multiline
                || utils.should_format_multiline(self.variants.len(), false);

            let body_content = if force_multiline {
                utils.comma_separated_breakable(variants)
            } else {
                utils.comma_separated(variants)
            };

            doc = doc.append(RcDoc::space()).append(utils.block(body_content));
        }

        // Add documentation if present and enabled
        if context.include_docs {
            if let Some(docs) = &self.documentation {
                let doc_comment = DocComment::new(docs.clone());
                doc = doc_comment
                    .to_rcdoc_with_context(context)?
                    .append(RcDoc::line())
                    .append(doc);
            }
        }

        Ok(doc)
    }
}
