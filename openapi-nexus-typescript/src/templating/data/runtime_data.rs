//! Runtime data structures for template generation

use serde::Serialize;
use utoipa::openapi::OpenApi;

/// Data structure for runtime template generation
#[derive(Clone, Serialize)]
pub struct RuntimeData {
    pub base_path: String,
    pub package_name: String,
    pub openapi_info: utoipa::openapi::Info,
}

impl RuntimeData {
    /// Create runtime data from OpenAPI specification
    pub fn from_openapi(openapi: &OpenApi) -> Self {
        let base_path = Self::extract_base_path(openapi);

        Self {
            base_path,
            package_name: "generated-api".to_string(), // Default, can be overridden by config
            openapi_info: openapi.info.clone(),
        }
    }

    /// Extract base path from OpenAPI servers
    fn extract_base_path(openapi: &OpenApi) -> String {
        if let Some(servers) = &openapi.servers {
            if let Some(first_server) = servers.first() {
                return first_server.url.clone();
            }
        }
        "http://localhost".to_string()
    }
}
