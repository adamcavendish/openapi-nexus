//! TypeScript class definition

use serde::{Deserialize, Serialize};

use crate::ast::{
    DocComment, ExtendsClause, Generic, GenericList, ImplementsClause, Property, TsMethod,
};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub methods: Vec<TsMethod>,
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
        let mut doc = if self.is_export {
            RcDoc::text("export ")
        } else {
            RcDoc::nil()
        };

        doc = doc
            .append(RcDoc::text("class"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        let generic_list = GenericList::new(self.generics.clone());
        doc = doc.append(generic_list.to_rcdoc_with_context(context)?);

        // Add extends clause
        if let Some(extends) = &self.extends {
            let extends_clause = ExtendsClause::single(extends.clone());
            doc = doc.append(extends_clause.to_rcdoc_with_context(context)?);
        }

        // Add implements clause
        if !self.implements.is_empty() {
            let implements_clause = ImplementsClause::new(self.implements.clone());
            doc = doc.append(implements_clause.to_rcdoc_with_context(context)?);
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
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let body_content = RcDoc::intersperse(body_items, RcDoc::line());
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
                .to_rcdoc()
                .unwrap_or_else(|_| RcDoc::text("// Error generating comment"))
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }
}
