//! TypeScript comment AST

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript comment types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Comment {
    /// Single line comment (// comment)
    Single(String),
    /// Multi-line comment (/* comment */)
    Multi(Vec<String>),
    /// Multi-line single-line comments (// comment\n// comment)
    MultiLine(Vec<String>),
}

impl Comment {
    /// Create a single line comment
    pub fn single(text: String) -> Self {
        Self::Single(text)
    }

    /// Create a multi-line comment
    pub fn multi(lines: Vec<String>) -> Self {
        Self::Multi(lines)
    }

    /// Create a multi-line single-line comment
    pub fn multi_line(lines: Vec<String>) -> Self {
        Self::MultiLine(lines)
    }

    /// Create a comment from a multi-line string
    pub fn from_text(text: &str) -> Self {
        let lines: Vec<String> = text.lines().map(|s| s.to_string()).collect();
        if lines.len() == 1 {
            Self::Single(lines[0].clone())
        } else {
            Self::MultiLine(lines)
        }
    }
}

impl ToRcDocWithContext for Comment {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            Comment::Single(text) => Ok(RcDoc::text("// ")
                .append(RcDoc::text(text.clone()))
                .append(RcDoc::line())),
            Comment::Multi(lines) => {
                let mut doc = RcDoc::text("/*");
                for line in lines {
                    doc = doc
                        .append(RcDoc::line())
                        .append(RcDoc::text(" * "))
                        .append(RcDoc::text(line.clone()));
                }
                Ok(doc
                    .append(RcDoc::line())
                    .append(RcDoc::text(" */"))
                    .append(RcDoc::line()))
            }
            Comment::MultiLine(lines) => {
                let mut doc = RcDoc::nil();
                for line in lines {
                    if line.trim().is_empty() {
                        doc = doc.append(RcDoc::text("//")).append(RcDoc::line());
                    } else {
                        doc = doc
                            .append(RcDoc::text("// "))
                            .append(RcDoc::text(line.clone()))
                            .append(RcDoc::line());
                    }
                }
                Ok(doc)
            }
        }
    }
}
