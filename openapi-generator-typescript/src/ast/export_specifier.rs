//! TypeScript export specifier definitions

use serde::{Deserialize, Serialize};

/// TypeScript export specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportSpecifier {
    Named(String),
    Default(String),
    All(String),
    From(String, Vec<String>),
}
