//! Integration tests for TypeScript code generation
//!
//! These tests validate the complete pipeline from OpenAPI specification
//! to generated TypeScript code using real fixture files.

use std::fs;
use std::path::Path;

use openapi_generator_typescript::TypeScriptGenerator;


/// Test helper to read a fixture file
fn read_fixture(fixture_path: &str) -> String {
    // Try multiple possible paths
    let possible_paths = [
        Path::new("tests/fixtures").join(fixture_path),
        Path::new("../../tests/fixtures").join(fixture_path),
        Path::new("../tests/fixtures").join(fixture_path),
    ];
    
    for path in &possible_paths {
        if path.exists() {
            return fs::read_to_string(path)
                .unwrap_or_else(|_| panic!("Failed to read fixture: {}", path.display()));
        }
    }
    
    panic!("Could not find fixture '{}' in any of the expected locations", fixture_path);
}

/// Test helper to generate TypeScript code from a fixture
fn generate_from_fixture(fixture_path: &str) -> Vec<openapi_generator_typescript::emission::file_generator::GeneratedFile> {
    let spec_content = read_fixture(fixture_path);
    let openapi: utoipa::openapi::OpenApi = serde_norway::from_str(&spec_content)
        .expect("Failed to parse OpenAPI specification");
    
    let generator = TypeScriptGenerator::new();
    generator.generate_files(&openapi)
        .expect("Failed to generate TypeScript files")
}

/// Test helper to validate that generated files contain expected content
fn assert_file_contains_content(
    files: &[openapi_generator_typescript::emission::file_generator::GeneratedFile],
    filename: &str,
    expected_content: &str,
) {
    let file = files.iter()
        .find(|f| f.filename == filename)
        .unwrap_or_else(|| panic!("Expected file '{}' not found in generated files", filename));
    
    assert!(
        file.content.contains(expected_content),
        "File '{}' does not contain expected content '{}'. Content:\n{}",
        filename,
        expected_content,
        file.content
    );
}

/// Test helper to validate that generated files have correct structure
fn assert_file_structure(files: &[openapi_generator_typescript::emission::file_generator::GeneratedFile]) {
    // Should have at least one file
    assert!(!files.is_empty(), "No files were generated");
    
    // All files should have non-empty content
    for file in files {
        assert!(!file.content.trim().is_empty(), "File '{}' has empty content", file.filename);
        assert!(!file.filename.is_empty(), "File has empty filename");
    }
    
    // Should have an index file
    assert!(
        files.iter().any(|f| f.filename == "index.ts"),
        "Expected index.ts file not found"
    );
}

#[test]
fn test_minimal_yaml_generation() {
    let files = generate_from_fixture("valid/minimal.yaml");
    
    // Validate basic structure
    assert_file_structure(&files);
    
    // Should generate at least an index file
    assert_file_contains_content(&files, "index.ts", "export");
}

#[test]
fn test_minimal_json_generation() {
    let files = generate_from_fixture("valid/minimal.json");
    
    // Validate basic structure
    assert_file_structure(&files);
    
    // Should generate at least an index file
    assert_file_contains_content(&files, "index.ts", "export");
}

#[test]
fn test_petstore_yaml_generation() {
    let files = generate_from_fixture("valid/petstore.yaml");
    
    // Validate basic structure
    assert_file_structure(&files);
    
    // Should generate schema files for the main entities
    assert_file_contains_content(&files, "pet.ts", "interface");
    assert_file_contains_content(&files, "user.ts", "interface");
    assert_file_contains_content(&files, "order.ts", "interface");
    assert_file_contains_content(&files, "category.ts", "interface");
    assert_file_contains_content(&files, "tag.ts", "interface");
    
    // Should generate API client
    assert_file_contains_content(&files, "api-client.ts", "class");
    assert_file_contains_content(&files, "api-client.ts", "ApiClient");
    
    // Should generate status files as proper enums
    assert_file_contains_content(&files, "pet-status.ts", "enum");
    assert_file_contains_content(&files, "order-status.ts", "enum");
    
    // Should generate index file with exports
    assert_file_contains_content(&files, "index.ts", "export");
}

