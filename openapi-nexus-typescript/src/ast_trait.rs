//! AST traits for converting nodes to RcDoc (simplified)

use pretty::RcDoc;

use crate::emission::error::EmitError;

/// Emission context for controlling output formatting
#[derive(Debug, Clone)]
pub struct EmissionContext {
    /// Whether to include documentation comments
    pub include_docs: bool,
    /// Whether to force multiline formatting
    pub force_multiline: bool,
    /// Current indentation level
    pub indent_level: usize,
}

/// Trait for converting AST nodes to RcDoc with context
pub trait ToRcDocWithContext {
    /// Convert to RcDoc with emission context
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError>;
}

impl EmissionContext {
    /// Create a new emission context with defaults
    pub fn new() -> Self {
        Self {
            include_docs: true,
            force_multiline: false,
            indent_level: 0,
        }
    }

    /// Create context without documentation
    pub fn without_docs() -> Self {
        Self {
            include_docs: false,
            force_multiline: false,
            indent_level: 0,
        }
    }

    /// Create context with forced multiline formatting
    pub fn multiline() -> Self {
        Self {
            include_docs: true,
            force_multiline: true,
            indent_level: 0,
        }
    }

    /// Increment indentation level
    pub fn increment_indent(&self) -> Self {
        Self {
            include_docs: self.include_docs,
            force_multiline: self.force_multiline,
            indent_level: self.indent_level + 1,
        }
    }

    /// Set indentation level
    pub fn with_indent(&self, level: usize) -> Self {
        Self {
            include_docs: self.include_docs,
            force_multiline: self.force_multiline,
            indent_level: level,
        }
    }

    /// Toggle documentation inclusion
    pub fn with_docs(&self, include_docs: bool) -> Self {
        Self {
            include_docs,
            force_multiline: self.force_multiline,
            indent_level: self.indent_level,
        }
    }

    /// Toggle multiline formatting
    pub fn with_multiline(&self, force_multiline: bool) -> Self {
        Self {
            include_docs: self.include_docs,
            force_multiline,
            indent_level: self.indent_level,
        }
    }
}

impl Default for EmissionContext {
    fn default() -> Self {
        Self::new()
    }
}
