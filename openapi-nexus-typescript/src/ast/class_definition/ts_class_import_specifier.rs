use serde::{Deserialize, Serialize};

/// Import specifier for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsClassImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
    pub is_type: bool,
}

impl TsClassImportSpecifier {
    /// Create a new import specifier
    pub fn new(name: String) -> Self {
        Self {
            name,
            alias: None,
            is_type: false,
        }
    }

    /// Create a type import specifier
    pub fn new_type(name: String) -> Self {
        Self {
            name,
            alias: None,
            is_type: true,
        }
    }

    /// Set alias
    pub fn with_alias(mut self, alias: String) -> Self {
        self.alias = Some(alias);
        self
    }
}
