//! TypeScript type alias definition

use serde::{Deserialize, Serialize};

use crate::ast::{Generic, TypeExpression};

/// TypeScript type alias definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAlias {
    pub name: String,
    pub type_expr: TypeExpression,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}
