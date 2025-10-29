//! Optimized runtime module generator for TypeScript using hybrid template + AST approach

use crate::ast::{TsNode, TypeExpression};
use crate::core::GeneratorError;
use crate::emission::TypeScriptFileCategory;
use crate::generator::file_generator::GeneratedFile;
// use crate::generator::runtime_component_builder::RuntimeComponentBuilder;  // Disabled
use crate::generator::template_generator::TemplateGenerator;

/// Optimized runtime module generator using hybrid template + AST approach
pub struct RuntimeGenerator {
    // component_builder: RuntimeComponentBuilder,  // Disabled
}

impl Default for RuntimeGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeGenerator {
    /// Create a new optimized runtime generator
    pub fn new() -> Self {
        Self {
            component_builder: RuntimeComponentBuilder::default(),
        }
    }

    /// Generate multiple runtime files using hybrid approach
    pub fn generate_runtime_files(&self) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();

        // Generate core utilities using static files
        files.extend(self.generate_core_files()?);

        // Generate configuration using hybrid approach
        files.extend(self.generate_configuration_files()?);

        // Generate API components using static files
        files.extend(self.generate_api_files()?);

        Ok(files)
    }

    /// Generate core utility files using static templates
    fn generate_core_files(&self) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();
        let runtime_files = TemplateGenerator::get_runtime_files_by_category();

        // Core functions
        if let Some(function_files) = runtime_files.get("functions") {
            for (filename, content) in function_files {
                files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                    file_category: TypeScriptFileCategory::Runtime,
                });
            }
        }

        // Utility types
        if let Some(type_files) = runtime_files.get("types") {
            for (filename, content) in type_files {
            files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                file_category: TypeScriptFileCategory::Runtime,
            });
            }
        }

        Ok(files)
    }

    /// Generate configuration files using hybrid approach
    fn generate_configuration_files(&self) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();

        // Generate Configuration class using AST (for complex logic)
        let config_class = self.generate_configuration_class()?;

        // Get constants from static files
        let base_path = TemplateGenerator::get_runtime_file("constants/base_path.ts")
            .ok_or_else(|| GeneratorError::Generic {
                message: "Base path constant not found".to_string(),
            })?;
        
        let default_config = TemplateGenerator::get_runtime_file("constants/default_config.ts")
            .ok_or_else(|| GeneratorError::Generic {
                message: "Default config constant not found".to_string(),
            })?;

        // Get ConfigurationParameters interface from static file
        let config_params_interface = TemplateGenerator::get_runtime_file("interfaces/configuration_parameters.ts")
            .ok_or_else(|| GeneratorError::Generic {
                message: "ConfigurationParameters interface not found".to_string(),
            })?;

        // Combine constants and interface
        let constants = format!("{}\n\n{}\n\n{}", config_params_interface, base_path, default_config);

        // Generate config.ts file
        let config_content = self.component_builder.generate_nodes_batch(
            &[config_class],
            None,
            Some(&constants),
        )?;

        files.push(GeneratedFile {
            filename: "config.ts".to_string(),
            content: config_content,
            file_category: TypeScriptFileCategory::Runtime,
        });

        Ok(files)
    }

    /// Generate API files using static templates
    fn generate_api_files(&self) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();
        let runtime_files = TemplateGenerator::get_runtime_files_by_category();

            // Interfaces
        if let Some(interface_files) = runtime_files.get("interfaces") {
            for (filename, content) in interface_files {
                files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                    file_category: TypeScriptFileCategory::Runtime,
                });
            }
        }

        // Classes
        if let Some(class_files) = runtime_files.get("classes") {
            for (filename, content) in class_files {
                files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                    file_category: TypeScriptFileCategory::Runtime,
                });
            }
        }

        // Constants
        if let Some(constant_files) = runtime_files.get("constants") {
            for (filename, content) in constant_files {
                files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                    file_category: TypeScriptFileCategory::Runtime,
        });
            }
        }

        // Core files (BaseAPI, utils)
        if let Some(core_files) = runtime_files.get("core") {
            for (filename, content) in core_files {
                files.push(GeneratedFile {
                    filename: filename.clone(),
                    content: self.add_file_header(content),
                    file_category: TypeScriptFileCategory::Runtime,
                });
            }
        }

        Ok(files)
    }


    /// Generate Configuration class using AST
    fn generate_configuration_class(&self) -> Result<TsNode, GeneratorError> {
        let mut methods = Vec::new();

        // Constructor method
        methods.push(self.component_builder.build_constructor(
            vec![self.component_builder.build_parameter(
                "configuration",
                Some(TypeExpression::Reference("ConfigurationParameters".to_string())),
                true,
                Some("{}".to_string()),
            )],
                "this.configuration = configuration;".to_string(),
        ));

        // Getter methods for all configuration properties
        let getter_methods = vec![
            ("basePath", "string", "return this.configuration.basePath != null ? this.configuration.basePath : BASE_PATH;"),
            ("fetchApi", "FetchAPI | undefined", "return this.configuration.fetchApi;"),
            ("middleware", "Middleware[]", "return this.configuration.middleware || [];"),
            ("queryParamsStringify", "(params: HTTPQuery) => string", "return this.configuration.queryParamsStringify || querystring;"),
            ("username", "string | undefined", "return this.configuration.username;"),
            ("password", "string | undefined", "return this.configuration.password;"),
            ("apiKey", "((name: string) => string | Promise<string>) | undefined", "const apiKey = this.configuration.apiKey; if (apiKey) { return typeof apiKey === 'function' ? apiKey : () => apiKey; } return undefined;"),
            ("accessToken", "((name?: string, scopes?: string[]) => string | Promise<string>) | undefined", "const accessToken = this.configuration.accessToken; if (accessToken) { return typeof accessToken === 'function' ? accessToken : async () => accessToken; } return undefined;"),
            ("headers", "HTTPHeaders | undefined", "return this.configuration.headers;"),
            ("credentials", "RequestCredentials | undefined", "return this.configuration.credentials;"),
        ];

        for (name, return_type, body) in getter_methods {
            methods.push(self.component_builder.build_getter(
                name,
                TypeExpression::Reference(return_type.to_string()),
                    body.to_string(),
            ));
        }

        Ok(self.component_builder.build_class(
            "Configuration",
            vec![self.component_builder.build_property(
                "configuration",
                TypeExpression::Reference("ConfigurationParameters".to_string()),
                false,
                None,
            )],
            methods,
            None,
            Some("Configuration class for API client".to_string()),
        ))
    }

    /// Add generated file header to content
    fn add_file_header(&self, content: &str) -> String {
        const HEADER: &str = "// DO NOT EDIT - This file is automatically generated.
// Any manual changes will be overwritten on the next generation.
// To make changes, modify the source code and regenerate this file.

";
        format!("{}{}", HEADER, content)
    }
}
