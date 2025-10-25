//! Intermediate representation for OpenAPI code generation
//!
//! This crate provides utilities for working with utoipa's OpenAPI types
//! as our intermediate representation, including traversal, analysis,
//! and transformation helpers.

pub mod analysis;
pub mod traversal;
pub mod utils;

// Re-export key utoipa types for convenience
pub use utoipa::openapi::path::{Operation, Parameter};
pub use utoipa::openapi::{
    Components, ExternalDocs, Info, OpenApi, PathItem, Paths, RefOr, Response, Schema,
    SecurityRequirement, Server, Tag,
};

pub use analysis::*;
pub use traversal::*;
pub use utils::*;
