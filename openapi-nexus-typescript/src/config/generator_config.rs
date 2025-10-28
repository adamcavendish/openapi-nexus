//! Generator configuration

/// Configuration for TypeScript generation
#[derive(Debug, Clone, Default)]
pub struct GeneratorConfig {
    /// File generation configuration
    pub file_config: super::FileConfig,
    /// Emission configuration
    pub emission_config: super::EmissionConfig,
    /// Package generation configuration
    pub package_config: super::PackageConfig,
}
