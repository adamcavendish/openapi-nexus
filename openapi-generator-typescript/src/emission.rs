//! Code emission and formatting for TypeScript

pub mod emitter;
pub mod file_generator;

pub use emitter::TypeScriptEmitter;
pub use file_generator::{FileGenerator, FileGeneratorError, FileType, GeneratedFile};
