//! TypeScript export statement definition

use serde::{Deserialize, Serialize};

use crate::ast::ExportSpecifier;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript export statement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Export {
    pub specifier: ExportSpecifier,
    pub is_type_only: bool,
}

impl ToRcDocWithContext for Export {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("export");

        if self.is_type_only {
            doc = doc.append(RcDoc::text(" type"));
        }

        doc = doc
            .append(RcDoc::space())
            .append(self.specifier.to_rcdoc_with_context(context)?);

        Ok(doc)
    }
}
