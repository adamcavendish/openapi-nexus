//! TypeScript export specifier definitions

use serde::{Deserialize, Serialize};

use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use pretty::RcDoc;

/// TypeScript export specifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExportSpecifier {
    Named(String),
    Default(String),
    All(String),
    From(String, Vec<String>),
}

impl ToRcDocWithContext for ExportSpecifier {
    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let utils = TypeScriptPrettyUtils::new();

        match self {
            ExportSpecifier::Named(name) => Ok(RcDoc::text(name.clone())),
            ExportSpecifier::Default(name) => {
                Ok(RcDoc::text("default ").append(RcDoc::text(name.clone())))
            }
            ExportSpecifier::All(name) => {
                Ok(RcDoc::text("* as ").append(RcDoc::text(name.clone())))
            }
            ExportSpecifier::From(module, names) => {
                let name_docs: Vec<RcDoc<'static, ()>> =
                    names.iter().map(|n| RcDoc::text(n.clone())).collect();
                Ok(RcDoc::text("{ ")
                    .append(utils.comma_separated(name_docs))
                    .append(RcDoc::text(" } from "))
                    .append(utils.single_quoted(module)))
            }
        }
    }
}
