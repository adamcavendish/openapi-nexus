//! Integration tests for OpenAPI parser

use openapi_generator_parser::{OpenApiParser, ParserConfig};

fn fixtures_path() -> &'static str {
    "../tests/fixtures"
}

#[test]
fn test_petstore_integration() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok(), "Failed to parse petstore.yaml");

    let parse_result = result.unwrap();
    let openapi = &parse_result.openapi;

    // Verify basic structure
    assert!(matches!(
        openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));
    assert_eq!(openapi.info.title, "Petstore API");
    assert_eq!(openapi.info.version, "1.0.0");
    assert_eq!(
        openapi.info.description,
        Some(
            "This is a sample Pet Store Server based on the OpenAPI 3.1 specification".to_string()
        )
    );

    // Verify contact information
    assert!(openapi.info.contact.is_some());
    let contact = openapi.info.contact.as_ref().unwrap();
    assert_eq!(contact.email, Some("apiteam@swagger.io".to_string()));

    // Verify license information
    assert!(openapi.info.license.is_some());
    let _license = openapi.info.license.as_ref().unwrap();

    // Verify paths exist
    assert!(!openapi.paths.paths.is_empty());

    // Check specific paths
    assert!(openapi.paths.paths.contains_key("/pet"));
    assert!(openapi.paths.paths.contains_key("/pet/{petId}"));
    assert!(openapi.paths.paths.contains_key("/store/inventory"));
    assert!(openapi.paths.paths.contains_key("/user"));
    assert!(openapi.paths.paths.contains_key("/user/{username}"));

    // Verify components exist
    assert!(openapi.components.is_some());
    let components = openapi.components.as_ref().unwrap();
    assert!(!components.schemas.is_empty());

    // Check key schemas exist
    assert!(components.schemas.contains_key("Pet"));
    assert!(components.schemas.contains_key("User"));
    assert!(components.schemas.contains_key("Order"));
    assert!(components.schemas.contains_key("Category"));
    assert!(components.schemas.contains_key("Tag"));
    assert!(components.schemas.contains_key("PetStatus"));
    assert!(components.schemas.contains_key("OrderStatus"));
    assert!(components.schemas.contains_key("ApiResponse"));
    assert!(components.schemas.contains_key("ErrorResponse"));

    // Verify tags exist
    assert!(openapi.tags.is_some());
    let tags = openapi.tags.as_ref().unwrap();
    assert!(!tags.is_empty());
    let tag_names: Vec<&String> = tags.iter().map(|tag| &tag.name).collect();
    assert!(tag_names.contains(&&"pet".to_string()));
    assert!(tag_names.contains(&&"store".to_string()));
    assert!(tag_names.contains(&&"user".to_string()));

    // Verify no warnings for valid spec
    assert!(
        parse_result.warnings.is_empty(),
        "Unexpected warnings: {:?}",
        parse_result.warnings
    );
}

#[test]
fn test_petstore_schema_validation() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    let openapi = &parse_result.openapi;

    // Verify Pet schema structure
    let components = openapi.components.as_ref().unwrap();
    let pet_schema = components.schemas.get("Pet").unwrap();

    // Pet schema should be an object type
    match pet_schema {
        utoipa::openapi::RefOr::T(schema) => {
            match schema {
                utoipa::openapi::Schema::Object(obj_schema) => {
                    // Check required fields
                    assert!(obj_schema.required.contains(&"name".to_string()));
                    assert!(obj_schema.required.contains(&"photo_urls".to_string()));

                    // Check properties exist
                    assert!(obj_schema.properties.contains_key("id"));
                    assert!(obj_schema.properties.contains_key("name"));
                    assert!(obj_schema.properties.contains_key("photo_urls"));
                    assert!(obj_schema.properties.contains_key("status"));
                    assert!(obj_schema.properties.contains_key("category"));
                    assert!(obj_schema.properties.contains_key("tags"));
                }
                _ => panic!("Pet schema should be an object"),
            }
        }
        _ => panic!("Pet schema should be a direct schema, not a reference"),
    }

    // Verify User schema structure
    let user_schema = components.schemas.get("User").unwrap();
    match user_schema {
        utoipa::openapi::RefOr::T(schema) => {
            match schema {
                utoipa::openapi::Schema::Object(obj_schema) => {
                    // User should have various properties
                    assert!(obj_schema.properties.contains_key("id"));
                    assert!(obj_schema.properties.contains_key("username"));
                    assert!(obj_schema.properties.contains_key("email"));
                    assert!(obj_schema.properties.contains_key("first_name"));
                    assert!(obj_schema.properties.contains_key("last_name"));
                }
                _ => panic!("User schema should be an object"),
            }
        }
        _ => panic!("User schema should be a direct schema, not a reference"),
    }
}

