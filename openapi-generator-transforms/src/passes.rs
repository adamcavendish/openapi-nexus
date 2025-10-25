//! Transformation passes for OpenAPI specifications

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

/// Error type for transformation passes
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum TransformError {
    #[snafu(display("Transform error: {}", message))]
    Generic { message: String },
}

/// Base trait for transformation passes
pub trait TransformPass {
    /// Apply the transformation to the OpenAPI specification
    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError>;
}

/// Naming convention transformation pass
pub struct NamingConventionPass {
    pub target_case: NamingConvention,
}

pub enum NamingConvention {
    CamelCase,
    PascalCase,
    SnakeCase,
    KebabCase,
}

impl TransformPass for NamingConventionPass {
    fn transform(&self, _openapi: &mut OpenApi) -> Result<(), TransformError> {
        // TODO: Implement naming convention transformation
        Ok(())
    }
}
