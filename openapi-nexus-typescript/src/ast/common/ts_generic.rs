use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript generic parameter definition
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TsGeneric {
    pub name: String,
    pub constraint: Option<String>,
    pub default: Option<String>,
}

impl TsGeneric {
    /// Create a new generic parameter
    pub fn new(name: String) -> Self {
        Self {
            name,
            constraint: None,
            default: None,
        }
    }

    /// Add constraint (extends clause)
    pub fn with_constraint(mut self, constraint: String) -> Self {
        self.constraint = Some(constraint);
        self
    }

    /// Add default type
    pub fn with_default(mut self, default: String) -> Self {
        self.default = Some(default);
        self
    }
}

impl ToRcDocWithContext for TsGeneric {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if let Some(constraint) = &self.constraint {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(constraint.clone()));
        }

        if let Some(default) = &self.default {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("="))
                .append(RcDoc::space())
                .append(RcDoc::text(default.clone()));
        }

        Ok(doc)
    }
}
