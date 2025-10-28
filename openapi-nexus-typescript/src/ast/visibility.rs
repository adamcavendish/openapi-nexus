//! TypeScript visibility modifier definitions

use serde::{Deserialize, Serialize};

/// TypeScript visibility modifiers
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Visibility {
    Public,
    Private,
    Protected,
}
