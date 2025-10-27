//! TypeScript type expression emitter

use pretty::RcDoc;

use crate::ast::{PrimitiveType, TypeExpression};
use crate::emission::error::EmitError;

/// Helper struct for emitting TypeScript type expressions
pub struct TypeExpressionEmitter;

impl TypeExpressionEmitter {
    /// Emit a TypeExpression as a pretty-printed RcDoc
    pub fn emit_type_expression_doc(
        &self,
        type_expr: &TypeExpression,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        self.emit_type_expression_doc_with_indent(type_expr, 0)
    }

    /// Emit a TypeExpression as a pretty-printed RcDoc with specified indentation level
    pub fn emit_type_expression_doc_with_indent(
        &self,
        type_expr: &TypeExpression,
        indent_level: usize,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match type_expr {
            TypeExpression::Primitive(primitive) => {
                let type_name = match primitive {
                    PrimitiveType::String => "string",
                    PrimitiveType::Number => "number",
                    PrimitiveType::Boolean => "boolean",
                    PrimitiveType::Null => "null",
                    PrimitiveType::Undefined => "undefined",
                    PrimitiveType::Any => "any",
                    PrimitiveType::Unknown => "unknown",
                    PrimitiveType::Void => "void",
                    PrimitiveType::Never => "never",
                };
                Ok(RcDoc::text(type_name.to_string()))
            }
            TypeExpression::Array(item_type) => {
                let item_doc =
                    self.emit_type_expression_doc_with_indent(item_type, indent_level + 1)?;
                Ok(RcDoc::text("Array<".to_string())
                    .append(item_doc)
                    .append(RcDoc::text(">".to_string())))
            }
            TypeExpression::Union(types) => {
                let type_docs: Result<Vec<RcDoc<'static, ()>>, _> = types
                    .iter()
                    .map(|t| self.emit_type_expression_doc_with_indent(t, indent_level + 1))
                    .collect();
                let docs = type_docs?;
                if docs.len() == 1 {
                    Ok(docs[0].clone())
                } else {
                    let separator = RcDoc::text(" | ");
                    Ok(RcDoc::intersperse(docs, separator))
                }
            }
            TypeExpression::Intersection(types) => {
                let type_docs: Result<Vec<RcDoc<'static, ()>>, _> = types
                    .iter()
                    .map(|t| self.emit_type_expression_doc_with_indent(t, indent_level))
                    .collect();
                let docs = type_docs?;
                if docs.len() == 1 {
                    Ok(docs[0].clone())
                } else {
                    let separator = RcDoc::text(" & ");
                    Ok(RcDoc::intersperse(docs, separator))
                }
            }
            TypeExpression::Reference(name) => Ok(RcDoc::text(name.clone())),
            TypeExpression::Literal(value) => Ok(RcDoc::text(value.clone())),
            TypeExpression::Object(properties) => {
                if properties.is_empty() {
                    Ok(RcDoc::text("{}"))
                } else {
                    // Check if this object should be formatted inline or multiline
                    let should_multiline = self.should_format_object_multiline(properties);
                    if should_multiline {
                        // Multi-line format with proper indentation
                        let mut result = RcDoc::text("{");
                        result = result.append(RcDoc::line());

                        let current_indent = "  ".repeat(indent_level + 1);
                        for (i, (name, type_expr)) in properties.iter().enumerate() {
                            let type_doc = self.emit_type_expression_doc_with_indent(
                                type_expr,
                                indent_level + 1,
                            )?;
                            let prop_doc = RcDoc::text(current_indent.clone())
                                .append(RcDoc::text(name.clone()))
                                .append(RcDoc::text(": "))
                                .append(type_doc)
                                .append(RcDoc::text(";"));

                            result = result.append(prop_doc);
                            if i < properties.len() - 1 {
                                result = result.append(RcDoc::line());
                            }
                        }

                        result = result.append(RcDoc::line());
                        let closing_indent = "  ".repeat(indent_level);
                        result = result.append(RcDoc::text(closing_indent));
                        result = result.append(RcDoc::text("}"));
                        Ok(result)
                    } else {
                        // Inline format for simple objects
                        let prop_docs: Result<Vec<RcDoc<'_, ()>>, _> = properties
                            .iter()
                            .map(|(name, type_expr)| {
                                let type_doc = self.emit_type_expression_doc_with_indent(
                                    type_expr,
                                    indent_level,
                                )?;
                                Ok(RcDoc::text(name.clone())
                                    .append(RcDoc::text(": "))
                                    .append(type_doc))
                            })
                            .collect();

                        let props = prop_docs?;
                        let separator = RcDoc::text("; ");
                        Ok(RcDoc::text("{ ")
                            .append(RcDoc::intersperse(props, separator))
                            .append(RcDoc::text(" }"))
                            .group())
                    }
                }
            }
            TypeExpression::Function(signature) => {
                let param_docs: Vec<RcDoc<'_, ()>> = signature
                    .parameters
                    .iter()
                    .map(|param| {
                        let mut param_doc = RcDoc::text(param.name.clone());
                        if param.optional {
                            param_doc = param_doc.append(RcDoc::text("?"));
                        }
                        if let Some(type_expr) = &param.type_expr {
                            param_doc = param_doc.append(RcDoc::text(": "));
                            match self.emit_type_expression_doc_with_indent(type_expr, indent_level)
                            {
                                Ok(type_doc) => param_doc = param_doc.append(type_doc),
                                Err(_) => param_doc = param_doc.append(RcDoc::text("any")),
                            }
                        }
                        param_doc
                    })
                    .collect();

                let params = if param_docs.is_empty() {
                    RcDoc::text("()")
                } else {
                    RcDoc::text("(")
                        .append(RcDoc::intersperse(param_docs, RcDoc::text(", ")))
                        .append(RcDoc::text(")"))
                };

                let mut func_doc = params;
                if let Some(return_type) = &signature.return_type {
                    let return_doc =
                        self.emit_type_expression_doc_with_indent(return_type, indent_level)?;
                    func_doc = func_doc.append(RcDoc::text(" => ")).append(return_doc);
                }

                Ok(func_doc)
            }
            TypeExpression::Tuple(types) => {
                let type_docs: Result<Vec<RcDoc<'static, ()>>, _> = types
                    .iter()
                    .map(|t| self.emit_type_expression_doc_with_indent(t, indent_level))
                    .collect();
                let docs = type_docs?;
                Ok(RcDoc::text("[")
                    .append(RcDoc::intersperse(docs, RcDoc::text(", ")))
                    .append(RcDoc::text("]")))
            }
            TypeExpression::Generic(name) => Ok(RcDoc::text(name.clone())),
            TypeExpression::IndexSignature(key_type, value_type) => {
                let value_doc =
                    self.emit_type_expression_doc_with_indent(value_type, indent_level)?;
                Ok(RcDoc::text("[key: ")
                    .append(RcDoc::text(key_type.clone()))
                    .append(RcDoc::text("]: "))
                    .append(value_doc))
            }
        }
    }

