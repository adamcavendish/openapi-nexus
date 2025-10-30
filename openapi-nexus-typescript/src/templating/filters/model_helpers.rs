//! Template filters for generating model helper lines with controlled indentation

/// Filter: instance_guard_filter(value: &str, indent: usize) -> String
/// Builds a guard line for required properties in instanceOf function.
pub fn instance_guard_filter(prop: &str, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    format!(
        "{}if (!('{}' in value) || (value as any)['{}'] === undefined) return false;",
        indent_str, prop, prop
    )
}

/// Filter: from_json_line_filter(value: &str, indent: usize, optional: bool) -> String
/// Builds a FromJSONTyped mapping line for a property.
pub fn from_json_line_filter(prop: &str, indent: usize, optional: bool) -> String {
    let indent_str = "  ".repeat(indent);
    if optional {
        format!("{}'{}': json['{}'] ?? undefined,", indent_str, prop, prop)
    } else {
        format!("{}'{}': json['{}'],", indent_str, prop, prop)
    }
}

/// Filter: to_json_line_filter(value: &str, indent: usize) -> String
/// Builds a ToJSONTyped mapping line for a property.
pub fn to_json_line_filter(prop: &str, indent: usize) -> String {
    let indent_str = "  ".repeat(indent);
    format!("{}'{}': value['{}'],", indent_str, prop, prop)
}
