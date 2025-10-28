//! TypeScript class emitter

use pretty::RcDoc;

use crate::ast::Class;
use crate::emission::body_emitter::MethodContext;
use crate::emission::error::EmitError;
use crate::emission::interface_emitter::InterfaceEmitter;
use crate::emission::method_emitter::MethodEmitter;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// Helper struct for emitting TypeScript classes
pub struct ClassEmitter {
    interface_emitter: InterfaceEmitter,
    method_emitter: MethodEmitter,
    utils: TypeScriptPrettyUtils,
}

impl Default for ClassEmitter {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassEmitter {
    pub fn new() -> Self {
        Self {
            interface_emitter: InterfaceEmitter::new(),
            method_emitter: MethodEmitter::new(),
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a TypeScript class as RcDoc
    pub fn emit_class_doc(&self, class_def: &Class) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = self
            .utils
            .export_prefix()
            .append(RcDoc::text("class"))
            .append(RcDoc::space())
            .append(RcDoc::text(class_def.name.clone()));

        // Add generics
        doc = doc.append(self.utils.generics(&class_def.generics)?);

        // Add extends clause
        if let Some(extends) = &class_def.extends {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(extends.clone()));
        }

        // Add implements clause
        doc = doc.append(self.utils.implements_clause(&class_def.implements));

        // Add body with properties and methods
        let mut body_items = Vec::new();

        // Add properties
        for prop in &class_def.properties {
            body_items.push(self.interface_emitter.emit_property_doc(prop)?);
        }

        // Add methods using RcDoc
        for method in &class_def.methods {
            let context = MethodContext {
                class_name: class_def.name.clone(),
                extends: class_def.extends.clone(),
            };
            let method_doc = self.method_emitter.emit_method_doc(method, &context)?;
            body_items.push(method_doc);
        }

        let force_multiline = self.utils.should_format_multiline(
            body_items.len(),
            !class_def.properties.is_empty() || !class_def.methods.is_empty(),
        );

        let body_content = if body_items.is_empty() {
            RcDoc::nil()
        } else if force_multiline {
            // For multiline, join items with line breaks
            RcDoc::intersperse(body_items, RcDoc::line())
        } else {
            self.utils.comma_separated(body_items)
        };

        doc = doc
            .append(RcDoc::space())
            .append(self.utils.block(body_content));

        // Add documentation if present
        if let Some(docs) = &class_def.documentation {
            doc = self
                .utils
                .doc_comment(docs)
                .append(RcDoc::line())
                .append(doc);
        }

        Ok(doc)
    }

    /// Emit a TypeScript class as string
    pub fn emit_class_string(&self, class_def: &Class) -> Result<String, EmitError> {
        let doc = self.emit_class_doc(class_def)?;
        Ok(doc.pretty(80).to_string())
    }
}
