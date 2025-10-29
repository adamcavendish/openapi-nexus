//! Individual API class generator for TypeScript

use heck::{ToLowerCamelCase as _, ToPascalCase as _};
use http::Method;
use utoipa::openapi::RefOr;
use utoipa::openapi::path::Operation;

use crate::ast::{
    ClassDefinition, ClassMethod, ImportStatement, Parameter, TsNode, TypeExpression,
};
use crate::core::GeneratorError;
use crate::generator::parameter_extractor::ParameterExtractor;
use crate::generator::template_generator::{
    ApiMethodData, ParameterData as TemplateParameterData, Template, TemplateGenerator,
};
use crate::utils::schema_mapper::SchemaMapper;

/// Individual API class generator
pub struct ApiClassGenerator {
    parameter_extractor: ParameterExtractor,
    schema_mapper: SchemaMapper,
    template_generator: TemplateGenerator,
}

impl ApiClassGenerator {
    /// Create a new API class generator
    pub fn new() -> Self {
        Self {
            parameter_extractor: ParameterExtractor::new(),
            schema_mapper: SchemaMapper::new(),
            template_generator: TemplateGenerator::new(),
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
            ClassMethod::new("constructor".to_string())
                .with_parameters(vec![Parameter::optional(
                    "configuration".to_string(),
                    Some(TypeExpression::Reference("Configuration".to_string())),
                )])
                .with_docs("Initialize the API client".to_string())
                .with_body_template("constructor_base_api".to_string(), None),
        ];

        // Generate methods for each operation
        for (path, method_name, operation) in operations {
            let http_method =
                method_name
                    .parse::<Method>()
                    .map_err(|e| GeneratorError::Generic {
                        message: format!("Invalid HTTP method '{}': {}", method_name, e),
                    })?;

            // Generate single method (using default response mode)
            let method = self.generate_operation_method(path, &http_method, operation)?;
            methods.push(method);
        }

        // Create imports
        let imports = vec![
            ImportStatement::new("../runtime/base_api".to_string())
                .with_import("BaseAPI".to_string(), None),
            ImportStatement::new("../runtime/configuration".to_string())
                .with_type_import("Configuration".to_string(), None),
            ImportStatement::new("../runtime/classes/json_api_response".to_string())
                .with_import("JSONApiResponse".to_string(), None),
            ImportStatement::new("../runtime/classes/response_error".to_string())
                .with_import("ResponseError".to_string(), None),
        ];

        let api_class = ClassDefinition::new(class_name.clone())
            .with_methods(methods)
            .with_extends("BaseAPI".to_string())
            .with_docs(format!("API client for {} operations", tag))
            .with_imports(imports);

        Ok(TsNode::Class(api_class))
    }

    /// Generate a method for a specific operation
    fn generate_operation_method(
        &self,
        path: &str,
        http_method: &Method,
        operation: &Operation,
    ) -> Result<ClassMethod, GeneratorError> {
        let method_name = self.generate_method_name(path, operation, http_method);
        let parameters = self.generate_method_parameters(path, operation)?;
        let return_type = self.generate_return_type(operation)?;

        // Determine template based on HTTP method
        let template_name = match http_method {
            &Method::GET => "api_method_get",
            &Method::POST | &Method::PUT | &Method::PATCH => "api_method_post_put",
            &Method::DELETE => "api_method_delete",
            _ => "default_method",
        };

        // Create template data
        let template_data = self.create_method_template_data(path, http_method, operation)?;

        let mut method = ClassMethod::new(method_name)
            .with_parameters(parameters)
            .with_async()
            .with_body_template(template_name.to_string(), Some(template_data));

        if let Some(return_type) = return_type {
            method = method.with_return_type(return_type);
        }

        if let Some(docs) = operation
            .summary
            .clone()
            .or_else(|| operation.description.clone())
        {
            method = method.with_docs(docs);
        }

        Ok(method)
    }

