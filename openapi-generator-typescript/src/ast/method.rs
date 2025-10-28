//! TypeScript method definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{CodeBlock, DocComment, Parameter, Statement, TypeExpression, Visibility};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// TypeScript method definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsMethod {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub is_async: bool,
    pub is_static: bool,
    pub visibility: Visibility,
    pub documentation: Option<String>,
    /// Optional pre-generated method body (overrides automatic generation)
    pub body: Option<String>,
}

impl ToRcDocWithContext for TsMethod {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();

        let mut doc = RcDoc::nil();

        // Add visibility modifier
        match self.visibility {
            Visibility::Public => {}
            Visibility::Private => doc = doc.append(RcDoc::text("private ")),
            Visibility::Protected => doc = doc.append(RcDoc::text("protected ")),
        }

        // Add static modifier
        if self.is_static {
            doc = doc.append(RcDoc::text("static "));
        }

        // Add async modifier
        if self.is_async {
            doc = doc.append(RcDoc::text("async "));
        }

        doc = doc.append(RcDoc::text(self.name.clone()));

        // Add parameter list
        doc = doc.append(utils.parameter_list(&self.parameters)?);

        // Add return type
        doc = doc.append(utils.return_type(&self.return_type)?);

        // Generate method body
        let body_doc = if let Some(body) = &self.body {
            RcDoc::text(body.clone())
        } else {
            // For methods, we need to create a simple body since we don't have MethodContext here
            let statements = vec![
                Statement::Comment("TODO: Implement method body".to_string()),
                Statement::Simple("throw new Error('Not implemented')".to_string()),
            ];
            let code_block = CodeBlock::new(statements);
            code_block.to_rcdoc_with_context(context)?
        };

        // Combine signature and body
        let method_doc = doc
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
                .append(method_doc));
        }

        Ok(method_doc)
    }
}
