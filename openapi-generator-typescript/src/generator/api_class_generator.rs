//! Individual API class generator for TypeScript

use utoipa::openapi::path::Operation;
use utoipa::openapi::{RefOr, Schema};

use crate::ast::{Class, Method, Parameter, TsNode, TypeExpression, Visibility};
use crate::core::GeneratorError;
use crate::generator::parameter_extractor::ParameterExtractor;

/// Individual API class generator
pub struct ApiClassGenerator {
    parameter_extractor: ParameterExtractor,
}

impl ApiClassGenerator {
    /// Create a new API class generator
    pub fn new() -> Self {
        Self {
            parameter_extractor: ParameterExtractor::new(),
        }
    }

    /// Generate an API class for a specific tag with operations
    pub fn generate_api_class(
        &self,
        tag: &str,
        operations: &[(String, String, Operation)],
        _openapi: &utoipa::openapi::OpenApi,
    ) -> Result<TsNode, GeneratorError> {
        let class_name = format!("{}Api", self.to_pascal_case(tag));

        let mut methods = vec![
            // Constructor method
            Method {
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
            },
        ];

        // Generate methods for each operation
        for (path, method_name, operation) in operations {
            let method = self.generate_operation_method(path, method_name, operation)?;
            methods.push(method);
        }

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

    /// Generate a method for a specific operation
    fn generate_operation_method(
        &self,
        path: &str,
        method_name: &str,
        operation: &Operation,
    ) -> Result<Method, GeneratorError> {
        let method_name = self.generate_method_name(path, operation, method_name);
        let parameters = self.generate_method_parameters(path, operation)?;
        let return_type = self.generate_return_type(operation)?;

        Ok(Method {
            name: method_name,
            parameters,
            return_type,
            is_async: true,
            is_static: false,
            visibility: Visibility::Public,
            documentation: operation
                .summary
                .clone()
                .or_else(|| operation.description.clone()),
        })
    }

    /// Generate method name from operation
    fn generate_method_name(&self, path: &str, operation: &Operation, http_method: &str) -> String {
        // Use operationId if available, otherwise generate from path and method
        if let Some(operation_id) = &operation.operation_id {
            self.to_camel_case(operation_id)
        } else {
            // Generate from path and HTTP method
            let path_parts: Vec<&str> = path.split('/').collect();
            let mut method_name = String::new();

            // Add HTTP method prefix
            method_name.push_str(&http_method.to_lowercase());

            // Add path parts
            for part in path_parts {
                if !part.is_empty() && !part.starts_with('{') {
                    method_name.push_str(&self.to_pascal_case(part));
                }
            }

            self.to_camel_case(&method_name)
        }
    }

    /// Generate method parameters from operation
    fn generate_method_parameters(
        &self,
        path: &str,
        operation: &Operation,
    ) -> Result<Vec<Parameter>, GeneratorError> {
        let mut parameters = Vec::new();

        // Extract parameters using the parameter extractor
        let extracted = self
            .parameter_extractor
            .extract_parameters(operation, path)?;

        // Add path parameters
        for param_info in extracted.path_params {
            parameters.push(Parameter {
                name: param_info.name,
                type_expr: Some(param_info.type_expr),
                optional: !param_info.required,
                default_value: param_info.default_value,
            });
        }

        // Add query parameters
        for param_info in extracted.query_params {
            parameters.push(Parameter {
                name: param_info.name,
                type_expr: Some(param_info.type_expr),
                optional: !param_info.required,
                default_value: param_info.default_value,
            });
        }

        // Add header parameters
        for param_info in extracted.header_params {
            parameters.push(Parameter {
                name: param_info.name,
                type_expr: Some(param_info.type_expr),
                optional: !param_info.required,
                default_value: param_info.default_value,
            });
        }

        // Add request body parameter
        if let Some(body_param) = extracted.body_param {
            parameters.push(Parameter {
                name: body_param.name,
                type_expr: Some(body_param.type_expr),
                optional: !body_param.required,
                default_value: body_param.default_value,
            });
        }

        Ok(parameters)
    }

    /// Generate return type from operation
    fn generate_return_type(
        &self,
        operation: &Operation,
    ) -> Result<Option<TypeExpression>, GeneratorError> {
        // Look for successful response (200, 201, etc.)
        for (status_code, response_ref) in operation.responses.responses.iter() {
            if status_code.starts_with("2") {
                // 2xx status codes
                match response_ref {
                    RefOr::T(response) => {
                        if let Some(json_content) = response.content.get("application/json")
                            && let Some(schema_ref) = &json_content.schema
                        {
                            let return_type = self.map_schema_ref_to_type(schema_ref);
                            return Ok(Some(TypeExpression::Reference(format!(
                                "Promise<{}>",
                                self.type_expression_to_string(&return_type)
                            ))));
                        }
                        // If no JSON content, return generic response
                        return Ok(Some(TypeExpression::Reference(
                            "Promise<Response>".to_string(),
                        )));
                    }
                    RefOr::Ref(_) => {
                        // TODO: Handle response references
                    }
                }
            }
        }

        // Default return type
        Ok(Some(TypeExpression::Reference("Promise<any>".to_string())))
    }

    /// Map schema reference to TypeScript type
    fn map_schema_ref_to_type(&self, schema_ref: &RefOr<Schema>) -> TypeExpression {
        match schema_ref {
            RefOr::T(schema) => match schema {
                Schema::Object(obj_schema) => {
                    if obj_schema.properties.is_empty() {
                        TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                    } else {
                        TypeExpression::Reference("object".to_string())
                    }
                }
                Schema::Array(_) => TypeExpression::Array(Box::new(TypeExpression::Primitive(
                    crate::ast::PrimitiveType::String,
                ))),
                _ => TypeExpression::Primitive(crate::ast::PrimitiveType::String),
            },
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

    /// Convert TypeExpression to string representation
    #[allow(clippy::only_used_in_recursion)]
    fn type_expression_to_string(&self, type_expr: &TypeExpression) -> String {
        match type_expr {
            TypeExpression::Primitive(primitive) => match primitive {
                crate::ast::PrimitiveType::String => "string".to_string(),
                crate::ast::PrimitiveType::Number => "number".to_string(),
                crate::ast::PrimitiveType::Boolean => "boolean".to_string(),
                crate::ast::PrimitiveType::Any => "any".to_string(),
                _ => "any".to_string(),
            },
            TypeExpression::Reference(name) => name.clone(),
            TypeExpression::Array(item_type) => {
                format!("Array<{}>", self.type_expression_to_string(item_type))
            }
            TypeExpression::Union(types) => {
                let type_strings: Vec<String> = types
                    .iter()
                    .map(|t| self.type_expression_to_string(t))
                    .collect();
                type_strings.join(" | ")
            }
            TypeExpression::Intersection(types) => {
                let type_strings: Vec<String> = types
                    .iter()
                    .map(|t| self.type_expression_to_string(t))
                    .collect();
                type_strings.join(" & ")
            }
            TypeExpression::Function(func) => {
                let params: Vec<String> = func
                    .parameters
                    .iter()
                    .map(|param| {
                        let param_type = if let Some(type_expr) = &param.type_expr {
                            self.type_expression_to_string(type_expr)
                        } else {
                            "any".to_string()
                        };
                        if param.optional {
                            format!("{}?: {}", param.name, param_type)
                        } else {
                            format!("{}: {}", param.name, param_type)
                        }
                    })
                    .collect();
                let return_type = if let Some(ret_type) = &func.return_type {
                    self.type_expression_to_string(ret_type)
                } else {
                    "void".to_string()
                };
                format!("({}) => {}", params.join(", "), return_type)
            }
            TypeExpression::Object(properties) => {
                let prop_strings: Vec<String> = properties
                    .iter()
                    .map(|(name, type_expr)| {
                        let prop_type = self.type_expression_to_string(type_expr);
                        format!("{}: {}", name, prop_type)
                    })
                    .collect();
                format!("{{ {} }}", prop_strings.join("; "))
            }
            TypeExpression::Tuple(types) => {
                let type_strings: Vec<String> = types
                    .iter()
                    .map(|t| self.type_expression_to_string(t))
                    .collect();
                format!("[{}]", type_strings.join(", "))
            }
            TypeExpression::Literal(value) => value.clone(),
            TypeExpression::Generic(name) => name.clone(),
            TypeExpression::IndexSignature(key, value_type) => {
                format!("[{}: {}]", key, self.type_expression_to_string(value_type))
            }
        }
    }

    /// Generate implementation body for an API method
    pub fn generate_method_implementation(
        &self,
        _method_name: &str,
        http_method: &str,
        path: &str,
        operation: &Operation,
    ) -> Result<String, GeneratorError> {
        let params = self.generate_method_parameters(path, operation)?;

        // Extract path parameters
        let path_params: Vec<_> = params
            .iter()
            .filter(|p| self.is_path_parameter(p.name.as_str(), path))
            .collect();

        // Build URL with path parameter substitution
        let mut url = path.to_string();
        for param in &path_params {
            let placeholder = format!("{{{}}}", param.name);
            url = url.replace(&placeholder, &format!("${{{}}}", param.name));
        }

        // Build query parameters string
        let query_params: Vec<_> = params
            .iter()
            .filter(|p| !self.is_path_parameter(p.name.as_str(), path))
            .filter(|p| p.name != "body")
            .collect();

        // Build request body parameter
        let body_param = params.iter().find(|p| p.name == "body");

        let mut body = String::new();

        match http_method.to_uppercase().as_str() {
            "GET" => {
                if !query_params.is_empty() {
                    let query_string = query_params
                        .iter()
                        .map(|p| {
                            if p.optional {
                                format!("{0} && `${{{0}}}=${{{0}.toString()}}`", p.name)
                            } else {
                                format!("`${{{0}}}=${{{0}.toString()}}`", p.name)
                            }
                        })
                        .collect::<Vec<_>>()
                        .join(" + '&' + ");
                    body.push_str(&format!(
                        "    const queryParams = [{}].filter(Boolean).join('&');\n",
                        query_string
                    ));
                    body.push_str("    const url = `${this.configuration?.basePath || ''}");
                    for param in &path_params {
                        body.push_str("/${{");
                        body.push_str(&param.name);
                        body.push_str("}}");
                    }
                    body.push_str("}${queryParams ? '?' + queryParams : ''}`;\n");
                } else {
                    body.push_str("    const url = `${this.configuration?.basePath || ''}");
                    for param in &path_params {
                        body.push_str("/${{");
                        body.push_str(&param.name);
                        body.push_str("}}");
                    }
                    body.push_str("}`;\n");
                }
                body.push_str("    return this.request({ url, init: { method: 'GET' } }).then(response => response.json());\n");
            }
            "POST" | "PUT" | "PATCH" => {
                body.push_str("    const url = `${this.configuration?.basePath || ''}");
                body.push_str(&url);
                body.push_str("`;\n");
                if let Some(body_param) = body_param {
                    body.push_str("    return this.request({\n");
                    body.push_str("      url,\n");
                    body.push_str("      init: {\n");
                    body.push_str(&format!(
                        "        method: '{}',\n",
                        http_method.to_uppercase()
                    ));
                    body.push_str("        headers: { 'Content-Type': 'application/json' },\n");
                    body.push_str(&format!(
                        "        body: JSON.stringify({})\n",
                        body_param.name
                    ));
                    body.push_str("      }\n");
                    body.push_str("    }).then(response => response.json());\n");
                } else {
                    let method = http_method.to_uppercase();
                    body.push_str("    return this.request({ url, init: { method: '");
                    body.push_str(&method);
                    body.push_str("' } }).then(response => response.json());\n");
                }
            }
            "DELETE" => {
                body.push_str("    const url = `${this.configuration?.basePath || ''}");
                body.push_str(&url);
                body.push_str("`;\n");
                body.push_str("    return this.request({ url, init: { method: 'DELETE' } });\n");
            }
            _ => {
                body.push_str("    // Unsupported HTTP method\n");
                body.push_str(
                    "    throw new Error(`HTTP method ${http_method} is not supported`);\n",
                );
            }
        }

        Ok(body)
    }

    /// Check if a parameter is a path parameter based on the path template
    fn is_path_parameter(&self, param_name: &str, path: &str) -> bool {
        path.contains(&format!("{{{}}}", param_name))
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
}

impl Default for ApiClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}
