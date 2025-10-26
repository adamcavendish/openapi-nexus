//! TypeScript import statement definition

use serde::{Deserialize, Serialize};

use crate::ast::ImportSpecifier;

/// TypeScript import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub imports: Vec<ImportSpecifier>,
    pub is_type_only: bool,
}
