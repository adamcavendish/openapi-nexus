//! Template-based TypeScript code emitter
//!
//! This module handles template-based emission for API classes and other
//! template-driven code generation.

use minijinja::{Environment, context};
use utoipa::openapi::OpenApi;

use super::data::RuntimeData;
use super::filters::{
    create_format_class_signature_filter, create_format_doc_comment_filter,
    create_format_generic_list_filter, create_format_import_filter,
    create_format_interface_signature_filter, create_format_method_signature_filter,
    create_format_method_signature_iface_filter, create_format_ts_class_property_filter,
    create_format_ts_property_filter, create_format_type_expr_filter, from_json_line_filter,
    instance_guard_filter, to_json_line_filter,
};
use super::functions::{do_not_edit, file_header, http_method_body};
use crate::ast::{
    TsClassDefinition, TsExpression, TsInterfaceDefinition, TsInterfaceSignature, TsProperty,
};
use crate::emission::error::EmitError;
use openapi_nexus_core::traits::EmissionContext;

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
    max_line_width: usize,
}

impl TemplatingEmitter {
    /// Create a new template-based emitter with initialized templates
    pub fn new(max_line_width: usize) -> Self {
        let env = Self::create_template_environment(max_line_width);
        Self {
            env,
            max_line_width,
        }
    }

    /// Emit TypeScript code from a class definition
    pub fn emit_class(&self, class: &TsClassDefinition) -> Result<String, EmitError> {
        // Build interface methods list from class methods (exclude constructor)
        let _ctx = EmissionContext {
            indent: 0,
            max_line_width: self.max_line_width,
        };

        let class = class.clone();

        // Build interface signature (export interface FooInterface ...)
        let interface_signature =
            TsInterfaceSignature::new(format!("{}Interface", class.signature.name))
                .with_generics(class.signature.generics.clone());
        // Convert methods into function-typed properties for the interface
        let interface_properties: Vec<TsProperty> = class
            .methods
            .clone()
            .into_iter()
            .filter(|m| m.name != "constructor")
            .map(|m| {
                let func_type = TsExpression::Function {
                    parameters: m.parameters,
                    return_type: m.return_type.map(Box::new),
                };
                TsProperty {
                    name: m.name,
                    type_expr: func_type,
                    optional: false,
                    documentation: m.documentation,
                }
            })
            .collect();
        let api_interface =
            TsInterfaceDefinition::new(interface_signature).with_properties(interface_properties);

        let template_data = context! {
            class => class,
            imports => class.imports.clone(),
            api_interface => api_interface,
        };

        // Get the API class template and render directly
        let template =
            self.env
                .get_template("api/api_class.j2")
                .map_err(|e| EmitError::TemplateError {
                    message: format!("Failed to get api/api_class.j2 template: {}", e),
                })?;

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
        env.set_trim_blocks(true);
        env.set_lstrip_blocks(true);

        // Load all embedded templates
        minijinja_embed::load_templates!(&mut env);

        // Model helpers filters
        env.add_filter("instance_guard", instance_guard_filter);
        env.add_filter("from_json_line", from_json_line_filter);
        env.add_filter("to_json_line", to_json_line_filter);

        // Add filters that need max_line_width
        add_mlw_filters!(env, max_line_width, {
            "format_class_signature" => create_format_class_signature_filter,
            "format_doc_comment" => create_format_doc_comment_filter,
            "format_generic_list" => create_format_generic_list_filter,
            "format_import" => create_format_import_filter,
            "format_interface_signature" => create_format_interface_signature_filter,
            "format_method_signature" => create_format_method_signature_filter,
            "format_method_signature_iface" => create_format_method_signature_iface_filter,
            "format_ts_class_property" => create_format_ts_class_property_filter,
            "format_ts_property" => create_format_ts_property_filter,
            "format_type_expr" => create_format_type_expr_filter,
        });

        // Add custom functions
        env.add_function("do_not_edit", do_not_edit);
        env.add_function("file_header", file_header);
        env.add_function("http_method_body", http_method_body);

        env
    }

    /// Get a reference to the template environment
    pub fn environment(&self) -> &Environment<'static> {
        &self.env
    }
}
