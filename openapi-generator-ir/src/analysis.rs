//! Analysis utilities for OpenAPI specifications

use utoipa::openapi::{OpenApi, Schema, Components};

/// Analyze an OpenAPI specification and extract useful information
pub struct Analyzer;

impl Analyzer {
    /// Get all schemas from the OpenAPI specification
    pub fn get_all_schemas(openapi: &OpenApi) -> Vec<(&String, &utoipa::openapi::RefOr<Schema>)> {
        openapi.components
            .as_ref()
            .map(|components| components.schemas.iter().collect())
            .unwrap_or_default()
    }

    /// Get all operations from the OpenAPI specification
    pub fn get_all_operations(openapi: &OpenApi) -> Vec<(&String, &utoipa::openapi::path::Operation)> {
        openapi.paths
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
}
