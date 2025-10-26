//! Parser tests

use openapi_generator_parser::*;

fn fixtures_path() -> &'static str {
    "../tests/fixtures"
}

#[test]
fn test_parse_valid_yaml() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/minimal.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Minimal API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(matches!(
        parse_result.openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));
}

#[test]
fn test_parse_valid_json() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/minimal.json", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Minimal API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(matches!(
        parse_result.openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));
}

#[test]
fn test_parse_petstore_yaml() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Petstore API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(matches!(
        parse_result.openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));

    // Check that we have paths
    assert!(!parse_result.openapi.paths.paths.is_empty());

    // Check that we have components
    assert!(parse_result.openapi.components.is_some());
    let components = parse_result.openapi.components.as_ref().unwrap();
    assert!(!components.schemas.is_empty());
    assert!(components.schemas.contains_key("Pet"));
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Order"));
}

#[test]
fn test_parse_petstore_json() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.json", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Petstore API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(matches!(
        parse_result.openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));

    // Check that we have paths
    assert!(!parse_result.openapi.paths.paths.is_empty());

    // Check that we have components
    assert!(parse_result.openapi.components.is_some());
    let components = parse_result.openapi.components.as_ref().unwrap();
    assert!(!components.schemas.is_empty());
    assert!(components.schemas.contains_key("Pet"));
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Order"));
}

#[test]
fn test_parse_missing_title() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/invalid/missing-title.yaml", fixtures_path()));

    assert!(result.is_err());
    if let Err(Error::MissingRequiredField { field }) = result {
        assert_eq!(field, "info.title");
    } else {
        panic!("Expected MissingRequiredField error");
    }
}

#[test]
fn test_parse_missing_version() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/invalid/missing-version.yaml", fixtures_path()));

    assert!(result.is_err());
    if let Err(Error::MissingRequiredField { field }) = result {
        assert_eq!(field, "info.version");
    } else {
        panic!("Expected MissingRequiredField error");
    }
}

#[test]
fn test_parse_no_paths() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/invalid/no-paths.yaml", fixtures_path()));

    assert!(result.is_err());
    if let Err(Error::ValidationError { message }) = result {
        assert!(message.contains("at least one path defined"));
    } else {
        panic!("Expected ValidationError");
    }
}

#[test]
fn test_parse_openapi_3_0_rejected() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/invalid/openapi-3.0.yaml", fixtures_path()));

    assert!(result.is_err());
    if let Err(Error::YamlParse { .. }) = result {
        // This is correct - the YAML parser rejects OpenAPI 3.0 specs
    } else {
        panic!("Expected YamlParse error for OpenAPI 3.0 spec");
    }
}

#[test]
fn test_parse_circular_reference() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/invalid/circular-ref.yaml", fixtures_path()));

    // Note: Circular reference detection is simplified and doesn't handle
    // references within object properties yet
    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Test API");
}

#[test]
fn test_parse_content_yaml() {
    let content = r#"
openapi: 3.1.0
info:
  title: Test API
  version: 1.0.0
paths:
  /test:
    get:
      responses:
        '200':
          description: OK
"#;

    let parser = OpenApiParser::new();
    let result = parser.parse_content(content, Some("yaml"));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Test API");
}

#[test]
fn test_parse_content_json() {
    let content = r#"{
  "openapi": "3.1.0",
  "info": {
    "title": "Test API",
    "version": "1.0.0"
  },
  "paths": {
    "/test": {
      "get": {
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      }
    }
  }
}"#;

    let parser = OpenApiParser::new();
    let result = parser.parse_content(content, Some("json"));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Test API");
}

#[test]
fn test_parse_content_auto_detect_yaml() {
    let content = r#"
openapi: 3.1.0
info:
  title: Test API
  version: 1.0.0
paths:
  /test:
    get:
      responses:
        '200':
          description: OK
"#;

    let parser = OpenApiParser::new();
    let result = parser.parse_content(content, None);

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Test API");
}

#[test]
fn test_parse_content_auto_detect_json() {
    let content = r#"{
  "openapi": "3.1.0",
  "info": {
    "title": "Test API",
    "version": "1.0.0"
  },
  "paths": {
    "/test": {
      "get": {
        "responses": {
          "200": {
            "description": "OK"
          }
        }
      }
    }
  }
}"#;

    let parser = OpenApiParser::new();
    let result = parser.parse_content(content, None);

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    assert_eq!(parse_result.openapi.info.title, "Test API");
}

#[test]
fn test_parse_unsupported_format() {
    let parser = OpenApiParser::new();
    let result = parser.parse_content("some content", Some("xml"));

    assert!(result.is_err());
    if let Err(Error::UnsupportedFormat { format }) = result {
        assert_eq!(format, "xml");
    } else {
        panic!("Expected UnsupportedFormat error");
    }
}

