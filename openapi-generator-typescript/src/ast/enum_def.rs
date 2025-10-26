//! TypeScript enum definition

use serde::{Deserialize, Serialize};

use crate::ast::EnumVariant;

/// TypeScript enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_const: bool,
    pub documentation: Option<String>,
}
