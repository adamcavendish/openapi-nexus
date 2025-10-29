//! Template filter for formatting ClassProperty as TypeScript string

use minijinja::value::ViaDeserialize;

use crate::ast::class_definition::ClassProperty;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting ClassProperty as TypeScript string
pub fn format_property_filter(property: ViaDeserialize<ClassProperty>) -> String {
    let ctx = EmissionContext {
        indent_level: 0,
        max_line_width: 80,
    };
    property
        .to_rcdoc_with_context(&ctx)
        .map(|doc| format!("{}", doc.pretty(80)))
        .unwrap_or_else(|_| "unknown".to_string())
}
