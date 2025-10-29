//! Template filter for formatting documentation comments

use crate::ast::metadata::DocComment;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting documentation comments
pub fn format_doc_comment_filter(
    value: &str,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let doc_comment = DocComment::new(value.to_string());
    let ctx = EmissionContext {
        indent_level: indent_level.unwrap_or(0),
        max_line_width,
    };

    doc_comment
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| format!("/** {} */", value))
}

/// Create a format_doc_comment filter with the given max_line_width
pub fn create_format_doc_comment_filter(
    max_line_width: usize,
) -> impl Fn(&str, Option<usize>) -> String + Send + Sync + 'static {
    move |value, indent_level| format_doc_comment_filter(value, indent_level, max_line_width)
}
