//! Parameter extraction utilities for OpenAPI operations

use utoipa::openapi::path::Operation;

use crate::ast::TypeExpression;
use crate::core::GeneratorError;

/// Extracted parameters from an OpenAPI operation
#[derive(Debug, Clone)]
pub struct ExtractedParameters {
    /// Path parameters (e.g., {id} in /users/{id})
    pub path_params: Vec<ParameterInfo>,
    /// Query parameters (e.g., ?page=1&limit=10)
    pub query_params: Vec<ParameterInfo>,
    /// Header parameters
    pub header_params: Vec<ParameterInfo>,
    /// Request body parameter
    pub body_param: Option<ParameterInfo>,
}

/// Information about a parameter
#[derive(Debug, Clone)]
pub struct ParameterInfo {
    /// Parameter name
    pub name: String,
    /// Parameter type
    pub type_expr: TypeExpression,
    /// Whether the parameter is required
    pub required: bool,
    /// Parameter description
    pub description: Option<String>,
    /// Default value if any
    pub default_value: Option<String>,
}

/// Parameter extractor for OpenAPI operations
pub struct ParameterExtractor;

impl Default for ParameterExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl ParameterExtractor {
    /// Create a new parameter extractor
    pub fn new() -> Self {
        Self
    }

    /// Extract all parameters from an OpenAPI operation
    pub fn extract_parameters(
        &self,
        operation: &Operation,
        path: &str,
    ) -> Result<ExtractedParameters, GeneratorError> {
        let mut path_params = Vec::new();
        let mut query_params = Vec::new();
        let mut header_params = Vec::new();
        let mut body_param = None;

        // Extract path parameters from the path string
        let path_param_names = self.extract_path_parameter_names(path);

        // Extract parameters from the operation
        if let Some(parameters) = &operation.parameters {
            for param in parameters {
                let param_info = ParameterInfo {
                    name: param.name.clone(),
                    type_expr: if let Some(schema) = &param.schema {
                        self.map_parameter_schema_to_type(schema)
                    } else {
                        TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                    },
                    required: matches!(param.required, utoipa::openapi::Required::True),
                    description: param.description.clone(),
                    default_value: None, // TODO: Extract default value from schema
                };

                match param.parameter_in {
                    utoipa::openapi::path::ParameterIn::Path => {
                        // Validate that this parameter actually exists in the path
                        if path_param_names.contains(&param.name) {
                            path_params.push(param_info);
                        } else {
                            // If parameter is marked as Path but not in path, treat as query parameter
                            query_params.push(param_info);
                        }
                    }
                    utoipa::openapi::path::ParameterIn::Query => {
                        query_params.push(param_info);
                    }
                    utoipa::openapi::path::ParameterIn::Header => {
                        header_params.push(param_info);
                    }
                    _ => {
                        // Skip other parameter locations for now
                    }
                }
            }
        }

        // Extract request body parameter
        if let Some(request_body) = &operation.request_body
            && let Some(json_content) = request_body.content.get("application/json")
            && let Some(schema_ref) = &json_content.schema
        {
            body_param = Some(ParameterInfo {
                name: "body".to_string(),
                type_expr: self.map_schema_ref_to_type(schema_ref),
                required: matches!(request_body.required, Some(utoipa::openapi::Required::True)),
                description: request_body.description.clone(),
                default_value: None,
            });
        }

        Ok(ExtractedParameters {
            path_params,
            query_params,
            header_params,
            body_param,
        })
    }

    /// Extract path parameter names from a path string
    fn extract_path_parameter_names(&self, path: &str) -> Vec<String> {
        let mut param_names = Vec::new();
        let mut chars = path.chars();

        while let Some(c) = chars.next() {
            if c == '{' {
                let mut param_name = String::new();
                for c in chars.by_ref() {
                    if c == '}' {
                        break;
                    }
                    param_name.push(c);
                }
                if !param_name.is_empty() {
                    param_names.push(param_name);
                }
            }
        }

        param_names
    }

    /// Map parameter schema to TypeScript type
    fn map_parameter_schema_to_type(
        &self,
        schema_ref: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> TypeExpression {
        match schema_ref {
            utoipa::openapi::RefOr::T(schema) => {
                match schema {
                    utoipa::openapi::Schema::Object(obj_schema) => {
                        if obj_schema.properties.is_empty() {
                            // This is likely a primitive type
                            TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                        } else {
                            TypeExpression::Reference("object".to_string())
                        }
                    }
                    utoipa::openapi::Schema::Array(_) => TypeExpression::Array(Box::new(
                        TypeExpression::Primitive(crate::ast::PrimitiveType::String),
                    )),
                    _ => TypeExpression::Primitive(crate::ast::PrimitiveType::String),
                }
            }
            utoipa::openapi::RefOr::Ref(reference) => {
                let ref_path = &reference.ref_location;
                if let Some(schema_name) = ref_path.strip_prefix("#/components/schemas/") {
                    TypeExpression::Reference(schema_name.to_string())
                } else {
                    TypeExpression::Primitive(crate::ast::PrimitiveType::String)
                }
            }
        }
    }

    /// Map schema reference to TypeScript type
    fn map_schema_ref_to_type(
        &self,
        schema_ref: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> TypeExpression {
        self.map_parameter_schema_to_type(schema_ref)
    }

    /// Generate a request interface name from operation details
    pub fn generate_request_interface_name(
        &self,
        operation_id: Option<&str>,
        method: &str,
        path: &str,
    ) -> String {
        let base_name = if let Some(id) = operation_id {
            // Use operation ID if available
            id.to_string()
        } else {
            // Generate name from method and path
            let method_upper = method.to_uppercase();
            let path_clean = path
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>()
                .trim_matches('_')
                .to_string();

            format!("{}{}Request", method_upper, path_clean)
        };

        // Convert to PascalCase
        self.to_pascal_case(&base_name)
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
}
