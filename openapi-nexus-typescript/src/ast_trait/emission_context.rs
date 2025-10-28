//! Context for emission that can be passed through the AST

/// Context for emission that can be passed through the AST
#[derive(Debug, Clone)]
pub struct EmissionContext {
    /// Current indentation level
    pub indent_level: usize,
    /// Whether to force multiline formatting
    pub force_multiline: bool,
    /// Whether to include documentation comments
    pub include_docs: bool,
}

impl Default for EmissionContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            force_multiline: false,
            include_docs: true,
        }
    }
}

impl EmissionContext {
    /// Create a new emission context
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a context with specific indentation
    pub fn with_indent(indent_level: usize) -> Self {
        Self {
            indent_level,
            force_multiline: false,
            include_docs: true,
        }
    }

    /// Create a context that forces multiline formatting
    pub fn force_multiline() -> Self {
        Self {
            indent_level: 0,
            force_multiline: true,
            include_docs: true,
        }
    }

    /// Increment the indentation level
    pub fn increment_indent(&self) -> Self {
        Self {
            indent_level: self.indent_level + 1,
            force_multiline: self.force_multiline,
            include_docs: self.include_docs,
        }
    }
}
