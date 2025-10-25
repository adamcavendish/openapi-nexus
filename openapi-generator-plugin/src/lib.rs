//! Plugin system for OpenAPI code generation
//!
//! This crate defines the trait interfaces for extending the code generator
//! with custom language generators, transformation passes, and emitters.

pub mod traits;
pub mod registry;

pub use traits::*;
pub use registry::*;
