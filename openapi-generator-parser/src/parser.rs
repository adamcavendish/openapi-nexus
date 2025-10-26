//! OpenAPI parser implementation

use std::path::Path;

use snafu::ResultExt as _;
use utoipa::openapi::OpenApi;

use crate::error::{
    Error, FileReadSnafu, JsonParseSnafu, ParseWarning, SourceLocation, YamlParseSnafu,
};

/// Parser configuration options
#[derive(Debug, Clone)]
pub struct ParserConfig {
    pub allow_external_refs: bool,
    pub strict_mode: bool,
    pub validate_schemas: bool,
    pub max_reference_depth: usize,
}

impl Default for ParserConfig {
    fn default() -> Self {
        Self {
            allow_external_refs: false,
            strict_mode: true,
            validate_schemas: true,
            max_reference_depth: 10,
        }
    }
}

/// Result of parsing an OpenAPI specification
#[derive(Clone)]
pub struct ParseResult {
    pub openapi: OpenApi,
    pub warnings: Vec<ParseWarning>,
}

impl ParseResult {
    pub fn new(openapi: OpenApi) -> Self {
        Self {
            openapi,
            warnings: Vec::new(),
        }
    }

    pub fn with_warnings(openapi: OpenApi, warnings: Vec<ParseWarning>) -> Self {
        Self { openapi, warnings }
    }
}

/// OpenAPI parser with configuration support
pub struct OpenApiParser {
    config: ParserConfig,
}

impl OpenApiParser {
    /// Create a new parser with default configuration
    pub fn new() -> Self {
        Self {
            config: ParserConfig::default(),
        }
    }

    /// Create a new parser with custom configuration
    pub fn with_config(config: ParserConfig) -> Self {
        Self { config }
    }

    /// Parse an OpenAPI specification from a file
    pub fn parse_file<P: AsRef<Path>>(&self, path: P) -> Result<ParseResult, Error> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path).context(FileReadSnafu {
            path: path.to_string_lossy().to_string(),
        })?;

        let file_extension = path.extension().and_then(|ext| ext.to_str());
        self.parse_content(&content, file_extension)
    }

    /// Parse OpenAPI content from a string
    pub fn parse_content(
        &self,
        content: &str,
        file_extension: Option<&str>,
    ) -> Result<ParseResult, Error> {
        let openapi = match file_extension {
            Some("json") => self.parse_json(content)?,
            Some("yaml") | Some("yml") => self.parse_yaml(content)?,
            Some(ext) => {
                return Err(Error::UnsupportedFormat {
                    format: ext.to_string(),
                });
            }
            None => {
                // Try JSON first, then YAML
                self.parse_json(content)
                    .or_else(|_| self.parse_yaml(content))?
            }
        };

        let mut warnings = Vec::new();

        // Validate the parsed OpenAPI specification
        self.validate_openapi(&openapi, &mut warnings)?;

        Ok(ParseResult::with_warnings(openapi, warnings))
    }

    fn parse_json(&self, content: &str) -> Result<OpenApi, Error> {
        serde_json::from_str(content).context(JsonParseSnafu)
    }

    fn parse_yaml(&self, content: &str) -> Result<OpenApi, Error> {
        serde_norway::from_str(content).context(YamlParseSnafu)
    }

    /// Validate an OpenAPI specification
    fn validate_openapi(
        &self,
        openapi: &OpenApi,
        warnings: &mut Vec<ParseWarning>,
    ) -> Result<(), Error> {
        // Check required fields
        if openapi.info.title.is_empty() {
            return Err(Error::MissingRequiredField {
                field: "info.title".to_string(),
            });
        }

        if openapi.info.version.is_empty() {
            return Err(Error::MissingRequiredField {
                field: "info.version".to_string(),
            });
        }

        // Check if there are any paths defined
        if openapi.paths.paths.is_empty() {
            return Err(Error::ValidationError {
                message: "OpenAPI must have at least one path defined".to_string(),
            });
        }

        // Check for external references if not allowed
        if !self.config.allow_external_refs {
            self.check_external_references(openapi, warnings)?;
        }

        // Check for circular references
        if self.config.validate_schemas {
            self.check_circular_references(openapi, warnings)?;
        }

        Ok(())
    }

    /// Check for external references in the OpenAPI spec
    fn check_external_references(
        &self,
        openapi: &OpenApi,
        warnings: &mut Vec<ParseWarning>,
    ) -> Result<(), Error> {
        // This is a simplified check - in a full implementation, we would traverse
        // all schemas, responses, parameters, etc. to find external references
        if let Some(components) = &openapi.components {
            for (name, schema) in &components.schemas {
                if let Some(ref_path) = self.get_schema_reference(schema)
                    && (ref_path.starts_with("http://") || ref_path.starts_with("https://"))
                {
                    warnings.push(ParseWarning::new(
                        format!(
                            "External reference found in schema '{}': {}",
                            name, ref_path
                        ),
                        SourceLocation::new()
                            .with_openapi_path(format!("/components/schemas/{}", name)),
                    ));
                }
            }
        }
        Ok(())
    }

    /// Check for circular references in schemas
    fn check_circular_references(
        &self,
        openapi: &OpenApi,
        _warnings: &mut Vec<ParseWarning>,
    ) -> Result<(), Error> {
        // This is a simplified check - in a full implementation, we would build
        // a dependency graph and detect cycles
        if let Some(components) = &openapi.components {
            for (name, schema) in &components.schemas {
                if let Some(ref_path) = self.get_schema_reference(schema)
                    && ref_path.contains(&format!("#/components/schemas/{}", name))
                {
                    return Err(Error::CircularReference {
                        reference: format!("Schema '{}' references itself", name),
                    });
                }
            }
        }
        Ok(())
    }

    /// Extract reference path from a schema (simplified implementation)
    fn get_schema_reference(
        &self,
        schema: &utoipa::openapi::RefOr<utoipa::openapi::Schema>,
    ) -> Option<String> {
        // This is a simplified implementation - in practice, we'd need to handle
        // all the different schema types and their reference patterns
        match schema {
            utoipa::openapi::RefOr::Ref(ref_schema) => Some(ref_schema.ref_location.clone()),
            _ => None,
        }
    }
}

impl Default for OpenApiParser {
    fn default() -> Self {
        Self::new()
    }
}
