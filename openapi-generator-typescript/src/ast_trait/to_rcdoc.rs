//! Trait for converting AST nodes to RcDoc

use pretty::RcDoc;

use super::emission_context::EmissionContext;
use super::to_rcdoc_with_context::ToRcDocWithContext;
use crate::emission::error::EmitError;

/// Trait for converting AST nodes to RcDoc for pretty printing
pub trait ToRcDoc {
    /// Convert this AST node to an RcDoc
    fn to_rcdoc(&self) -> Result<RcDoc<'static, ()>, EmitError>;
}

// Default implementations that delegate to context-aware versions
impl<T> ToRcDoc for T
where
    T: ToRcDocWithContext,
{
    fn to_rcdoc(&self) -> Result<RcDoc<'static, ()>, EmitError> {
        self.to_rcdoc_with_context(&EmissionContext::default())
    }
}
