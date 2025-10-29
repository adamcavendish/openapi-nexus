//! TypeScript type definitions
//!
//! This module consolidates Interface, TypeAlias, and Enum definitions into a unified
//! TypeDefinition enum with RcDoc emission for complex type formatting.

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::ast::{EnumVariant, Generic, Property, TypeExpression};
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};
use crate::emission::error::EmitError;
use crate::emission::ts_type_emitter::TsTypeEmitter;

/// Unified TypeScript type definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeDefinition {
    Interface(InterfaceDefinition),
    TypeAlias(TypeAliasDefinition),
    Enum(EnumDefinition),
}

/// TypeScript interface definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceDefinition {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

/// TypeScript type alias definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeAliasDefinition {
    pub name: String,
    pub type_expr: TypeExpression,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}

/// TypeScript enum definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumDefinition {
    pub name: String,
    pub variants: Vec<EnumVariant>,
    pub is_const: bool,
    pub documentation: Option<String>,
}

impl InterfaceDefinition {
    /// Create a new interface
    pub fn new(name: String) -> Self {
        Self {
            name,
            properties: Vec::new(),
            extends: Vec::new(),
            generics: Vec::new(),
            documentation: None,
        }
    }

    /// Add a property
    pub fn with_property(mut self, property: Property) -> Self {
        self.properties.push(property);
        self
    }

    /// Add multiple properties
    pub fn with_properties(mut self, properties: Vec<Property>) -> Self {
        self.properties.extend(properties);
        self
    }

    /// Add extends clause
    pub fn with_extends(mut self, extends: Vec<String>) -> Self {
        self.extends = extends;
        self
    }

    /// Add generics
    pub fn with_generics(mut self, generics: Vec<Generic>) -> Self {
        self.generics = generics;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl TypeAliasDefinition {
    /// Create a new type alias
    pub fn new(name: String, type_expr: TypeExpression) -> Self {
        Self {
            name,
            type_expr,
            generics: Vec::new(),
            documentation: None,
        }
    }

    /// Add generics
    pub fn with_generics(mut self, generics: Vec<Generic>) -> Self {
        self.generics = generics;
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl EnumDefinition {
    /// Create a new enum
    pub fn new(name: String) -> Self {
        Self {
            name,
            variants: Vec::new(),
            is_const: false,
            documentation: None,
        }
    }

    /// Create a const enum
    pub fn new_const(name: String) -> Self {
        Self {
            name,
            variants: Vec::new(),
            is_const: true,
            documentation: None,
        }
    }

    /// Add a variant
    pub fn with_variant(mut self, variant: EnumVariant) -> Self {
        self.variants.push(variant);
        self
    }

    /// Add multiple variants
    pub fn with_variants(mut self, variants: Vec<EnumVariant>) -> Self {
        self.variants.extend(variants);
        self
    }

    /// Add documentation
    pub fn with_docs(mut self, documentation: String) -> Self {
        self.documentation = Some(documentation);
        self
    }
}

impl ToRcDocWithContext for TypeDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match self {
            TypeDefinition::Interface(interface) => interface.to_rcdoc_with_context(context),
            TypeDefinition::TypeAlias(type_alias) => type_alias.to_rcdoc_with_context(context),
            TypeDefinition::Enum(enum_def) => enum_def.to_rcdoc_with_context(context),
        }
    }
}

impl ToRcDocWithContext for InterfaceDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        if !self.generics.is_empty() {
            let generic_docs: Result<Vec<_>, _> = self
                .generics
                .iter()
                .map(|g| g.to_rcdoc_with_context(context))
                .collect();
            let generic_strings: Vec<String> = generic_docs?
                .iter()
                .map(|doc| format!("{}", doc.pretty(80)))
                .collect();
            doc = doc.append(RcDoc::text(format!("<{}>", generic_strings.join(", "))));
        }

        // Add extends clause
        if !self.extends.is_empty() {
            doc = doc
                .append(RcDoc::space())
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(RcDoc::text(self.extends.join(", ")));
        }

        // Add body with properties
        if self.properties.is_empty() {
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let prop_docs: Result<Vec<_>, _> = self
                .properties
                .iter()
                .map(|p| {
                    let type_emitter = TsTypeEmitter;
                    let mut property_line = RcDoc::text(p.name.clone());

                    if p.optional {
                        property_line = property_line.append(RcDoc::text("?"));
                    }

                    let type_doc = type_emitter.emit_type_expression_doc(&p.type_expr)?;
                    property_line = property_line.append(RcDoc::text(": ")).append(type_doc);

                    Ok(property_line)
                })
                .collect();
            let properties = prop_docs?;

            let force_multiline = self.properties.len() > 2
                || self
                    .properties
                    .iter()
                    .any(|p| TsTypeEmitter::is_complex_type(&p.type_expr));

            let body_content = if force_multiline {
                RcDoc::intersperse(properties, RcDoc::text(",").append(RcDoc::line()))
            } else {
                RcDoc::intersperse(properties, RcDoc::text(", "))
            };

            doc = doc.append(RcDoc::space()).append(
                RcDoc::text("{")
                    .append(RcDoc::line())
                    .append(body_content)
                    .append(RcDoc::line())
                    .append(RcDoc::text("}")),
            );
        }

        // Add documentation if present
        if let Some(docs) = &self.documentation {
            let doc_comment = format_doc_comment(docs);
            doc = RcDoc::text(doc_comment).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }
}

impl ToRcDocWithContext for TypeAliasDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let type_emitter = TsTypeEmitter;

        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text("type"))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add generics
        if !self.generics.is_empty() {
            let generic_docs: Result<Vec<_>, _> = self
                .generics
                .iter()
                .map(|g| g.to_rcdoc_with_context(context))
                .collect();
            let generic_strings: Vec<String> = generic_docs?
                .iter()
                .map(|doc| format!("{}", doc.pretty(80)))
                .collect();
            doc = doc.append(RcDoc::text(format!("<{}>", generic_strings.join(", "))));
        }

        // Add type expression
        let type_doc = type_emitter.emit_type_expression_doc(&self.type_expr)?;
        doc = doc.append(RcDoc::text(" = ")).append(type_doc);

        // Add documentation if present
        if let Some(docs) = &self.documentation {
            let doc_comment = format_doc_comment(docs);
            doc = RcDoc::text(doc_comment).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }
}

