//! TypeScript generic type parameter list

use serde::{Deserialize, Serialize};

use crate::ast::Generic;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript generic type parameter list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenericList(pub Vec<Generic>);

impl GenericList {
    /// Create a new generic list
    pub fn new(generics: Vec<Generic>) -> Self {
        Self(generics)
    }

    /// Create an empty generic list
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl ToRcDocWithContext for GenericList {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.0.is_empty() {
            Ok(RcDoc::nil())
        } else {
            let generic_docs: Result<Vec<RcDoc<'static, ()>>, EmitError> = self
                .0
                .iter()
                .map(|g| g.to_rcdoc_with_context(_context))
                .collect();
            let docs = generic_docs?;

            Ok(RcDoc::text("<")
                .append(RcDoc::intersperse(docs, RcDoc::text(", ")))
                .append(RcDoc::text(">")))
        }
    }
}
