//! TypeScript code generation for OpenAPI specifications
//!
//! This crate provides TypeScript AST definitions and code generation
//! capabilities for OpenAPI 3.1 specifications.

pub mod ast;
pub mod ast_trait;
pub mod config;
pub mod core;
pub mod emission;
pub mod generator;
pub mod templating;
pub mod utils;

// Re-export main types for convenience
pub use config::FileConfig;
pub use core::GeneratorError;
pub use emission::TsFileCategory;
pub use generator::{GeneratedFile, TypeScriptFileGenerator, TypeScriptGenerator};
