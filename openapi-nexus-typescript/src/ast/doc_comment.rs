//! TypeScript documentation comment AST

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript documentation comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocComment {
    /// The documentation text (can be multiline)
    pub text: String,
    /// Whether this is a multiline comment
    pub is_multiline: bool,
}

impl DocComment {
    /// Create a new documentation comment
    pub fn new(text: String) -> Self {
        Self {
            is_multiline: text.contains('\n'),
            text,
        }
    }

    /// Create a multiline documentation comment
    pub fn multiline(lines: Vec<String>) -> Self {
        Self {
            text: lines.join("\n"),
            is_multiline: true,
        }
    }
}

impl ToRcDocWithContext for DocComment {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.is_multiline {
            let lines: Vec<&str> = self.text.lines().collect();
            let mut doc = RcDoc::text("/**");
            for line in lines {
                doc = doc
                    .append(RcDoc::line())
                    .append(RcDoc::text(" * "))
                    .append(RcDoc::text(line.to_string()));
            }
            Ok(doc.append(RcDoc::line()).append(RcDoc::text(" */")))
        } else {
            Ok(RcDoc::text("/**")
                .append(RcDoc::line())
                .append(RcDoc::text(" * "))
                .append(RcDoc::text(self.text.clone()))
                .append(RcDoc::line())
                .append(RcDoc::text(" */")))
        }
    }
}
