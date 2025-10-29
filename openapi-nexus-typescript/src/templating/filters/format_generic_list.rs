//! Template filter for formatting generic lists

use minijinja::value::ViaDeserialize;

use crate::ast::common::Generic;
use crate::ast_trait::{EmissionContext, ToRcDocWithContext};

/// Template filter for formatting generic lists
pub fn format_generic_list_filter(generics: ViaDeserialize<Vec<Generic>>) -> String {
    if generics.is_empty() {
        String::new()
    } else {
        let ctx = EmissionContext {
            indent_level: 0,
            max_line_width: 80,
        };
        let generic_strings: Vec<String> = generics
            .iter()
            .filter_map(|g| {
                g.to_rcdoc_with_context(&ctx)
                    .ok()
                    .map(|doc| format!("{}", doc.pretty(80)))
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