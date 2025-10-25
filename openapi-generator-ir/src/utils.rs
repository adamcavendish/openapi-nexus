//! Utility functions for working with OpenAPI specifications

use utoipa::openapi::{RefOr, Schema};

/// Utility functions for OpenAPI processing
pub struct Utils;

impl Utils {
    /// Check if a schema is a reference
    pub fn is_reference(schema: &RefOr<Schema>) -> bool {
        matches!(schema, RefOr::Ref(_))
    }

    /// Get the reference name if this is a reference
    pub fn get_reference_name(schema: &RefOr<Schema>) -> Option<&str> {
        match schema {
            RefOr::Ref(reference) => Some(&reference.ref_location),
            _ => None,
        }
    }

    /// Check if a schema is nullable
    pub fn is_nullable(_schema: &Schema) -> bool {
        // Note: OpenAPI 3.1 uses nullable differently than 3.0
        // For now, we'll return false as the nullable field structure may vary
        false
    }
}
