//! Core traits for language generators

pub mod code_generator;
pub mod emission;
pub mod file_writer;
pub mod types;

pub use emission::{EmissionContext, ToRcDocWithContext};
