//! TypeScript method and function body emission using RcDoc
//!
//! This module provides utilities for emitting method and function bodies
//! using the pretty printing library for consistent formatting.

use pretty::RcDoc;

use crate::ast::{Function, Method};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// Context information for method generation
pub struct MethodContext {
    pub class_name: String,
    pub extends: Option<String>,
}

/// Emitter for method and function bodies using RcDoc
pub struct BodyEmitter {
    utils: TypeScriptPrettyUtils,
    type_emitter: TypeExpressionEmitter,
}

impl BodyEmitter {
    pub fn new() -> Self {
        Self {
            utils: TypeScriptPrettyUtils::new(),
            type_emitter: TypeExpressionEmitter,
        }
    }

    /// Generate body for BaseAPI.request method
    pub fn generate_base_api_request_body(&self) -> RcDoc<'static, ()> {
        let statements = vec![
            self.utils.statement("const { url, init } = context"),
            self.utils.statement("const baseUrl = this.configuration?.basePath || ''"),
            self.utils.statement("const fullUrl = baseUrl ? `${baseUrl}${url}` : url"),
            RcDoc::line(),
            self.utils.comment("Build headers with authentication"),
            self.utils.object_assignment(
                "const headers",
                vec![
                    ("'Content-Type'".to_string(), Some(RcDoc::text("'application/json'"))),
                    ("...this.configuration?.headers".to_string(), None),
                ],
            ),
            RcDoc::line(),
            self.utils.comment("Add authentication headers"),
            self.utils.if_statement(
                RcDoc::text("this.configuration?.apiKey"),
                self.utils.statement("headers['X-API-Key'] = this.configuration.apiKey"),
            ),
            self.utils.if_statement(
                RcDoc::text("this.configuration?.accessToken"),
                self.utils.statement("headers['Authorization'] = `Bearer ${this.configuration.accessToken}`"),
            ),
            self.utils.if_statement_block(
                RcDoc::text("this.configuration?.username && this.configuration?.password"),
                vec![
                    self.utils.statement("const credentials = btoa(`${this.configuration.username}:${this.configuration.password}`)"),
                    self.utils.statement("headers['Authorization'] = `Basic ${credentials}`"),
                ],
            ),
            RcDoc::line(),
            self.utils.comment("Merge request init options"),
            self.utils.object_assignment(
                "const requestInit",
                vec![
                    ("...init".to_string(), None),
                    ("headers".to_string(), Some(self.utils.object_literal(vec![
                        ("...headers".to_string(), None),
                        ("...init?.headers".to_string(), None),
                    ]))),
                ],
            ),
            RcDoc::line(),
            self.utils.comment("Make the fetch request"),
            self.utils.return_statement(
                self.utils.function_call("fetch", vec![
                    RcDoc::text("fullUrl"),
                    RcDoc::text("requestInit"),
                ])
            ),
        ];

