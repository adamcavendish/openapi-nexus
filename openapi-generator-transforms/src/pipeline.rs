//! Transformation pipeline for OpenAPI specifications

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

use crate::passes::{TransformPass, TransformError};

/// Pipeline for applying multiple transformation passes
pub struct TransformPipeline {
    passes: Vec<Box<dyn TransformPass>>,
}

impl TransformPipeline {
    /// Create a new transformation pipeline
    pub fn new() -> Self {
        Self {
            passes: Vec::new(),
        }
    }

    /// Add a transformation pass to the pipeline
    pub fn add_pass<P: TransformPass + 'static>(mut self, pass: P) -> Self {
        self.passes.push(Box::new(pass));
        self
    }

    /// Apply all transformation passes to the OpenAPI specification
    pub fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        for pass in &self.passes {
            pass.transform(openapi)?;
        }
        Ok(())
    }
}
