//! TypeScript parameter definition

use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;

/// TypeScript parameter definition
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub type_expr: Option<TypeExpression>,
    pub optional: bool,
    pub default_value: Option<String>,
}
