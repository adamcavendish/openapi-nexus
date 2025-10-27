//! TypeScript type expression definitions

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

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