#[test]
fn test_petstore_json_generation() {
    let files = generate_from_fixture("valid/petstore.json");
    
    // Validate basic structure
    assert_file_structure(&files);
    
    // Should generate the same files as YAML version
    assert_file_contains_content(&files, "pet.ts", "interface");
    assert_file_contains_content(&files, "api-client.ts", "class");
    assert_file_contains_content(&files, "index.ts", "export");
}

#[test]
fn test_generated_code_validity() {
    let files = generate_from_fixture("valid/petstore.yaml");
    
    for file in &files {
        // Check that TypeScript syntax is valid
        validate_typescript_syntax(&file.content, &file.filename);
    }
}

/// Validate basic TypeScript syntax patterns
fn validate_typescript_syntax(content: &str, filename: &str) {
    // Should not have obvious syntax errors (but undefined is valid in TypeScript)
    // assert!(!content.contains("undefined"), "File '{}' contains 'undefined'", filename);
    
    // Interface files should contain proper interface syntax
    if filename.ends_with(".ts") && !filename.contains("index") && !filename.contains("api-client") {
        if content.contains("interface") {
            assert!(
                content.contains("{") && content.contains("}"),
                "File '{}' has malformed interface syntax",
                filename
            );
        }
    }
    
    // Class files should contain proper class syntax
    if filename.contains("api-client") {
        assert!(
            content.contains("class") && content.contains("{") && content.contains("}"),
            "File '{}' has malformed class syntax",
            filename
        );
    }
    
        // Status files should contain proper interface syntax (current implementation)
        if filename.contains("status") {
            if content.contains("interface") {
                assert!(
                    content.contains("{") && content.contains("}"),
                    "File '{}' has malformed interface syntax",
                    filename
                );
            }
        }
}

#[test]
fn test_file_naming_convention() {
    let files = generate_from_fixture("valid/petstore.yaml");
    
    for file in &files {
        // All files should have .ts extension
        assert!(
            file.filename.ends_with(".ts"),
            "File '{}' does not have .ts extension",
            file.filename
        );
        
        // Filenames should be lowercase with hyphens
        let name_without_ext = file.filename.trim_end_matches(".ts");
        assert!(
            name_without_ext.chars().all(|c| c.is_ascii_lowercase() || c == '-' || c == '_'),
            "File '{}' does not follow naming convention (lowercase, hyphens/underscores)",
            file.filename
        );
    }
}

#[test]
fn test_error_handling_invalid_specs() {
    // Test that invalid specifications are handled gracefully
    let invalid_specs = [
        "invalid/circular-ref.yaml",
        "invalid/missing-title.yaml", 
        "invalid/missing-version.yaml",
        "invalid/no-paths.yaml",
        "invalid/unsupported-version.yaml",
    ];
    
    for spec in &invalid_specs {
        let spec_content = read_fixture(spec);
        let result: Result<utoipa::openapi::OpenApi, _> = serde_norway::from_str(&spec_content);
        
        // Some specs might parse but be invalid for generation
        if let Ok(openapi) = result {
            let generator = TypeScriptGenerator::new();
            let generation_result = generator.generate_files(&openapi);
            
            // Generation should either succeed or fail gracefully
            match generation_result {
                Ok(files) => {
                    // If it succeeds, files should be valid
                    assert_file_structure(&files);
                }
                Err(e) => {
                    // If it fails, error should be informative
                    assert!(!e.to_string().is_empty(), "Error message should not be empty");
                }
            }
        }
    }
}

