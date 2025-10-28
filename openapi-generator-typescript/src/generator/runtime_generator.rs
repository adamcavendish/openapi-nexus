//! Runtime module generator for TypeScript

use crate::ast::{
    Class, CodeBlock, Function, Interface, Parameter, PrimitiveType, Property, Statement, TsMethod,
    TsNode, TypeExpression, Visibility,
};
use crate::ast_trait::to_rcdoc::ToRcDoc;
use crate::core::GeneratorError;
use crate::emission::TypeScriptFileCategory;
use crate::generator::file_generator::GeneratedFile;

/// Runtime module generator for creating TypeScript runtime utilities
pub struct RuntimeGenerator;

impl Default for RuntimeGenerator {
    fn default() -> Self {
        Self
    }
}

impl RuntimeGenerator {
    /// Create a new runtime generator
    pub fn new() -> Self {
        Self
    }

    /// Generate multiple runtime files
    pub fn generate_runtime_files(&self) -> Result<Vec<GeneratedFile>, GeneratorError> {
        let mut files = Vec::new();

        // Generate core runtime utilities
        let core_nodes = vec![
            self.generate_to_json_function()?,
            self.generate_from_json_function()?,
        ];
        files.push(GeneratedFile {
            filename: "core.ts".to_string(),
            content: self.nodes_to_string(&core_nodes)?,
            file_category: TypeScriptFileCategory::Runtime,
        });

        // Generate configuration interfaces
        let config_nodes = vec![
            self.generate_configuration_interface()?,
            self.generate_configuration_parameters_interface()?,
        ];
        files.push(GeneratedFile {
            filename: "config.ts".to_string(),
            content: self.nodes_to_string(&config_nodes)?,
            file_category: TypeScriptFileCategory::Runtime,
        });

        // Generate API base classes
        let api_nodes = vec![
            self.generate_base_api_class()?,
            self.generate_required_error_class()?,
            self.generate_request_context_interface()?,
        ];
        files.push(GeneratedFile {
            filename: "api.ts".to_string(),
            content: self.nodes_to_string_with_imports(
                &api_nodes,
                "import { Configuration } from './config';\n",
            )?,
            file_category: TypeScriptFileCategory::Runtime,
        });

        Ok(files)
    }

