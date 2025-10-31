//! TypeScript code emitter

use pretty::RcDoc;
use tracing::error;

use crate::ast::{TsClassDefinition, TsNode, TsTypeDefinition};
use crate::emission::error::EmitError;
use crate::templating::TemplatingEmitter;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// OpenAPI specification metadata for file headers
#[derive(Debug, Clone)]
#[derive(Default)]
pub struct OpenApiMetadata {
    pub title: Option<String>,
    pub description: Option<String>,
    pub version: Option<String>,
}


/// TypeScript code emitter that routes between template and RcDoc paths
#[derive(Debug, Clone)]
pub struct TsLanguageEmitter {
    templating: TemplatingEmitter,
    metadata: OpenApiMetadata,
}

impl TsLanguageEmitter {
    /// Create a new TypeScript emitter
    pub fn new(max_line_width: usize) -> Self {
        Self {
            templating: TemplatingEmitter::new(max_line_width),
            metadata: OpenApiMetadata::default(),
        }
    }

    /// Create a new TypeScript emitter with OpenAPI metadata
    pub fn with_metadata(max_line_width: usize, metadata: OpenApiMetadata) -> Self {
        Self {
            templating: TemplatingEmitter::new(max_line_width),
            metadata,
        }
    }

    /// Emit TypeScript code from a class definition (template-based)
    pub fn emit_class(&self, class: &TsClassDefinition) -> Result<String, EmitError> {
        self.templating.emit_class(class)
    }

    /// Emit TypeScript code from type definitions (RcDoc-based)
    pub fn emit_type_definitions(
        &self,
        type_defs: &[&TsTypeDefinition],
    ) -> Result<String, EmitError> {
        if type_defs.is_empty() {
            return Ok(String::new());
        }

        let context = EmissionContext::default();
        let mut docs = Vec::new();

        // Add generated file header
        docs.push(RcDoc::text(self.get_default_header()));

        // Convert type definitions to RcDoc
        for type_def in type_defs {
            let doc = type_def.to_rcdoc_with_context(&context)?;
            docs.push(doc);
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line().append(RcDoc::line()));
        Ok(combined.pretty(80).to_string())
    }

    /// Emit TypeScript code from AST nodes with context
    pub fn emit_with_context(
        &self,
        nodes: &[TsNode],
        context: &EmissionContext,
    ) -> Result<String, EmitError> {
        let mut docs = Vec::new();

        // Add generated file header
        docs.push(RcDoc::text(self.get_default_header()));

        // Process nodes based on type
        for node in nodes {
            match node {
                TsNode::Class(class) => {
                    let class_code = self.templating.emit_class(class)?;
                    docs.push(RcDoc::text(class_code));
                }
                TsNode::TypeDefinition(type_def) => {
                    // Use RcDoc-based emission for type definitions
                    let doc = type_def.to_rcdoc_with_context(context)?;
                    docs.push(doc);
                }
                TsNode::Import(import) => {
                    // Use RcDoc-based emission for imports
                    let doc = import.to_rcdoc_with_context(context)?;
                    docs.push(doc);
                }
            }
        }

        let combined = RcDoc::intersperse(docs, RcDoc::line().append(RcDoc::line()));
        Ok(combined.pretty(80).to_string())
    }

    /// Get default file header using template
    fn get_default_header(&self) -> String {
        self.templating
            .emit_file_header(
                self.metadata.title.as_deref().unwrap_or(""),
                self.metadata.description.as_deref().unwrap_or(""),
                self.metadata.version.as_deref().unwrap_or(""),
            )
            .map(|s| s.trim_end_matches('\n').to_string())
            .unwrap_or_else(|e| {
                error!(error = %e, "Failed to render file header template");
                String::new()
            })
    }
}
