//! Runtime generator for TypeScript HTTP client utilities

use crate::emission::file_generator::{FileType, GeneratedFile};
use minijinja::Environment;
use utoipa::openapi::OpenApi;

/// Generator for runtime.ts file containing HTTP client utilities
pub struct RuntimeGenerator;

impl RuntimeGenerator {
    /// Create a new runtime generator
    pub fn new() -> Self {
        Self
    }

    /// Generate runtime.ts file content
    pub fn generate_runtime(&self, _openapi: &OpenApi) -> GeneratedFile {
        let title = "OpenAPI";
        let version = "1.0.0";
        let description = "OpenAPI client";

        // Load the template
        let template_str = include_str!("../../templates/runtime.ts.j2");

        // Create minijinja environment
        let mut env = Environment::new();
        env.add_template("runtime.ts", template_str)
            .expect("Failed to add runtime template");

        // Prepare template context
        let ctx = minijinja::context! {
            title => title,
            version => version,
            description => description,
        };

        // Render the template
        let tmpl = env
            .get_template("runtime.ts")
            .expect("Failed to get runtime template");
        let content = tmpl.render(ctx).expect("Failed to render runtime template");

        GeneratedFile {
            filename: "runtime.ts".to_string(),
            content,
            file_type: FileType::Runtime,
        }
    }
}

impl Default for RuntimeGenerator {
    fn default() -> Self {
        Self::new()
    }
}
