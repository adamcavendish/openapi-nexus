//! TypeScript function emitter

use pretty::RcDoc;

use crate::ast::Function;
use crate::emission::body_emitter::BodyEmitter;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// Helper struct for emitting TypeScript functions
pub struct FunctionEmitter {
    utils: TypeScriptPrettyUtils,
    body_emitter: BodyEmitter,
}

impl Default for FunctionEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl FunctionEmitter {
    pub fn new() -> Self {
        Self {
            utils: TypeScriptPrettyUtils::new(),
            body_emitter: BodyEmitter::new(),
        }
    }

    /// Emit a function signature as RcDoc (without body)
    pub fn emit_function_signature_doc(
        &self,
        function: &Function,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = self.utils.export_prefix();

        if function.is_async {
            doc = doc.append(RcDoc::text("async "));
        }

        doc = doc
            .append(RcDoc::text("function "))
            .append(RcDoc::text(function.name.clone()));

        // Add generics
        doc = doc.append(self.utils.generics(&function.generics)?);

        // Add parameter list
        doc = doc.append(self.utils.parameter_list(&function.parameters)?);

        // Add return type
        doc = doc.append(self.utils.return_type(&function.return_type)?);

        // Add documentation if present
        if let Some(docs) = &function.documentation {
            doc = self
                .utils
                .doc_comment(docs)
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }

    /// Emit a complete function as RcDoc (signature + body)
    pub fn emit_function_doc(&self, function: &Function) -> Result<RcDoc<'static, ()>, EmitError> {
        // Generate function signature
        let mut signature_doc = self.utils.export_prefix();

        if function.is_async {
            signature_doc = signature_doc.append(RcDoc::text("async "));
        }

        signature_doc = signature_doc
            .append(RcDoc::text("function "))
            .append(RcDoc::text(function.name.clone()));

        // Add generics
        signature_doc = signature_doc.append(self.utils.generics(&function.generics)?);

        // Add parameter list
        signature_doc = signature_doc.append(self.utils.parameter_list(&function.parameters)?);

        // Add return type
        signature_doc = signature_doc.append(self.utils.return_type(&function.return_type)?);

        // Generate function body (use pre-generated body if available)
        let body_doc = if let Some(body) = &function.body {
            RcDoc::text(body.clone())
        } else {
            self.body_emitter.generate_function_body(function)
        };

        // Combine signature and body
        let function_doc = signature_doc
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::line())
            .append(self.utils.indent(body_doc))
            .append(RcDoc::line())
            .append(RcDoc::text("}"));

        // Add documentation if present
        if let Some(docs) = &function.documentation {
            Ok(self
                .utils
                .doc_comment(docs)
                .append(RcDoc::line())
                .append(function_doc))
        } else {
            Ok(function_doc)
        }
    }

    /// Emit a TypeScript function as string
    pub fn emit_function_string(&self, function: &Function) -> Result<String, EmitError> {
        let doc = self.emit_function_doc(function)?;
        Ok(doc.pretty(80).to_string())
    }
}
