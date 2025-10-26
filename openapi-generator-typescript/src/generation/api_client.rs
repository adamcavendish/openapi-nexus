//! API client generation logic for TypeScript

use crate::ast::{
    Class, Method, Parameter, PrimitiveType, Property, TsNode, TypeExpression, Visibility,
};
use crate::core::GeneratorError;

/// API client generator for creating TypeScript API client classes
pub struct ApiClientGenerator;

impl ApiClientGenerator {
    /// Create a new API client generator
    pub fn new() -> Self {
        Self
    }

    /// Generate API client class with methods from operations
    pub fn generate_api_client_with_methods(
        &self,
        _openapi: &utoipa::openapi::OpenApi,
    ) -> Result<TsNode, GeneratorError> {
        let methods = vec![
            Method {
                name: "get".to_string(),
                parameters: vec![Parameter {
                    name: "path".to_string(),
                    type_expr: Some(TypeExpression::Primitive(PrimitiveType::String)),
                    optional: false,
                    default_value: None,
                }],
                return_type: Some(TypeExpression::Reference("Promise<any>".to_string())),
                is_async: true,
                is_static: false,
                visibility: Visibility::Public,
                documentation: Some("Make a GET request".to_string()),
            },
            Method {
                name: "post".to_string(),
                parameters: vec![
                    Parameter {
                        name: "path".to_string(),
                        type_expr: Some(TypeExpression::Primitive(PrimitiveType::String)),
                        optional: false,
                        default_value: None,
                    },
                    Parameter {
                        name: "body".to_string(),
                        type_expr: Some(TypeExpression::Reference("any".to_string())),
                        optional: true,
                        default_value: None,
                    },
                ],
                return_type: Some(TypeExpression::Reference("Promise<any>".to_string())),
                is_async: true,
                is_static: false,
                visibility: Visibility::Public,
                documentation: Some("Make a POST request".to_string()),
            },
        ];

        // Add constructor method
        let constructor = Method {
            name: "constructor".to_string(),
            parameters: vec![
                Parameter {
                    name: "baseUrl".to_string(),
                    type_expr: Some(TypeExpression::Primitive(PrimitiveType::String)),
                    optional: false,
                    default_value: None,
                },
                Parameter {
                    name: "headers".to_string(),
                    type_expr: Some(TypeExpression::Reference("Record<string, string>".to_string())),
                    optional: true,
                    default_value: None,
                },
            ],
            return_type: None, // Constructor doesn't have return type
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Initialize the API client".to_string()),
        };

        let mut all_methods = vec![constructor];
        all_methods.extend(methods);

        let client_class = Class {
            name: "ApiClient".to_string(),
            properties: vec![
                Property {
                    name: "baseUrl".to_string(),
                    type_expr: TypeExpression::Primitive(PrimitiveType::String),
                    optional: false,
                    documentation: Some("Base URL for API requests".to_string()),
                },
                Property {
                    name: "headers".to_string(),
                    type_expr: TypeExpression::Reference("Record<string, string>".to_string()),
                    optional: true,
                    documentation: Some("Default headers for requests".to_string()),
                },
            ],
            methods: all_methods,
            extends: None,
            implements: vec![],
            generics: vec![],
            is_export: true,
            documentation: Some("Generated API client with HTTP methods".to_string()),
        };

        Ok(TsNode::Class(client_class))
    }
}

impl Default for ApiClientGenerator {
    fn default() -> Self {
        Self::new()
    }
}