    /// Create template data for method body generation
    fn create_method_template_data(
        &self,
        path: &str,
        http_method: &Method,
        operation: &Operation,
    ) -> Result<serde_json::Value, GeneratorError> {
        let parameters = self.generate_method_parameters(path, operation)?;
        let return_type = self.generate_return_type(operation)?;

        // Extract different parameter types
        let mut path_params = Vec::new();
        let mut query_params = Vec::new();
        let mut header_params = Vec::new();
        let mut body_param = None;

        for param in &parameters {
            if param.name.contains("path") {
                path_params.push(TemplateParameterData {
                    name: param.name.clone(),
                    type_expr: param.type_expr.as_ref().map(|t| t.to_typescript_string()),
                    optional: param.optional,
                });
            } else if param.name.contains("query") {
                query_params.push(TemplateParameterData {
                    name: param.name.clone(),
                    type_expr: param.type_expr.as_ref().map(|t| t.to_typescript_string()),
                    optional: param.optional,
                });
            } else if param.name.contains("header") {
                header_params.push(TemplateParameterData {
                    name: param.name.clone(),
                    type_expr: param.type_expr.as_ref().map(|t| t.to_typescript_string()),
                    optional: param.optional,
                });
            } else if param.name == "body" {
                body_param = Some(TemplateParameterData {
                    name: param.name.clone(),
                    type_expr: param.type_expr.as_ref().map(|t| t.to_typescript_string()),
                    optional: param.optional,
                });
            }
        }

        let method_data = ApiMethodData {
            method_name: self.generate_method_name(path, operation, http_method),
            http_method: http_method.to_string(),
            path: path.to_string(),
            path_params,
            query_params,
            header_params,
            body_param,
            return_type: return_type
                .map(|t| t.to_typescript_string())
                .unwrap_or_else(|| "Promise<any>".to_string()),
            has_auth: true, // Assume auth is needed
            has_error_handling: true,
        };

        Ok(serde_json::to_value(method_data).unwrap_or_default())
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
                                "Promise<JSONApiResponse<{}>>",
                                return_type
                            ))));
                        }
                        // If no JSON content, return VoidApiResponse for DELETE, JSONApiResponse for others
                        return Ok(Some(TypeExpression::Reference(
                            "Promise<JSONApiResponse<any>>".to_string(),
                        )));
                    }
                    RefOr::Ref(_) => {
                        // TODO: Handle response references
                    }
                }
            }
        }

        // Default return type - wrapped response
        Ok(Some(TypeExpression::Reference(
            "Promise<JSONApiResponse<any>>".to_string(),
        )))
    }

    /// Generate implementation body for an API method using templates
    pub fn generate_method_implementation(
        &self,
        method_name: &str,
        http_method: &Method,
        path: &str,
        operation: &Operation,
    ) -> Result<String, GeneratorError> {
        // Use ParameterExtractor to get all parameters properly categorized
        let extracted_params = self
            .parameter_extractor
            .extract_parameters(operation, path)?;

        // Convert parameters to template format
        let template_path_params: Vec<TemplateParameterData> = extracted_params
            .path_params
            .iter()
            .map(|p| TemplateParameterData {
                name: p.name.clone(),
                type_expr: Some(format!("{}", p.type_expr)),
                optional: !p.required,
            })
            .collect();

        let template_query_params: Vec<TemplateParameterData> = extracted_params
            .query_params
            .iter()
            .map(|p| TemplateParameterData {
                name: p.name.clone(),
                type_expr: Some(format!("{}", p.type_expr)),
                optional: !p.required,
            })
            .collect();

        let template_header_params: Vec<TemplateParameterData> = extracted_params
            .header_params
            .iter()
            .map(|p| TemplateParameterData {
                name: p.name.clone(),
                type_expr: Some(format!("{}", p.type_expr)),
                optional: !p.required,
            })
            .collect();

        let template_body_param = extracted_params.body_param.map(|p| TemplateParameterData {
            name: p.name.clone(),
            type_expr: Some(format!("{}", p.type_expr)),
            optional: !p.required,
        });

        // Create API method data for template
        let api_method_data = ApiMethodData {
            method_name: method_name.to_string(),
            http_method: http_method.as_str().to_string(),
            path: path.to_string(),
            path_params: template_path_params,
            query_params: template_query_params,
            header_params: template_header_params,
            body_param: template_body_param,
            return_type: "Promise<ApiResponse>".to_string(),
            has_auth: true,
            has_error_handling: true,
        };

        // Generate method body using appropriate template
        let template = match *http_method {
            Method::GET => Template::ApiMethodGet(api_method_data),
            Method::POST | Method::PUT | Method::PATCH => {
                Template::ApiMethodPostPut(api_method_data)
            }
            Method::DELETE => Template::ApiMethodDelete(api_method_data),
            _ => Template::DefaultMethod,
        };

        let lines = self
            .template_generator
            .generate_lines(&template)
            .map_err(|e| GeneratorError::Generic {
                message: format!("Template generation failed: {}", e),
            })?;

        Ok(lines.join("\n"))
    }
}

impl Default for ApiClassGenerator {
    fn default() -> Self {
        Self::new()
    }
}
