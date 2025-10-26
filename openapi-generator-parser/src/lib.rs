//! OpenAPI parser using utoipa types
//!
//! This crate handles loading and parsing OpenAPI 3.1 specifications
//! from JSON/YAML files into utoipa's OpenAPI types.

pub mod error;
pub mod parser;

pub use error::{Error, ParseWarning, SourceLocation};
pub use parser::{
    OpenApiParser, ParseResult, ParserConfig, parse_content, parse_content_with_validation,
    parse_file, parse_file_with_validation, validate_openapi,
};
