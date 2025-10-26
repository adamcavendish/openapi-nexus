//! TypeScript export statement definition

use serde::{Deserialize, Serialize};

use crate::ast::ExportSpecifier;

/// TypeScript export statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub specifier: ExportSpecifier,
    pub is_type_only: bool,
}
