//! Code emission configuration

/// Configuration for code emission
#[derive(Debug, Clone)]
pub struct EmissionConfig {
    /// Whether to include JSDoc comments
    pub include_documentation: bool,
    /// Whether to use prettier formatting
    pub use_prettier: bool,
    /// Indentation style
    pub indentation: IndentationStyle,
}

/// Indentation styles
#[derive(Debug, Clone)]
pub enum IndentationStyle {
    Spaces(usize),
    Tabs,
}

impl Default for EmissionConfig {
    fn default() -> Self {
        Self {
            include_documentation: true,
            use_prettier: false,
            indentation: IndentationStyle::Spaces(2),
        }
    }
}
