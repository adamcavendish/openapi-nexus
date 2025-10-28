//! TypeScript interface definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, Generic, Property, GenericList, ExtendsClause};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::error::EmitError;
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
        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        let generic_list = GenericList::new(self.generics.clone());
        doc = doc.append(generic_list.to_rcdoc_with_context(context)?);

        // Add extends clause
        if !self.extends.is_empty() {
            let extends_clause = ExtendsClause::new(self.extends.clone());
            doc = doc.append(extends_clause.to_rcdoc_with_context(context)?);
        }

        // Add body with properties
        if self.properties.is_empty() {
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let prop_docs: Result<Vec<_>, _> = self
                .properties
                .iter()
                .map(|p| p.to_rcdoc_with_context(&context.increment_indent()))
                .collect();
            let properties = prop_docs?;

            let force_multiline = context.force_multiline
                || self.properties.len() > 2
                || self.properties
                    .iter()
                    .any(|p| TypeExpressionEmitter::is_complex_type(&p.type_expr));

            let body_content = if force_multiline {
                RcDoc::intersperse(properties, RcDoc::text(",").append(RcDoc::line()))
            } else {
                RcDoc::intersperse(properties, RcDoc::text(", "))
            };

            doc = doc.append(RcDoc::space()).append(
                RcDoc::text("{")
                    .append(RcDoc::line())
                    .append(body_content)
                    .append(RcDoc::line())
                    .append(RcDoc::text("}"))
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
