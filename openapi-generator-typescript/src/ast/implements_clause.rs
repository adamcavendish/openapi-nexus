//! TypeScript implements clause

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript implements clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplementsClause(pub Vec<String>);

impl ImplementsClause {
    /// Create a new implements clause
    pub fn new(implements: Vec<String>) -> Self {
        Self(implements)
    }

    /// Create an empty implements clause
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Create an implements clause with a single interface
    pub fn single(implements: impl Into<String>) -> Self {
        Self(vec![implements.into()])
    }
}

impl ToRcDocWithContext for ImplementsClause {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.0.is_empty() {
            Ok(RcDoc::nil())
        } else {
            let impl_docs: Vec<RcDoc<'static, ()>> = self
                .0
                .iter()
                .map(|i| RcDoc::text(i.clone()))
                .collect();
            Ok(RcDoc::space()
                .append(RcDoc::text("implements"))
                .append(RcDoc::space())
                .append(RcDoc::intersperse(impl_docs, RcDoc::text(", "))))
        }
    }
}
