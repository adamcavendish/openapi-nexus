use serde::{Deserialize, Serialize};

/// TypeScript visibility modifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TsVisibility {
    Public,
    Private,
    Protected,
}

impl Default for TsVisibility {
    fn default() -> Self {
        Self::Public
    }
}
