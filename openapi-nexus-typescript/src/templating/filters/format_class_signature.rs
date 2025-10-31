//! Template filter for formatting TsClassSignature as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::TsClassSignature;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting TsClassSignature as a single-line string
pub fn format_class_signature_filter(
    signature: ViaDeserialize<TsClassSignature>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent: indent_level.unwrap_or(0),
        max_line_width,
    };
    signature
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "class".to_string())
}

/// Create a format_class_signature filter with the given max_line_width
pub fn create_format_class_signature_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsClassSignature>, Option<usize>) -> String + Send + Sync + 'static {
    move |signature, indent_level| {
        format_class_signature_filter(signature, indent_level, max_line_width)
    }
}
