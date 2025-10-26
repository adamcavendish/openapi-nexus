//! TypeScript enum variant definition

use serde::{Deserialize, Serialize};

/// TypeScript enum variant definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumVariant {
    pub name: String,
    pub value: Option<String>,
    pub documentation: Option<String>,
}
