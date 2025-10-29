//! TypeScript type expression definitions

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::ast::PrimitiveType;

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
    /// Convert to TypeScript string representation
    pub fn to_typescript_string(&self) -> String {
        self.to_string()
    }
}
