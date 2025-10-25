//! TypeScript code generator

use snafu::prelude::*;
use utoipa::openapi::OpenApi;

use crate::ast::*;
use crate::emitter::TypeScriptEmitter;

/// Error type for TypeScript generation
#[derive(Debug, Snafu)]
#[snafu(visibility(pub))]
pub enum GeneratorError {
    #[snafu(display("Generator error: {}", message))]
    Generic { message: String },
}

/// TypeScript code generator
pub struct TypeScriptGenerator {
    emitter: TypeScriptEmitter,
}

impl TypeScriptGenerator {
    /// Create a new TypeScript generator
    pub fn new() -> Self {
        Self {
            emitter: TypeScriptEmitter,
        }
    }

    /// Generate TypeScript code from OpenAPI specification
    pub fn generate(&self, _openapi: &OpenApi) -> Result<String, GeneratorError> {
        let mut nodes = Vec::new();
        
        // For now, generate a simple example interface
        let example_interface = Interface {
            name: "ExampleModel".to_string(),
            properties: vec![
                Property {
                    name: "id".to_string(),
                    type_expr: TypeExpression::Primitive(PrimitiveType::String),
                    optional: false,
                    documentation: Some("Unique identifier".to_string()),
                },
                Property {
                    name: "name".to_string(),
                    type_expr: TypeExpression::Primitive(PrimitiveType::String),
                    optional: true,
                    documentation: Some("Display name".to_string()),
                },
            ],
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: Some("Example model generated from OpenAPI".to_string()),
        };
        
        nodes.push(TsNode::Interface(example_interface));

        // Generate API client class
        let client_class = Class {
            name: "ApiClient".to_string(),
            properties: Vec::new(),
            methods: Vec::new(),
            extends: None,
            implements: Vec::new(),
            generics: Vec::new(),
            is_export: true,
            documentation: Some("Generated API client".to_string()),
        };
        
        nodes.push(TsNode::Class(client_class));

        // Emit all nodes
        self.emitter
            .emit(&nodes)
            .map_err(|e| GeneratorError::Generic {
                message: e.to_string(),
            })
    }

}