impl ToRcDocWithContext for EnumDefinition {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text("export ")
            .append(RcDoc::text(if self.is_const {
                "const enum"
            } else {
                "enum"
            }))
            .append(RcDoc::space())
            .append(RcDoc::text(self.name.clone()));

        // Add enum body
        if self.variants.is_empty() {
            doc = doc.append(RcDoc::space()).append(RcDoc::text("{}"));
        } else {
            let variant_docs: Vec<RcDoc<'static, ()>> = self
                .variants
                .iter()
                .map(|variant| {
                    let mut variant_doc = RcDoc::text(variant.name.clone());
                    if let Some(value) = &variant.value {
                        variant_doc = variant_doc
                            .append(RcDoc::text(" = "))
                            .append(RcDoc::text(value.clone()));
                    }
                    variant_doc
                })
                .collect();

            let force_multiline = self.variants.len() > 2;

            let body_content = if force_multiline {
                RcDoc::intersperse(variant_docs, RcDoc::text(",").append(RcDoc::line()))
            } else {
                RcDoc::intersperse(variant_docs, RcDoc::text(", "))
            };

            doc = doc.append(RcDoc::space()).append(
                RcDoc::text("{")
                    .append(RcDoc::line())
                    .append(body_content)
                    .append(RcDoc::line())
                    .append(RcDoc::text("}")),
            );
        }

        // Add documentation if present
        if let Some(docs) = &self.documentation {
            let doc_comment = format_doc_comment(docs);
            doc = RcDoc::text(doc_comment).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }
}

/// Helper function to format documentation comments
fn format_doc_comment(docs: &str) -> String {
    if docs.contains('\n') {
        // Multi-line comment
        let lines: Vec<&str> = docs.lines().collect();
        let mut result = String::from("/**\n");
        for line in lines {
            result.push_str(&format!(" * {}\n", line));
        }
        result.push_str(" */");
        result
    } else {
        // Single line comment
        format!("/** {} */", docs)
    }
}
