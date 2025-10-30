//! Template filter for formatting ClassProperty as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::TsClassProperty;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting ClassProperty as TypeScript string
pub fn format_property_filter(
    property: ViaDeserialize<TsClassProperty>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent: indent_level.unwrap_or(0),
        max_line_width,
    };
    property
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Create a format_property filter with the given max_line_width
pub fn create_format_property_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsClassProperty>, Option<usize>) -> String + Send + Sync + 'static {
    move |property, indent_level| format_property_filter(property, indent_level, max_line_width)
}
