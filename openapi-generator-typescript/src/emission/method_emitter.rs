//! TypeScript method emitter

use pretty::RcDoc;

use crate::ast::Method;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// Helper struct for emitting TypeScript methods
pub struct MethodEmitter {
    type_emitter: TypeExpressionEmitter,
    utils: TypeScriptPrettyUtils,
}

impl MethodEmitter {
    pub fn new() -> Self {
        Self {
            type_emitter: TypeExpressionEmitter,
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a method signature as RcDoc (without body)
    pub fn emit_method_signature_doc(&self, method: &Method) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(method.name.clone());

        // Add parameter list
        doc = doc.append(self.utils.parameter_list(&method.parameters)?);

        // Add return type
        doc = doc.append(self.utils.return_type(&method.return_type)?);

        // Add documentation if present
        if let Some(docs) = &method.documentation {
            doc = self.utils.doc_comment(docs).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }

    /// Emit a method as a string
    pub fn emit_method_string(
        &self,
        method: &Method,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<String, EmitError> {
        self.emit_method_string_with_indent(method, class_name, extends, true)
    }

    /// Emit a method as a string with optional indentation
    pub fn emit_method_string_with_indent(
        &self,
        method: &Method,
        class_name: &str,
        extends: &Option<String>,
        add_indentation: bool,
    ) -> Result<String, EmitError> {
        let mut result = String::new();

        // Handle documentation separately if present
        if let Some(docs) = &method.documentation {
            let doc_comment = self.utils.doc_comment(docs);
            let doc_string = doc_comment.pretty(80).to_string();
            let indented_doc = doc_string
                .lines()
                .map(|line| if line.trim().is_empty() { line.to_string() } else { format!("  {}", line) })
                .collect::<Vec<_>>()
                .join("\n");
            result.push_str(&indented_doc);
            result.push_str("\n");
        }

        // Use RcDoc for the signature part (without documentation)
        let mut signature_doc = RcDoc::text(method.name.clone());
        signature_doc = signature_doc.append(self.utils.parameter_list(&method.parameters)?);
        signature_doc = signature_doc.append(self.utils.return_type(&method.return_type)?);
        
        let signature_string = signature_doc.pretty(80).to_string();
        
        // Add 2-space indentation to method signature
        let indented_signature = signature_string
            .lines()
            .map(|line| if line.trim().is_empty() { line.to_string() } else { format!("  {}", line) })
            .collect::<Vec<_>>()
            .join("\n");
        result.push_str(&indented_signature);
        result.push_str(" {\n");

        // Add method implementation based on method name
        match method.name.as_str() {
            "request" if class_name == "BaseAPI" => {
                result.push_str(r#"    const { url, init } = context;
    const baseUrl = this.configuration?.basePath || '';
    const fullUrl = baseUrl ? `${baseUrl}${url}` : url;

    // Build headers with authentication
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...this.configuration?.headers,
    };

    // Add authentication headers
    if (this.configuration?.apiKey) {
      headers['X-API-Key'] = this.configuration.apiKey;
    }
    if (this.configuration?.accessToken) {
      headers['Authorization'] = `Bearer ${this.configuration.accessToken}`;
    }
    if (this.configuration?.username && this.configuration?.password) {
      const credentials = btoa(`${this.configuration.username}:${this.configuration.password}`);
      headers['Authorization'] = `Basic ${credentials}`;
    }

    // Merge request init options
    const requestInit: RequestInit = {
      ...init,
      headers: {
        ...headers,
        ...init?.headers,
      },
    };

    // Make the fetch request
    return fetch(fullUrl, requestInit);
"#)
            }
            "constructor" => {
                match class_name {
                    "BaseAPI" => {
                        // BaseAPI has no parent class, just assign configuration
                        result.push_str("    this.configuration = configuration;\n");
                    }
                    "RequiredError" => {
                        // RequiredError extends Error, pass the field parameter to super
                        result.push_str("    super(field);\n");
                        result.push_str("    this.field = field;\n");
                    }
                    _ => {
                        // For other classes, use the default super call
                        if extends.is_some() {
                            result.push_str("    super(configuration);\n");
                        } else {
                            result.push_str("    // TODO: Implement constructor\n");
                        }
                    }
                }
            }
            "get" => {
                result.push_str("    return fetch(`${this.baseUrl}${path}`, {\n");
                result.push_str("      method: 'GET',\n");
                result.push_str("      headers: this.headers,\n");
                result.push_str("    }).then(response => response.json());\n");
            }
            "post" => {
                result.push_str("    return fetch(`${this.baseUrl}${path}`, {\n");
                result.push_str("      method: 'POST',\n");
                result.push_str("      headers: {\n");
                result.push_str("        'Content-Type': 'application/json',\n");
                result.push_str("        ...this.headers,\n");
                result.push_str("      },\n");
                result.push_str("      body: body ? JSON.stringify(body) : undefined,\n");
                result.push_str("    }).then(response => response.json());\n");
            }
            "put" => {
                result.push_str("    return fetch(`${this.baseUrl}${path}`, {\n");
                result.push_str("      method: 'PUT',\n");
                result.push_str("      headers: {\n");
                result.push_str("        'Content-Type': 'application/json',\n");
                result.push_str("        ...this.headers,\n");
                result.push_str("      },\n");
                result.push_str("      body: body ? JSON.stringify(body) : undefined,\n");
                result.push_str("    }).then(response => response.json());\n");
            }
            "delete" => {
                result.push_str("    return fetch(`${this.baseUrl}${path}`, {\n");
                result.push_str("      method: 'DELETE',\n");
                result.push_str("      headers: this.headers,\n");
                result.push_str("    }).then(response => response.json());\n");
            }
            _ => {
                // For API methods, generate basic implementation
                // Generate implementation based on whether method returns Promise
                if let Some(return_type) = &method.return_type {
                    let return_type_str = self
                        .type_emitter
                        .emit_type_expression_string(return_type)
                        .unwrap_or_else(|_| "any".to_string());
                    
                    // Check if it's a Promise type
                    if return_type_str.starts_with("Promise<") {
                        // Check if it's Promise<Response> (DELETE methods)
                        if return_type_str == "Promise<Response>" {
                            // DELETE method
                            result.push_str("    const url = this.configuration?.basePath || '';\n");
                            result.push_str("    return this.request({ url, init: { method: 'DELETE' } });\n");
                        }
                        // Has parameters - check if it has a 'body' parameter (POST/PUT)
                        else {
                            let has_body_param = method.parameters.iter().any(|p| p.name == "body");
                        
                            if has_body_param {
                                // Determine method type from method name
                                let http_method = if method.name.starts_with("update") || method.name.starts_with("put") {
                                    "PUT"
                                } else if method.name.starts_with("delete") {
                                    "DELETE"
                                } else if method.name.starts_with("add") || method.name.starts_with("create") {
                                    "POST"
                                } else {
                                    "POST"
                                };
                                
                                result.push_str("    const url = this.configuration?.basePath || '';\n");
                                result.push_str("    return this.request({\n");
                                result.push_str("      url,\n");
                                result.push_str("      init: {\n");
                                result.push_str(&format!("        method: '{}',\n", http_method));
                                result.push_str("        headers: { 'Content-Type': 'application/json' },\n");
                                result.push_str("        body: JSON.stringify(body)\n");
                                result.push_str("      }\n");
                                result.push_str("    }).then(response => response.json());\n");
                            } else {
                                // GET method
                                result.push_str("    const url = this.configuration?.basePath || '';\n");
                                result.push_str("    return this.request({ url, init: { method: 'GET' } }).then(response => response.json());\n");
                            }
                        }
                    } else {
                        // Not a Promise, might be void or Response
                        result.push_str("    const url = this.configuration?.basePath || '';\n");
                        result.push_str("    return this.request({ url, init: { method: 'GET' } });\n");
                    }
                } else {
                    // No return type - might be DELETE
                    let http_method = if method.name.starts_with("delete") {
                        "DELETE"
                    } else {
                        "GET"
                    };
                    result.push_str("    const url = this.configuration?.basePath || '';\n");
                    result.push_str(&format!("    return this.request({{ url, init: {{ method: '{}' }} }});\n", http_method));
                }
            }
        };
        
        result.push_str("  }");

        Ok(result)
    }
}
