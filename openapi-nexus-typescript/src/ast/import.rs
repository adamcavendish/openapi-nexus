//! TypeScript import handling (module root)
//!
//! Split into separate files for TsImport and TsImportSpecifier.

pub mod ts_import;
pub mod ts_import_specifier;

pub use ts_import::TsImport;
pub use ts_import_specifier::TsImportSpecifier;
