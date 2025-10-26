//! TypeScript type expression definitions

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::ast::{FunctionSignature, PrimitiveType};

/// TypeScript type expression
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Union(Vec<TypeExpression>),
    Intersection(Vec<TypeExpression>),
    Array(Box<TypeExpression>),
    Object(HashMap<String, TypeExpression>),
    Reference(String),
    Generic(String),
    Function(Box<FunctionSignature>),
    Literal(String),
    IndexSignature(String, Box<TypeExpression>),
    Tuple(Vec<TypeExpression>),
}
