//! TypeScript import emitter

use pretty::RcDoc;

use crate::ast::Import;
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// Helper struct for emitting TypeScript imports
pub struct ImportEmitter {
    utils: TypeScriptPrettyUtils,
}

impl ImportEmitter {
    pub fn new() -> Self {
        Self {
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a TypeScript import statement as RcDoc
    pub fn emit_import_doc(&self, import: &Import) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("import");

        if import.is_type_only {
            doc = doc.append(RcDoc::text(" type"));
        }

        doc = doc.append(RcDoc::space());

        if import.imports.is_empty() {
            // Namespace import: import * as module from 'module'
            doc = doc
                .append(RcDoc::text("* as "))
                .append(RcDoc::text(import.module.clone()));
        } else {
            // Named imports: import { name1, name2 as alias } from 'module'
            let specifier_docs: Vec<RcDoc<'static, ()>> = import
                .imports
                .iter()
                .map(|spec| {
                    if let Some(alias) = &spec.alias {
                        RcDoc::text(spec.name.clone())
                            .append(RcDoc::text(" as "))
                            .append(RcDoc::text(alias.clone()))
                    } else {
                        RcDoc::text(spec.name.clone())
                    }
                })
                .collect();

            let force_multiline = self.utils.should_format_multiline(
                import.imports.len(),
                false, // import specifiers are typically simple
            );

            doc = doc.append(
                self.utils
                    .adaptive_list(specifier_docs, "{ ", " }", force_multiline),
            );
        }

        doc = doc
            .append(RcDoc::text(" from "))
            .append(self.utils.single_quoted(&import.module))
            .append(RcDoc::text(";"));

        Ok(doc)
    }

    /// Emit a TypeScript import statement as string
    pub fn emit_import_string(&self, import: &Import) -> Result<String, EmitError> {
        let doc = self.emit_import_doc(import)?;
        Ok(doc.pretty(80).to_string())
    }
}

impl Default for ImportEmitter {
    fn default() -> Self {
        Self::new()
    }
}

/// Emit a TypeScript import statement as string (legacy function for compatibility)
pub fn emit_import_string(import: &Import) -> Result<String, EmitError> {
    let emitter = ImportEmitter::new();
    emitter.emit_import_string(import)
}
