//! Trait for converting AST nodes to RcDoc with context

use pretty::RcDoc;

use super::emission_context::EmissionContext;
use crate::emission::error::EmitError;

/// Trait for converting AST nodes to RcDoc with additional context
pub trait ToRcDocWithContext {
    /// Convert this AST node to an RcDoc with additional context
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError>;
}
