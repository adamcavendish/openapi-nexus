//! TypeScript function emitter

use pretty::RcDoc;

use crate::ast::Function;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// Helper struct for emitting TypeScript functions
pub struct FunctionEmitter {
    utils: TypeScriptPrettyUtils,
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

    /// Emit a TypeScript function as string
    pub fn emit_function_string(&self, function: &Function) -> Result<String, EmitError> {
        let mut result = String::new();

        // Use RcDoc for the signature part
        let signature_doc = self.emit_function_signature_doc(function)?;
        let signature_string = signature_doc.pretty(80).to_string();

        result.push_str(&signature_string);
        result.push_str(" {\n");

        // Add function implementation based on function name
        match function.name.as_str() {
            "ToJSON" => {
                result.push_str("  return JSON.parse(JSON.stringify(value));\n");
            }
            "FromJSON" => {
                result.push_str("  return json as T;\n");
            }
            _ => {
                result.push_str("  // TODO: Implement function body\n");
                result.push_str("  throw new Error('Not implemented');\n");
            }
        }

        result.push('}');

        Ok(result)
    }
}
