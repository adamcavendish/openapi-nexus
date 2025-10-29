//! Template filter for formatting import statements

use minijinja::value::ViaDeserialize;

use crate::ast::class_definition::ImportStatement;

/// Template filter for formatting import statements
pub fn format_import_filter(import: ViaDeserialize<ImportStatement>) -> String {
    if import.imports.is_empty() {
        return format!("import '{}';", import.module_path);
    }

    // Check if this should be a default import
    // Default import: single specifier with no alias
    if import.imports.len() == 1 {
        let spec = &import.imports[0];
        if spec.alias.is_none() {
            // This could be a default import
            return format!("import {} from '{}';", spec.name, import.module_path);
        }
    }

    // Named imports
    let mut import_parts = Vec::new();

    // Type-only imports
    if import.is_type_only {
        import_parts.push("type".to_string());
    }

    // Import specifiers
    let specifiers: Vec<String> = import.imports
        .iter()
        .map(|spec| {
            let mut s = String::new();
            if spec.is_type && !import.is_type_only {
                s.push_str("type ");
            }
            s.push_str(&spec.name);
            if let Some(alias) = &spec.alias {
                s.push_str(" as ");
                s.push_str(alias);
            }
            s
        })
        .collect();

    if !specifiers.is_empty() {
        import_parts.push(format!("{{ {} }}", specifiers.join(", ")));
    }

    format!("import {} from '{}';", import_parts.join(" "), import.module_path)
}
