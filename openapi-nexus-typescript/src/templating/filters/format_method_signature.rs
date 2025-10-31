//! Template filter for formatting a TypeScript method signature from TsClassMethod

use minijinja::value::ViaDeserialize;

use crate::ast::TsClassMethod;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting method signature (no body)
pub fn format_method_signature_filter(
    method: ViaDeserialize<TsClassMethod>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent: indent_level.unwrap_or(0),
        max_line_width,
    };
    method
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "/* invalid method */".to_string())
}

/// Create a format_method_signature filter with the given max_line_width
pub fn create_format_method_signature_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsClassMethod>, Option<usize>) -> String + Send + Sync + 'static {
    move |method, indent_level| format_method_signature_filter(method, indent_level, max_line_width)
}

/// Template filter for formatting interface method signature (no async keyword)
pub fn format_method_signature_iface_filter(
    method: ViaDeserialize<TsClassMethod>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent: indent_level.unwrap_or(0),
        max_line_width,
    };
    let mut m = method.0.clone();
    m.is_async = false;
    m.to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "/* invalid method */".to_string())
}

/// Create a format_method_signature_iface filter with the given max_line_width
pub fn create_format_method_signature_iface_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsClassMethod>, Option<usize>) -> String + Send + Sync + 'static {
    move |method, indent_level| {
        format_method_signature_iface_filter(method, indent_level, max_line_width)
    }
}
