//! Template filter for formatting generic lists

use minijinja::value::ViaDeserialize;

use crate::ast::TsGeneric;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting generic lists
pub fn format_generic_list_filter(
    generics: ViaDeserialize<Vec<TsGeneric>>,
    indent_level: Option<usize>,
    max_line_width: usize,
) -> String {
    if generics.is_empty() {
        String::new()
    } else {
        let ctx = EmissionContext {
            indent: indent_level.unwrap_or(0),
            max_line_width,
        };
        let generic_strings: Vec<String> = generics
            .iter()
            .filter_map(|g| {
                g.to_rcdoc_with_context(&ctx)
                    .ok()
                    .map(|doc| format!("{}", doc.pretty(max_line_width)))
            })
            .collect();

        let extends_count = generics.iter().filter(|g| g.constraint.is_some()).count();

        if generics.len() > 4 || extends_count >= 2 {
            // Multi-line format for long generic lists or when there are multiple extends
            format!("<\n  {}\n>", generic_strings.join(",\n  "))
        } else {
            // Single line format
            format!("<{}>", generic_strings.join(", "))
        }
    }
}

/// Create a format_generic_list filter with the given max_line_width
pub fn create_format_generic_list_filter(
    max_line_width: usize,
) -> impl Fn(ViaDeserialize<Vec<TsGeneric>>, Option<usize>) -> String + Send + Sync + 'static {
    move |generics, indent_level| format_generic_list_filter(generics, indent_level, max_line_width)
}
