//! TypeScript code block AST

use serde::{Deserialize, Serialize};

use crate::ast::Statement;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript code block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeBlock {
    /// The statements in this block
    pub statements: Vec<Statement>,
    /// Whether this is an empty block
    pub is_empty: bool,
}

impl CodeBlock {
    /// Create a new code block with statements
    pub fn new(statements: Vec<Statement>) -> Self {
        Self {
            is_empty: statements.is_empty(),
            statements,
        }
    }

    /// Create an empty code block
    pub fn empty() -> Self {
        Self {
            statements: Vec::new(),
            is_empty: true,
        }
    }
}

impl ToRcDocWithContext for CodeBlock {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.is_empty {
            Ok(RcDoc::text("{}"))
        } else {
            let mut doc = RcDoc::text("{").append(RcDoc::line());

            for stmt in &self.statements {
                doc = doc
                    .append(RcDoc::text("  "))
                    .append(stmt.to_rcdoc_with_context(context)?)
                    .append(RcDoc::line());
            }

            Ok(doc.append(RcDoc::text("}")))
        }
    }
}
