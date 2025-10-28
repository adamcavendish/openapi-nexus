//! TypeScript interface definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, Generic, Property};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for Interface {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();

        let mut doc = utils
            .export_prefix()
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        doc = doc.append(utils.generics(&self.generics)?);

        // Add extends clause
        doc = doc.append(utils.extends_clause(&self.extends));

        // Add body with properties
        if self.properties.is_empty() {
            doc = doc.append(RcDoc::space()).append(utils.empty_block());
        } else {
            let prop_docs: Result<Vec<_>, _> = self
                .properties
                .iter()
                .map(|p| p.to_rcdoc_with_context(&context.increment_indent()))
                .collect();
            let properties = prop_docs?;

            let force_multiline = context.force_multiline
                || utils.should_format_multiline(
                    self.properties.len(),
                    self.properties
                        .iter()
                        .any(|p| TypeExpressionEmitter::is_complex_type(&p.type_expr)),
                );

            let body_content = if force_multiline {
                utils.comma_separated_breakable(properties)
            } else {
                utils.comma_separated(properties)
            };

            doc = doc.append(RcDoc::space()).append(utils.block(body_content));
        }

        // Add documentation if present and enabled
        if context.include_docs
            && let Some(docs) = &self.documentation
        {
            let doc_comment = DocComment::new(docs.clone());
            doc = doc_comment
                .to_rcdoc()
                .unwrap_or_else(|_| RcDoc::text("// Error generating comment"))
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }
}
