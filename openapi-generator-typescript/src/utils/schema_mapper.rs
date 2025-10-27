//! Unified schema mapping utilities for OpenAPI to TypeScript conversion

use utoipa::openapi::{RefOr, Schema};

use crate::ast::TypeExpression;

/// Unified schema mapper for converting OpenAPI schemas to TypeScript types
pub struct SchemaMapper;

impl SchemaMapper {
    /// Create a new schema mapper
    pub fn new() -> Self {
        Self
    }

    /// Map a RefOr<Schema> to a TypeScript type expression
    pub fn map_ref_or_schema_to_type(&self, schema_ref: &RefOr<Schema>) -> TypeExpression {
        match schema_ref {
            RefOr::T(schema) => self.map_schema_to_type(schema),
            RefOr::Ref(reference) => {
                let ref_path = &reference.ref_location;
                if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                    TypeExpression::Reference(schema_name.to_string())
                } else {
                    TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                }
            }
        }
    }

    /// TODO: review this
    /// Map a Schema to a TypeScript type expression
    pub fn map_schema_to_type(&self, schema: &Schema) -> TypeExpression {
        match schema {
            Schema::Object(obj_schema) => {
                if obj_schema.properties.is_empty() {
                    // This is likely a primitive type
                    TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                } else {
                    TypeExpression::Reference("object".to_string())
                }
            }
            Schema::Array(_) => TypeExpression::Array(Box::new(TypeExpression::Primitive(
                crate::ast::PrimitiveType::String,
            ))),
            _ => TypeExpression::Primitive(crate::ast::PrimitiveType::String),
        }
    }

    /// Map parameter schema to TypeScript type
    pub fn map_parameter_schema_to_type(&self, schema_ref: &RefOr<Schema>) -> TypeExpression {
        self.map_ref_or_schema_to_type(schema_ref)
    }
}

impl Default for SchemaMapper {
    fn default() -> Self {
        Self::new()
    }
}
