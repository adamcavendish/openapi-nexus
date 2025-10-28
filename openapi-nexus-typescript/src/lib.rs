//! TypeScript code generation for OpenAPI specifications
//!
//! This crate provides TypeScript AST definitions and code generation
//! capabilities for OpenAPI 3.1 specifications.
//!
//! ## Architecture
//!
//! The crate is organized into focused modules:
//!
//! - `core` - Core types and traits
//! - `ast` - TypeScript AST definitions
//! - `config` - Configuration types
//! - `emission` - Code emission and formatting
//! - `generator` - Code generators

pub mod ast;
pub mod ast_trait;
pub mod config;
pub mod core;
pub mod emission;
pub mod generator;
pub mod utils;

// Re-export main types for convenience
pub use config::FileConfig;
pub use core::GeneratorError;
pub use emission::TypeScriptFileCategory;
pub use generator::{GeneratedFile, TypeScriptFileGenerator, TypeScriptGenerator};
