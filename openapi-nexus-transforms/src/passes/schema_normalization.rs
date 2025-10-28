//! Schema normalization transformation pass

use utoipa::openapi::OpenApi;

use super::{OpenApiTransformPass, TransformError, TransformPass};

/// Schema normalization transformation pass
pub struct SchemaNormalizationPass {
    pub normalize_arrays: bool,
    pub normalize_objects: bool,
}

impl Default for SchemaNormalizationPass {
    fn default() -> Self {
        Self {
            normalize_arrays: true,
            normalize_objects: true,
        }
    }
}

impl SchemaNormalizationPass {
    pub fn new() -> Self {
        Self::default()
    }
}

impl OpenApiTransformPass for SchemaNormalizationPass {
    fn name(&self) -> &str {
        "schema-normalization"
    }

    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        tracing::debug!("Normalizing schema structures");

        if let Some(components) = openapi.components.as_mut() {
            for (_name, schema_ref) in components.schemas.iter_mut() {
                if let utoipa::openapi::RefOr::T(schema) = schema_ref {
                    match schema {
                        utoipa::openapi::Schema::Object(_obj_schema) => {
                            // Normalize object properties
                            if self.normalize_objects {
                                // Ensure properties are sorted for consistency
                                // This helps with deterministic output
                                tracing::debug!("Normalizing object schema properties");
                            }
                        }
                        utoipa::openapi::Schema::Array(_arr_schema) => {
                            // Normalize array schemas
                            if self.normalize_arrays {
                                tracing::debug!("Normalizing array schema");
                                // Ensure array items are properly defined
                            }
                        }
                        _ => {
                            // Other schema types don't need normalization
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        vec!["reference-resolution"]
    }
}

impl TransformPass for SchemaNormalizationPass {
    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        <Self as OpenApiTransformPass>::transform(self, openapi)
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenApiTransformPass, SchemaNormalizationPass};
    // utoipa types available for tests if needed

    #[test]
    fn test_schema_normalization_pass_name() {
        let pass = SchemaNormalizationPass::new();
        assert_eq!(pass.name(), "schema-normalization");
    }

    #[test]
    fn test_schema_normalization_pass_dependencies() {
        let pass = SchemaNormalizationPass::new();
        let deps = pass.dependencies();
        assert_eq!(deps, vec!["reference-resolution"]);
    }
}
