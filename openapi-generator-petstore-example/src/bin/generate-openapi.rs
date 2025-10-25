//! Script to generate OpenAPI YAML specification from the Petstore API

use openapi_generator_petstore_example::ApiDoc;
use utoipa::OpenApi;

fn main() {
    // Generate the OpenAPI specification
    let openapi = ApiDoc::openapi();
    
    // Convert to YAML
    let yaml = serde_norway::to_string(&openapi).expect("Failed to serialize OpenAPI spec to YAML");
    
    // Write to file
    std::fs::write("petstore-api.yaml", yaml).expect("Failed to write OpenAPI spec to file");
    
    println!("✅ OpenAPI specification generated: petstore-api.yaml");
    println!("📊 Endpoints: {}", openapi.paths.paths.len());
    println!("📋 Components: {}", openapi.components.as_ref().map_or(0, |c| c.schemas.len()));
}
