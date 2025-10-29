//! TypeScript type expression definitions

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::TsPrimitiveType;
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript type expression
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TsTypeExpression {
    Primitive(TsPrimitiveType),
    Union(BTreeSet<TsTypeExpression>),
    Intersection(BTreeSet<TsTypeExpression>),
    Array(Box<TsTypeExpression>),
    Object(BTreeMap<String, TsTypeExpression>),
    Reference(String),
    Generic(String),
    Function {
        parameters: Vec<String>,
        return_type: Option<Box<TsTypeExpression>>,
    },
    Literal(String),
    IndexSignature(String, Box<TsTypeExpression>),
    Tuple(Vec<TsTypeExpression>),
}

impl fmt::Display for TsTypeExpression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TsTypeExpression::Primitive(primitive) => match primitive {
                TsPrimitiveType::String => write!(f, "string"),
                TsPrimitiveType::Number => write!(f, "number"),
                TsPrimitiveType::Boolean => write!(f, "boolean"),
                TsPrimitiveType::Any => write!(f, "any"),
                TsPrimitiveType::Unknown => write!(f, "unknown"),
                TsPrimitiveType::Void => write!(f, "void"),
                TsPrimitiveType::Never => write!(f, "never"),
                TsPrimitiveType::Null => write!(f, "null"),
                TsPrimitiveType::Undefined => write!(f, "undefined"),
            },
            TsTypeExpression::Reference(name) => write!(f, "{}", name),
            TsTypeExpression::Array(item_type) => write!(f, "Array<{}>", item_type),
            TsTypeExpression::Union(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", type_strings.join(" | "))
            }
            TsTypeExpression::Intersection(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "{}", type_strings.join(" & "))
            }
            TsTypeExpression::Function {
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
            TsTypeExpression::Object(properties) => {
                let prop_strings: Vec<String> = properties
                    .iter()
                    .map(|(name, type_expr)| format!("{}: {}", name, type_expr))
                    .collect();
                write!(f, "{{ {} }}", prop_strings.join("; "))
            }
            TsTypeExpression::Tuple(types) => {
                let type_strings: Vec<String> = types.iter().map(|t| t.to_string()).collect();
                write!(f, "[{}]", type_strings.join(", "))
            }
            TsTypeExpression::Literal(value) => write!(f, "{}", value),
            TsTypeExpression::Generic(name) => write!(f, "{}", name),
            TsTypeExpression::IndexSignature(key, value_type) => {
                write!(f, "[{}: {}]", key, value_type)
            }
        }
    }
}

impl TsTypeExpression {}

impl ToRcDocWithContext for TsTypeExpression {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let doc = match self {
            TsTypeExpression::Primitive(primitive) => {
                let s = match primitive {
                    TsPrimitiveType::String => "string",
                    TsPrimitiveType::Number => "number",
                    TsPrimitiveType::Boolean => "boolean",
                    TsPrimitiveType::Any => "any",
                    TsPrimitiveType::Unknown => "unknown",
                    TsPrimitiveType::Void => "void",
                    TsPrimitiveType::Never => "never",
                    TsPrimitiveType::Null => "null",
                    TsPrimitiveType::Undefined => "undefined",
                };
                RcDoc::text(s)
            }
            TsTypeExpression::Reference(name) | TsTypeExpression::Generic(name) => {
                RcDoc::text(name.clone())
            }
            TsTypeExpression::Array(item_type) => RcDoc::text("Array")
                .append(RcDoc::text("<"))
                .append(item_type.to_rcdoc_with_context(_context)?)
                .append(RcDoc::text(">")),
            TsTypeExpression::Union(types) => {
                let docs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| t.to_rcdoc_with_context(_context))
                    .collect();
                RcDoc::intersperse(
                    docs?,
                    RcDoc::space()
                        .append(RcDoc::text("|"))
                        .append(RcDoc::space()),
                )
            }
            TsTypeExpression::Intersection(types) => {
                let docs: Result<Vec<_>, _> = types
                    .iter()
                    .map(|t| t.to_rcdoc_with_context(_context))
                    .collect();
                RcDoc::intersperse(
                    docs?,
                    RcDoc::space()
                        .append(RcDoc::text("&"))
                        .append(RcDoc::space()),
                )
            }
            TsTypeExpression::Function {
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
            TsTypeExpression::Object(properties) => {
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
            TsTypeExpression::Tuple(types) => {
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
            TsTypeExpression::Literal(value) => RcDoc::text(value.clone()),
            TsTypeExpression::IndexSignature(key, value_type) => RcDoc::text("[")
                .append(RcDoc::text(key.clone()))
                .append(RcDoc::text(":"))
                .append(RcDoc::space())
                .append(value_type.to_rcdoc_with_context(_context)?)
                .append(RcDoc::text("]")),
        };
        Ok(doc)
    }
}
