//! TypeScript class definition

use serde::{Deserialize, Serialize};

use crate::ast::{Generic, Method, Property};

/// TypeScript class definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Class {
    pub name: String,
    pub properties: Vec<Property>,
    pub methods: Vec<Method>,
    pub extends: Option<String>,
    pub implements: Vec<String>,
    pub generics: Vec<Generic>,
    pub is_export: bool,
    pub documentation: Option<String>,
}
