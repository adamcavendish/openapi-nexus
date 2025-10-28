//! Runtime component builder for standardized AST generation patterns

use crate::ast::{
    Class, Function, Interface, Parameter, Property, Statement, TsMethod,
    TsNode, TypeExpression, Visibility,
};
use crate::ast_trait::to_rcdoc::ToRcDoc;
use crate::core::GeneratorError;
use crate::generator::template_generator::TemplateGenerator;

/// Builder for runtime components with common patterns
pub struct RuntimeComponentBuilder {
    template_generator: TemplateGenerator,
}

impl RuntimeComponentBuilder {
    /// Create a new runtime component builder
    pub fn new() -> Result<Self, GeneratorError> {
        Ok(Self {
            template_generator: TemplateGenerator::new(),
        })
    }

    /// Build a simple interface with common patterns
    pub fn build_interface(
        &self,
        name: &str,
        properties: Vec<Property>,
        documentation: Option<String>,
    ) -> TsNode {
        TsNode::Interface(Interface {
            name: name.to_string(),
            properties,
            extends: Vec::new(),
            generics: Vec::new(),
            documentation,
        })
    }

    /// Build a simple class with common patterns
    pub fn build_class(
        &self,
        name: &str,
        properties: Vec<Property>,
        methods: Vec<TsMethod>,
        extends: Option<String>,
        documentation: Option<String>,
    ) -> TsNode {
        TsNode::Class(Class {
            name: name.to_string(),
            properties,
            methods,
            extends,
            implements: Vec::new(),
            generics: Vec::new(),
            is_export: true,
            documentation,
        })
    }

    /// Build a simple function with common patterns
    pub fn build_function(
        &self,
        name: &str,
        parameters: Vec<Parameter>,
        return_type: Option<TypeExpression>,
        body: Option<String>,
        documentation: Option<String>,
    ) -> TsNode {
        let body_code = body.map(|b| {
            crate::ast::CodeBlock::from_statements(vec![Statement::Simple(b)])
        });

        TsNode::Function(Function {
            name: name.to_string(),
            parameters,
            return_type,
            generics: Vec::new(),
            is_async: false,
            is_export: true,
            documentation,
            body: body_code,
        })
    }

    /// Build a constructor method
    pub fn build_constructor(
        &self,
        parameters: Vec<Parameter>,
        body: String,
    ) -> TsMethod {
        TsMethod {
            name: "constructor".to_string(),
            parameters,
            return_type: None,
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: None,
            body: Some(crate::ast::CodeBlock::from_statements(vec![
                Statement::Simple(body),
            ])),
        }
    }

    /// Build a getter method
    pub fn build_getter(
        &self,
        property_name: &str,
        return_type: TypeExpression,
        body: String,
    ) -> TsMethod {
        TsMethod {
            name: format!("get {}", property_name),
            parameters: vec![],
            return_type: Some(return_type),
            is_async: false,
            is_static: false,
            visibility: Visibility::Public,
            documentation: None,
            body: Some(crate::ast::CodeBlock::from_statements(vec![
                Statement::Simple(body),
            ])),
        }
    }

    /// Build a property with common patterns
    pub fn build_property(
        &self,
        name: &str,
        type_expr: TypeExpression,
        optional: bool,
        documentation: Option<String>,
    ) -> Property {
        Property {
            name: name.to_string(),
            type_expr,
            optional,
            documentation,
        }
    }

    /// Build a parameter with common patterns
    pub fn build_parameter(
        &self,
        name: &str,
        type_expr: Option<TypeExpression>,
        optional: bool,
        default_value: Option<String>,
    ) -> Parameter {
        Parameter {
            name: name.to_string(),
            type_expr,
            optional,
            default_value,
        }
    }

    /// Generate multiple nodes as a single string with optimized formatting
    pub fn generate_nodes_batch(
        &self,
        nodes: &[TsNode],
        imports: Option<&str>,
        constants: Option<&str>,
    ) -> Result<String, GeneratorError> {
        let mut docs = Vec::new();

        // Add generated file header
        let header = crate::ast::GeneratedFileHeader::new();
        docs.push(header.to_rcdoc().map_err(|e| GeneratorError::Generic {
            message: format!("Header emission error: {}", e),
        })?);

        // Add imports if provided
        if let Some(imports) = imports {
            docs.push(pretty::RcDoc::text(imports.to_string()));
        }

        // Add constants if provided
        if let Some(constants) = constants {
            docs.push(pretty::RcDoc::text(constants.to_string()));
        }

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

    /// Get template generator for hybrid approach
    pub fn template_generator(&self) -> &TemplateGenerator {
        &self.template_generator
    }
}

impl Default for RuntimeComponentBuilder {
    fn default() -> Self {
        Self::new().expect("Failed to create RuntimeComponentBuilder")
    }
}
