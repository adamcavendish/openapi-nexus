//! TypeScript interface definition

use serde::{Deserialize, Serialize};

use crate::ast::{Generic, Property};

/// TypeScript interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}
