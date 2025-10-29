//! TypeScript code generation for OpenAPI specifications
//!
//! This crate provides TypeScript AST definitions and code generation
//! capabilities for OpenAPI 3.1 specifications.

pub mod ast;
pub mod config;
pub mod core;
pub mod emission;
pub mod generator;
pub mod templating;
pub mod ts_lang_generator;
pub mod utils;

// Re-export main types for convenience
pub use ts_lang_generator::TsLangGenerator;
