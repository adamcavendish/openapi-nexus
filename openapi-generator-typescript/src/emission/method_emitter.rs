//! TypeScript method emitter

use pretty::RcDoc;

use crate::ast::Method;
use crate::emission::body_emitter::{BodyEmitter, MethodContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
/// Helper struct for emitting TypeScript methods
pub struct MethodEmitter {
    utils: TypeScriptPrettyUtils,
    body_emitter: BodyEmitter,
}

impl Default for MethodEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl MethodEmitter {
    pub fn new() -> Self {
        Self {
            utils: TypeScriptPrettyUtils::new(),
            body_emitter: BodyEmitter::new(),
        }
    }

    /// Emit a method signature as RcDoc (without body)
    pub fn emit_method_signature_doc(
        &self,
        method: &Method,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(method.name.clone());

        // Add parameter list
        doc = doc.append(self.utils.parameter_list(&method.parameters)?);

        // Add return type
        doc = doc.append(self.utils.return_type(&method.return_type)?);

        // Add documentation if present
        if let Some(docs) = &method.documentation {
            doc = self
                .utils
                .doc_comment(docs)
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }

    /// Emit a complete method as RcDoc (signature + body)
    pub fn emit_method_doc(
        &self,
        method: &Method,
        context: &MethodContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        // Generate method signature
        let mut signature_doc = RcDoc::text(method.name.clone());
        signature_doc = signature_doc.append(self.utils.parameter_list(&method.parameters)?);
        signature_doc = signature_doc.append(self.utils.return_type(&method.return_type)?);

        // Generate method body (use pre-generated body if available)
        let body_doc = if let Some(body) = &method.body {
            RcDoc::text(body.clone())
        } else {
            self.body_emitter.generate_method_body(method, context)?
        };

        // Combine signature and body
        let method_doc = signature_doc
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::line())
            .append(self.utils.indent(body_doc))
            .append(RcDoc::line())
            .append(RcDoc::text("}"));

        // Add documentation if present
        if let Some(docs) = &method.documentation {
            Ok(self
                .utils
                .doc_comment(docs)
                .append(RcDoc::line())
                .append(method_doc))
        } else {
            Ok(method_doc)
        }
    }

    /// Emit a method as a string
    pub fn emit_method_string(
        &self,
        method: &Method,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<String, EmitError> {
        let context = MethodContext {
            class_name: class_name.to_string(),
            extends: extends.clone(),
        };
        let doc = self.emit_method_doc(method, &context)?;
        Ok(doc.pretty(80).to_string())
    }

    /// Emit a method as a string with optional indentation
    pub fn emit_method_string_with_indent(
        &self,
        method: &Method,
        class_name: &str,
        extends: &Option<String>,
        add_indentation: bool,
    ) -> Result<String, EmitError> {
        let context = MethodContext {
            class_name: class_name.to_string(),
            extends: extends.clone(),
        };
        let doc = self.emit_method_doc(method, &context)?;
        let method_string = doc.pretty(80).to_string();
        
        if add_indentation {
            // Add 2-space indentation to each line
            let indented = method_string
                .lines()
                .map(|line| {
                    if line.trim().is_empty() {
                        line.to_string()
                    } else {
                        format!("  {}", line)
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            Ok(indented)
        } else {
            Ok(method_string)
        }
    }
}
