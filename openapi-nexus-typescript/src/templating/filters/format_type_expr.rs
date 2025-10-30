//! Template filter for formatting TypeExpression as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::TsExpression;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting TypeExpression as TypeScript string
pub fn format_type_expr_filter(
    type_expr: ViaDeserialize<TsExpression>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent_level: indent_level.unwrap_or(0),
        max_line_width,
    };
    type_expr
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Create a format_type_expr filter with the given max_line_width
pub fn create_format_type_expr_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsExpression>, Option<usize>) -> String + Send + Sync + 'static {
    move |type_expr, indent_level| format_type_expr_filter(type_expr, indent_level, max_line_width)
}
