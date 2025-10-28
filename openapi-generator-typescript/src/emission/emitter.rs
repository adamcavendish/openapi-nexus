//! TypeScript code emitter

use pretty::RcDoc;

use std::collections::HashMap;

use crate::ast::{GeneratedFileHeader, ImportCollection, ImportResolver, TsNode};
use crate::ast_trait::{EmissionContext, ToRcDoc, ToRcDocWithContext};
use crate::emission::dependency_analyzer::DependencyAnalyzer;
use crate::emission::error::EmitError;

/// TypeScript code emitter
pub struct TypeScriptEmitter {
    dependency_analyzer: DependencyAnalyzer,
}

impl TypeScriptEmitter {
    pub fn new() -> Self {
        Self {
            dependency_analyzer: DependencyAnalyzer::new(),
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
        let header = GeneratedFileHeader::new();
        docs.push(header.to_rcdoc()?);

        // Generate imports based on dependencies using AST-centric approach
        let dependencies = self.dependency_analyzer.analyze_dependencies(nodes);
        let import_resolver =
            ImportResolver::new(schema_to_file_map.clone(), current_file.to_string());
        let imports = import_resolver.resolve_dependencies(&dependencies)?;

        if !imports.is_empty() {
            // Convert Vec<Import> to ImportCollection for rendering
            let mut import_collection = ImportCollection::new();
            for import in imports {
                import_collection.add_import(import);
            }
            let import_doc = import_collection.to_rcdoc()?;
            docs.push(import_doc);
        }

        // Convert AST nodes to RcDoc using traits
        for node in nodes {
            let doc = node.to_rcdoc()?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    /// Emit TypeScript code from AST nodes with custom context
    pub fn emit_with_custom_context(
        &self,
        nodes: &[TsNode],
        context: &EmissionContext,
        current_file: &str,
        schema_to_file_map: &HashMap<String, String>,
    ) -> Result<String, EmitError> {
        let mut docs = Vec::new();

        // Add generated file header
        let header = GeneratedFileHeader::new();
        docs.push(header.to_rcdoc()?);

        // Generate imports based on dependencies using AST-centric approach
        let dependencies = self.dependency_analyzer.analyze_dependencies(nodes);
        let import_resolver =
            ImportResolver::new(schema_to_file_map.clone(), current_file.to_string());
        let imports = import_resolver.resolve_dependencies(&dependencies)?;

        if !imports.is_empty() {
            // Convert Vec<Import> to ImportCollection for rendering
            let mut import_collection = ImportCollection::new();
            for import in imports {
                import_collection.add_import(import);
            }
            let import_doc = import_collection.to_rcdoc()?;
            docs.push(import_doc);
        }

        // Convert AST nodes to RcDoc using traits with context
        for node in nodes {
            let doc = node.to_rcdoc_with_context(context)?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }
}

impl Default for TypeScriptEmitter {
    fn default() -> Self {
        Self::new()
    }
}