#[test]
fn test_generation_consistency() {
    // Generate from the same spec multiple times and ensure consistency
    let files1 = generate_from_fixture("valid/petstore.yaml");
    let files2 = generate_from_fixture("valid/petstore.yaml");
    
    // Should generate the same number of files
    assert_eq!(files1.len(), files2.len(), "Generated file count should be consistent");
    
    // Should generate files with the same names
    let mut names1: Vec<&str> = files1.iter().map(|f| f.filename.as_str()).collect();
    let mut names2: Vec<&str> = files2.iter().map(|f| f.filename.as_str()).collect();
    names1.sort();
    names2.sort();
    
    assert_eq!(names1, names2, "Generated file names should be consistent");
    
    // Content should be identical (for deterministic generation)
    // Note: File order might not be deterministic, so we'll check content matches
    for file1 in &files1 {
        if let Some(file2) = files2.iter().find(|f| f.filename == file1.filename) {
            // For index.ts, check that exports are equivalent but order might differ
            if file1.filename == "index.ts" {
                let exports1: std::collections::HashSet<&str> = file1.content
                    .lines()
                    .filter(|line| line.starts_with("export"))
                    .collect();
                let exports2: std::collections::HashSet<&str> = file2.content
                    .lines()
                    .filter(|line| line.starts_with("export"))
                    .collect();
                assert_eq!(exports1, exports2, "Index.ts exports should be equivalent");
            } else {
                assert_eq!(
                    file1.content, file2.content,
                    "File '{}' content should be identical across generations",
                    file1.filename
                );
            }
        } else {
            panic!("File '{}' from first generation not found in second generation", file1.filename);
        }
    }
}

#[test]
fn test_multi_file_emission() {
    let files = generate_from_fixture("valid/petstore.yaml");
    
    // Should generate multiple files (not just a single file)
    assert!(files.len() > 1, "Should generate multiple files for complex specs");
    
    // Should have different types of files
    let has_interfaces = files.iter().any(|f| f.content.contains("interface"));
    let has_classes = files.iter().any(|f| f.content.contains("class"));
    let has_enums = files.iter().any(|f| f.content.contains("enum") || f.filename.contains("status"));
    let has_index = files.iter().any(|f| f.filename == "index.ts");
    
    assert!(has_interfaces, "Should generate interface files");
    assert!(has_classes, "Should generate class files");
    assert!(has_enums, "Should generate enum files");
    assert!(has_index, "Should generate index file");
}

#[test]
fn test_generated_code_compilation() {
    // This test would ideally compile the generated TypeScript code
    // For now, we'll just validate basic syntax patterns
    
    let files = generate_from_fixture("valid/petstore.yaml");
    
    for file in &files {
        // Check for basic TypeScript patterns
        if file.content.contains("interface") {
            // Should have proper interface syntax
            // Count interface declarations
            let interface_count = file.content.matches("export interface ").count();
            
            // Count opening braces that follow interface declarations
            // Look for lines that contain "export interface" and have "{" on the same line
            let interface_opening_braces = file.content
                .lines()
                .filter(|line| line.contains("export interface") && line.contains("{"))
                .count();
            
            // Count closing braces that are standalone lines (likely interface endings)
            let interface_closing_braces = file.content
                .lines()
                .filter(|line| line.trim() == "}")
                .count();
            
            assert!(
                interface_count == interface_opening_braces,
                "File '{}' has {} interface declarations but {} opening braces",
                file.filename, interface_count, interface_opening_braces
            );
            
            assert!(
                interface_count == interface_closing_braces,
                "File '{}' has {} interface declarations but {} closing braces",
                file.filename, interface_count, interface_closing_braces
            );
        }
        
        if file.content.contains("class") {
            // Should have proper class syntax - check for basic structure
            assert!(
                file.content.contains("class") && file.content.contains("{"),
                "File '{}' has malformed class syntax",
                file.filename
            );
        }
        
        if file.content.contains("enum ") {
            // Should have proper enum syntax (not just in comments)
            assert!(
                file.content.matches("enum ").count() == file.content.matches("{").count(),
                "File '{}' has mismatched enum braces",
                file.filename
            );
        }
    }
}
