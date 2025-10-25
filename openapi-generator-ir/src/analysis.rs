//! Analysis utilities for OpenAPI specifications

use utoipa::openapi::{OpenApi, Schema};
use utoipa::openapi::security::SecurityScheme;

/// Analyze an OpenAPI specification and extract useful information
pub struct Analyzer;

impl Analyzer {
    /// Get all schemas from the OpenAPI specification
    pub fn get_all_schemas(openapi: &OpenApi) -> Vec<(&String, &utoipa::openapi::RefOr<Schema>)> {
        openapi
            .components
            .as_ref()
            .map(|components| components.schemas.iter().collect())
            .unwrap_or_default()
    }

    /// Get all operations from the OpenAPI specification
    pub fn get_all_operations(
        openapi: &OpenApi,
    ) -> Vec<(&String, &utoipa::openapi::path::Operation)> {
        openapi
            .paths
            .paths
            .iter()
            .flat_map(|(path, path_item)| {
                // Access operations through individual HTTP methods
                let mut operations = Vec::new();
                if let Some(op) = &path_item.get {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.post {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.put {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.delete {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.patch {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.head {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.options {
                    operations.push((path, op));
                }
                if let Some(op) = &path_item.trace {
                    operations.push((path, op));
                }
                operations
            })
            .collect()
    }

    /// Get all response schemas from the OpenAPI specification
    pub fn get_all_responses(openapi: &OpenApi) -> Vec<(&String, &utoipa::openapi::RefOr<utoipa::openapi::Response>)> {
        openapi.components
            .as_ref()
            .map(|components| components.responses.iter().collect())
            .unwrap_or_default()
    }

    /// Get all parameters from the OpenAPI specification
    /// Note: Parameters are typically defined inline in operations, not in components
    pub fn get_all_parameters(_openapi: &OpenApi) -> Vec<(&String, &utoipa::openapi::RefOr<utoipa::openapi::path::Parameter>)> {
        // TODO: Extract parameters from operations
        Vec::new()
    }

    /// Get all security schemes from the OpenAPI specification
    pub fn get_all_security_schemes(openapi: &OpenApi) -> Vec<(&String, &SecurityScheme)> {
        openapi.components
            .as_ref()
            .map(|components| components.security_schemes.iter().collect())
            .unwrap_or_default()
    }
}
