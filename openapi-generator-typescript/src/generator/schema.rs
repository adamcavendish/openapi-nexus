//! Schema generation logic for TypeScript

use crate::ast::{Interface, PrimitiveType, Property, TsNode, TypeAlias, TypeExpression};
use crate::core::GeneratorError;
use crate::mapping::TypeMapper;

/// Schema generator for converting OpenAPI schemas to TypeScript AST nodes
pub struct SchemaGenerator {
    _type_mapper: TypeMapper,
}

impl SchemaGenerator {
    /// Create a new schema generator
    pub fn new() -> Self {
        Self {
            _type_mapper: TypeMapper::new(),
        }
    }

    /// Convert a schema reference to a TypeScript AST node
    pub fn schema_to_ts_node(
        &self,
        name: &str,
        schema_ref: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> Result<TsNode, GeneratorError> {
        // Ensure the name is PascalCase for TypeScript interfaces
        let pascal_name = self.to_pascal_case(name);

        match schema_ref {
            utoipa::openapi::RefOr::T(schema) => {
                // Check if this is an object schema with properties
                match schema {
                    utoipa::openapi::Schema::Object(obj_schema) => {
                        if !obj_schema.properties.is_empty() {
                            // This is a proper object schema with properties
                            let interface = self.schema_to_interface(&pascal_name, schema)?;
                            Ok(TsNode::Interface(interface))
                        } else if let Some(enum_values) = &obj_schema.enum_values {
                            if !enum_values.is_empty() {
                                // This is an enum schema
                                let enum_node = self.schema_to_enum(&pascal_name, schema)?;
                                Ok(TsNode::Enum(enum_node))
                            } else {
                                // Empty enum, treat as type alias
                                Ok(TsNode::TypeAlias(TypeAlias {
                                    name: pascal_name,
                                    type_expr: TypeExpression::Primitive(PrimitiveType::Any),
                                    generics: vec![],
                                    documentation: obj_schema.description.clone(),
                                }))
                            }
                        } else {
                            // This is likely a primitive type alias
                            // For now, create a type alias to 'any'
                            Ok(TsNode::TypeAlias(TypeAlias {
                                name: pascal_name,
                                type_expr: TypeExpression::Primitive(PrimitiveType::Any),
                                generics: vec![],
                                documentation: obj_schema.description.clone(),
                            }))
                        }
                    }
                    _ => {
                        // For other schema types, create a type alias to 'any'
                        Ok(TsNode::TypeAlias(TypeAlias {
                            name: pascal_name,
                            type_expr: TypeExpression::Primitive(PrimitiveType::Any),
                            generics: vec![],
                            documentation: None,
                        }))
                    }
                }
            }
            utoipa::openapi::RefOr::Ref(reference) => {
                // Handle reference - for now, create a simple interface
                let interface = Interface {
                    name: pascal_name,
                    properties: vec![],
                    extends: vec![],
                    generics: vec![],
                    documentation: Some(format!("Reference to {}", reference.ref_location)),
                };
                Ok(TsNode::Interface(interface))
            }
        }
    }

    /// Convert a schema to a TypeScript enum
    fn schema_to_enum(
        &self,
        name: &str,
        schema: &utoipa::openapi::Schema,
    ) -> Result<crate::ast::Enum, GeneratorError> {
        match schema {
            utoipa::openapi::Schema::Object(obj_schema) => {
                let mut variants = Vec::new();

                if let Some(enum_values) = &obj_schema.enum_values {
                    for enum_value in enum_values {
                        // Convert serde_json::Value to string
                        let value_str = match enum_value {
                            serde_json::Value::String(s) => s.clone(),
                            _ => enum_value.to_string().trim_matches('"').to_string(),
                        };

                        let variant_name = self.enum_value_to_variant_name(&value_str);
                        let variant = crate::ast::EnumVariant {
                            name: variant_name,
                            value: Some(value_str),
                            documentation: None,
                        };
                        variants.push(variant);
                    }
                }

                Ok(crate::ast::Enum {
                    name: name.to_string(),
                    variants,
                    documentation: obj_schema.description.clone(),
                    is_const: false, // Regular enum, not const enum
                })
            }
            _ => Err(GeneratorError::Generic {
                message: "Expected object schema for enum".to_string(),
            }),
        }
    }

    /// Convert enum value to variant name
    fn enum_value_to_variant_name(&self, value: &str) -> String {
        // Convert enum value to PascalCase variant name
        // e.g., "available" -> "Available", "pending" -> "Pending"
        self.to_pascal_case(value)
    }

    /// Convert a string to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        // If the string is already PascalCase, return it as-is
        if s.chars().next().is_some_and(|c| c.is_uppercase()) {
            return s.to_string();
        }

        // Convert first character to uppercase
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }

    /// Convert a schema to a TypeScript interface
    fn schema_to_interface(
        &self,
        name: &str,
        schema: &utoipa::openapi::Schema,
    ) -> Result<Interface, GeneratorError> {
        match schema {
            utoipa::openapi::Schema::Object(obj_schema) => {
                let mut properties = Vec::new();

                // Extract properties from the object schema
                for (prop_name, prop_schema) in &obj_schema.properties {
                    let type_expr = self
                        ._type_mapper
                        .map_property_schema_to_typescript_type(prop_schema);
                    let is_required = obj_schema.required.contains(prop_name);

                    let property = Property {
                        name: prop_name.clone(),
                        type_expr,
                        optional: !is_required,
                        documentation: None, // TODO: Extract description from property schema
                    };
                    properties.push(property);
                }

                Ok(Interface {
                    name: name.to_string(),
                    properties,
                    extends: vec![],
                    generics: vec![],
                    documentation: obj_schema.description.clone(),
                })
            }
            _ => {
                // For non-object schemas, create an empty interface
                // These are typically enum schemas or primitive type aliases
                Ok(Interface {
                    name: name.to_string(),
                    properties: vec![],
                    extends: vec![],
                    generics: vec![],
                    documentation: None,
                })
            }
        }
    }
}

impl Default for SchemaGenerator {
    fn default() -> Self {
        Self::new()
    }
}
