//! TypeScript primitive type definitions

use serde::{Deserialize, Serialize};

/// TypeScript primitive types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
    Any,
    Unknown,
    Void,
    Never,
}
