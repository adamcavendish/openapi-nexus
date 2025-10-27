//! Code emission and formatting for TypeScript

pub mod class_emitter;
pub mod constants;
pub mod emitter;
pub mod enum_emitter;
pub mod error;
pub mod file_category;
pub mod file_generator;
pub mod function_emitter;
pub mod import_emitter;
pub mod interface_emitter;
pub mod method_emitter;
pub mod pretty_utils;
pub mod type_alias_emitter;
pub mod type_expression_emitter;

pub use emitter::TypeScriptEmitter;
pub use file_category::TypeScriptFileCategory;
pub use file_generator::{GeneratedFile, TypeScriptFileGenerator};
