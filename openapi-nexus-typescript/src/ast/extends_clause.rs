//! TypeScript extends clause

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript extends clause
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtendsClause(pub Vec<String>);

impl ExtendsClause {
    /// Create a new extends clause
    pub fn new(extends: Vec<String>) -> Self {
        Self(extends)
    }

    /// Create an empty extends clause
    pub fn empty() -> Self {
        Self(Vec::new())
    }

    /// Create an extends clause with a single type
    pub fn single(extends: impl Into<String>) -> Self {
        Self(vec![extends.into()])
    }
}

impl ToRcDocWithContext for ExtendsClause {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.0.is_empty() {
            Ok(RcDoc::nil())
        } else {
            let extend_docs: Vec<RcDoc<'static, ()>> =
                self.0.iter().map(|e| RcDoc::text(e.clone())).collect();
            Ok(RcDoc::space()
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::intersperse(extend_docs, RcDoc::text(", "))))
        }
    }
}
