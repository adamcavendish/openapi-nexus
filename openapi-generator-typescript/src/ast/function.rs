//! TypeScript function definition

use serde::{Deserialize, Serialize};

use crate::ast::{DocComment, Generic, Parameter, TypeExpression};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::body_emitter::BodyEmitter;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use pretty::RcDoc;

/// TypeScript function definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub generics: Vec<Generic>,
    pub is_async: bool,
    pub is_export: bool,
    pub documentation: Option<String>,
    /// Optional pre-generated function body (overrides automatic generation)
    pub body: Option<String>,
}

impl ToRcDocWithContext for Function {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();
        let body_emitter = BodyEmitter::new();

        let mut signature_doc = if self.is_export {
            utils.export_prefix()
        } else {
            RcDoc::nil()
        };

        if self.is_async {
            signature_doc = signature_doc.append(RcDoc::text("async "));
        }

        signature_doc = signature_doc
            .append(RcDoc::text("function "))
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        signature_doc = signature_doc.append(utils.generics(&self.generics)?);

        // Add parameter list
        signature_doc = signature_doc.append(utils.parameter_list(&self.parameters)?);

        // Add return type
        signature_doc = signature_doc.append(utils.return_type(&self.return_type)?);

        // Generate function body
        let body_doc = if let Some(body) = &self.body {
            RcDoc::text(body.clone())
        } else {
            body_emitter.generate_function_body(self)?
        };

        // Combine signature and body
        let function_doc = signature_doc
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::line())
            .append(utils.indent(body_doc))
            .append(RcDoc::line())
            .append(RcDoc::text("}"));

        // Add documentation if present and enabled
        if context.include_docs
            && let Some(docs) = &self.documentation
        {
            let doc_comment = DocComment::new(docs.clone());
            return Ok(doc_comment
                .to_rcdoc_with_context(context)?
                .append(RcDoc::line())
                .append(function_doc));
        }

        Ok(function_doc)
    }
}
