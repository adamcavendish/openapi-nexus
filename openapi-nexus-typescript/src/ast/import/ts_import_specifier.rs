use serde::{Deserialize, Serialize};

/// TypeScript import specifier
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct TsImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
    pub is_type: bool,
}

impl TsImportSpecifier {
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

    /// Make this a type import
    pub fn with_type(mut self) -> Self {
        self.is_type = true;
        self
    }
}
