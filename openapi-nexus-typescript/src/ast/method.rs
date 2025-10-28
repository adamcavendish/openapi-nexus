//! TypeScript method definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::code_block::SnippetLines;
use crate::ast::{
    CodeBlock, DocComment, Parameter, ParameterList, ReturnType, Statement, TypeExpression,
    Visibility,
};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

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
    pub body: Option<CodeBlock>,
}

impl ToRcDocWithContext for TsMethod {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
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
        let parameter_list = ParameterList::new(self.parameters.clone());
        doc = doc.append(parameter_list.to_rcdoc_with_context(context)?);

        // Add return type
        let return_type = ReturnType::new(self.return_type.clone());
        doc = doc.append(return_type.to_rcdoc_with_context(context)?);

        // Combine signature and body
        let method_doc = if let Some(body) = &self.body {
            // For CodeBlock content, emit it directly without extra braces
            match body {
                CodeBlock::Statements(statements) => {
                    if statements.is_empty() {
                        doc.append(RcDoc::space()).append(RcDoc::text("{}"))
                    } else {
                        let mut body_doc = RcDoc::text("{").append(RcDoc::line());
                        for stmt in statements {
                            body_doc = body_doc
                                .append(RcDoc::text("  "))
                                .append(stmt.to_rcdoc_with_context(context)?)
                                .append(RcDoc::line());
                        }
                        doc.append(RcDoc::space())
                            .append(body_doc.append(RcDoc::text("}")))
                    }
                }
                CodeBlock::Snippets(snippets) => {
                    let lines = match snippets {
                        SnippetLines::MethodBody(lines) => lines,
                        SnippetLines::InterfaceBody(lines) => lines,
                        SnippetLines::ClassBody(lines) => lines,
                        SnippetLines::FunctionBody(lines) => lines,
                        SnippetLines::EnumBody(lines) => lines,
                        SnippetLines::Generic(lines) => lines,
                    };

                    if lines.is_empty() {
                        doc.append(RcDoc::space()).append(RcDoc::text("{}"))
                    } else {
                        let mut body_doc = RcDoc::text("{").append(RcDoc::line());
                        for line in lines {
                            body_doc = body_doc
                                .append(RcDoc::text("  "))
                                .append(RcDoc::text(line.clone()))
                                .append(RcDoc::line());
                        }
                        doc.append(RcDoc::space())
                            .append(body_doc.append(RcDoc::text("}")))
                    }
                }
            }
        } else {
            // For methods without body, create a simple body
            let statements = vec![
                Statement::Comment("TODO: Implement method body".to_string()),
                Statement::Simple("throw new Error('Not implemented')".to_string()),
            ];
            let code_block = CodeBlock::from_statements(statements);
            let body_doc = code_block.to_rcdoc_with_context(context)?;
            doc.append(RcDoc::space()).append(body_doc)
        };

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
