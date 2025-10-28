//! TypeScript function definition

use serde::{Deserialize, Serialize};

use crate::ast::{
    CodeBlock, DocComment, Generic, GenericList, Parameter, ParameterList, ReturnType,
    TypeExpression,
};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::body_emitter::BodyEmitter;
use crate::emission::error::EmitError;
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
    pub body: Option<CodeBlock>,
}

impl ToRcDocWithContext for Function {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let body_emitter = BodyEmitter::new();

        let mut signature_doc = if self.is_export {
            RcDoc::text("export ")
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
        let generic_list = GenericList::new(self.generics.clone());
        signature_doc = signature_doc.append(generic_list.to_rcdoc_with_context(context)?);

        // Add parameter list
        let parameter_list = ParameterList::new(self.parameters.clone());
        signature_doc = signature_doc.append(parameter_list.to_rcdoc_with_context(context)?);

        // Add return type
        let return_type = ReturnType::new(self.return_type.clone());
        signature_doc = signature_doc.append(return_type.to_rcdoc_with_context(context)?);

        // Generate function body
        let body_doc = if let Some(body) = &self.body {
            body.to_rcdoc_with_context(context)?
        } else {
            body_emitter.generate_function_body(self)?
        };

        // Combine signature and body
        let function_doc = signature_doc
            .append(RcDoc::space())
            .append(RcDoc::text("{"))
            .append(RcDoc::line())
            .append(body_doc.nest(2))
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
