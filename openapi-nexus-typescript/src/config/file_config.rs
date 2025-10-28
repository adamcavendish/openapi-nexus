//! File organization configuration

use std::path::PathBuf;

/// Configuration for file organization
#[derive(Debug, Clone)]
pub struct FileConfig {
    /// Output directory for generated files
    pub output_dir: PathBuf,
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
    /// Use PascalCase for file names
    PascalCase,
}

impl Default for FileConfig {
    fn default() -> Self {
        Self {
            output_dir: PathBuf::from("generated"),
            naming_convention: NamingConvention::PascalCase,
        }
    }
}
