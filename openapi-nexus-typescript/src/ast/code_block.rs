//! TypeScript code block AST

use serde::{Deserialize, Serialize};

use crate::ast::Statement;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// Template-generated code snippets that only need indentation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnippetLines {
    /// Method body snippets (constructors, API methods, etc.)
    MethodBody(Vec<String>),
    /// Interface property snippets
    InterfaceBody(Vec<String>),
    /// Class member snippets  
    ClassBody(Vec<String>),
    /// Function body snippets
    FunctionBody(Vec<String>),
    /// Enum variant snippets
    EnumBody(Vec<String>),
    /// Generic code block snippets
    Generic(Vec<String>),
}

/// TypeScript code block - either statements or pre-rendered snippets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CodeBlock {
    /// Traditional AST statements requiring full processing
    Statements(Vec<Statement>),
    /// Pre-rendered template lines needing only indentation
    Snippets(SnippetLines),
}

impl CodeBlock {
    /// Create a new code block with statements
    pub fn from_statements(statements: Vec<Statement>) -> Self {
        Self::Statements(statements)
    }

    /// Create a code block from template-generated snippets
    pub fn from_snippets(snippets: SnippetLines) -> Self {
        Self::Snippets(snippets)
    }

    /// Create an empty code block
    pub fn empty() -> Self {
        Self::Statements(Vec::new())
    }

    /// Check if this code block is empty
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Statements(statements) => statements.is_empty(),
            Self::Snippets(snippets) => match snippets {
                SnippetLines::MethodBody(lines) => lines.is_empty(),
                SnippetLines::InterfaceBody(lines) => lines.is_empty(),
                SnippetLines::ClassBody(lines) => lines.is_empty(),
                SnippetLines::FunctionBody(lines) => lines.is_empty(),
                SnippetLines::EnumBody(lines) => lines.is_empty(),
                SnippetLines::Generic(lines) => lines.is_empty(),
            },
        }
    }
}

impl ToRcDocWithContext for CodeBlock {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            CodeBlock::Statements(statements) => {
                if statements.is_empty() {
                    Ok(RcDoc::text("{}"))
                } else {
                    let mut doc = RcDoc::text("{").append(RcDoc::line());

                    for stmt in statements {
                        doc = doc
                            .append(RcDoc::text("  "))
                            .append(stmt.to_rcdoc_with_context(context)?)
                            .append(RcDoc::line());
                    }

                    Ok(doc.append(RcDoc::text("}")))
                }
            }
            CodeBlock::Snippets(snippets) => {
                let lines = match snippets {
                    SnippetLines::MethodBody(lines) => lines,
                    SnippetLines::InterfaceBody(lines) => lines,
                    SnippetLines::ClassBody(lines) => lines,
                    SnippetLines::FunctionBody(lines) => lines,
                    SnippetLines::EnumBody(lines) => lines,
                    SnippetLines::Generic(lines) => lines,
                };

                if lines.is_empty() {
                    Ok(RcDoc::text("{}"))
                } else {
                    let mut doc = RcDoc::text("{").append(RcDoc::line());

                    for line in lines {
                        doc = doc
                            .append(RcDoc::text("  "))
                            .append(RcDoc::text(line.clone()))
                            .append(RcDoc::line());
                    }

                    Ok(doc.append(RcDoc::text("}")))
                }
            }
        }
    }
}
