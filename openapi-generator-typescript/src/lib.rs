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
//! - `mapping` - Type mapping logic
//! - `emission` - Code emission and formatting
//! - `generator` - Code generators

pub mod ast;
pub mod config;
pub mod core;
pub mod emission;
pub mod generator;
pub mod mapping;

// Re-export main types for convenience
pub use config::FileConfig;
pub use core::{GeneratorError, TypeScriptGenerator as TypeScriptGeneratorTrait};
pub use emission::{FileGenerator, GeneratedFile};
pub use generator::TypeScriptGenerator;