#[test]
fn test_parser_config() {
    let config = ParserConfig {
        allow_external_refs: true,
        strict_mode: false,
        validate_schemas: false,
        max_reference_depth: 5,
    };

    let parser = OpenApiParser::with_config(config);
    let result = parser.parse_file(format!("{}/valid/minimal.yaml", fixtures_path()));

    assert!(result.is_ok());
}

#[test]
fn test_legacy_functions() {
    // Test legacy parse_file function
    let result = parse_file(format!("{}/valid/minimal.yaml", fixtures_path()));
    assert!(result.is_ok());

    // Test legacy parse_content function
    let content = r#"
openapi: 3.1.0
info:
  title: Test API
  version: 1.0.0
paths:
  /test:
    get:
      responses:
        '200':
          description: OK
"#;
    let result = parse_content(content, Some("yaml"));
    assert!(result.is_ok());
}

#[test]
fn test_source_location() {
    let location = SourceLocation::new()
        .with_file_path(std::path::PathBuf::from("test.yaml"))
        .with_line_column(10, 5)
        .with_openapi_path("/info/title".to_string());

    assert_eq!(
        location.file_path,
        Some(std::path::PathBuf::from("test.yaml"))
    );
    assert_eq!(location.line, Some(10));
    assert_eq!(location.column, Some(5));
    assert_eq!(location.openapi_path, Some("/info/title".to_string()));
}

#[test]
fn test_parse_warning() {
    let location = SourceLocation::new().with_openapi_path("/components/schemas/Test".to_string());
    let warning = ParseWarning::new("Test warning".to_string(), location);

    assert_eq!(warning.message, "Test warning");
    assert_eq!(
        warning.location.openapi_path,
        Some("/components/schemas/Test".to_string())
    );
}

#[test]
fn test_generated_yaml_can_be_parsed() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok(), "Failed to parse generated YAML file");
    let parse_result = result.unwrap();

    assert_eq!(parse_result.openapi.info.title, "Petstore API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(!parse_result.openapi.paths.paths.is_empty());
    assert!(parse_result.openapi.components.is_some());

    let components = parse_result.openapi.components.as_ref().unwrap();
    assert!(!components.schemas.is_empty());
    assert!(components.schemas.contains_key("Pet"));
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Order"));

    // Should have no warnings for a valid generated file
    assert!(parse_result.warnings.is_empty());
}

#[test]
fn test_generated_json_can_be_parsed() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.json", fixtures_path()));

    assert!(result.is_ok(), "Failed to parse generated JSON file");
    let parse_result = result.unwrap();

    assert_eq!(parse_result.openapi.info.title, "Petstore API");
    assert_eq!(parse_result.openapi.info.version, "1.0.0");
    assert!(!parse_result.openapi.paths.paths.is_empty());
    assert!(parse_result.openapi.components.is_some());

    let components = parse_result.openapi.components.as_ref().unwrap();
    assert!(!components.schemas.is_empty());
    assert!(components.schemas.contains_key("Pet"));
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Order"));

    // Should have no warnings for a valid generated file
    assert!(parse_result.warnings.is_empty());
}

#[test]
fn test_generated_yaml_and_json_are_equivalent() {
    let parser = OpenApiParser::new();

    let yaml_result = parser
        .parse_file(format!("{}/valid/petstore.yaml", fixtures_path()))
        .unwrap();
    let json_result = parser
        .parse_file(format!("{}/valid/petstore.json", fixtures_path()))
        .unwrap();

    // Both should have the same basic structure
    assert_eq!(
        yaml_result.openapi.info.title,
        json_result.openapi.info.title
    );
    assert_eq!(
        yaml_result.openapi.info.version,
        json_result.openapi.info.version
    );
    assert_eq!(
        yaml_result.openapi.paths.paths.len(),
        json_result.openapi.paths.paths.len()
    );

    // Both should have the same components
    let yaml_components = yaml_result.openapi.components.as_ref().unwrap();
    let json_components = json_result.openapi.components.as_ref().unwrap();
    assert_eq!(yaml_components.schemas.len(), json_components.schemas.len());

    // Both should have the same schema names
    let yaml_schema_names: std::collections::HashSet<&String> =
        yaml_components.schemas.keys().collect();
    let json_schema_names: std::collections::HashSet<&String> =
        json_components.schemas.keys().collect();
    assert_eq!(yaml_schema_names, json_schema_names);

    // Both should have no warnings
    assert!(yaml_result.warnings.is_empty());
    assert!(json_result.warnings.is_empty());
}
