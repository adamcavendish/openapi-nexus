//! Template filter for indenting text

/// Template filter for indenting text
pub fn indent_filter(value: &str, spaces: usize) -> String {
    let indent_str = " ".repeat(spaces);
    value
        .lines()
        .map(|line| {
            if line.trim().is_empty() {
                line.to_string()
            } else {
                format!("{}{}", indent_str, line)
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
