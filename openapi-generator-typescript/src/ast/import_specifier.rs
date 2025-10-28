//! TypeScript import specifier definition

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript import specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportSpecifier {
    pub name: String,
    pub alias: Option<String>,
}

impl ImportSpecifier {
    /// Create a new import specifier with just a name
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias: None,
        }
    }

    /// Create a new import specifier with an alias
    pub fn with_alias(name: impl Into<String>, alias: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            alias: Some(alias.into()),
        }
    }

    /// Add an alias to an existing specifier
    pub fn alias(mut self, alias: impl Into<String>) -> Self {
        self.alias = Some(alias.into());
        self
    }
}

impl ToRcDocWithContext for ImportSpecifier {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(self.name.clone());

        if let Some(alias) = &self.alias {
            doc = doc
                .append(RcDoc::text(" as "))
                .append(RcDoc::text(alias.clone()));
        }

        Ok(doc)
    }
}
