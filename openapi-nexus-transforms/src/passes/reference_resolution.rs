//! Reference resolution transformation pass

use utoipa::openapi::OpenApi;

use super::{OpenApiTransformPass, TransformError, TransformPass};

/// Reference resolution transformation pass
pub struct ReferenceResolutionPass;

impl Default for ReferenceResolutionPass {
    fn default() -> Self {
        Self::new()
    }
}

impl ReferenceResolutionPass {
    pub fn new() -> Self {
        Self
    }
}

impl OpenApiTransformPass for ReferenceResolutionPass {
    fn name(&self) -> &str {
        "reference-resolution"
    }

    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        tracing::debug!("Resolving references");

        // Use ReferenceResolver from openapi-nexus-ir
        use openapi_nexus_ir::ReferenceResolver;

        let resolver = ReferenceResolver::new(openapi);

        // For now, just validate that references can be resolved
        // Full resolution would require deep cloning and replacement
        // which is complex with the utoipa types
        if let Some(components) = &openapi.components {
            for (name, schema_ref) in &components.schemas {
                if let utoipa::openapi::RefOr::Ref(ref_ref) = schema_ref {
                    let ref_location = &ref_ref.ref_location;
                    if ref_location.starts_with("#/components/schemas/") {
                        let schema_name = ref_location.trim_start_matches("#/components/schemas/");
                        tracing::debug!("Found reference {} -> {}", name, schema_name);

                        // Validate the reference exists
                        if let Err(e) = resolver.resolve_schema_ref(ref_location) {
                            tracing::warn!("Invalid reference {}: {}", ref_location, e);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn dependencies(&self) -> Vec<&str> {
        vec![]
    }
}

impl TransformPass for ReferenceResolutionPass {
    fn transform(&self, openapi: &mut OpenApi) -> Result<(), TransformError> {
        <Self as OpenApiTransformPass>::transform(self, openapi)
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenApiTransformPass, ReferenceResolutionPass};
    use utoipa::openapi::{Info, OpenApi, Paths};

    #[test]
    fn test_reference_resolution_pass_name() {
        let pass = ReferenceResolutionPass::new();
        assert_eq!(pass.name(), "reference-resolution");
    }

    #[test]
    fn test_reference_resolution_pass_dependencies() {
        let pass = ReferenceResolutionPass::new();
        let deps = pass.dependencies();
        assert!(deps.is_empty());
    }

    #[test]
    fn test_reference_resolution_pass_transform() {
        let pass = ReferenceResolutionPass::new();
        let mut openapi = OpenApi::new(Info::new("Test API", "1.0.0"), Paths::new());

        // Should not panic or error on empty OpenAPI
        assert!(OpenApiTransformPass::transform(&pass, &mut openapi).is_ok());
    }
}
