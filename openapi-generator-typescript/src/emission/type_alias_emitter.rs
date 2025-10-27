//! TypeScript type alias emitter

use pretty::RcDoc;

use crate::ast::TypeAlias;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// Helper struct for emitting TypeScript type aliases
pub struct TypeAliasEmitter {
    type_emitter: TypeExpressionEmitter,
    utils: TypeScriptPrettyUtils,
}

impl TypeAliasEmitter {
    pub fn new() -> Self {
        Self {
            type_emitter: TypeExpressionEmitter,
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a TypeScript type alias as RcDoc
    pub fn emit_type_alias_doc(&self, type_alias: &TypeAlias) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = self.utils.export_prefix()
            .append(RcDoc::text("type"))
            .append(RcDoc::space())
            .append(RcDoc::text(type_alias.name.clone()));

        // Add generics
        doc = doc.append(self.utils.generics(&type_alias.generics)?);

        // Add type expression
        let type_doc = self.type_emitter.emit_type_expression_doc(&type_alias.type_expr)?;
        doc = doc.append(RcDoc::text(" = ")).append(type_doc);

        // Add semicolon
        doc = self.utils.with_semicolon(doc);

        // Add documentation if present
        if let Some(docs) = &type_alias.documentation {
            doc = self.utils.doc_comment(docs).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }

    /// Emit a TypeScript type alias as string
    pub fn emit_type_alias_string(&self, type_alias: &TypeAlias) -> Result<String, EmitError> {
        let doc = self.emit_type_alias_doc(type_alias)?;
        Ok(doc.pretty(80).to_string())
    }
}
