//! TypeScript AST definitions
//!
//! This module contains all TypeScript Abstract Syntax Tree definitions,
//! organized into individual files for better maintainability.

pub mod class;
pub mod code_block;
pub mod comment;
pub mod doc_comment;
pub mod enum_def;
pub mod enum_variant;
pub mod export;
pub mod export_specifier;
pub mod function;
pub mod function_signature;
pub mod generated_file_header;
pub mod generic;
pub mod import;
pub mod import_collection;
pub mod import_resolver;
pub mod import_specifier;
pub mod interface;
pub mod method;
pub mod node;
pub mod parameter;
pub mod primitive_type;
pub mod property;
pub mod statement;
pub mod type_alias;
pub mod type_expression;
pub mod visibility;

// Re-export all types for convenience
pub use class::Class;
pub use code_block::CodeBlock;
pub use comment::Comment;
pub use doc_comment::DocComment;
pub use enum_def::Enum;
pub use enum_variant::EnumVariant;
pub use export::Export;
pub use export_specifier::ExportSpecifier;
pub use function::Function;
pub use function_signature::FunctionSignature;
pub use generated_file_header::GeneratedFileHeader;
pub use generic::Generic;
pub use import::Import;
pub use import_collection::ImportCollection;
pub use import_resolver::ImportResolver;
pub use import_specifier::ImportSpecifier;
pub use interface::Interface;
pub use method::Method;
pub use node::TsNode;
pub use parameter::Parameter;
pub use primitive_type::PrimitiveType;
pub use property::Property;
pub use statement::{Expression, Statement};
pub use type_alias::TypeAlias;
pub use type_expression::TypeExpression;
pub use visibility::Visibility;
