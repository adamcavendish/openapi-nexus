//! Template filter for formatting generic lists

use minijinja::value::ViaDeserialize;

use crate::ast::common::Generic;

/// Template filter for formatting generic lists
pub fn format_generic_list_filter(generics: ViaDeserialize<Vec<Generic>>) -> String {
    if generics.is_empty() {
        String::new()
    } else {
        let generic_strings: Vec<String> =
            generics.iter().map(|g| g.to_typescript_string()).collect();
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
