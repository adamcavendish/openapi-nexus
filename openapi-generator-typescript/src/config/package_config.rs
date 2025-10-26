//! NPM package generation configuration

/// Configuration for npm package generation
#[derive(Debug, Clone)]
pub struct PackageConfig {
    /// Package scope/prefix (e.g., "@studio-ams")
    pub scope: Option<String>,
    /// Whether to generate npm package files
    pub generate_package: bool,
    /// TypeScript compiler target
    pub typescript_target: String,
    /// TypeScript module system
    pub typescript_module: TypeScriptModule,
    /// Whether to generate ESM configuration
    pub generate_esm_config: bool,
    /// Whether to include build scripts in package.json
    pub include_build_scripts: bool,
}

/// TypeScript module systems
#[derive(Debug, Clone)]
pub enum TypeScriptModule {
    CommonJS,
    ESNext,
    ES2020,
    ES2022,
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            scope: None,
            generate_package: true,
            typescript_target: "es6".to_string(),
            typescript_module: TypeScriptModule::CommonJS,
            generate_esm_config: true,
            include_build_scripts: false,
        }
    }
}
