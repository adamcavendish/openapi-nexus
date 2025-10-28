//! TypeScript import collection AST

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{Import, ImportSpecifier};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// A collection of imports for a TypeScript file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportCollection {
    /// All imports in this collection
    imports: Vec<Import>,
}

impl ImportCollection {
    /// Create a new empty import collection
    pub fn new() -> Self {
        Self {
            imports: Vec::new(),
        }
    }

    /// Add an import to the collection
    pub fn add_import(&mut self, import: Import) {
        self.imports.push(import);
    }

    /// Add a runtime import (non-type-only)
    pub fn add_runtime_import(
        &mut self,
        module: impl Into<String>,
        specifiers: Vec<ImportSpecifier>,
    ) {
        self.imports
            .push(Import::new(module).with_specifiers(specifiers));
    }

    /// Add a runtime import with named imports
    pub fn add_runtime_named_imports(
        &mut self,
        module: impl Into<String>,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.imports
            .push(Import::new(module).with_named_imports(names));
    }

    /// Add a model import (type-only)
    pub fn add_model_import(
        &mut self,
        module: impl Into<String>,
        specifiers: Vec<ImportSpecifier>,
    ) {
        self.imports
            .push(Import::type_only(module).with_specifiers(specifiers));
    }

    /// Add a model import with named imports
    pub fn add_model_named_imports(
        &mut self,
        module: impl Into<String>,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.imports
            .push(Import::type_only(module).with_named_imports(names));
    }

    /// Add an external import (non-type-only)
    pub fn add_external_import(
        &mut self,
        module: impl Into<String>,
        specifiers: Vec<ImportSpecifier>,
    ) {
        self.imports
            .push(Import::new(module).with_specifiers(specifiers));
    }

    /// Add an external import with named imports
    pub fn add_external_named_imports(
        &mut self,
        module: impl Into<String>,
        names: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.imports
            .push(Import::new(module).with_named_imports(names));
    }

    /// Get all imports
    pub fn imports(&self) -> &[Import] {
        &self.imports
    }

    /// Check if the collection is empty
    pub fn is_empty(&self) -> bool {
        self.imports.is_empty()
    }

    /// Get the number of imports
    pub fn len(&self) -> usize {
        self.imports.len()
    }

    /// Sort all imports for consistent output
    pub fn sort(&mut self) {
        self.imports.sort_by(|a, b| a.module.cmp(&b.module));
    }

    /// Merge another import collection into this one
    pub fn merge(&mut self, other: ImportCollection) {
        self.imports.extend(other.imports);
    }
}

impl Default for ImportCollection {
    fn default() -> Self {
        Self::new()
    }
}

impl ToRcDocWithContext for ImportCollection {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.is_empty() {
            return Ok(RcDoc::nil());
        }

        let docs: Result<Vec<_>, _> = self
            .imports
            .iter()
            .map(|import| import.to_rcdoc())
            .collect();

        // Join with newlines and add extra spacing
        Ok(RcDoc::intersperse(docs?, RcDoc::line()).append(RcDoc::line()))
    }
}
