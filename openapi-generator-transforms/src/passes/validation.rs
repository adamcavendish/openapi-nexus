//! Validation transformation pass

use utoipa::openapi::OpenApi;

use super::{OpenApiTransformPass, TransformError, TransformPass};

/// Validation transformation pass
pub struct ValidationPass;

impl Default for ValidationPass {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationPass {
    pub fn new() -> Self {
        Self
    }
}

impl OpenApiTransformPass for ValidationPass {
    fn name(&self) -> &str {
        "validation"
    }

    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        tracing::debug!("Validating OpenAPI specification");

        // Basic validation
        if openapi.info.title.is_empty() {
            return Err(TransformError::Generic {
                message: "OpenAPI info.title is required".to_string(),
            });
        }

        if openapi.info.version.is_empty() {
            return Err(TransformError::Generic {
                message: "OpenAPI info.version is required".to_string(),
            });
        }

        if openapi.paths.paths.is_empty() {
            return Err(TransformError::Generic {
                message: "OpenAPI must have at least one path defined".to_string(),
            });
        }

        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }
}

impl TransformPass for ValidationPass {
    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        <Self as OpenApiTransformPass>::transform(self, openapi)
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenApiTransformPass, ValidationPass};
    use utoipa::openapi::{Info, OpenApi, Paths};

    #[test]
    fn test_validation_pass_name() {
        let pass = ValidationPass::new();
        assert_eq!(pass.name(), "validation");
    }

    #[test]
    fn test_validation_pass_dependencies() {
        let pass = ValidationPass::new();
        let deps = pass.dependencies();
        assert!(deps.is_empty());
    }

    #[test]
    fn test_validation_pass_valid_spec() {
        let pass = ValidationPass::new();
        let mut openapi = OpenApi::new(Info::new("Test API", "1.0.0"), Paths::new());

        // Should fail because no paths are defined
        assert!(OpenApiTransformPass::transform(&pass, &mut openapi).is_err());
    }
}
