//! TypeScript AST node definitions

use serde::{Deserialize, Serialize};

use crate::ast::{ClassDefinition, Import, TypeDefinition};

/// Top-level TypeScript AST node (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    /// Import statement
    Import(Import),
    /// Class definition (for template rendering)
    Class(ClassDefinition),
    /// Type definition (interface, type alias, or enum)
    TypeDefinition(TypeDefinition),
}
