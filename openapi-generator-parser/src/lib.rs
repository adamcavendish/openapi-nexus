//! OpenAPI parser using utoipa types
//!
//! This crate handles loading and parsing OpenAPI 3.1 specifications
//! from JSON/YAML files into utoipa's OpenAPI types.

pub mod error;
pub mod parser;

pub use error::Error;
pub use parser::{OpenApiParser, ParseResult, ParserConfig};
