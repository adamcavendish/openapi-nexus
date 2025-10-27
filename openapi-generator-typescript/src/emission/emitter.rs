//! TypeScript code emitter

use pretty::RcDoc;

use crate::ast::TsNode;
use crate::emission::class_emitter::ClassEmitter;
use crate::emission::constants::GENERATED_FILE_HEADER;
use crate::emission::enum_emitter::emit_enum_string;
use crate::emission::error::EmitError;
use crate::emission::function_emitter::FunctionEmitter;
use crate::emission::import_emitter::emit_import_string;
use crate::emission::interface_emitter::InterfaceEmitter;
use crate::emission::type_alias_emitter::TypeAliasEmitter;

/// TypeScript code emitter
pub struct TypeScriptEmitter {
    interface_emitter: InterfaceEmitter,
    type_alias_emitter: TypeAliasEmitter,
    function_emitter: FunctionEmitter,
    class_emitter: ClassEmitter,
}

impl TypeScriptEmitter {
    pub fn new() -> Self {
        Self {
            interface_emitter: InterfaceEmitter::new(),
            type_alias_emitter: TypeAliasEmitter::new(),
            function_emitter: FunctionEmitter::new(),
            class_emitter: ClassEmitter::new(),
        }
    }

    /// Emit TypeScript code from AST nodes
    pub fn emit(&self, nodes: &[TsNode]) -> Result<String, EmitError> {
        let mut docs = Vec::new();

        // Add generated file header
        docs.push(RcDoc::text(GENERATED_FILE_HEADER));

        // Check if this is an API class file and add runtime imports
        let needs_runtime_imports = self.needs_runtime_imports(nodes);
        if needs_runtime_imports {
            docs.push(RcDoc::text(
                "import { BaseAPI, RequestContext } from '../runtime/api';\n",
            ));
            docs.push(RcDoc::text(
                "import { Configuration } from '../runtime/config';\n",
            ));
        }

        for node in nodes {
            let doc = self.emit_node(node)?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    /// Check if nodes contain API classes that need runtime imports
    fn needs_runtime_imports(&self, nodes: &[TsNode]) -> bool {
        nodes.iter().any(|node| {
            if let TsNode::Class(class) = node {
                class
                    .extends
                    .as_ref()
                    .is_some_and(|extends| extends == "BaseAPI")
            } else {
                false
            }
        })
    }

    fn emit_node(&self, node: &TsNode) -> Result<RcDoc<'_, ()>, EmitError> {
        match node {
            TsNode::Interface(interface) => {
                let result = self.interface_emitter.emit_interface_string(interface)?;
                Ok(RcDoc::text(result))
            }
            TsNode::TypeAlias(type_alias) => {
                let result = self.type_alias_emitter.emit_type_alias_string(type_alias)?;
                Ok(RcDoc::text(result))
            }
            TsNode::Enum(enum_def) => {
                let result = emit_enum_string(enum_def)?;
                Ok(RcDoc::text(result))
            }
            TsNode::Function(function) => {
                let result = self.function_emitter.emit_function_string(function)?;
                Ok(RcDoc::text(result))
            }
            TsNode::Class(class_def) => {
                let result = self.class_emitter.emit_class_string(class_def)?;
                Ok(RcDoc::text(result))
            }
            TsNode::Import(import) => {
                let result = emit_import_string(import)?;
                Ok(RcDoc::text(result))
            }
            TsNode::Export(_) => Ok(RcDoc::text("// TODO: Export emission")),
        }
    }
}