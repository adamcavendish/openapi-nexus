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
pub mod type_definition;
pub mod type_expression;
pub mod primitive_type;

// Re-export all types for convenience
pub use class_definition::{ClassDefinition, ClassMethod, ClassProperty, ImportSpecifier as ClassImportSpecifier, ImportStatement};
pub use common::{EnumVariant, Generic, Parameter, Property, Visibility};
pub use file::{FileCategory, FileContent, TypeScriptFile, TypeScriptProject};
pub use import::{Import, ImportCollection, ImportResolver, ImportSpecifier};
pub use metadata::{Comment, DocComment, GeneratedFileHeader};
pub use node::TsNode;
pub use type_definition::{EnumDefinition, InterfaceDefinition, TypeAliasDefinition, TypeDefinition};
pub use type_expression::TypeExpression;
pub use primitive_type::PrimitiveType;
