use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TsGeneric;
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript interface signature (single-line declaration header)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsInterfaceSignature {
    pub is_export: bool,
    pub name: String,
    pub generics: Vec<TsGeneric>,
    pub extends: Vec<String>,
}

impl TsInterfaceSignature {
    pub fn new(name: String) -> Self {
        Self {
            is_export: true,
            name,
            generics: Vec::new(),
            extends: Vec::new(),
        }
    }

    pub fn with_generics(mut self, generics: Vec<TsGeneric>) -> Self {
        self.generics = generics;
        self
    }

    pub fn with_extends(mut self, extends: Vec<String>) -> Self {
        self.extends = extends;
        self
    }

    pub fn with_export(mut self, is_export: bool) -> Self {
        self.is_export = is_export;
        self
    }
}

impl ToRcDocWithContext for TsInterfaceSignature {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::nil();

        if self.is_export {
            doc = doc.append(RcDoc::text("export")).append(RcDoc::space());
        }

        doc = doc
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        if !self.generics.is_empty() {
            let generics_docs = self
                .generics
                .iter()
                .map(|g| g.to_rcdoc_with_context(context))
                .collect::<Result<Vec<_>, _>>()?;
            doc = doc
                .append(RcDoc::text("<"))
                .append(RcDoc::intersperse(
                    generics_docs,
                    RcDoc::text(",").append(RcDoc::space()),
                ))
                .append(RcDoc::text(">"));
        }

        if !self.extends.is_empty() {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(self.extends.join(",")));
        }

        Ok(doc)
    }
}
