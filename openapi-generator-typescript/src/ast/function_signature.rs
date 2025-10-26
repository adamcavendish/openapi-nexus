//! TypeScript function signature definition

use serde::{Deserialize, Serialize};

use crate::ast::{Parameter, TypeExpression};

/// TypeScript function signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub parameters: Vec<Parameter>,
    pub return_type: Option<Box<TypeExpression>>,
}
