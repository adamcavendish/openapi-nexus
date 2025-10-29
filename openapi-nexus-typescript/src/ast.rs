//! TypeScript AST definitions (consolidated)
//!
//! This module contains consolidated TypeScript Abstract Syntax Tree definitions,
//! organized into fewer, more focused modules for better maintainability.

pub mod class_definition;
pub mod common;
pub mod file;
pub mod import;
pub mod metadata;
pub mod node;
pub mod primitive_type;
pub mod type_definition;
pub mod type_expression;

// Re-export all types for convenience
pub use class_definition::{
    TsClassDefinition, TsClassMethod, TsClassProperty, TsClassImportSpecifier,
    TsImportStatement,
};
pub use common::{TsEnumVariant, TsGeneric, TsParameter, TsProperty, TsVisibility};
pub use file::{TsFileCategory, TsFileContent, TsFile};
pub use import::{TsImport, TsImportSpecifier};
pub use metadata::TsDocComment;
pub use node::TsNode;
pub use primitive_type::TsPrimitiveType;
pub use type_definition::{
    TsEnumDefinition, TsInterfaceDefinition, TsTypeAliasDefinition, TsTypeDefinition,
};
pub use type_expression::TsTypeExpression;
