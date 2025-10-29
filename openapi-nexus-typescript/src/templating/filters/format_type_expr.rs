//! Template filter for formatting TypeExpression as TypeScript string

use crate::ast::TypeExpression;
use minijinja::value::ViaDeserialize;

/// Template filter for formatting TypeExpression as TypeScript string
pub fn format_type_expr_filter(type_expr: ViaDeserialize<TypeExpression>) -> String {
    type_expr.to_typescript_string()
}
