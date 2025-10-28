//! TypeScript method and function body emission using AST nodes and templates
//!
//! This module provides utilities for emitting method and function bodies
//! using a hybrid approach: templates for complex logic, AST for simple cases.

use pretty::RcDoc;

use crate::ast::code_block::SnippetLines;
use crate::ast::{CodeBlock, Expression, Function, Statement, TsMethod};
use crate::ast_trait::ToRcDoc;
use crate::emission::error::EmitError;
use crate::generator::template_generator::{ApiMethodData, TemplateGenerator};

/// Context information for method generation
pub struct MethodContext {
    pub class_name: String,
    pub extends: Option<String>,
}

/// Emitter for method and function bodies using AST nodes and templates
pub struct BodyEmitter {
    template_generator: TemplateGenerator,
}

impl BodyEmitter {
    pub fn new() -> Self {
        Self {
            template_generator: TemplateGenerator::new(),
        }
    }

    /// Generate body for BaseAPI.request method using templates
    pub fn generate_base_api_request_body(&self) -> Result<RcDoc<'static, ()>, EmitError> {
        // Use template for complex BaseAPI.request logic
        let lines = self
            .template_generator
            .generate_base_api_request_lines()
            .map_err(|e| EmitError::Generic {
                message: format!("Template generation failed: {}", e),
            })?;

        let code_block = CodeBlock::from_snippets(SnippetLines::MethodBody(lines));
        code_block.to_rcdoc()
    }

    /// Generate constructor body using specific templates for each case
    pub fn generate_constructor_body(
        &self,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        // Use specific templates for each constructor type
        let lines = match class_name {
            "BaseAPI" => self
                .template_generator
                .generate_base_api_constructor_lines(),
            "RequiredError" => self
                .template_generator
                .generate_required_error_constructor_lines(),
            _ => {
                if extends.is_some() {
                    self.template_generator.generate_extends_constructor_lines()
                } else {
                    self.template_generator.generate_default_constructor_lines()
                }
            }
        };

        let lines = lines.map_err(|e| EmitError::Generic {
            message: format!("Template generation failed: {}", e),
        })?;

        let code_block = CodeBlock::from_snippets(SnippetLines::MethodBody(lines));
        code_block.to_rcdoc()
    }

    /// Generate HTTP method body using templates for complex logic
    pub fn generate_http_method_body(
        &self,
        method: &TsMethod,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        // Use templates for HTTP method implementations
        let api_method_data = ApiMethodData {
            method_name: method.name.clone(),
            http_method: method.name.to_uppercase(),
            path: "/path".to_string(), // This would be passed from context
            path_params: vec![],
            query_params: vec![],
            body_param: None,
            return_type: "Promise<any>".to_string(),
            has_auth: true,
            has_error_handling: true,
        };

        let lines = match method.name.as_str() {
            "get" => self
                .template_generator
                .generate_get_method_lines(&api_method_data),
            "post" | "put" | "patch" => self
                .template_generator
                .generate_post_put_method_lines(&api_method_data),
            "delete" => self
                .template_generator
                .generate_delete_method_lines(&api_method_data),
            _ => self.template_generator.generate_default_method_lines(),
        };

        let lines = lines.map_err(|e| EmitError::Generic {
            message: format!("Template generation failed: {}", e),
        })?;

        let code_block = CodeBlock::from_snippets(SnippetLines::MethodBody(lines));
        code_block.to_rcdoc()
    }

    /// Generate API method body using templates
    pub fn generate_api_method_body(
        &self,
        method: &TsMethod,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        // Use templates for API method implementations
        let api_method_data = ApiMethodData {
            method_name: method.name.clone(),
            http_method: "GET".to_string(), // Default, would be determined from context
            path: "/api/endpoint".to_string(), // Would be passed from context
            path_params: vec![],
            query_params: vec![],
            body_param: None,
            return_type: "Promise<any>".to_string(),
            has_auth: true,
            has_error_handling: true,
        };

        let lines = self
            .template_generator
            .generate_complex_api_method_lines(&api_method_data)
            .map_err(|e| EmitError::Generic {
                message: format!("Template generation failed: {}", e),
            })?;

        let code_block = CodeBlock::from_snippets(SnippetLines::MethodBody(lines));
        code_block.to_rcdoc()
    }

    /// Generate method body based on method name and context
    pub fn generate_method_body(
        &self,
        method: &TsMethod,
        context: &MethodContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match method.name.as_str() {
            "request" if context.class_name == "BaseAPI" => self.generate_base_api_request_body(),
            "constructor" => self.generate_constructor_body(&context.class_name, &context.extends),
            "get" | "post" | "put" | "delete" => self.generate_http_method_body(method),
            _ => self.generate_api_method_body(method),
        }
    }

    /// Generate function body based on function name
    pub fn generate_function_body(
        &self,
        function: &Function,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let statements = match function.name.as_str() {
            "ToJSON" => vec![Statement::Return(Some(Expression::FunctionCall {
                name: "JSON.stringify".to_string(),
                arguments: vec![Expression::Simple("value".to_string())],
            }))],
            "FromJSON" => vec![Statement::Return(Some(Expression::Simple(
                "json as T".to_string(),
            )))],
            _ => vec![
                Statement::Comment("TODO: Implement function body".to_string()),
                Statement::Simple("throw new Error('Not implemented')".to_string()),
            ],
        };

        let code_block = CodeBlock::from_statements(statements);
        code_block.to_rcdoc()
    }
}

impl Default for BodyEmitter {
    fn default() -> Self {
        Self::new()
    }
}
