//! TypeScript type expression definitions

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ast::{FunctionSignature, PrimitiveType};

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
    Function(Box<FunctionSignature>),
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
            TypeExpression::Function(func) => {
                let params: Vec<String> = func
                    .parameters
                    .iter()
                    .map(|param| {
                        let param_type = if let Some(type_expr) = &param.type_expr {
                            type_expr.to_string()
                        } else {
                            "any".to_string()
                        };
                        if param.optional {
                            format!("{}?: {}", param.name, param_type)
                        } else {
                            format!("{}: {}", param.name, param_type)
                        }
                    })
                    .collect();
                let return_type = if let Some(ret_type) = &func.return_type {
                    ret_type.to_string()
                } else {
                    "void".to_string()
                };
                write!(f, "({}) => {}", params.join(", "), return_type)
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
