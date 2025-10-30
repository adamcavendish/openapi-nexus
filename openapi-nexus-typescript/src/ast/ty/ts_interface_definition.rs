use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TsDocComment;
use crate::ast::{TsGeneric, TsProperty};
use crate::emission::error::EmitError;
use crate::emission::ts_type_emitter::TsTypeEmitter;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsInterfaceDefinition {
    pub name: String,
    pub properties: Vec<TsProperty>,
    pub extends: Vec<String>,
    pub generics: Vec<TsGeneric>,
    pub documentation: Option<TsDocComment>,
}

impl TsInterfaceDefinition {
    /// Create a new interface
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: Vec::new(),
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: None,
        }
    }

    /// Add a property
    pub fn with_property(mut self, property: TsProperty) -> Self {
        self.properties.push(property);
        self
    }

    /// Add multiple properties
    pub fn with_properties(mut self, properties: Vec<TsProperty>) -> Self {
        self.properties.extend(properties);
        self
    }

    /// Add extends clause
    pub fn with_extends(mut self, extends: Vec<String>) -> Self {
        self.extends = extends;
        self
    }

    /// Add generics
    pub fn with_generics(mut self, generics: Vec<TsGeneric>) -> Self {
        self.generics = generics;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: TsDocComment) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl ToRcDocWithContext for TsInterfaceDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        if !self.generics.is_empty() {
            let generic_docs: Result<Vec<_>, _> = self
                .generics
                .iter()
                .map(|g| g.to_rcdoc_with_context(context))
                .collect();
            let generic_strings: Vec<String> = generic_docs?
                .iter()
                .map(|doc| format!("{}", doc.pretty(80)))
                .collect();
            doc = doc.append(RcDoc::text(format!("<{}>", generic_strings.join(", "))));
        }

        // Add extends clause
        if !self.extends.is_empty() {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(self.extends.join(", ")));
        }

        // Add body with properties
        if self.properties.is_empty() {
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let prop_docs: Result<Vec<_>, _> = self
                .properties
                .iter()
                .map(|p| {
                    let type_emitter = TsTypeEmitter;
                    let mut property_line = RcDoc::text(p.name.clone());

                    if p.optional {
                        property_line = property_line.append(RcDoc::text("?"));
                    }

                    let type_doc = type_emitter.emit_type_expression_doc(&p.type_expr)?;
                    property_line = property_line.append(RcDoc::text(": ")).append(type_doc);

                    Ok(property_line)
                })
                .collect();
            let properties = prop_docs?;

            let force_multiline = self.properties.len() > 2
                || self
                    .properties
                    .iter()
                    .any(|p| TsTypeEmitter::is_complex_type(&p.type_expr));

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
                    .append(RcDoc::text("}")),
            );
        }

        // Add documentation if present
        if let Some(docs) = &self.documentation {
            doc = docs
                .to_rcdoc_with_context(context)?
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }
}
