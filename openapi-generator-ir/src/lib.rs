//! Intermediate representation for OpenAPI code generation
//!
//! This crate provides utilities for working with utoipa's OpenAPI types
//! as our intermediate representation, including traversal, analysis,
//! and transformation helpers.
//!
//! The IR layer follows the design principles outlined in RFD 0002, providing:
//! - Schema analysis and dependency tracking
//! - Reference resolution with circular reference detection
//! - Visitor pattern for traversing OpenAPI specifications
//! - Comprehensive error handling with source location tracking
//!
//! # Example
//!
//! ```rust
//! use openapi_generator_ir::{SchemaAnalyzer, ReferenceResolver, OpenApiTraverser};
//! use utoipa::openapi::{OpenApi, Info, Paths};
//!
//! // Create a simple OpenAPI specification
//! let info = Info::new("Test API", "1.0.0");
//! let paths = Paths::new();
//! let openapi = OpenApi::new(info, paths);
//!
//! // Analyze an OpenAPI specification
//! let analyzer = SchemaAnalyzer::new(&openapi);
//! let schemas = analyzer.find_all_schemas();
//! let circular_refs = analyzer.detect_circular_references().unwrap();
//!
//! // Resolve references
//! let resolver = ReferenceResolver::new(&openapi);
//! // Note: This would fail for non-existent references
//! // let schema = resolver.resolve_schema_ref("#/components/schemas/User").unwrap();
//!
//! // Traverse with visitor pattern
//! struct MyVisitor;
//! impl openapi_generator_ir::OpenApiVisitor for MyVisitor {
//!     type Error = openapi_generator_ir::IrError;
//! }
//! let mut visitor = MyVisitor;
//! OpenApiTraverser::traverse(&openapi, &mut visitor).unwrap();
//! ```

pub mod analysis;
pub mod error;
pub mod traversal;
pub mod utils;

// Re-export key utoipa types for convenience
pub use utoipa::openapi::path::{Operation, Parameter};
pub use utoipa::openapi::{
    Components, ExternalDocs, Info, OpenApi, PathItem, Paths, RefOr, Response, Schema,
    SecurityRequirement, Server, Tag,
};

// Re-export IR types
pub use analysis::{Analyzer, CircularRef, SchemaAnalyzer};
pub use error::IrError;
pub use traversal::{OpenApiTraverser, OpenApiVisitor};
pub use utils::{ReferenceResolver, Utils};
