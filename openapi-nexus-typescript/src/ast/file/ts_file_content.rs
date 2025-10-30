use serde::{Deserialize, Serialize};

use crate::ast::{TsClassDefinition, TsTypeDefinition};

/// Content types that can be in a TypeScript file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsFileContent {
    /// Single API class file
    ApiClass(TsClassDefinition),
    /// Single type definition file
    TypeDefinition(TsTypeDefinition),
    /// Multiple type definitions in one file
    TypeDefinitions(Vec<TsTypeDefinition>),
    /// Mixed content (classes and types)
    Mixed {
        classes: Vec<TsClassDefinition>,
        types: Vec<TsTypeDefinition>,
    },
    /// Raw TypeScript content (for runtime files, etc.)
    Raw(String),
}
