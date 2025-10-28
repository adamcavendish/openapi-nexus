//! Unit tests for TemplateGenerator

use super::super::template_generator::{ApiMethodData, ParameterData, Template, TemplateGenerator};

#[test]
fn test_template_generator_creation() {
    let _generator = TemplateGenerator::new();
    // Test that the generator can be created successfully
    assert!(true);
}

#[test]
fn test_template_generator_default() {
    let _generator = TemplateGenerator::default();
    // Test that the generator can be created with default
    assert!(true);
}

#[test]
fn test_api_method_get_template() {
    let generator = TemplateGenerator::new();

    let api_method_data = ApiMethodData {
        method_name: "getUser".to_string(),
        http_method: "GET".to_string(),
        path: "/users/{id}".to_string(),
        path_params: vec![ParameterData {
            name: "id".to_string(),
            type_expr: Some("string".to_string()),
            optional: false,
        }],
        query_params: vec![],
        header_params: vec![],
        body_param: None,
        return_type: "Promise<User>".to_string(),
        has_auth: true,
        has_error_handling: true,
    };

    let template = Template::ApiMethodGet(api_method_data);
    let result = generator.generate(&template).unwrap();

    // Verify the generated code contains expected elements
    assert!(result.contains("const url ="));
    assert!(result.contains("method: 'GET'"));
    assert!(result.contains("response.json()"));
}

#[test]
fn test_api_method_post_template_with_body() {
    let generator = TemplateGenerator::new();

    let api_method_data = ApiMethodData {
        method_name: "createUser".to_string(),
        http_method: "POST".to_string(),
        path: "/users".to_string(),
        path_params: vec![],
        query_params: vec![],
        header_params: vec![],
        body_param: Some(ParameterData {
            name: "body".to_string(),
            type_expr: Some("CreateUserRequest".to_string()),
            optional: false,
        }),
        return_type: "Promise<User>".to_string(),
        has_auth: true,
        has_error_handling: true,
    };

    let template = Template::ApiMethodPostPutPatch(api_method_data);
    let result = generator.generate(&template).unwrap();

    // Verify the generated code contains expected elements
    assert!(result.contains("'Content-Type': 'application/json'"));
    assert!(result.contains("JSON.stringify(body)"));
    assert!(result.contains("method: 'POST'"));
    assert!(result.contains("response.json()"));
}

#[test]
fn test_template_lines_generation() {
    let generator = TemplateGenerator::new();

    let api_method_data = ApiMethodData {
        method_name: "testMethod".to_string(),
        http_method: "GET".to_string(),
        path: "/test".to_string(),
        path_params: vec![],
        query_params: vec![],
        header_params: vec![],
        body_param: None,
        return_type: "Promise<any>".to_string(),
        has_auth: false,
        has_error_handling: false,
    };

    let template = Template::ApiMethodGet(api_method_data);
    let lines = generator.generate_lines(&template).unwrap();

    // Verify that lines are generated
    assert!(!lines.is_empty());
}
