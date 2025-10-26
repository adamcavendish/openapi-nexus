//! High-level TypeScript code generation

pub mod api_client;
pub mod schema;

use std::collections::HashMap;
use utoipa::openapi::OpenApi;

use crate::core::{GeneratorConfig, GeneratorError};
use crate::emission::{
    TypeScriptEmitter,
    file_generator::{FileGenerator, GeneratedFile},
};
use api_client::ApiClientGenerator;
use schema::SchemaGenerator;

/// Main TypeScript code generator
pub struct TypeScriptGenerator {
    emitter: TypeScriptEmitter,
    schema_generator: SchemaGenerator,
    api_client_generator: ApiClientGenerator,
    config: GeneratorConfig,
}

impl TypeScriptGenerator {
    /// Create a new TypeScript generator with default configuration
    pub fn new() -> Self {
        Self {
            emitter: TypeScriptEmitter,
            schema_generator: SchemaGenerator::new(),
            api_client_generator: ApiClientGenerator::new(),
            config: GeneratorConfig::default(),
        }
    }

    /// Create a new TypeScript generator with custom configuration
    pub fn with_config(config: GeneratorConfig) -> Self {
        Self {
            emitter: TypeScriptEmitter,
            schema_generator: SchemaGenerator::new(),
            api_client_generator: ApiClientGenerator::new(),
            config,
        }
    }

    /// Generate TypeScript code from OpenAPI specification (single file)
    pub fn generate(&self, openapi: &OpenApi) -> Result<String, GeneratorError> {
        let mut nodes = Vec::new();

        // Generate interfaces and types from schemas
        if let Some(components) = &openapi.components {
            for (name, schema_ref) in &components.schemas {
                match self.schema_generator.schema_to_ts_node(name, schema_ref) {
                    Ok(node) => nodes.push(node),
                    Err(e) => {
                        tracing::warn!("Failed to convert schema {}: {}", name, e);
                    }
                }
            }
        }

        // Generate API client class with methods from operations
        let api_client = self
            .api_client_generator
            .generate_api_client_with_methods(openapi)?;
        nodes.push(api_client);

        // Emit all nodes
        self.emitter
            .emit(&nodes)
            .map_err(|e| GeneratorError::Generic {
                message: e.to_string(),
            })
    }

    /// Generate multiple TypeScript files from OpenAPI specification
    pub fn generate_files(&self, openapi: &OpenApi) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut schemas = HashMap::new();

        // Generate interfaces and types from schemas
        if let Some(components) = &openapi.components {
            for (name, schema_ref) in &components.schemas {
                match self.schema_generator.schema_to_ts_node(name, schema_ref) {
                    Ok(node) => {
                        schemas.insert(name.clone(), node);
                    }
                    Err(e) => {
                        tracing::warn!("Failed to convert schema {}: {}", name, e);
                    }
                }
            }
        }

        // Generate API client class with methods from operations
        let api_client = self
            .api_client_generator
            .generate_api_client_with_methods(openapi)?;
        schemas.insert("ApiClient".to_string(), api_client);

        // Generate files using file generator
        let file_generator = FileGenerator::new(self.config.file_config.clone());
        file_generator
            .generate_files(&schemas)
            .map_err(|e| GeneratorError::Generic {
                message: format!("File generation error: {}", e),
            })
    }
}

impl Default for TypeScriptGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl crate::core::TypeScriptGenerator for TypeScriptGenerator {
    fn generate(&self, openapi: &OpenApi) -> Result<String, GeneratorError> {
        self.generate(openapi)
    }

    fn generate_files(&self, openapi: &OpenApi) -> Result<Vec<GeneratedFile>, GeneratorError> {
        self.generate_files(openapi)
    }
}
