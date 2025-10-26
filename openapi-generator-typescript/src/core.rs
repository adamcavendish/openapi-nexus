/// Error type for TypeScript generation
#[derive(Debug, snafu::Snafu)]
#[snafu(visibility(pub))]
pub enum GeneratorError {
    #[snafu(display("Generator error: {}", message))]
    Generic { message: String },
}

/// Core trait for TypeScript code generation
pub trait TypeScriptGenerator {
    /// Generate TypeScript code from OpenAPI specification
    fn generate(&self, openapi: &utoipa::openapi::OpenApi) -> Result<String, GeneratorError>;

    /// Generate multiple TypeScript files from OpenAPI specification
    fn generate_files(
        &self,
        openapi: &utoipa::openapi::OpenApi,
    ) -> Result<Vec<crate::emission::file_generator::GeneratedFile>, GeneratorError>;
}
