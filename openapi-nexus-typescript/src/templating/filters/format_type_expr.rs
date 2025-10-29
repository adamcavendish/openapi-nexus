//! Template filter for formatting TypeExpression as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::TypeExpression;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting TypeExpression as TypeScript string
pub fn format_type_expr_filter(
    type_expr: ViaDeserialize<TypeExpression>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent_level: 0,
        max_line_width,
    };
    type_expr
        .to_rcdoc_with_context(&ctx)
        .map(|doc| format!("{}", doc.pretty(max_line_width)))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Create a format_type_expr filter with the given max_line_width
pub fn create_format_type_expr_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TypeExpression>) -> String + Send + Sync + 'static {
    move |type_expr| format_type_expr_filter(type_expr, max_line_width)
}

