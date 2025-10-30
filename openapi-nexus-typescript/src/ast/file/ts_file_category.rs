use serde::{Deserialize, Serialize};

/// File category for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TsFileCategory {
    /// API client classes
    Api,
    /// Type definitions and interfaces
    Models,
    /// Runtime support files
    Runtime,
    /// Configuration files
    Config,
    /// Utility files
    Utils,
}
