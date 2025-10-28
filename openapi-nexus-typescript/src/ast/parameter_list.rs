//! TypeScript parameter list

use serde::{Deserialize, Serialize};

use crate::ast::Parameter;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use pretty::RcDoc;

/// TypeScript parameter list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterList(pub Vec<Parameter>);

impl ParameterList {
    /// Create a new parameter list
    pub fn new(parameters: Vec<Parameter>) -> Self {
        Self(parameters)
    }

    /// Create an empty parameter list
    pub fn empty() -> Self {
        Self(Vec::new())
    }
}

impl ToRcDocWithContext for ParameterList {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if self.0.is_empty() {
            Ok(RcDoc::text("()"))
        } else {
            let param_docs: Result<Vec<RcDoc<'static, ()>>, EmitError> = self
                .0
                .iter()
                .map(|param| param.to_rcdoc_with_context(context))
                .collect();
            let params = param_docs?;

            // For long parameter lists, format across multiple lines
            if self.0.len() > 3 {
                Ok(RcDoc::text("(")
                    .append(RcDoc::line_())
                    .append(
                        RcDoc::intersperse(params, RcDoc::text(",").append(RcDoc::line())).nest(2),
                    )
                    .append(RcDoc::line_())
                    .append(RcDoc::text(")")))
            } else {
                Ok(RcDoc::text("(")
                    .append(RcDoc::intersperse(params, RcDoc::text(", ")))
                    .append(RcDoc::text(")")))
            }
        }
    }
}
