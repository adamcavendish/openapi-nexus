//! TypeScript AST node types

use serde::{Deserialize, Serialize};

use crate::ast::{Class, Enum, Export, Function, Import, Interface, TypeAlias};

/// TypeScript AST node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
    Import(Import),
    Export(Export),
}
