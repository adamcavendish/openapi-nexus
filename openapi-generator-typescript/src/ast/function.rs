//! TypeScript function definition

use serde::{Deserialize, Serialize};

use crate::ast::{Generic, Parameter, TypeExpression};

/// TypeScript function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub generics: Vec<Generic>,
    pub is_async: bool,
    pub is_export: bool,
    pub documentation: Option<String>,
}
