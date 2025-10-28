//! TypeScript class definition

use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, Generic, Method, Property};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use pretty::RcDoc;

/// TypeScript class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub generics: Vec<Generic>,
    pub is_export: bool,
    pub documentation: Option<String>,
}

impl ToRcDocWithContext for Class {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();

        let mut doc = if self.is_export {
            utils.export_prefix()
        } else {
            RcDoc::nil()
        };

        doc = doc
            .append(RcDoc::text("class"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        doc = doc.append(utils.generics(&self.generics)?);

        // Add extends clause
        if let Some(extends) = &self.extends {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(extends.clone()));
        }

        // Add implements clause
        if !self.implements.is_empty() {
            let impl_docs: Vec<RcDoc<'static, ()>> = self
                .implements
                .iter()
                .map(|i| RcDoc::text(i.clone()))
                .collect();
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("implements"))
                .append(RcDoc::space())
                .append(utils.comma_separated(impl_docs));
        }

        // Add class body
        let mut body_items = Vec::new();

        // Add properties
        for property in &self.properties {
            body_items.push(property.to_rcdoc_with_context(&context.increment_indent())?);
        }

        // Add methods
        for method in &self.methods {
            body_items.push(method.to_rcdoc_with_context(&context.increment_indent())?);
        }

        if body_items.is_empty() {
            doc = doc.append(RcDoc::space()).append(utils.empty_block());
        } else {
            let body_content = RcDoc::intersperse(body_items, RcDoc::line());
            doc = doc.append(RcDoc::space()).append(utils.block(body_content));
        }

        // Add documentation if present and enabled
        if context.include_docs {
            if let Some(docs) = &self.documentation {
                let doc_comment = DocComment::new(docs.clone());
                doc = doc_comment
                    .to_rcdoc()
                    .unwrap_or_else(|_| RcDoc::text("// Error generating comment"))
                    .append(RcDoc::line())
                    .append(doc);
            }
        }

        Ok(doc)
    }
}
