//! Core orchestration for OpenAPI code generation

pub mod error;
pub mod generator;
pub mod generator_registry;
pub mod openapi_code_generator;
pub mod traits;

// Re-export the main struct for convenience
pub use generator_registry::GeneratorRegistry;
pub use openapi_code_generator::OpenApiCodeGenerator;
