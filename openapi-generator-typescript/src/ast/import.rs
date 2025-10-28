//! TypeScript import statement definition

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::ImportSpecifier;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;

/// TypeScript import statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub module: String,
    pub imports: Vec<ImportSpecifier>,
    pub is_type_only: bool,
}

impl Import {
    /// Create a new import statement
    pub fn new(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            imports: Vec::new(),
            is_type_only: false,
        }
    }

    /// Create a new type-only import statement
    pub fn type_only(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            imports: Vec::new(),
            is_type_only: true,
        }
    }

    /// Add import specifiers to the import
    pub fn with_specifiers(mut self, specifiers: Vec<ImportSpecifier>) -> Self {
        self.imports = specifiers;
        self
    }

    /// Add a single named import
    pub fn with_named_import(mut self, name: impl Into<String>) -> Self {
        self.imports.push(ImportSpecifier::new(name));
        self
    }

    /// Add multiple named imports
    pub fn with_named_imports(
        mut self,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        for name in names {
            self.imports.push(ImportSpecifier::new(name));
        }
        self
    }

    /// Set whether this is a type-only import
    pub fn set_type_only(mut self, is_type_only: bool) -> Self {
        self.is_type_only = is_type_only;
        self
    }

    /// Make this import type-only
    pub fn make_type_only(mut self) -> Self {
        self.is_type_only = true;
        self
    }
}

impl ToRcDocWithContext for Import {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();

        let mut doc = RcDoc::text("import");

        if self.is_type_only {
            doc = doc.append(RcDoc::text(" type"));
        }

        if self.imports.is_empty() {
            // Default import
            doc = doc.append(RcDoc::text(" * as "));
        } else {
            // Named imports
            let import_docs: Result<Vec<_>, _> = self
                .imports
                .iter()
                .map(|spec| spec.to_rcdoc_with_context(context))
                .collect();
            let imports = import_docs?;
            doc = doc
                .append(RcDoc::text(" { "))
                .append(utils.comma_separated(imports))
                .append(RcDoc::text(" }"));
        }

        doc = doc
            .append(RcDoc::text(" from "))
            .append(utils.single_quoted(&self.module));

        Ok(doc)
    }
}
