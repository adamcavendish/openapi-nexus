//! API client generation logic for TypeScript

use crate::ast::{Class, Method, Parameter, PrimitiveType, TsNode, TypeExpression, Visibility};
use crate::core::GeneratorError;
use utoipa::openapi::path::Operation;

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
        openapi: &utoipa::openapi::OpenApi,
    ) -> Result<TsNode, GeneratorError> {
        // Generate individual API classes for each tag/group
        let mut api_classes = Vec::new();

        // Group operations by tags
        let mut tag_operations: std::collections::HashMap<
            String,
            Vec<(String, String, Operation)>,
        > = std::collections::HashMap::new();

        for (path, path_item) in openapi.paths.paths.iter() {
            // Check each HTTP method
            if let Some(operation) = &path_item.get {
                self.add_operation_to_tags(&mut tag_operations, path, "GET", operation);
            }
            if let Some(operation) = &path_item.post {
                self.add_operation_to_tags(&mut tag_operations, path, "POST", operation);
            }
            if let Some(operation) = &path_item.put {
                self.add_operation_to_tags(&mut tag_operations, path, "PUT", operation);
            }
            if let Some(operation) = &path_item.delete {
                self.add_operation_to_tags(&mut tag_operations, path, "DELETE", operation);
            }
            if let Some(operation) = &path_item.patch {
                self.add_operation_to_tags(&mut tag_operations, path, "PATCH", operation);
            }
            if let Some(operation) = &path_item.options {
                self.add_operation_to_tags(&mut tag_operations, path, "OPTIONS", operation);
            }
            if let Some(operation) = &path_item.head {
                self.add_operation_to_tags(&mut tag_operations, path, "HEAD", operation);
            }
        }

        // Generate API class for each tag
        for (tag, operations) in tag_operations {
            let api_class = self.generate_api_class_for_tag(&tag, &operations)?;
            api_classes.push(api_class);
        }

        // If no operations found, generate a default API client
        if api_classes.is_empty() {
            let default_class = self.generate_default_api_client()?;
            api_classes.push(default_class);
        }

        // For now, return the first API class (in a real implementation, we'd return multiple)
        Ok(api_classes.into_iter().next().unwrap())
    }

    /// Add operation to tag groups
    fn add_operation_to_tags(
        &self,
        tag_operations: &mut std::collections::HashMap<String, Vec<(String, String, Operation)>>,
        path: &str,
        method: &str,
        operation: &Operation,
    ) {
        let default_tags = vec!["default".to_string()];
        let tags = operation.tags.as_ref().unwrap_or(&default_tags);
        for tag in tags {
            let entry = tag_operations.entry(tag.clone()).or_default();
            entry.push((path.to_string(), method.to_string(), operation.clone()));
        }
    }

    /// Generate API class for a specific tag
    fn generate_api_class_for_tag(
        &self,
        tag: &str,
        operations: &[(String, String, Operation)],
    ) -> Result<TsNode, GeneratorError> {
        let mut methods = Vec::new();

        // Add constructor
        let constructor = Method {
            name: "constructor".to_string(),
            parameters: vec![Parameter {
                name: "configuration".to_string(),
                type_expr: Some(TypeExpression::Reference("Configuration".to_string())),
                optional: true,
                default_value: None,
            }],
            return_type: None,
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Initialize the API client".to_string()),
        };
        methods.push(constructor);

        // Generate methods for each operation
        for (path, method, operation) in operations {
            let method_name =
                self.generate_method_name(operation.operation_id.as_deref(), method, path);
            let api_method =
                self.generate_method_from_operation(&method_name, path, method, operation)?;
            methods.push(api_method);
        }

        let class_name = format!("{}Api", self.to_pascal_case(tag));
        let api_class = Class {
            name: class_name.clone(),
            properties: vec![],
            methods,
            extends: Some("BaseAPI".to_string()),
            implements: vec![],
            generics: vec![],
            is_export: true,
            documentation: Some(format!("API client for {} operations", tag)),
        };

        Ok(TsNode::Class(api_class))
    }

    /// Generate a default API client
    fn generate_default_api_client(&self) -> Result<TsNode, GeneratorError> {
        let constructor = Method {
            name: "constructor".to_string(),
            parameters: vec![Parameter {
                name: "configuration".to_string(),
                type_expr: Some(TypeExpression::Reference("Configuration".to_string())),
                optional: true,
                default_value: None,
            }],
            return_type: None,
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Initialize the API client".to_string()),
        };

        let api_class = Class {
            name: "ApiClient".to_string(),
            properties: vec![],
            methods: vec![constructor],
            extends: Some("BaseAPI".to_string()),
            implements: vec![],
            generics: vec![],
            is_export: true,
            documentation: Some("Generated API client".to_string()),
        };

        Ok(TsNode::Class(api_class))
    }

    /// Generate method from OpenAPI operation
    fn generate_method_from_operation(
        &self,
        method_name: &str,
        _path: &str,
        http_method: &str,
        operation: &Operation,
    ) -> Result<Method, GeneratorError> {
        let mut parameters = Vec::new();

        // Add path parameters (simplified for now)
        if let Some(params) = &operation.parameters {
            for _param in params {
                // For now, add all parameters as string types
                // TODO: Implement proper parameter type mapping
                parameters.push(Parameter {
                    name: "param".to_string(), // TODO: Extract actual parameter name
                    type_expr: Some(TypeExpression::Primitive(PrimitiveType::String)),
                    optional: true,
                    default_value: None,
                });
            }
        }

        // Add request body parameter for POST/PUT/PATCH
        if matches!(http_method, "POST" | "PUT" | "PATCH") {
            parameters.push(Parameter {
                name: "body".to_string(),
                type_expr: Some(TypeExpression::Reference("any".to_string())),
                optional: true,
                default_value: None,
            });
        }

        // Add options parameter
        parameters.push(Parameter {
            name: "options".to_string(),
            type_expr: Some(TypeExpression::Reference("RequestInit".to_string())),
            optional: true,
            default_value: None,
        });

        let return_type = TypeExpression::Reference("Promise<any>".to_string());

        Ok(Method {
            name: method_name.to_string(),
            parameters,
            return_type: Some(return_type),
            is_async: true,
            is_static: false,
            visibility: Visibility::Public,
            documentation: operation.description.clone(),
        })
    }

    /// Generate method name from operation
    fn generate_method_name(
        &self,
        operation_id: Option<&str>,
        http_method: &str,
        path: &str,
    ) -> String {
        if let Some(id) = operation_id {
            return self.to_camel_case(id);
        }

        // Generate name from HTTP method and path
        let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();
        let resource = path_parts.last().copied().unwrap_or("resource");
        let action = match http_method {
            "GET" => "get",
            "POST" => "create",
            "PUT" => "update",
            "PATCH" => "patch",
            "DELETE" => "delete",
            _ => "request",
        };

        self.to_camel_case(&format!("{}{}", action, self.to_pascal_case(resource)))
    }

    /// Convert to PascalCase
    fn to_pascal_case(&self, s: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in s.chars() {
            if c.is_alphanumeric() {
                if capitalize_next {
                    result.push(c.to_uppercase().next().unwrap());
                    capitalize_next = false;
                } else {
                    result.push(c.to_lowercase().next().unwrap());
                }
            } else {
                capitalize_next = true;
            }
        }

        result
    }

    /// Convert to camelCase
    fn to_camel_case(&self, s: &str) -> String {
        let pascal = self.to_pascal_case(s);
        if pascal.is_empty() {
            return pascal;
        }

        let mut chars = pascal.chars();
        let first = chars.next().unwrap().to_lowercase().next().unwrap();
        format!("{}{}", first, chars.as_str())
    }
}

impl Default for ApiClientGenerator {
    fn default() -> Self {
        Self::new()
    }
}
