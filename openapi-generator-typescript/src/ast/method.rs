//! TypeScript method definition

use serde::{Deserialize, Serialize};

use crate::ast::{Parameter, TypeExpression, Visibility};

/// TypeScript method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Method {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub is_async: bool,
    pub is_static: bool,
    pub visibility: Visibility,
    pub documentation: Option<String>,
    /// Optional pre-generated method body (overrides automatic generation)
    pub body: Option<String>,
}
