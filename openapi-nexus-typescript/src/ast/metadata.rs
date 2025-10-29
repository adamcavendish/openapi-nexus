//! TypeScript metadata and documentation
//!
//! This module consolidates comment handling, documentation, and file headers.

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// TypeScript documentation comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsDocComment {
    pub content: String,
    pub is_multiline: bool,
}

impl TsDocComment {
    /// Create a new documentation comment
    pub fn new(content: String) -> Self {
        let is_multiline = content.contains('\n');
        Self {
            content,
            is_multiline,
        }
    }

    /// Create a single-line doc comment
    pub fn single_line(content: String) -> Self {
        Self {
            content,
            is_multiline: false,
        }
    }

    /// Create a multi-line doc comment
    pub fn multi_line(content: String) -> Self {
        Self {
            content,
            is_multiline: true,
        }
    }
}

/// Helper functions for formatting documentation
pub fn format_param_doc(name: &str, description: &str) -> String {
    format!("@param {} {}", name, description)
}

pub fn format_return_doc(description: &str) -> String {
    format!("@returns {}", description)
}

pub fn format_throws_doc(error_type: &str, description: &str) -> String {
    format!("@throws {{{}}} {}", error_type, description)
}

pub fn format_example_doc(example: &str) -> String {
    format!("@example\n{}", example)
}

/// Create a complete method documentation comment
pub fn create_method_doc(
    description: &str,
    params: &[(String, String)],
    return_desc: Option<&str>,
    throws: &[(String, String)],
    example: Option<&str>,
) -> TsDocComment {
    let mut lines = vec![description.to_string()];

    if !params.is_empty() {
        lines.push(String::new()); // Empty line
        for (name, desc) in params {
            lines.push(format_param_doc(name, desc));
        }
    }

    if let Some(return_desc) = return_desc {
        lines.push(String::new()); // Empty line
        lines.push(format_return_doc(return_desc));
    }

    if !throws.is_empty() {
        lines.push(String::new()); // Empty line
        for (error_type, desc) in throws {
            lines.push(format_throws_doc(error_type, desc));
        }
    }

    if let Some(example) = example {
        lines.push(String::new()); // Empty line
        lines.push(format_example_doc(example));
    }

    TsDocComment::multi_line(lines.join("\n"))
}

/// Create a simple class or interface documentation comment
pub fn create_type_doc(description: &str, additional_info: Option<&str>) -> TsDocComment {
    if let Some(info) = additional_info {
        TsDocComment::multi_line(format!("{}\n\n{}", description, info))
    } else {
        TsDocComment::single_line(description.to_string())
    }
}

// ToRcDocWithContext implementations
impl ToRcDocWithContext for TsDocComment {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        let indent_str = " ".repeat(context.indent_level);

        let doc = if self.is_multiline {
            let lines: Vec<&str> = self.content.lines().collect();
            let mut parts = vec![RcDoc::text(format!("{}/**", indent_str))];
            for line in lines {
                parts.push(RcDoc::hardline());
                parts.push(RcDoc::text(format!("{} * {}", indent_str, line)));
            }
            parts.push(RcDoc::hardline());
            parts.push(RcDoc::text(format!("{} */", indent_str)));
            RcDoc::concat(parts)
        } else {
            RcDoc::text(format!("{}/** {} */", indent_str, self.content))
        };

        Ok(doc)
    }
}

// Note: simple line/block `Comment` and `GeneratedFileHeader` types were removed as they were unused or minimal.
