//! TypeScript property definition

use serde::{Deserialize, Serialize};

use crate::ast::TypeExpression;

/// TypeScript property definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub documentation: Option<String>,
}
