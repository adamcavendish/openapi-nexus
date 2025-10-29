//! TypeScript type expression definitions

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::PrimitiveType;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// TypeScript type expression
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Union(BTreeSet<TypeExpression>),
    Intersection(BTreeSet<TypeExpression>),
    Array(Box<TypeExpression>),
    Object(BTreeMap<String, TypeExpression>),
    Reference(String),
    Generic(String),
    Function {
        parameters: Vec<String>,
        return_type: Option<Box<TypeExpression>>,
    },
    Literal(String),
    IndexSignature(String, Box<TypeExpression>),
    Tuple(Vec<TypeExpression>),
}

impl fmt::Display for TypeExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TypeExpression::Primitive(primitive) => match primitive {
                PrimitiveType::String => write!(f, "string"),
                PrimitiveType::Number => write!(f, "number"),
                PrimitiveType::Boolean => write!(f, "boolean"),
                PrimitiveType::Any => write!(f, "any"),
                PrimitiveType::Unknown => write!(f, "unknown"),
                PrimitiveType::Void => write!(f, "void"),
                PrimitiveType::Never => write!(f, "never"),
                PrimitiveType::Null => write!(f, "null"),
                PrimitiveType::Undefined => write!(f, "undefined"),
            },
            TypeExpression::Reference(name) => write!(f, "{}", name),
            TypeExpression::Array(item_type) => write!(f, "Array<{}>", item_type),
            TypeExpression::Union(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", type_strings.join(" | "))
            }
            TypeExpression::Intersection(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", type_strings.join(" & "))
            }
            TypeExpression::Function {
                parameters,
                return_type,
            } => {
                let return_type_str = if let Some(ret_type) = return_type {
                    ret_type.to_string()
                } else {
                    "void".to_string()
                };
                write!(f, "({}) => {}", parameters.join(", "), return_type_str)
            }
            TypeExpression::Object(properties) => {
                let prop_strings: Vec<String> = properties
                    .iter()
                    .map(|(name, type_expr)| format!("{}: {}", name, type_expr))
                    .collect();
                write!(f, "{{ {} }}", prop_strings.join("; "))
            }
            TypeExpression::Tuple(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "[{}]", type_strings.join(", "))
            }
            TypeExpression::Literal(value) => write!(f, "{}", value),
            TypeExpression::Generic(name) => write!(f, "{}", name),
            TypeExpression::IndexSignature(key, value_type) => {
                write!(f, "[{}: {}]", key, value_type)
            }
        }
    }
}

impl TypeExpression {
}

impl ToRcDocWithContext for TypeExpression {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let doc = match self {
            TypeExpression::Primitive(primitive) => {
                let s = match primitive {
                    PrimitiveType::String => "string",
                    PrimitiveType::Number => "number",
                    PrimitiveType::Boolean => "boolean",
                    PrimitiveType::Any => "any",
                    PrimitiveType::Unknown => "unknown",
                    PrimitiveType::Void => "void",
                    PrimitiveType::Never => "never",
                    PrimitiveType::Null => "null",
                    PrimitiveType::Undefined => "undefined",
                };
                RcDoc::text(s)
            }
            TypeExpression::Reference(name) | TypeExpression::Generic(name) => {
                RcDoc::text(name.clone())
            }
            TypeExpression::Array(item_type) => RcDoc::text("Array")
                .append(RcDoc::text("<"))
                .append(item_type.to_rcdoc_with_context(_context)?)
                .append(RcDoc::text(">")),
            TypeExpression::Union(types) => {
                let docs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| t.to_rcdoc_with_context(_context))
                    .collect();
                RcDoc::intersperse(docs?, RcDoc::space().append(RcDoc::text("|")).append(RcDoc::space()))
            }
            TypeExpression::Intersection(types) => {
                let docs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| t.to_rcdoc_with_context(_context))
                    .collect();
                RcDoc::intersperse(docs?, RcDoc::space().append(RcDoc::text("&")).append(RcDoc::space()))
            }
            TypeExpression::Function {
                parameters,
                return_type,
            } => {
                let params = RcDoc::text("(")
                    .append(RcDoc::intersperse(
                        parameters.iter().map(|p| RcDoc::text(p.clone())),
                        RcDoc::text(",").append(RcDoc::space()),
                    ))
                    .append(RcDoc::text(")"));
                let ret = if let Some(ret_type) = return_type {
                    ret_type.to_rcdoc_with_context(_context)?
                } else {
                    RcDoc::text("void")
                };
                params
                    .append(RcDoc::space())
                    .append(RcDoc::text("=>"))
                    .append(RcDoc::space())
                    .append(ret)
            }
            TypeExpression::Object(properties) => {
                if properties.is_empty() {
                    RcDoc::text("{}")
                } else {
                    let prop_docs: Result<Vec<_>, _> = properties
                        .iter()
                        .map(|(name, type_expr)| {
                            Ok(RcDoc::text(name.clone())
                                .append(RcDoc::text(":"))
                                .append(RcDoc::space())
                                .append(type_expr.to_rcdoc_with_context(_context)?))
                        })
                        .collect();
                    RcDoc::text("{")
                        .append(RcDoc::space())
                        .append(RcDoc::intersperse(
                            prop_docs?,
                            RcDoc::text(";").append(RcDoc::space()),
                        ))
                        .append(RcDoc::space())
                        .append(RcDoc::text("}"))
                }
            }
            TypeExpression::Tuple(types) => {
                let docs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| t.to_rcdoc_with_context(_context))
                    .collect();
                RcDoc::text("[")
                    .append(RcDoc::intersperse(
                        docs?,
                        RcDoc::text(",").append(RcDoc::space()),
                    ))
                    .append(RcDoc::text("]"))
            }
            TypeExpression::Literal(value) => RcDoc::text(value.clone()),
            TypeExpression::IndexSignature(key, value_type) => RcDoc::text("[")
                .append(RcDoc::text(key.clone()))
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(value_type.to_rcdoc_with_context(_context)?)
                .append(RcDoc::text("]")),
        };
        Ok(doc)
    }
}
