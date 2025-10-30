//! Template functions to improve readability of model helper templates

/// Generate a guard line for required properties in instanceOf function
pub fn instance_guard_line(indent: usize, prop: &str) -> String {
    let indent_str = "  ".repeat(indent);
    let snippet = format!(
        "if (!('{prop}' in value) || (value as any)['{prop}'] === undefined) return false;",
    );
    indent_str + &snippet
}

/// Generate a FromJSONTyped mapping line for a property
pub fn from_json_line(indent: usize, prop: &str, optional: bool) -> String {
    let indent_str = "  ".repeat(indent);
    let snippet = if optional {
        format!("'{prop}': json['{prop}'] ?? undefined,")
    } else {
        format!("'{prop}': json['{prop}'],")
    };
    indent_str + &snippet
}

/// Generate a ToJSONTyped mapping line for a property
pub fn to_json_line(indent: usize, prop: &str) -> String {
    let indent_str = "  ".repeat(indent);
    let snippet = format!("'{prop}': value['{prop}'],");
    indent_str + &snippet
}
