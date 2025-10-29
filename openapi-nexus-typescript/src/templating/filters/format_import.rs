//! Template filter for formatting import statements

use minijinja::value::ViaDeserialize;

use crate::ast::TsImportStatement;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting import statements
pub fn format_import_filter(
    import: ViaDeserialize<TsImportStatement>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    let ctx = EmissionContext {
        indent_level: indent_level.unwrap_or(0),
        max_line_width,
    };
    import
        .to_rcdoc_with_context(&ctx)
        .map(|doc| doc.pretty(max_line_width).to_string())
        .unwrap_or_else(|_| "import '???';".to_string())
}

/// Create a format_import filter with the given max_line_width
pub fn create_format_import_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<TsImportStatement>, Option<usize>) -> String + Send + Sync + 'static {
    move |import, indent_level| format_import_filter(import, indent_level, max_line_width)
}
