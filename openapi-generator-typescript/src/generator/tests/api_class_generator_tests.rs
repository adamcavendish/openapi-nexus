//! Unit tests for ApiClassGenerator

use super::super::api_class_generator::ApiClassGenerator;
use super::super::super::ast::TsNode;
use utoipa::openapi::OpenApi;

#[test]
fn test_api_class_generator_creation() {
    let _generator = ApiClassGenerator::new();
    // Test that the generator can be created successfully
    assert!(true);
}

#[test]
fn test_api_class_generator_default() {
    let _generator = ApiClassGenerator::default();
    // Test that the generator can be created with default
    assert!(true);
}

#[test]
fn test_generate_api_class_with_no_operations() {
    let generator = ApiClassGenerator::new();
    let operations = vec![];
    let openapi = OpenApi::default();

    let result = generator.generate_api_class("user", &operations, &openapi).unwrap();

    match result {
        TsNode::Class(class) => {
            assert_eq!(class.name, "UserApi");
            assert_eq!(class.extends, Some("BaseAPI".to_string()));
            assert!(class.is_export);
            assert_eq!(class.methods.len(), 1); // Only constructor
        }
        _ => panic!("Expected Class node"),
    }
}