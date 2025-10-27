//! TypeScript enum emitter

use pretty::RcDoc;

use crate::ast::Enum;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// Helper struct for emitting TypeScript enums
pub struct EnumEmitter {
    utils: TypeScriptPrettyUtils,
}

impl EnumEmitter {
    pub fn new() -> Self {
        Self {
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a TypeScript enum as RcDoc
    pub fn emit_enum_doc(&self, enum_def: &Enum) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = self.utils.export_prefix()
            .append(RcDoc::text("enum"))
            .append(RcDoc::space())
            .append(RcDoc::text(enum_def.name.clone()));

        // Add enum body
        if enum_def.variants.is_empty() {
            doc = doc.append(RcDoc::space()).append(self.utils.empty_block());
        } else {
            let variant_docs: Vec<RcDoc<'static, ()>> = enum_def.variants
                .iter()
                .map(|variant| {
                    let mut variant_doc = RcDoc::text(variant.name.clone());
                    if let Some(value) = &variant.value {
                        variant_doc = variant_doc
                            .append(RcDoc::text(" = "))
                            .append(self.utils.quoted(value));
                    }
                    variant_doc
                })
                .collect();

            let force_multiline = self.utils.should_format_multiline(
                enum_def.variants.len(),
                false // enum variants are typically simple
            );

            let body_content = if force_multiline {
                // Convert each variant to string and add proper indentation
                let variant_strings: Vec<RcDoc<'static, ()>> = variant_docs
                    .into_iter()
                    .map(|variant| {
                        let variant_string = variant.pretty(80).to_string();
                        RcDoc::text(self.utils.indent_lines(&variant_string))
                    })
                    .collect();
                RcDoc::intersperse(variant_strings, RcDoc::text(",").append(RcDoc::line()))
                    .append(RcDoc::text(",")) // trailing comma
            } else {
                // For single line, add indentation to each variant
                let variant_strings: Vec<RcDoc<'static, ()>> = variant_docs
                    .into_iter()
                    .map(|variant| {
                        let variant_string = variant.pretty(80).to_string();
                        RcDoc::text(self.utils.indent_lines(&variant_string))
                    })
                    .collect();
                self.utils.comma_separated(variant_strings).append(RcDoc::text(",")) // trailing comma
            };

            doc = doc.append(RcDoc::space()).append(self.utils.block(body_content));
        }

        // Add documentation if present
        if let Some(docs) = &enum_def.documentation {
            doc = self.utils.doc_comment(docs).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }

    /// Emit a TypeScript enum as string
    pub fn emit_enum_string(&self, enum_def: &Enum) -> Result<String, EmitError> {
        let doc = self.emit_enum_doc(enum_def)?;
        Ok(doc.pretty(80).to_string())
    }
}

impl Default for EnumEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit a TypeScript enum as string (legacy function for compatibility)
pub fn emit_enum_string(enum_def: &Enum) -> Result<String, EmitError> {
    let emitter = EnumEmitter::new();
    emitter.emit_enum_string(enum_def)
}
