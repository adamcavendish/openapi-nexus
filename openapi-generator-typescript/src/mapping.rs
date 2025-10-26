//! Type mapping logic for converting OpenAPI types to TypeScript types

use crate::ast::TypeExpression;

/// Type mapper for converting OpenAPI schemas to TypeScript types
pub struct TypeMapper;

impl TypeMapper {
    /// Create a new type mapper
    pub fn new() -> Self {
        Self
    }

    /// Map a RefOr<Schema> to a TypeScript type expression
    pub fn map_ref_or_schema_to_typescript_type(
        &self,
        schema_ref: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> TypeExpression {
        match schema_ref {
            utoipa::openapi::RefOr::T(schema) => self.map_schema_to_typescript_type(schema),
            utoipa::openapi::RefOr::Ref(reference) => {
                // Extract the schema name from the reference path
                let ref_path = &reference.ref_location;
                if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                    TypeExpression::Reference(schema_name.to_string())
                } else {
                    TypeExpression::Reference("any".to_string())
                }
            }
        }
    }

    /// Map a Schema to a TypeScript type expression
    pub fn map_schema_to_typescript_type(
        &self,
        schema: &utoipa::openapi::Schema,
    ) -> TypeExpression {
        match schema {
            utoipa::openapi::Schema::Object(obj_schema) => {
                // Check if this is a primitive type by looking at the schema properties
                if obj_schema.properties.is_empty() {
                    // This might be a primitive type schema
                    // For now, we'll use a heuristic based on common patterns
                    // TODO: Implement proper type detection from schema metadata
                    TypeExpression::Primitive(crate::ast::PrimitiveType::Any)
                } else {
                    // This is an object schema with properties
                    TypeExpression::Reference("object".to_string())
                }
            }
            utoipa::openapi::Schema::Array(_arr_schema) => {
                // Map array schema to TypeScript array type
                TypeExpression::Array(Box::new(TypeExpression::Reference("any".to_string())))
            }
            utoipa::openapi::Schema::OneOf(one_of) => {
                // Map oneOf to union type
                let types: Vec<TypeExpression> = one_of
                    .items
                    .iter()
                    .map(|schema_ref| self.map_ref_or_schema_to_typescript_type(schema_ref))
                    .collect();
                TypeExpression::Union(types)
            }
            utoipa::openapi::Schema::AllOf(all_of) => {
                // Map allOf to intersection type
                let types: Vec<TypeExpression> = all_of
                    .items
                    .iter()
                    .map(|schema_ref| self.map_ref_or_schema_to_typescript_type(schema_ref))
                    .collect();
                TypeExpression::Intersection(types)
            }
            _ => {
                // Fallback for unknown schema types
                TypeExpression::Primitive(crate::ast::PrimitiveType::Any)
            }
        }
    }

    /// Map a property schema to a TypeScript type expression with better type detection
    pub fn map_property_schema_to_typescript_type(
        &self,
        schema_ref: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> TypeExpression {
        match schema_ref {
            utoipa::openapi::RefOr::T(schema) => {
                match schema {
                    utoipa::openapi::Schema::Object(obj_schema) => {
                        // For property schemas, we need to determine the actual type
                        // This is a simplified approach - in reality, we'd need to check
                        // the schema's type field, format, etc.
                        if obj_schema.properties.is_empty() {
                            // This is likely a primitive type
                            // We'll use a heuristic based on common patterns
                            TypeExpression::Primitive(crate::ast::PrimitiveType::Any)
                        } else {
                            // This is an object type
                            TypeExpression::Reference("object".to_string())
                        }
                    }
                    _ => self.map_schema_to_typescript_type(schema),
                }
            }
            utoipa::openapi::RefOr::Ref(reference) => {
                // Extract the schema name from the reference path
                let ref_path = &reference.ref_location;
                if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                    TypeExpression::Reference(schema_name.to_string())
                } else {
                    TypeExpression::Reference("any".to_string())
                }
            }
        }
    }
}

impl Default for TypeMapper {
    fn default() -> Self {
        Self::new()
    }
}