        self.utils.code_block(statements)
    }

    /// Generate constructor body based on class name and inheritance
    pub fn generate_constructor_body(
        &self,
        class_name: &str,
        extends: &Option<String>,
    ) -> RcDoc<'static, ()> {
        match class_name {
            "BaseAPI" => {
                // BaseAPI has no parent class, just assign configuration
                self.utils.statement("this.configuration = configuration")
            }
            "RequiredError" => {
                // RequiredError extends Error, pass the field parameter to super
                let statements = vec![
                    self.utils.statement("super(field)"),
                    self.utils.statement("this.field = field"),
                ];
                self.utils.code_block(statements)
            }
            _ => {
                // For other classes, use the default super call
                if extends.is_some() {
                    self.utils.statement("super(configuration)")
                } else {
                    self.utils.comment("TODO: Implement constructor")
                }
            }
        }
    }

    /// Generate HTTP method body (GET, POST, PUT, DELETE)
    pub fn generate_http_method_body(&self, method: &Method) -> Result<RcDoc<'static, ()>, EmitError> {
        match method.name.as_str() {
            "get" => Ok(self.utils.code_block(vec![
                self.utils.return_statement(
                    self.utils.function_call("fetch", vec![
                        RcDoc::text("`${this.baseUrl}${path}`"),
                        self.utils.object_literal(vec![
                            ("method".to_string(), Some(RcDoc::text("'GET'"))),
                            ("headers".to_string(), Some(RcDoc::text("this.headers"))),
                        ]),
                    ])
                    .append(RcDoc::text(".then(response => response.json())"))
                )
            ])),
            "post" | "put" => {
                let http_method = method.name.to_uppercase();
                Ok(self.utils.code_block(vec![
                    self.utils.return_statement(
                        self.utils.function_call("fetch", vec![
                            RcDoc::text("`${this.baseUrl}${path}`"),
                            self.utils.object_literal(vec![
                                ("method".to_string(), Some(RcDoc::text(format!("'{}'", http_method)))),
                                ("headers".to_string(), Some(self.utils.object_literal(vec![
                                    ("'Content-Type'".to_string(), Some(RcDoc::text("'application/json'"))),
                                    ("...this.headers".to_string(), None),
                                ]))),
                                ("body".to_string(), Some(RcDoc::text("body ? JSON.stringify(body) : undefined"))),
                            ]),
                        ])
                        .append(RcDoc::text(".then(response => response.json())"))
                    )
                ]))
            }
            "delete" => Ok(self.utils.code_block(vec![
                self.utils.return_statement(
                    self.utils.function_call("fetch", vec![
                        RcDoc::text("`${this.baseUrl}${path}`"),
                        self.utils.object_literal(vec![
                            ("method".to_string(), Some(RcDoc::text("'DELETE'"))),
                            ("headers".to_string(), Some(RcDoc::text("this.headers"))),
                        ]),
                    ])
                    .append(RcDoc::text(".then(response => response.json())"))
                )
            ])),
            _ => Ok(self.utils.comment("TODO: Implement HTTP method")),
        }
    }

    /// Generate API method body based on method signature and return type
    pub fn generate_api_method_body(&self, method: &Method) -> Result<RcDoc<'static, ()>, EmitError> {
        if let Some(return_type) = &method.return_type {
            let return_type_str = self
                .type_emitter
                .emit_type_expression_string(return_type)
                .unwrap_or_else(|_| "any".to_string());

            // Check if it's a Promise type
            if return_type_str.starts_with("Promise<") {
                if return_type_str == "Promise<Response>" {
                    // DELETE method
                    Ok(self.utils.code_block(vec![
                        self.utils.statement("const url = this.configuration?.basePath || ''"),
                        self.utils.return_statement(
                        self.utils.function_call("this.request", vec![
                            self.utils.object_literal(vec![
                                ("url".to_string(), Some(RcDoc::text("url"))),
                                ("init".to_string(), Some(self.utils.object_literal(vec![
                                    ("method".to_string(), Some(RcDoc::text("'DELETE'"))),
                                ]))),
                            ])
                        ])
                        )
                    ]))
                } else {
                    // Check if it has a 'body' parameter (POST/PUT)
                    let has_body_param = method.parameters.iter().any(|p| p.name == "body");

                    if has_body_param {
                        // Determine method type from method name
                        let http_method = if method.name.starts_with("update")
                            || method.name.starts_with("put")
                        {
                            "PUT"
                        } else if method.name.starts_with("delete") {
                            "DELETE"
                        } else {
                            "POST"
                        };

                        Ok(self.utils.code_block(vec![
                            self.utils.statement("const url = this.configuration?.basePath || ''"),
                            self.utils.return_statement(
                                self.utils.function_call("this.request", vec![
                                    self.utils.object_literal(vec![
                                        ("url".to_string(), Some(RcDoc::text("url"))),
                                        ("init".to_string(), Some(self.utils.object_literal(vec![
                                            ("method".to_string(), Some(RcDoc::text(format!("'{}'", http_method)))),
                                            ("headers".to_string(), Some(self.utils.object_literal(vec![
                                                ("'Content-Type'".to_string(), Some(RcDoc::text("'application/json'"))),
                                            ]))),
                                            ("body".to_string(), Some(RcDoc::text("JSON.stringify(body)"))),
                                        ]))),
                                    ])
                                ])
                                .append(RcDoc::text(".then(response => response.json())"))
                            )
                        ]))
                    } else {
                        // GET method
                        Ok(self.utils.code_block(vec![
                            self.utils.statement("const url = this.configuration?.basePath || ''"),
                            self.utils.return_statement(
                                self.utils.function_call("this.request", vec![
                                    self.utils.object_literal(vec![
                                        ("url".to_string(), Some(RcDoc::text("url"))),
                                        ("init".to_string(), Some(self.utils.object_literal(vec![
                                            ("method".to_string(), Some(RcDoc::text("'GET'"))),
                                        ]))),
                                    ])
                                ])
                                .append(RcDoc::text(".then(response => response.json())"))
                            )
                        ]))
                    }
                }
            } else {
                // Not a Promise, might be void or Response
                Ok(self.utils.code_block(vec![
                    self.utils.statement("const url = this.configuration?.basePath || ''"),
                    self.utils.return_statement(
                        self.utils.function_call("this.request", vec![
                            self.utils.object_literal(vec![
                                ("url".to_string(), Some(RcDoc::text("url"))),
                                ("init".to_string(), Some(self.utils.object_literal(vec![
                                    ("method".to_string(), Some(RcDoc::text("'GET'"))),
                                ]))),
                            ])
                        ])
                    )
                ]))
            }
        } else {
            // No return type - might be DELETE
            let http_method = if method.name.starts_with("delete") {
                "DELETE"
            } else {
                "GET"
            };
            Ok(self.utils.code_block(vec![
                self.utils.statement("const url = this.configuration?.basePath || ''"),
                self.utils.return_statement(
                    self.utils.function_call("this.request", vec![
                        self.utils.object_literal(vec![
                            ("url".to_string(), Some(RcDoc::text("url"))),
                            ("init".to_string(), Some(self.utils.object_literal(vec![
                                ("method".to_string(), Some(RcDoc::text(format!("'{}'", http_method)))),
                            ]))),
                        ])
                    ])
                )
            ]))
        }
    }

    /// Generate method body based on method name and context
    pub fn generate_method_body(
        &self,
        method: &Method,
        context: &MethodContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match method.name.as_str() {
            "request" if context.class_name == "BaseAPI" => Ok(self.generate_base_api_request_body()),
            "constructor" => Ok(self.generate_constructor_body(&context.class_name, &context.extends)),
            "get" | "post" | "put" | "delete" => self.generate_http_method_body(method),
            _ => self.generate_api_method_body(method),
        }
    }

    /// Generate function body based on function name
    pub fn generate_function_body(&self, function: &Function) -> RcDoc<'static, ()> {
        match function.name.as_str() {
            "ToJSON" => self.utils.return_statement(
                self.utils.function_call("JSON.parse", vec![
                    self.utils.function_call("JSON.stringify", vec![RcDoc::text("value")])
                ])
            ),
            "FromJSON" => self.utils.return_statement(RcDoc::text("json as T")),
            _ => self.utils.code_block(vec![
                self.utils.comment("TODO: Implement function body"),
                self.utils.statement("throw new Error('Not implemented')"),
            ]),
        }
    }
}

impl Default for BodyEmitter {
    fn default() -> Self {
        Self::new()
    }
}