#[test]
fn test_petstore_operations_validation() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    let openapi = &parse_result.openapi;

    // Check pet operations
    let pet_path = openapi.paths.paths.get("/pet").unwrap();

    // PUT operation
    assert!(pet_path.put.is_some());
    let put_op = pet_path.put.as_ref().unwrap();
    assert_eq!(put_op.operation_id, Some("update_pet".to_string()));
    assert_eq!(put_op.summary, Some("Update an existing pet".to_string()));
    assert!(put_op.tags.as_ref().unwrap().contains(&"pet".to_string()));

    // POST operation
    assert!(pet_path.post.is_some());
    let post_op = pet_path.post.as_ref().unwrap();
    assert_eq!(post_op.operation_id, Some("add_pet".to_string()));
    assert_eq!(
        post_op.summary,
        Some("Add a new pet to the store".to_string())
    );

    // Check pet/{petId} operations
    let pet_id_path = openapi.paths.paths.get("/pet/{petId}").unwrap();

    // GET operation
    assert!(pet_id_path.get.is_some());
    let get_op = pet_id_path.get.as_ref().unwrap();
    assert_eq!(get_op.operation_id, Some("get_pet_by_id".to_string()));
    assert_eq!(get_op.summary, Some("Find pet by ID".to_string()));

    // DELETE operation
    assert!(pet_id_path.delete.is_some());
    let delete_op = pet_id_path.delete.as_ref().unwrap();
    assert_eq!(delete_op.operation_id, Some("delete_pet".to_string()));
    assert_eq!(delete_op.summary, Some("Delete a pet".to_string()));
}

#[test]
fn test_petstore_references_validation() {
    let parser = OpenApiParser::new();
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();
    let openapi = &parse_result.openapi;

    // Verify that references are properly structured
    let components = openapi.components.as_ref().unwrap();

    // Check Pet schema references
    let pet_schema = components.schemas.get("Pet").unwrap();
    match pet_schema {
        utoipa::openapi::RefOr::T(schema) => {
            match schema {
                utoipa::openapi::Schema::Object(obj_schema) => {
                    // Check that status references PetStatus
                    if let Some(utoipa::openapi::RefOr::Ref(ref_schema)) = obj_schema.properties.get("status") {
                        assert_eq!(
                            ref_schema.ref_location,
                            "#/components/schemas/PetStatus"
                        );
                    }

                    // Check that category references Category
                    if let Some(utoipa::openapi::RefOr::Ref(ref_schema)) = obj_schema.properties.get("category") {
                        assert_eq!(
                            ref_schema.ref_location,
                            "#/components/schemas/Category"
                        );
                    }
                }
                _ => panic!("Pet schema should be an object"),
            }
        }
        _ => panic!("Pet schema should be a direct schema, not a reference"),
    }
}

#[test]
fn test_petstore_with_custom_config() {
    let config = ParserConfig {
        allow_external_refs: false,
        strict_mode: true,
        validate_schemas: true,
        max_reference_depth: 10,
    };

    let parser = OpenApiParser::with_config(config);
    let result = parser.parse_file(format!("{}/valid/petstore.yaml", fixtures_path()));

    assert!(result.is_ok());
    let parse_result = result.unwrap();

    // Should parse successfully with strict validation
    assert_eq!(parse_result.openapi.info.title, "Petstore API");
    assert!(matches!(
        parse_result.openapi.openapi,
        utoipa::openapi::OpenApiVersion::Version31
    ));
}
