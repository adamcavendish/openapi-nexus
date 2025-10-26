//! Configuration types for TypeScript code generation

pub mod emission_config;
pub mod file_config;
pub mod generator_config;
pub mod package_config;

// Re-export all types for convenience
pub use emission_config::{EmissionConfig, IndentationStyle};
pub use file_config::{FileConfig, NamingConvention};
pub use generator_config::GeneratorConfig;
pub use package_config::{PackageConfig, TypeScriptModule};
