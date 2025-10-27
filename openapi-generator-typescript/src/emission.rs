//! Code emission and formatting for TypeScript

pub mod emitter;
pub mod file_category;
pub mod file_generator;

pub use emitter::TypeScriptEmitter;
pub use file_category::TypeScriptFileCategory;
pub use file_generator::{TypeScriptFileGenerator, GeneratedFile};
