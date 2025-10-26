//! TypeScript generic type parameter definition

use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;

/// TypeScript generic type parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Generic {
    pub name: String,
    pub constraint: Option<TypeExpression>,
    pub default: Option<TypeExpression>,
}
