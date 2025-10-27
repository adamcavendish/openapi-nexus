//! TypeScript interface and property emitter

use pretty::RcDoc;

use crate::ast::{Interface, Property};
use crate::emission::error::EmitError;
use crate::emission::pretty_utils::TypeScriptPrettyUtils;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// Helper struct for emitting TypeScript interfaces and properties
pub struct InterfaceEmitter {
    type_emitter: TypeExpressionEmitter,
    utils: TypeScriptPrettyUtils,
}

impl InterfaceEmitter {
    pub fn new() -> Self {
        Self {
            type_emitter: TypeExpressionEmitter,
            utils: TypeScriptPrettyUtils::new(),
        }
    }

    /// Emit a TypeScript interface as RcDoc
    pub fn emit_interface_doc(&self, interface: &Interface) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = self.utils.export_prefix()
            .append(RcDoc::text("interface"))
            .append(RcDoc::space())
            .append(RcDoc::text(interface.name.clone()));

        // Add generics
        doc = doc.append(self.utils.generics(&interface.generics)?);

        // Add extends clause
        doc = doc.append(self.utils.extends_clause(&interface.extends));

        // Add body with properties
        if interface.properties.is_empty() {
            doc = doc.append(RcDoc::space()).append(self.utils.empty_block());
        } else {
            let prop_docs: Result<Vec<_>, _> = interface.properties
                .iter()
                .map(|p| self.emit_property_doc(p))
                .collect();
            let properties = prop_docs?;
            
            let force_multiline = self.utils.should_format_multiline(
                interface.properties.len(),
                interface.properties.iter().any(|p| self.is_complex_property(p))
            );
            
            let body_content = if force_multiline {
                // Convert each property to string and add proper indentation and commas
                let mut body_parts = Vec::new();
                for (_i, prop) in properties.into_iter().enumerate() {
                    let prop_string = prop.pretty(80).to_string();
                    let indented_prop = self.utils.indent_lines(&prop_string);
                    
                    // Add comma to ALL properties (including the last one for trailing comma)
                    body_parts.push(self.utils.add_comma_to_last_line(&indented_prop));
                }
                RcDoc::text(body_parts.join("\n"))
            } else {
                // For single line, add indentation and trailing comma to each property
                let property_strings: Result<Vec<_>, _> = properties
                    .into_iter()
                    .map(|prop| {
                        let prop_string = prop.pretty(80).to_string();
                        let indented_prop = self.utils.indent_lines(&prop_string);
                        Ok(RcDoc::text(self.utils.add_comma_to_last_line(&indented_prop)))
                    })
                    .collect();
                let indented_properties = property_strings?;
                RcDoc::intersperse(indented_properties, RcDoc::text(" "))
            };
            
            doc = doc.append(RcDoc::space()).append(self.utils.block(body_content));
        }

        // Add documentation if present
        if let Some(docs) = &interface.documentation {
            doc = self.utils.doc_comment(docs).append(RcDoc::line()).append(doc);
        }

        Ok(doc)
    }

    /// Emit a TypeScript interface as string
    pub fn emit_interface_string(&self, interface: &Interface) -> Result<String, EmitError> {
        let doc = self.emit_interface_doc(interface)?;
        Ok(doc.pretty(80).to_string())
    }

    /// Emit a property as RcDoc
    pub fn emit_property_doc(&self, property: &Property) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut property_line = RcDoc::text(property.name.clone());

        if property.optional {
            property_line = property_line.append(RcDoc::text("?"));
        }

        let type_doc = self.type_emitter.emit_type_expression_doc(&property.type_expr)?;
        property_line = property_line.append(RcDoc::text(": ")).append(type_doc);

        // Add documentation if present
        if let Some(docs) = &property.documentation {
            let doc_comment = self.utils.doc_comment(docs);
            // Return documentation + property line, both will be indented by the caller
            Ok(doc_comment.append(RcDoc::line()).append(property_line))
        } else {
            Ok(property_line)
        }
    }

    /// Emit a property as a string
    pub fn emit_property_string(&self, property: &Property) -> Result<String, EmitError> {
        let doc = self.emit_property_doc(property)?;
        Ok(doc.pretty(80).to_string())
    }

    /// Check if a property is complex (for formatting decisions)
    fn is_complex_property(&self, property: &Property) -> bool {
        self.type_emitter.is_complex_type(&property.type_expr)
    }
}
