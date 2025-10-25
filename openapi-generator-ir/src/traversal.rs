//! Traversal utilities for OpenAPI specifications

use utoipa::openapi::{OpenApi, RefOr};

/// Traverse an OpenAPI specification
pub struct Traverser;

impl Traverser {
    /// Visit all schemas in the OpenAPI specification
    pub fn visit_schemas<F>(openapi: &OpenApi, mut visitor: F)
    where
        F: FnMut(&String, &utoipa::openapi::RefOr<utoipa::openapi::Schema>),
    {
        if let Some(components) = &openapi.components {
            for (name, schema) in &components.schemas {
                visitor(name, schema);
            }
        }
    }

    /// Visit all operations in the OpenAPI specification
    pub fn visit_operations<F>(openapi: &OpenApi, mut visitor: F)
    where
        F: FnMut(&String, &utoipa::openapi::path::Operation),
    {
        for (path, path_item) in &openapi.paths.paths {
            // Visit each HTTP method operation
            if let Some(op) = &path_item.get {
                visitor(path, op);
            }
            if let Some(op) = &path_item.post {
                visitor(path, op);
            }
            if let Some(op) = &path_item.put {
                visitor(path, op);
            }
            if let Some(op) = &path_item.delete {
                visitor(path, op);
            }
            if let Some(op) = &path_item.patch {
                visitor(path, op);
            }
            if let Some(op) = &path_item.head {
                visitor(path, op);
            }
            if let Some(op) = &path_item.options {
                visitor(path, op);
            }
            if let Some(op) = &path_item.trace {
                visitor(path, op);
            }
        }
    }
}
