//! Template-based TypeScript code emitter
//!
//! This module handles template-based emission for API classes and other
//! template-driven code generation.

use minijinja::Environment;
use utoipa::openapi::OpenApi;

use super::data::RuntimeData;
use super::filters::{
    create_format_doc_comment_filter, create_format_generic_list_filter,
    create_format_import_filter, create_format_property_filter, create_format_type_expr_filter,
    from_json_line_filter, indent_filter, instance_guard_filter, to_json_line_filter,
};
use super::functions::{do_not_edit, http_method_body};
use crate::ast::{TsClassDefinition, TsFile};
use crate::emission::error::EmitError;

/// Helper macro to register multiple max_line_width-dependent filters in one
/// shot to avoid repetition.
macro_rules! add_mlw_filters {
    ($env:expr, $max:expr, { $( $name:expr => $factory:path ),+ $(,)? }) => {
        $( $env.add_filter($name, $factory($max)); )+
    };
}

/// Template-based TypeScript code emitter
#[derive(Debug, Clone)]
pub struct TemplatingEmitter {
    env: Environment<'static>,
}

impl TemplatingEmitter {
    /// Create a new template-based emitter with initialized templates
    pub fn new(max_line_width: usize) -> Self {
        let env = Self::create_template_environment(max_line_width);
        Self { env }
    }

    /// Emit TypeScript code from a file using templates
    pub fn emit_file(&self, file: &TsFile) -> Result<String, EmitError> {
        match file.get_template_data() {
            Some(template_data) => self.emit_with_template_data(&template_data),
            None => Err(EmitError::TemplateError {
                message: "File does not support template rendering".to_string(),
            }),
        }
    }

    /// Emit TypeScript code from a class definition
    pub fn emit_class(&self, class: &TsClassDefinition) -> Result<String, EmitError> {
        let template_data = serde_json::json!({
            "class": class,
            "imports": class.imports
        });

        self.emit_with_template_data(&template_data)
    }

    /// Emit TypeScript code from template data
    pub fn emit_with_template_data(
        &self,
        template_data: &serde_json::Value,
    ) -> Result<String, EmitError> {
        // Get the API class template
        let template =
            self.env
                .get_template("api/api_class.j2")
                .map_err(|e| EmitError::TemplateError {
                    message: format!("Failed to get api/api_class.j2 template: {}", e),
                })?;

        // Render the template
        template
            .render(template_data)
            .map_err(|e| EmitError::TemplateError {
                message: format!("Failed to render template: {}", e),
            })
    }

    /// Emit runtime TypeScript code from OpenAPI specification
    pub fn emit_runtime_file(&self, openapi: &OpenApi) -> Result<String, EmitError> {
        let runtime_data = RuntimeData::from_openapi(openapi);
        self.emit_runtime_with_data(&runtime_data)
    }

    /// Emit runtime TypeScript code from runtime data
    pub fn emit_runtime_with_data(&self, runtime_data: &RuntimeData) -> Result<String, EmitError> {
        // Get the runtime template
        let template =
            self.env
                .get_template("runtime/runtime.j2")
                .map_err(|e| EmitError::TemplateError {
                    message: format!("Failed to get runtime/runtime.j2 template: {}", e),
                })?;

        // Render the template
        template
            .render(runtime_data)
            .map_err(|e| EmitError::TemplateError {
                message: format!("Failed to render runtime template: {}", e),
            })
    }

    /// Emit model helper functions (instanceOf/FromJSON/ToJSON/validation map)
    pub fn emit_model_helpers(&self, data: &serde_json::Value) -> Result<String, EmitError> {
        let template = self
            .env
            .get_template("models/model_helpers.j2")
            .map_err(|e| EmitError::TemplateError {
                message: format!("Failed to get models/model_helpers.j2 template: {}", e),
            })?;

        template.render(data).map_err(|e| EmitError::TemplateError {
            message: format!("Failed to render model helpers template: {}", e),
        })
    }

    /// Create template environment with custom filters and functions
    fn create_template_environment(max_line_width: usize) -> Environment<'static> {
        let mut env = Environment::new();

        // Load all embedded templates
        minijinja_embed::load_templates!(&mut env);

        // Common filters
        env.add_filter("indent", indent_filter);
        // Model helpers filters
        env.add_filter("instance_guard", instance_guard_filter);
        env.add_filter("from_json_line", from_json_line_filter);
        env.add_filter("to_json_line", to_json_line_filter);

        // Add filters that need max_line_width
        add_mlw_filters!(env, max_line_width, {
            "format_doc_comment" => create_format_doc_comment_filter,
            "format_generic_list" => create_format_generic_list_filter,
            "format_import" => create_format_import_filter,
            "format_property" => create_format_property_filter,
            "format_type_expr" => create_format_type_expr_filter,
        });

        // Add custom functions
        env.add_function("do_not_edit", do_not_edit);
        env.add_function("http_method_body", http_method_body);

        env
    }

    /// Get a reference to the template environment
    pub fn environment(&self) -> &Environment<'static> {
        &self.env
    }
}
