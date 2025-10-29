//! Template-based TypeScript code emitter
//!
//! This module handles template-based emission for API classes and other
//! template-driven code generation.

use minijinja::Environment;
use utoipa::openapi::OpenApi;

use super::data::RuntimeData;
use super::functions::{do_not_edit, get_method_body_template_function};
use crate::ast::{ClassDefinition, TypeScriptFile};
use crate::emission::error::EmitError;
use crate::templating::filters::{
    create_format_doc_comment_filter, create_format_generic_list_filter,
    create_format_import_filter, create_format_property_filter, create_format_type_expr_filter,
    indent_filter,
};

/// Template-based TypeScript code emitter
#[derive(Debug, Clone)]
pub struct Templating {
    env: Environment<'static>,
}

impl Templating {
    /// Create a new template-based emitter with initialized templates
    pub fn new(max_line_width: usize) -> Self {
        let env = Self::create_template_environment(max_line_width);
        Self { env }
    }

    /// Emit TypeScript code from a file using templates
    pub fn emit_file(&self, file: &TypeScriptFile) -> Result<String, EmitError> {
        match file.get_template_data() {
            Some(template_data) => self.emit_with_template_data(&template_data),
            None => Err(EmitError::TemplateError {
                message: "File does not support template rendering".to_string(),
            }),
        }
    }

    /// Emit TypeScript code from a class definition
    pub fn emit_class(&self, class: &ClassDefinition) -> Result<String, EmitError> {
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

    /// Create template environment with custom filters and functions
    fn create_template_environment(max_line_width: usize) -> Environment<'static> {
        let mut env = Environment::new();

        // Load all embedded templates
        minijinja_embed::load_templates!(&mut env);

        // Add custom filters that don't need configuration
        env.add_filter("indent", indent_filter);

        // Add filters that need max_line_width using factory functions
        env.add_filter(
            "format_doc_comment",
            create_format_doc_comment_filter(max_line_width),
        );
        env.add_filter(
            "format_generic_list",
            create_format_generic_list_filter(max_line_width),
        );
        env.add_filter("format_import", create_format_import_filter(max_line_width));
        env.add_filter(
            "format_property",
            create_format_property_filter(max_line_width),
        );
        env.add_filter(
            "format_type_expr",
            create_format_type_expr_filter(max_line_width),
        );

        // Add custom functions
        env.add_function("do_not_edit", do_not_edit);
        env.add_function(
            "get_method_body_template",
            get_method_body_template_function,
        );

        env
    }

    /// Get a reference to the template environment
    pub fn environment(&self) -> &Environment<'static> {
        &self.env
    }
}
