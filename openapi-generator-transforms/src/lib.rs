//! AST transformation passes for OpenAPI code generation
//! 
//! This crate provides a pipeline of transformation passes that can be
//! applied to OpenAPI specifications before code generation.

pub mod passes;
pub mod pipeline;

pub use passes::*;
pub use pipeline::*;
