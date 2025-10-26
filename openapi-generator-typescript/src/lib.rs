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
//! - `generation` - High-level generation logic

pub mod ast;
pub mod config;
pub mod core;
pub mod emission;
pub mod generation;
pub mod imports;
pub mod mapping;
pub mod templates;

// Re-export main types for convenience
pub use config::FileConfig as ConfigFileConfig;
pub use core::{GeneratorError, TypeScriptGenerator as TypeScriptGeneratorTrait};
pub use emission::file_generator::{FileConfig, FileGenerator, GeneratedFile};
pub use generation::TypeScriptGenerator;

// Legacy re-exports for backward compatibility
pub use ast::{
    Class, Interface, Method, Parameter, PrimitiveType, Property, TsNode, TypeExpression,
    Visibility,
};
pub use emission::TypeScriptEmitter;
