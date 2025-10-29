//! TypeScript-specific file categories

use openapi_nexus_core::traits::file_writer::FileCategory;

/// TypeScript-specific file categories
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsFileCategory {
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

impl From<TsFileCategory> for FileCategory {
    fn from(category: TsFileCategory) -> Self {
        match category {
            TsFileCategory::Api => FileCategory::Apis,
            TsFileCategory::Models | TsFileCategory::Schema => FileCategory::Models,
            TsFileCategory::Index
            | TsFileCategory::PackageJson
            | TsFileCategory::TsConfig
            | TsFileCategory::TsConfigEsm
            | TsFileCategory::Readme => FileCategory::ProjectFiles,
            TsFileCategory::Runtime | TsFileCategory::Utility => FileCategory::Runtime,
        }
    }
}
