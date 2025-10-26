//! TypeScript import specifier definition

use serde::{Deserialize, Serialize};

/// TypeScript import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
}
