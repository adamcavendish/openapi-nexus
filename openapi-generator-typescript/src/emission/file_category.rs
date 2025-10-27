//! TypeScript-specific file categories

use openapi_generator_core::traits::file_writer::FileCategory;

/// TypeScript-specific file categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TypeScriptFileCategory {
    /// Schema definitions
    Schema,
    /// Data models and schemas
    Models,
    /// API client classes
    Api,
    /// Main index file
    Index,
    /// Utility functions
    Utility,
    /// Runtime utilities
    Runtime,
    /// Package.json file
    PackageJson,
    /// TypeScript configuration
    TsConfig,
    /// TypeScript ESM configuration
    TsConfigEsm,
    /// README file
    Readme,
}

impl From<TypeScriptFileCategory> for FileCategory {
    fn from(category: TypeScriptFileCategory) -> Self {
        match category {
            TypeScriptFileCategory::Api => FileCategory::Apis,
            TypeScriptFileCategory::Models | TypeScriptFileCategory::Schema => FileCategory::Models,
            TypeScriptFileCategory::Index
            | TypeScriptFileCategory::PackageJson
            | TypeScriptFileCategory::TsConfig
            | TypeScriptFileCategory::TsConfigEsm
            | TypeScriptFileCategory::Readme => FileCategory::ProjectFiles,
            TypeScriptFileCategory::Runtime | TypeScriptFileCategory::Utility => {
                FileCategory::Runtime
            }
        }
    }
}
