//! Code emission and formatting for TypeScript

pub mod body_emitter;
pub mod dependency_analyzer;
pub mod emitter;
pub mod error;
pub mod file_category;
pub mod pretty_utils;
pub mod type_expression_emitter;

pub use emitter::TypeScriptEmitter;
pub use file_category::TypeScriptFileCategory;
