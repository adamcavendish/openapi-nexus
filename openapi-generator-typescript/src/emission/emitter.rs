//! TypeScript code emitter

use pretty::RcDoc;

use std::collections::HashMap;

use crate::ast::TsNode;
use crate::emission::class_emitter::ClassEmitter;
use crate::emission::constants::GENERATED_FILE_HEADER;
use crate::emission::enum_emitter::emit_enum_string;
use crate::emission::error::EmitError;
use crate::emission::function_emitter::FunctionEmitter;
use crate::emission::import_emitter::emit_import_string;
use crate::emission::import_manager::ImportManager;
use crate::emission::interface_emitter::InterfaceEmitter;
use crate::emission::type_alias_emitter::TypeAliasEmitter;

/// TypeScript code emitter
pub struct TypeScriptEmitter {
    interface_emitter: InterfaceEmitter,
    type_alias_emitter: TypeAliasEmitter,
    function_emitter: FunctionEmitter,
    class_emitter: ClassEmitter,
    import_manager: ImportManager,
}

impl TypeScriptEmitter {
    pub fn new() -> Self {
        Self {
            interface_emitter: InterfaceEmitter::new(),
            type_alias_emitter: TypeAliasEmitter::new(),
            function_emitter: FunctionEmitter::new(),
            class_emitter: ClassEmitter::new(),
            import_manager: ImportManager::new(),
        }
    }

    /// Emit TypeScript code from AST nodes
    pub fn emit(&self, nodes: &[TsNode]) -> Result<String, EmitError> {
        self.emit_with_context(nodes, "", &HashMap::new())
    }

    /// Emit TypeScript code from AST nodes with import context
    pub fn emit_with_context(
        &self,
        nodes: &[TsNode],
        current_file: &str,
        schema_to_file_map: &HashMap<String, String>,
    ) -> Result<String, EmitError> {
        let mut docs = Vec::new();

        // Add generated file header
        docs.push(RcDoc::text(GENERATED_FILE_HEADER));

        // Generate imports based on dependencies
        let imports = self.import_manager.generate_imports_for_file(
            nodes,
            current_file,
            schema_to_file_map,
        )?;

        if !imports.is_empty() {
            let import_code = self.import_manager.emit_imports(&imports)?;
            docs.push(RcDoc::text(import_code));
        }

        for node in nodes {
            let doc = self.emit_node(node)?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
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

impl Default for TypeScriptEmitter {
    fn default() -> Self {
        Self::new()
    }
}
