//! Template filter for formatting documentation comments

/// Template filter for formatting documentation comments
pub fn format_doc_comment_filter(value: &str) -> String {
    if value.contains('\n') {
        let lines: Vec<&str> = value.lines().collect();
        let mut result = String::from("/**\n");
        for line in lines {
            result.push_str(&format!(" * {}\n", line));
        }
        result.push_str(" */");
        result
    } else {
        format!("/** {} */", value)
    }
}