    /// Determine if an object should be formatted multiline based on complexity
    pub fn should_format_object_multiline(
        &self,
        properties: &std::collections::BTreeMap<String, TypeExpression>,
    ) -> bool {
        // Format multiline if:
        // 1. More than 2 properties
        // 2. Any property has a complex nested type
        if properties.len() > 2 {
            return true;
        }

        for type_expr in properties.values() {
            if self.is_complex_type(type_expr) {
                return true;
            }
        }

        false
    }

    /// Check if a type expression is complex (nested objects, arrays, unions, etc.)
    #[allow(clippy::only_used_in_recursion)]
    pub fn is_complex_type(&self, type_expr: &TypeExpression) -> bool {
        match type_expr {
            TypeExpression::Object(properties) => {
                // Only consider objects complex if they have more than 2 properties
                // or contain nested complex types
                if properties.len() > 2 {
                    return true;
                }
                for prop_type in properties.values() {
                    if self.is_complex_type(prop_type) {
                        return true;
                    }
                }
                false
            }
            TypeExpression::Array(_) => true,
            TypeExpression::Union(types) => types.len() > 1,
            TypeExpression::Intersection(types) => types.len() > 1,
            TypeExpression::Function(_) => true,
            TypeExpression::Tuple(types) => types.len() > 1,
            _ => false,
        }
    }

    /// Emit a TypeExpression as a string
    pub fn emit_type_expression_string(
        &self,
        type_expr: &TypeExpression,
    ) -> Result<String, EmitError> {
        let doc = self.emit_type_expression_doc(type_expr)?;
        Ok(doc.pretty(80).to_string())
    }
}
