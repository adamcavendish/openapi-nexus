//! TypeScript method and function body emission using AST nodes
//!
//! This module provides utilities for emitting method and function bodies
//! using AST nodes for consistent formatting.

use pretty::RcDoc;

use crate::ast::{CodeBlock, Expression, Function, TsMethod, Statement};
use crate::ast_trait::ToRcDoc;
use crate::emission::error::EmitError;

/// Context information for method generation
pub struct MethodContext {
    pub class_name: String,
    pub extends: Option<String>,
}

/// Emitter for method and function bodies using AST nodes
pub struct BodyEmitter;

impl BodyEmitter {
    pub fn new() -> Self {
        Self
    }

    /// Generate body for BaseAPI.request method
    pub fn generate_base_api_request_body(&self) -> Result<RcDoc<'static, ()>, EmitError> {
        let statements = vec![
            Statement::Simple("const { url, init } = context".to_string()),
            Statement::Simple("const baseUrl = this.configuration?.basePath || ''".to_string()),
            Statement::Simple("const fullUrl = baseUrl ? `${baseUrl}${url}` : url".to_string()),
            Statement::Comment("Build headers with authentication".to_string()),
            Statement::Simple("const headers = { 'Content-Type': 'application/json', ...this.configuration?.headers }".to_string()),
            Statement::Comment("Add authentication headers".to_string()),
            Statement::If {
                condition: Expression::Simple("this.configuration?.apiKey".to_string()),
                then_body: vec![Statement::Simple("headers['X-API-Key'] = this.configuration.apiKey".to_string())],
                else_body: None,
            },
            Statement::If {
                condition: Expression::Simple("this.configuration?.accessToken".to_string()),
                then_body: vec![Statement::Simple("headers['Authorization'] = `Bearer ${this.configuration.accessToken}`".to_string())],
                else_body: None,
            },
            Statement::If {
                condition: Expression::Simple("this.configuration?.username && this.configuration?.password".to_string()),
                then_body: vec![
                    Statement::Simple("const credentials = btoa(`${this.configuration.username}:${this.configuration.password}`)".to_string()),
                    Statement::Simple("headers['Authorization'] = `Basic ${credentials}`".to_string()),
                ],
                else_body: None,
            },
            Statement::Comment("Merge request init options".to_string()),
            Statement::Simple("const requestInit = { ...init, headers: { ...headers, ...init?.headers } }".to_string()),
            Statement::Comment("Make the fetch request".to_string()),
            Statement::Return(Some(Expression::FunctionCall {
                name: "fetch".to_string(),
                arguments: vec![
                    Expression::Simple("fullUrl".to_string()),
                    Expression::Simple("requestInit".to_string()),
                ],
            })),
        ];

        let code_block = CodeBlock::from_statements(statements);
        code_block.to_rcdoc()
    }

    /// Generate constructor body based on class name and inheritance
    pub fn generate_constructor_body(
        &self,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let statements = match class_name {
            "BaseAPI" => {
                // BaseAPI has no parent class, just assign configuration
                vec![Statement::Simple(
                    "this.configuration = configuration".to_string(),
                )]
            }
            "RequiredError" => {
                // RequiredError extends Error, pass the field parameter to super
                vec![
                    Statement::Simple("super(field)".to_string()),
                    Statement::Simple("this.field = field".to_string()),
                ]
            }
            _ => {
                // For other classes, use the default super call
                if extends.is_some() {
                    vec![Statement::Simple("super(configuration)".to_string())]
                } else {
                    vec![Statement::Comment(
                        "TODO: Implement constructor".to_string(),
                    )]
                }
            }
        };

        let code_block = CodeBlock::from_statements(statements);
        code_block.to_rcdoc()
    }

    /// Generate HTTP method body (GET, POST, PUT, DELETE)
    pub fn generate_http_method_body(
        &self,
        method: &TsMethod,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let statements = match method.name.as_str() {
            "get" => vec![Statement::Return(Some(Expression::FunctionCall {
                name: "fetch".to_string(),
                arguments: vec![
                    Expression::Simple("`${this.baseUrl}${path}`".to_string()),
                    Expression::ObjectLiteral(vec![
                        (
                            "method".to_string(),
                            Some(Box::new(Expression::Simple("'GET'".to_string()))),
                        ),
                        (
                            "headers".to_string(),
                            Some(Box::new(Expression::Simple("this.headers".to_string()))),
                        ),
                    ]),
                ],
            }))],
            "post" | "put" => {
                let http_method = method.name.to_uppercase();
                vec![Statement::Return(Some(Expression::FunctionCall {
                    name: "fetch".to_string(),
                    arguments: vec![
                        Expression::Simple("`${this.baseUrl}${path}`".to_string()),
                        Expression::ObjectLiteral(vec![
                            (
                                "method".to_string(),
                                Some(Box::new(Expression::Simple(format!("'{}'", http_method)))),
                            ),
                            (
                                "headers".to_string(),
                                Some(Box::new(Expression::ObjectLiteral(vec![
                                    (
                                        "'Content-Type'".to_string(),
                                        Some(Box::new(Expression::Simple(
                                            "'application/json'".to_string(),
                                        ))),
                                    ),
                                    ("...this.headers".to_string(), None),
                                ]))),
                            ),
                            (
                                "body".to_string(),
                                Some(Box::new(Expression::Simple(
                                    "JSON.stringify(data)".to_string(),
                                ))),
                            ),
                        ]),
                    ],
                }))]
            }
            "delete" => vec![Statement::Return(Some(Expression::FunctionCall {
                name: "fetch".to_string(),
                arguments: vec![
                    Expression::Simple("`${this.baseUrl}${path}`".to_string()),
                    Expression::ObjectLiteral(vec![
                        (
                            "method".to_string(),
                            Some(Box::new(Expression::Simple("'DELETE'".to_string()))),
                        ),
                        (
                            "headers".to_string(),
                            Some(Box::new(Expression::Simple("this.headers".to_string()))),
                        ),
                    ]),
                ],
            }))],
            _ => vec![Statement::Comment(
                "TODO: Implement HTTP method".to_string(),
            )],
        };

        let code_block = CodeBlock::from_statements(statements);
        code_block.to_rcdoc()
    }

    /// Generate API method body based on method signature and return type
    pub fn generate_api_method_body(
        &self,
        _method: &TsMethod,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let statements = vec![Statement::Comment(
            "TODO: Implement API method body".to_string(),
        )];
        let code_block = CodeBlock::from_statements(statements);
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
