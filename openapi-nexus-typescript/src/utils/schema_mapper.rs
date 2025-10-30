//! Unified schema mapping utilities for OpenAPI to TypeScript conversion

use utoipa::openapi::{RefOr, Schema};

use crate::ast::TsExpression;

/// Unified schema mapper for converting OpenAPI schemas to TypeScript types
#[derive(Debug, Clone)]
pub struct SchemaMapper;

impl SchemaMapper {
    /// Create a new schema mapper
    pub fn new() -> Self {
        Self
    }

    /// Map a RefOr<Schema> to a TypeScript type expression
    pub fn map_ref_or_schema_to_type(&self, schema_ref: &RefOr<Schema>) -> TsExpression {
        match schema_ref {
            RefOr::T(schema) => self.map_schema_to_type(schema),
            RefOr::Ref(reference) => {
                let ref_path = &reference.ref_location;
                if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                    TsExpression::Reference(schema_name.to_string())
                } else {
                    TsExpression::Primitive(crate::ast::TsPrimitive::String)
                }
            }
        }
    }

    /// TODO: review this
    /// Map a Schema to a TypeScript type expression
    pub fn map_schema_to_type(&self, schema: &Schema) -> TsExpression {
        match schema {
            Schema::Object(obj_schema) => {
                if obj_schema.properties.is_empty() {
                    // This is likely a primitive type
                    TsExpression::Primitive(crate::ast::TsPrimitive::String)
                } else {
                    TsExpression::Reference("object".to_string())
                }
            }
            Schema::Array(_) => TsExpression::Array(Box::new(TsExpression::Primitive(
                crate::ast::TsPrimitive::String,
            ))),
            _ => TsExpression::Primitive(crate::ast::TsPrimitive::String),
        }
    }

    /// Map parameter schema to TypeScript type
    pub fn map_parameter_schema_to_type(&self, schema_ref: &RefOr<Schema>) -> TsExpression {
        self.map_ref_or_schema_to_type(schema_ref)
    }
}

impl Default for SchemaMapper {
    fn default() -> Self {
        Self::new()
    }
}
