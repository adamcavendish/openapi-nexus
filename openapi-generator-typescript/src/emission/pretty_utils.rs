//! Shared pretty printing utilities for TypeScript code generation
//!
//! This module provides common combinators and utilities for generating
//! well-formatted TypeScript code using pretty.rs RcDoc combinators.

use pretty::RcDoc;

use crate::ast::{Generic, Parameter, TypeExpression};
use crate::emission::error::EmitError;
use crate::emission::type_expression_emitter::TypeExpressionEmitter;

/// TypeScript-specific pretty printing utilities and combinators
pub struct TypeScriptPrettyUtils {
    type_emitter: TypeExpressionEmitter,
}

impl TypeScriptPrettyUtils {
    pub fn new() -> Self {
        Self {
            type_emitter: TypeExpressionEmitter,
        }
    }

    /// Create a documentation comment block
    pub fn doc_comment(&self, text: &str) -> RcDoc<'static, ()> {
        RcDoc::text("/**")
            .append(RcDoc::line())
            .append(RcDoc::text(" * "))
            .append(RcDoc::text(text.to_string()))
            .append(RcDoc::line())
            .append(RcDoc::text(" */"))
    }

    /// Create a multi-line documentation comment block
    pub fn doc_comment_multiline(&self, lines: &[&str]) -> RcDoc<'static, ()> {
        let mut doc = RcDoc::text("/**");
        for line in lines {
            doc = doc
                .append(RcDoc::line())
                .append(RcDoc::text(" * "))
                .append(RcDoc::text(line.to_string()));
        }
        doc.append(RcDoc::line()).append(RcDoc::text(" */"))
    }

    /// Create indented content with 2-space indentation (TypeScript standard)
    pub fn indent(&self, doc: RcDoc<'static, ()>) -> RcDoc<'static, ()> {
        doc.nest(2)
    }

    /// Indent content by manually adding spaces to each line
    pub fn indent_lines(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| {
                if line.trim().is_empty() {
                    line.to_string()
                } else {
                    format!("  {}", line)
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    /// Add comma to the last line of content (for properties)
    pub fn add_comma_to_last_line(&self, content: &str) -> String {
        let lines: Vec<&str> = content.lines().collect();
        if lines.is_empty() {
            return content.to_string();
        }

        let mut result = lines[..lines.len() - 1].join("\n");
        if !result.is_empty() {
            result.push('\n');
        }
        result.push_str(&format!("{},", lines[lines.len() - 1]));
        result
    }

    /// Create a comma-separated list with proper spacing
    pub fn comma_separated(&self, docs: Vec<RcDoc<'static, ()>>) -> RcDoc<'static, ()> {
        RcDoc::intersperse(docs, RcDoc::text(", "))
    }

    /// Create a comma-separated list that can break across lines
    pub fn comma_separated_breakable(&self, docs: Vec<RcDoc<'static, ()>>) -> RcDoc<'static, ()> {
        if docs.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::intersperse(docs, RcDoc::text(",").append(RcDoc::line_()))
        }
    }

    /// Create generics list (e.g., `<T, U>`)
    pub fn generics(&self, generics: &[Generic]) -> Result<RcDoc<'static, ()>, EmitError> {
        if generics.is_empty() {
            Ok(RcDoc::nil())
        } else {
            let generic_docs: Vec<RcDoc<'static, ()>> = generics
                .iter()
                .map(|g| RcDoc::text(g.name.clone()))
                .collect();
            Ok(RcDoc::text("<")
                .append(self.comma_separated(generic_docs))
                .append(RcDoc::text(">")))
        }
    }

    /// Create extends clause (e.g., ` extends BaseClass`)
    pub fn extends_clause(&self, extends: &[String]) -> RcDoc<'static, ()> {
        if extends.is_empty() {
            RcDoc::nil()
        } else {
            let extend_docs: Vec<RcDoc<'static, ()>> =
                extends.iter().map(|e| RcDoc::text(e.clone())).collect();
            RcDoc::space()
                .append(RcDoc::text("extends"))
                .append(RcDoc::space())
                .append(self.comma_separated(extend_docs))
        }
    }

    /// Create implements clause (e.g., ` implements Interface1, Interface2`)
    pub fn implements_clause(&self, implements: &[String]) -> RcDoc<'static, ()> {
        if implements.is_empty() {
            RcDoc::nil()
        } else {
            let impl_docs: Vec<RcDoc<'static, ()>> =
                implements.iter().map(|i| RcDoc::text(i.clone())).collect();
            RcDoc::space()
                .append(RcDoc::text("implements"))
                .append(RcDoc::space())
                .append(self.comma_separated(impl_docs))
        }
    }

    /// Create parameter list for functions/methods
    pub fn parameter_list(
        &self,
        parameters: &[Parameter],
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        if parameters.is_empty() {
            Ok(RcDoc::text("()"))
        } else {
            let param_docs: Result<Vec<RcDoc<'static, ()>>, EmitError> = parameters
                .iter()
                .map(|param| self.parameter(param))
                .collect();
            let params = param_docs?;

            // For long parameter lists, format across multiple lines
            if parameters.len() > 3 {
                Ok(RcDoc::text("(")
                    .append(RcDoc::line_())
                    .append(self.indent(self.comma_separated_breakable(params)))
                    .append(RcDoc::line_())
                    .append(RcDoc::text(")")))
            } else {
                Ok(RcDoc::text("(")
                    .append(self.comma_separated(params))
                    .append(RcDoc::text(")")))
            }
        }
    }

    /// Create a single parameter
    pub fn parameter(&self, param: &Parameter) -> Result<RcDoc<'static, ()>, EmitError> {
        let mut doc = RcDoc::text(param.name.clone());

        if param.optional {
            doc = doc.append(RcDoc::text("?"));
        }

        if let Some(type_expr) = &param.type_expr {
            let type_doc = self.type_emitter.emit_type_expression_doc(type_expr)?;
            doc = doc.append(RcDoc::text(": ")).append(type_doc);
        }

        Ok(doc)
    }

    /// Create return type annotation (e.g., `: Promise<User>`)
    pub fn return_type(
        &self,
        return_type: &Option<TypeExpression>,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        match return_type {
            Some(type_expr) => {
                let type_doc = self.type_emitter.emit_type_expression_doc(type_expr)?;
                Ok(RcDoc::text(": ").append(type_doc))
            }
            None => Ok(RcDoc::nil()),
        }
    }

    /// Create a block with proper braces and indentation
    pub fn block(&self, content: RcDoc<'static, ()>) -> RcDoc<'static, ()> {
        RcDoc::text("{")
            .append(RcDoc::line())
            .append(content)
            .append(RcDoc::line())
            .append(RcDoc::text("}"))
    }

    /// Create an empty block
    pub fn empty_block(&self) -> RcDoc<'static, ()> {
        RcDoc::text("{}")
    }

    /// Add semicolon termination
    pub fn with_semicolon(&self, doc: RcDoc<'static, ()>) -> RcDoc<'static, ()> {
        doc.append(RcDoc::text(";"))
    }

    /// Create export prefix
    pub fn export_prefix(&self) -> RcDoc<'static, ()> {
        RcDoc::text("export ")
    }

    /// Create quoted string (always use double quotes for consistency)
    pub fn quoted(&self, text: &str) -> RcDoc<'static, ()> {
        RcDoc::text(format!("\"{}\"", text))
    }

    /// Create single quoted string (for imports)
    pub fn single_quoted(&self, text: &str) -> RcDoc<'static, ()> {
        RcDoc::text(format!("'{}'", text))
    }

    /// Determine if content should be formatted inline or multiline
    pub fn should_format_multiline(&self, item_count: usize, has_complex_items: bool) -> bool {
        item_count > 2 || has_complex_items
    }

    /// Create a list that can be either inline or multiline based on complexity
    pub fn adaptive_list(
        &self,
        items: Vec<RcDoc<'static, ()>>,
        open: &str,
        close: &str,
        force_multiline: bool,
    ) -> RcDoc<'static, ()> {
        if items.is_empty() {
            RcDoc::text(open.to_string()).append(RcDoc::text(close.to_string()))
        } else if force_multiline {
            RcDoc::text(open.to_string())
                .append(RcDoc::line())
                .append(self.indent(self.comma_separated_breakable(items)))
                .append(RcDoc::line())
                .append(RcDoc::text(close.to_string()))
        } else {
            RcDoc::text(open.to_string())
                .append(self.comma_separated(items))
                .append(RcDoc::text(close.to_string()))
        }
    }

    /// Create a code block with multiple statements
    pub fn code_block(&self, statements: Vec<RcDoc<'static, ()>>) -> RcDoc<'static, ()> {
        if statements.is_empty() {
            RcDoc::nil()
        } else {
            RcDoc::intersperse(statements, RcDoc::line())
        }
    }

    /// Create a simple statement (adds semicolon if not present)
    pub fn statement(&self, text: &str) -> RcDoc<'static, ()> {
        let text = text.trim_end_matches(';');
        RcDoc::text(format!("{};", text))
    }

    /// Create a comment
    pub fn comment(&self, text: &str) -> RcDoc<'static, ()> {
        RcDoc::text(format!("// {}", text))
    }

    /// Create an if statement with a single statement body
    pub fn if_statement(&self, condition: RcDoc<'static, ()>, body: RcDoc<'static, ()>) -> RcDoc<'static, ()> {
        RcDoc::text("if (")
            .append(condition)
            .append(RcDoc::text(") {"))
            .append(RcDoc::line())
            .append(self.indent(body))
            .append(RcDoc::line())
            .append(RcDoc::text("}"))
    }

    /// Create an if statement with multiple statements in body
    pub fn if_statement_block(&self, condition: RcDoc<'static, ()>, body: Vec<RcDoc<'static, ()>>) -> RcDoc<'static, ()> {
        RcDoc::text("if (")
            .append(condition)
            .append(RcDoc::text(") {"))
            .append(RcDoc::line())
            .append(self.indent(self.code_block(body)))
            .append(RcDoc::line())
            .append(RcDoc::text("}"))
    }

    /// Create an object literal
    pub fn object_literal(&self, properties: Vec<(String, Option<RcDoc<'static, ()>>)>) -> RcDoc<'static, ()> {
        if properties.is_empty() {
            RcDoc::text("{}")
        } else {
            let prop_docs: Vec<RcDoc<'static, ()>> = properties
                .into_iter()
                .map(|(key, value)| {
                    match value {
                        Some(val) => RcDoc::text(key).append(RcDoc::text(": ")).append(val),
                        None => RcDoc::text(key),
                    }
                })
                .collect();

            if prop_docs.len() > 2 {
                RcDoc::text("{")
                    .append(RcDoc::line())
                    .append(self.indent(RcDoc::intersperse(prop_docs, RcDoc::text(",").append(RcDoc::line()))))
                    .append(RcDoc::line())
                    .append(RcDoc::text("}"))
            } else {
                RcDoc::text("{ ")
                    .append(RcDoc::intersperse(prop_docs, RcDoc::text(", ")))
                    .append(RcDoc::text(" }"))
            }
        }
    }

    /// Create an object assignment (const name = { ... })
    pub fn object_assignment(&self, name: &str, properties: Vec<(String, Option<RcDoc<'static, ()>>)>) -> RcDoc<'static, ()> {
        RcDoc::text(format!("{} = ", name))
            .append(self.object_literal(properties))
            .append(RcDoc::text(";"))
    }

    /// Create a function call
    pub fn function_call(&self, name: &str, args: Vec<RcDoc<'static, ()>>) -> RcDoc<'static, ()> {
        RcDoc::text(name.to_string())
            .append(RcDoc::text("("))
            .append(self.comma_separated(args))
            .append(RcDoc::text(")"))
    }

    /// Create a return statement
    pub fn return_statement(&self, expr: RcDoc<'static, ()>) -> RcDoc<'static, ()> {
        RcDoc::text("return ")
            .append(expr)
            .append(RcDoc::text(";"))
    }
}

impl Default for TypeScriptPrettyUtils {
    fn default() -> Self {
        Self::new()
    }
}