    /// Convert nodes to string using the TypeScript emitter
    fn nodes_to_string(&self, nodes: &[TsNode]) -> Result<String, GeneratorError> {
        // For runtime files, we don't want dependency analysis - just emit the nodes directly
        let mut docs = Vec::new();

        // Add generated file header
        let header = crate::ast::GeneratedFileHeader::new();
        docs.push(header.to_rcdoc().map_err(|e| GeneratorError::Generic {
            message: format!("Header emission error: {}", e),
        })?);

        // Convert AST nodes to RcDoc using traits
        for node in nodes {
            let doc = node.to_rcdoc().map_err(|e| GeneratorError::Generic {
                message: format!("Node emission error: {}", e),
            })?;
            docs.push(doc);
        }

        let combined = pretty::RcDoc::intersperse(docs, pretty::RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    /// Convert nodes to string with additional imports
    fn nodes_to_string_with_imports(
        &self,
        nodes: &[TsNode],
        imports: &str,
    ) -> Result<String, GeneratorError> {
        // For runtime files, we don't want dependency analysis - just emit the nodes directly
        let mut docs = Vec::new();

        // Add generated file header
        let header = crate::ast::GeneratedFileHeader::new();
        docs.push(header.to_rcdoc().map_err(|e| GeneratorError::Generic {
            message: format!("Header emission error: {}", e),
        })?);

        // Add manual imports
        docs.push(pretty::RcDoc::text(imports.to_string()));

        // Convert AST nodes to RcDoc using traits
        for node in nodes {
            let doc = node.to_rcdoc().map_err(|e| GeneratorError::Generic {
                message: format!("Node emission error: {}", e),
            })?;
            docs.push(doc);
        }

        let combined = pretty::RcDoc::intersperse(docs, pretty::RcDoc::line());
        Ok(combined.pretty(80).to_string())
    }

    /// Generate the complete runtime module (legacy method for backward compatibility)
    pub fn generate_runtime_module(&self) -> Result<Vec<TsNode>, GeneratorError> {
        let nodes = vec![
            // Generate in the expected order to match golden files
            // Functions first
            self.generate_to_json_function()?,
            // Interfaces
            self.generate_configuration_interface()?,
            self.generate_configuration_parameters_interface()?,
            // Classes
            self.generate_base_api_class()?,
            self.generate_required_error_class()?,
            // More functions
            self.generate_from_json_function()?,
            // More interfaces
            self.generate_request_context_interface()?,
        ];

        Ok(nodes)
    }

    /// Generate Configuration interface
    fn generate_configuration_interface(&self) -> Result<TsNode, GeneratorError> {
        let properties = vec![
            // basePath property
            Property {
                name: "basePath".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Base path for API requests".to_string()),
            },
            // username property
            Property {
                name: "username".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Username for authentication".to_string()),
            },
            // password property
            Property {
                name: "password".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Password for authentication".to_string()),
            },
            // apiKey property
            Property {
                name: "apiKey".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("API key for authentication".to_string()),
            },
            // accessToken property
            Property {
                name: "accessToken".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Access token for authentication".to_string()),
            },
            // headers property
            Property {
                name: "headers".to_string(),
                type_expr: TypeExpression::Reference("Record<string, string>".to_string()),
                optional: true,
                documentation: Some("Additional headers for requests".to_string()),
            },
        ];

        Ok(TsNode::Interface(Interface {
            name: "Configuration".to_string(),
            properties,
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: Some("Configuration for API client".to_string()),
        }))
    }

    /// Generate ConfigurationParameters interface
    fn generate_configuration_parameters_interface(&self) -> Result<TsNode, GeneratorError> {
        let properties = vec![
            // basePath property
            Property {
                name: "basePath".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Base path for API requests".to_string()),
            },
            // username property
            Property {
                name: "username".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Username for authentication".to_string()),
            },
            // password property
            Property {
                name: "password".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Password for authentication".to_string()),
            },
            // apiKey property
            Property {
                name: "apiKey".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("API key for authentication".to_string()),
            },
            // accessToken property
            Property {
                name: "accessToken".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: true,
                documentation: Some("Access token for authentication".to_string()),
            },
        ];

        Ok(TsNode::Interface(Interface {
            name: "ConfigurationParameters".to_string(),
            properties,
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: Some("Configuration parameters for API client".to_string()),
        }))
    }

    /// Generate RequestContext interface
    fn generate_request_context_interface(&self) -> Result<TsNode, GeneratorError> {
        let properties = vec![
            // url property
            Property {
                name: "url".to_string(),
                type_expr: TypeExpression::Primitive(PrimitiveType::String),
                optional: false,
                documentation: Some("Request URL".to_string()),
            },
            // init property
            Property {
                name: "init".to_string(),
                type_expr: TypeExpression::Reference("RequestInit".to_string()),
                optional: true,
                documentation: Some("Request initialization options".to_string()),
            },
        ];

        Ok(TsNode::Interface(Interface {
            name: "RequestContext".to_string(),
            properties,
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: Some("Request context for API calls".to_string()),
        }))
    }

    /// Generate RequiredError class
    fn generate_required_error_class(&self) -> Result<TsNode, GeneratorError> {
        let mut properties = Vec::new();
        let mut methods = Vec::new();

        // field property
        properties.push(Property {
            name: "field".to_string(),
            type_expr: TypeExpression::Primitive(PrimitiveType::String),
            optional: false,
            documentation: Some("The field that is required".to_string()),
        });

        // Constructor method
        methods.push(TsMethod {
            name: "constructor".to_string(),
            parameters: vec![Parameter {
                name: "field".to_string(),
                type_expr: Some(TypeExpression::Primitive(PrimitiveType::String)),
                optional: false,
                default_value: None,
            }],
            return_type: None,
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Create a new RequiredError".to_string()),
            body: Some(CodeBlock::from_statements(vec![Statement::Simple(
                "super(`Field ${field} is required`); this.field = field;".to_string(),
            )])),
        });

        Ok(TsNode::Class(Class {
            name: "RequiredError".to_string(),
            properties,
            methods,
            extends: Some("Error".to_string()),
            implements: Vec::new(),
            generics: Vec::new(),
            is_export: true,
            documentation: Some("Error thrown when a required parameter is missing".to_string()),
        }))
    }

    /// Generate BaseAPI class
    fn generate_base_api_class(&self) -> Result<TsNode, GeneratorError> {
        let mut properties = Vec::new();
        let mut methods = Vec::new();

        // configuration property
        properties.push(Property {
            name: "configuration".to_string(),
            type_expr: TypeExpression::Reference("Configuration".to_string()),
            optional: true,
            documentation: Some("API configuration".to_string()),
        });

        // Constructor method
        methods.push(TsMethod {
            name: "constructor".to_string(),
            parameters: vec![Parameter {
                name: "configuration".to_string(),
                type_expr: Some(TypeExpression::Reference("Configuration".to_string())),
                optional: true,
                default_value: None,
            }],
            return_type: None,
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Initialize the BaseAPI".to_string()),
            body: Some(CodeBlock::from_statements(vec![Statement::Simple(
                "this.configuration = configuration;".to_string(),
            )])),
        });

        // request method
        methods.push(TsMethod {
            name: "request".to_string(),
            parameters: vec![Parameter {
                name: "context".to_string(),
                type_expr: Some(TypeExpression::Reference("RequestContext".to_string())),
                optional: false,
                default_value: None,
            }],
            return_type: Some(TypeExpression::Reference("Promise<Response>".to_string())),
            is_async: true,
            is_static: false,
            visibility: Visibility::Public,
            documentation: Some("Make an HTTP request".to_string()),
            body: None,
        });

        Ok(TsNode::Class(Class {
            name: "BaseAPI".to_string(),
            properties,
            methods,
            extends: None,
            implements: Vec::new(),
            generics: Vec::new(),
            is_export: true,
            documentation: Some("Base API class for all API clients".to_string()),
        }))
    }

    /// Generate FromJSON function
    fn generate_from_json_function(&self) -> Result<TsNode, GeneratorError> {
        Ok(TsNode::Function(Function {
            name: "FromJSON".to_string(),
            parameters: vec![Parameter {
                name: "json".to_string(),
                type_expr: Some(TypeExpression::Primitive(PrimitiveType::Any)),
                optional: false,
                default_value: None,
            }],
            return_type: Some(TypeExpression::Generic("T".to_string())),
            generics: vec![crate::ast::Generic {
                name: "T".to_string(),
                constraint: None,
                default: None,
            }],
            is_async: false,
            is_export: true,
            documentation: Some("Convert JSON object to typed object".to_string()),
            body: Some(CodeBlock::from_statements(vec![Statement::Simple(
                "return json as T;".to_string(),
            )])),
        }))
    }

    /// Generate ToJSON function
    fn generate_to_json_function(&self) -> Result<TsNode, GeneratorError> {
        Ok(TsNode::Function(Function {
            name: "ToJSON".to_string(),
            parameters: vec![Parameter {
                name: "value".to_string(),
                type_expr: Some(TypeExpression::Generic("T".to_string())),
                optional: false,
                default_value: None,
            }],
            return_type: Some(TypeExpression::Primitive(PrimitiveType::Any)),
            generics: vec![crate::ast::Generic {
                name: "T".to_string(),
                constraint: None,
                default: None,
            }],
            is_async: false,
            is_export: true,
            documentation: Some("Convert typed object to JSON".to_string()),
            body: Some(CodeBlock::from_statements(vec![Statement::Simple(
                "return JSON.stringify(value);".to_string(),
            )])),
        }))
    }
}
