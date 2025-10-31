use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{TsClassDefinition, TsGeneric};
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript class signature (single-line declaration header)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsClassSignature {
    pub is_export: bool,
    pub name: String,
    pub generics: Vec<TsGeneric>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
}

impl TsClassSignature {
    /// Create a signature from a class definition
    pub fn from_class(class: &TsClassDefinition) -> Self {
        class.signature.clone()
    }
}

impl ToRcDocWithContext for TsClassSignature {
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
            .append(RcDoc::text("class"))
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

        if let Some(ext) = &self.extends {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(ext.clone()));
        }

        if !self.implements.is_empty() {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("implements"))
                .append(RcDoc::space())
                .append(RcDoc::text(self.implements.join(",")))
        }

        Ok(doc)
    }
}
