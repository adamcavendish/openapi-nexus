//! TypeScript code emitter

use pretty::RcDoc;
use snafu::prelude::*;

use crate::ast::*;

/// Error type for TypeScript emission
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum EmitError {
    #[snafu(display("Emit error: {}", message))]
    Generic { message: String },
}

/// TypeScript code emitter
pub struct TypeScriptEmitter;

impl TypeScriptEmitter {
    /// Emit TypeScript code from AST nodes
    pub fn emit(&self, nodes: &[TsNode]) -> Result<String, EmitError> {
        let mut docs = Vec::new();

        for node in nodes {
            let doc = self.emit_node(node)?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    fn emit_node(&self, node: &TsNode) -> Result<RcDoc<()>, EmitError> {
        match node {
            TsNode::Interface(interface) => self.emit_interface(interface),
            TsNode::TypeAlias(type_alias) => self.emit_type_alias(type_alias),
            TsNode::Enum(enum_def) => self.emit_enum(enum_def),
            TsNode::Function(function) => self.emit_function(function),
            TsNode::Class(class_def) => self.emit_class(class_def),
        }
    }

    fn emit_interface(&self, interface: &Interface) -> Result<RcDoc<()>, EmitError> {
        // TODO: Implement interface emission
        Ok(RcDoc::text("// TODO: Interface emission"))
    }

    fn emit_type_alias(&self, type_alias: &TypeAlias) -> Result<RcDoc<()>, EmitError> {
        // TODO: Implement type alias emission
        Ok(RcDoc::text("// TODO: Type alias emission"))
    }

    fn emit_enum(&self, enum_def: &Enum) -> Result<RcDoc<()>, EmitError> {
        // TODO: Implement enum emission
        Ok(RcDoc::text("// TODO: Enum emission"))
    }

    fn emit_function(&self, function: &Function) -> Result<RcDoc<()>, EmitError> {
        // TODO: Implement function emission
        Ok(RcDoc::text("// TODO: Function emission"))
    }

    fn emit_class(&self, class_def: &Class) -> Result<RcDoc<()>, EmitError> {
        // TODO: Implement class emission
        Ok(RcDoc::text("// TODO: Class emission"))
    }
}
