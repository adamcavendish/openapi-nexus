//! Emission traits for converting AST nodes to formatted output

use pretty::RcDoc;
use serde::{Deserialize, Serialize};

/// Emission context for controlling output formatting
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionContext {
    /// Current indentation level
    pub indent_level: usize,
    /// Maximum line width for pretty printing
    pub max_line_width: usize,
}

/// Trait for converting AST nodes to RcDoc with context
pub trait ToRcDocWithContext {
    /// Error type for emission failures
    type Error: std::error::Error;

    /// Convert to RcDoc with emission context
    fn to_rcdoc_with_context(
        &self,
        context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, Self::Error>;
}

impl EmissionContext {
    /// Increment indentation level
    pub fn inc_indent(&self) -> Self {
        Self {
            indent_level: self.indent_level + 1,
            max_line_width: self.max_line_width,
        }
    }

    /// Decrement indentation level
    pub fn dec_indent(&self) -> Self {
        Self {
            indent_level: self.indent_level.saturating_sub(1),
            max_line_width: self.max_line_width,
        }
    }

    /// Set indentation level
    pub fn with_indent(&self, level: usize) -> Self {
        Self {
            indent_level: level,
            max_line_width: self.max_line_width,
        }
    }
}

impl Default for EmissionContext {
    fn default() -> Self {
        Self {
            indent_level: 0,
            max_line_width: 80,
        }
    }
}
