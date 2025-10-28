//! TypeScript statement AST

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// TypeScript statement types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// A simple statement (e.g., variable assignment, function call)
    Simple(String),
    /// A return statement
    Return(Option<Expression>),
    /// An if statement
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    /// A comment statement
    Comment(String),
    /// A block of statements
    Block(Vec<Statement>),
}

/// TypeScript expression types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// A simple expression (string literal)
    Simple(String),
    /// An object literal
    ObjectLiteral(Vec<(String, Option<Box<Expression>>)>),
    /// A function call
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
    /// An object assignment
    ObjectAssignment {
        name: String,
        properties: Vec<(String, Option<Box<Expression>>)>,
    },
}

impl ToRcDocWithContext for Statement {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            Statement::Simple(text) => Ok(RcDoc::text(text.clone())),
            Statement::Return(expr) => {
                if let Some(expr) = expr {
                    Ok(RcDoc::text("return ")
                        .append(expr.to_rcdoc_with_context(context)?)
                        .append(RcDoc::text(";")))
                } else {
                    Ok(RcDoc::text("return;"))
                }
            }
            Statement::If {
                condition,
                then_body,
                else_body,
            } => {
                let mut doc = RcDoc::text("if (")
                    .append(condition.to_rcdoc_with_context(context)?)
                    .append(RcDoc::text(") {"))
                    .append(RcDoc::line());

                // Add then body
                for stmt in then_body {
                    doc = doc
                        .append(RcDoc::text("  "))
                        .append(stmt.to_rcdoc_with_context(context)?)
                        .append(RcDoc::line());
                }

                doc = doc.append(RcDoc::text("}"));

                // Add else body if present
                if let Some(else_body) = else_body {
                    doc = doc.append(RcDoc::text(" else {")).append(RcDoc::line());
                    for stmt in else_body {
                        doc = doc
                            .append(RcDoc::text("  "))
                            .append(stmt.to_rcdoc_with_context(context)?)
                            .append(RcDoc::line());
                    }
                    doc = doc.append(RcDoc::text("}"));
                }

                Ok(doc)
            }
            Statement::Comment(text) => Ok(RcDoc::text("// ").append(RcDoc::text(text.clone()))),
            Statement::Block(statements) => {
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
    }
}

impl ToRcDocWithContext for Expression {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            Expression::Simple(text) => Ok(RcDoc::text(text.clone())),
            Expression::ObjectLiteral(properties) => {
                if properties.is_empty() {
                    return Ok(RcDoc::text("{}"));
                }

                let doc = RcDoc::text("{");
                let prop_docs: Result<Vec<_>, _> = properties
                    .iter()
                    .map(|(key, value)| {
                        if let Some(value) = value {
                            Ok(RcDoc::text(key.clone())
                                .append(RcDoc::text(": "))
                                .append(value.to_rcdoc_with_context(_context)?))
                        } else {
                            Ok(RcDoc::text(key.clone()))
                        }
                    })
                    .collect();
                let props = prop_docs?;

                if props.len() == 1 {
                    Ok(doc.append(props[0].clone()).append(RcDoc::text("}")))
                } else {
                    let mut result = doc.append(RcDoc::line());
                    for prop in props {
                        result = result
                            .append(RcDoc::text("  "))
                            .append(prop)
                            .append(RcDoc::text(","))
                            .append(RcDoc::line());
                    }
                    Ok(result.append(RcDoc::text("}")))
                }
            }
            Expression::FunctionCall { name, arguments } => {
                let arg_docs: Result<Vec<_>, _> = arguments
                    .iter()
                    .map(|arg| arg.to_rcdoc_with_context(_context))
                    .collect();
                let args = arg_docs?;

                Ok(RcDoc::text(name.clone())
                    .append(RcDoc::text("("))
                    .append(RcDoc::intersperse(args, RcDoc::text(", ")))
                    .append(RcDoc::text(")")))
            }
            Expression::ObjectAssignment { name, properties } => {
                let doc = RcDoc::text(name.clone()).append(RcDoc::text(" = {"));
                if properties.is_empty() {
                    return Ok(doc.append(RcDoc::text("}")));
                }

                let prop_docs: Result<Vec<_>, _> = properties
                    .iter()
                    .map(|(key, value)| {
                        if let Some(value) = value {
                            Ok(RcDoc::text(key.clone())
                                .append(RcDoc::text(": "))
                                .append(value.to_rcdoc_with_context(_context)?))
                        } else {
                            Ok(RcDoc::text(key.clone()))
                        }
                    })
                    .collect();
                let props = prop_docs?;

                if props.len() == 1 {
                    Ok(doc.append(props[0].clone()).append(RcDoc::text("}")))
                } else {
                    let mut result = doc.append(RcDoc::line());
                    for prop in props {
                        result = result
                            .append(RcDoc::text("  "))
                            .append(prop)
                            .append(RcDoc::text(","))
                            .append(RcDoc::line());
                    }
                    Ok(result.append(RcDoc::text("}")))
                }
            }
        }
    }
}
