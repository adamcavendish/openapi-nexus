//! TypeScript AST node definitions

use serde::{Deserialize, Serialize};

use crate::ast::{TsClassDefinition, TsImport, TsTypeDefinition};

/// Top-level TypeScript AST node (simplified)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    /// Import statement
    Import(TsImport),
    /// Class definition (for template rendering)
    Class(TsClassDefinition),
    /// Type definition (interface, type alias, or enum)
    TypeDefinition(TsTypeDefinition),
}
