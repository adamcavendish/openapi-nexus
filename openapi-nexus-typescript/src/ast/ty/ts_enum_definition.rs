use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{TsDocComment, TsEnumVariant};
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsEnumDefinition {
    pub name: String,
    pub variants: Vec<TsEnumVariant>,
    pub is_const: bool,
    pub documentation: Option<TsDocComment>,
}

impl TsEnumDefinition {
    /// Create a new enum
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: Vec::new(),
            is_const: false,
            documentation: None,
        }
    }

    /// Create a const enum
    pub fn new_const(name: String) -> Self {
        Self {
            name,
            variants: Vec::new(),
            is_const: true,
            documentation: None,
        }
    }

    /// Add a variant
    pub fn with_variant(mut self, variant: TsEnumVariant) -> Self {
        self.variants.push(variant);
        self
    }

    /// Add multiple variants
    pub fn with_variants(mut self, variants: Vec<TsEnumVariant>) -> Self {
        self.variants.extend(variants);
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: TsDocComment) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl ToRcDocWithContext for TsEnumDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
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
            let variant_docs: Vec<RcDoc<'static, ()>> = self
                .variants
                .iter()
                .map(|variant| {
                    let mut variant_doc = RcDoc::text(variant.name.clone());
                    if let Some(value) = &variant.value {
                        variant_doc = variant_doc
                            .append(RcDoc::text(" = "))
                            .append(RcDoc::text(value.clone()));
                    }
                    variant_doc
                })
                .collect();

            let force_multiline = self.variants.len() > 2;

            let body_content = if force_multiline {
                RcDoc::intersperse(variant_docs, RcDoc::text(",").append(RcDoc::line()))
            } else {
                RcDoc::intersperse(variant_docs, RcDoc::text(", "))
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
                .to_rcdoc_with_context(_context)?
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }
}
