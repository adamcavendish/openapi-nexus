//! Individual API class generator for TypeScript

use heck::{ToLowerCamelCase as _, ToPascalCase as _};
use http::Method;
use utoipa::openapi::RefOr;
use utoipa::openapi::path::Operation;

use crate::ast::{Class, Parameter, TsMethod, TsNode, TypeExpression, Visibility};
use crate::core::GeneratorError;
use crate::generator::parameter_extractor::ParameterExtractor;
use crate::utils::schema_mapper::SchemaMapper;

/// Individual API class generator
pub struct ApiClassGenerator {
    parameter_extractor: ParameterExtractor,
    schema_mapper: SchemaMapper,
}

impl ApiClassGenerator {
    /// Create a new API class generator
    pub fn new() -> Self {
        Self {
            parameter_extractor: ParameterExtractor::new(),
            schema_mapper: SchemaMapper::new(),
        }
    }

    /// Generate an API class for a specific tag with operations
    pub fn generate_api_class(
        &self,
        tag: &str,
        operations: &[(String, String, Operation)],
        _openapi: &utoipa::openapi::OpenApi,
    ) -> Result<TsNode, GeneratorError> {
        let class_name = format!("{}Api", tag.to_pascal_case());

        let mut methods = vec![
            // Constructor method
            TsMethod {
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
                body: Some("super(configuration);".to_string()),
            },
        ];

        // Generate methods for each operation
        for (path, method_name, operation) in operations {
            let http_method = method_name.parse::<Method>()
                .map_err(|e| GeneratorError::Generic {
                    message: format!("Invalid HTTP method '{}': {}", method_name, e),
                })?;
            let method = self.generate_operation_method(path, &http_method, operation)?;
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
        http_method: &Method,
        operation: &Operation,
    ) -> Result<TsMethod, GeneratorError> {
        let method_name = self.generate_method_name(path, operation, http_method);
        let parameters = self.generate_method_parameters(path, operation)?;
        let return_type = self.generate_return_type(operation)?;
        let body =
            self.generate_method_implementation(&method_name, http_method, path, operation)?;

        Ok(TsMethod {
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
            body: Some(body),
        })
    }

    /// Generate method name from operation
    fn generate_method_name(
        &self,
        path: &str,
        operation: &Operation,
        http_method: &Method,
    ) -> String {
        // Use operationId if available, otherwise generate from path and method
        if let Some(operation_id) = &operation.operation_id {
            operation_id.to_lower_camel_case()
        } else {
            // Generate from path and HTTP method
            let path_parts: Vec<&str> = path.split('/').collect();
            let mut method_name = String::new();

            // Add HTTP method prefix
            method_name.push_str(&http_method.as_str().to_lowercase());

            // Add path parts
            for part in path_parts {
                if !part.is_empty() && !part.starts_with('{') {
                    method_name.push_str(&part.to_pascal_case());
                }
            }

            method_name.to_lower_camel_case()
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
                            let return_type =
                                self.schema_mapper.map_ref_or_schema_to_type(schema_ref);
                            return Ok(Some(TypeExpression::Reference(format!(
                                "Promise<{}>",
                                return_type
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

    /// Generate implementation body for an API method
    pub fn generate_method_implementation(
        &self,
        _method_name: &str,
        http_method: &Method,
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

        match *http_method {
            Method::GET => {
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
                        body.push_str("/${");
                        body.push_str(&param.name);
                        body.push_str("}");
                    }
                    body.push_str("}${queryParams ? '?' + queryParams : ''}`;\n");
                } else {
                    body.push_str("    const url = `${this.configuration?.basePath || ''}");
                    for param in &path_params {
                        body.push_str("/${");
                        body.push_str(&param.name);
                        body.push_str("}");
                    }
                    body.push_str("}`;\n");
                }
                body.push_str("    return this.request({ url, init: { method: 'GET' } }).then(response => response.json());\n");
            }
            Method::POST | Method::PUT | Method::PATCH => {
                body.push_str("    const url = `${this.configuration?.basePath || ''}");
                body.push_str(&url);
                body.push_str("`;\n");
                if let Some(body_param) = body_param {
                    body.push_str("    return this.request({\n");
                    body.push_str("      url,\n");
                    body.push_str("      init: {\n");
                    body.push_str(&format!("        method: '{}',\n", http_method.as_str()));
                    body.push_str("        headers: { 'Content-Type': 'application/json' },\n");
                    body.push_str(&format!(
                        "        body: JSON.stringify({})\n",
                        body_param.name
                    ));
                    body.push_str("      }\n");
                    body.push_str("    }).then(response => response.json());\n");
                } else {
                    body.push_str("    return this.request({ url, init: { method: '");
                    body.push_str(http_method.as_str());
                    body.push_str("' } }).then(response => response.json());\n");
                }
            }
            Method::DELETE => {
                body.push_str("    const url = `${this.configuration?.basePath || ''}");
                body.push_str(&url);
                body.push_str("`;\n");
                body.push_str("    return this.request({ url, init: { method: 'DELETE' } });\n");
            }
            _ => {
                body.push_str("    // Unsupported HTTP method\n");
                body.push_str(&format!(
                    "    throw new Error(`HTTP method {} is not supported`);\n",
                    http_method.as_str()
                ));
            }
        }

        Ok(body)
    }

    /// Check if a parameter is a path parameter based on the path template
    fn is_path_parameter(&self, param_name: &str, path: &str) -> bool {
        path.contains(&format!("{{{}}}", param_name))
    }
}

impl Default for ApiClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}
