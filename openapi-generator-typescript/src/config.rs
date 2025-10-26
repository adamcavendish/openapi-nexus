//! Configuration types for TypeScript code generation

use std::path::PathBuf;

/// Configuration for file organization
#[derive(Debug, Clone)]
pub struct FileConfig {
    /// Output directory for generated files
    pub output_dir: PathBuf,
    /// Whether to use granular files (one per schema) or grouped files
    pub granular_files: bool,
    /// File naming convention
    pub naming_convention: NamingConvention,
}

/// File naming conventions
#[derive(Debug, Clone)]
pub enum NamingConvention {
    /// Use camelCase for file names
    CamelCase,
    /// Use kebab-case for file names
    KebabCase,
    /// Use snake_case for file names
    SnakeCase,
}

/// Configuration for type mapping
#[derive(Debug, Clone)]
pub struct TypeMappingConfig {
    /// Whether to generate strict types
    pub strict_types: bool,
    /// Whether to generate nullable types
    pub nullable_types: bool,
    /// Whether to generate union types for oneOf/anyOf
    pub union_types: bool,
}

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

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("generated"),
            granular_files: true,
            naming_convention: NamingConvention::KebabCase,
        }
    }
}

impl Default for TypeMappingConfig {
    fn default() -> Self {
        Self {
            strict_types: true,
            nullable_types: true,
            union_types: true,
        }
    }
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
