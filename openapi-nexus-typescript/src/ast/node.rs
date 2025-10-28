//! TypeScript AST node types

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{Class, Enum, Export, Function, Import, Interface, TypeAlias};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;

/// TypeScript AST node types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
    Import(Import),
    Export(Export),
}

// Implement ToRcDocWithContext for TsNode enum
impl ToRcDocWithContext for TsNode {
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            TsNode::Interface(interface) => interface.to_rcdoc_with_context(context),
            TsNode::TypeAlias(type_alias) => type_alias.to_rcdoc_with_context(context),
            TsNode::Enum(enum_def) => enum_def.to_rcdoc_with_context(context),
            TsNode::Function(function) => function.to_rcdoc_with_context(context),
            TsNode::Class(class_def) => class_def.to_rcdoc_with_context(context),
            TsNode::Import(import) => import.to_rcdoc_with_context(context),
            TsNode::Export(export) => export.to_rcdoc_with_context(context),
        }
    }
}
