//! Template filter for formatting ClassProperty as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::class_definition::ClassProperty;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting ClassProperty as TypeScript string
pub fn format_property_filter(
    property: ViaDeserialize<ClassProperty>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent_level: 0,
        max_line_width,
    };
    property
        .to_rcdoc_with_context(&ctx)
        .map(|doc| format!("{}", doc.pretty(max_line_width)))
        .unwrap_or_else(|_| "unknown".to_string())
}

/// Create a format_property filter with the given max_line_width
pub fn create_format_property_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<ClassProperty>) -> String + Send + Sync + 'static {
    move |property| format_property_filter(property, max_line_width)
}

