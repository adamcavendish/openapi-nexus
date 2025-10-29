//! TypeScript primitive type definitions

use serde::{Deserialize, Serialize};

/// TypeScript primitive types
#[derive(Debug, Clone, Ord, PartialOrd, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum TsPrimitiveType {
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
