//! Golden file tests for TypeScript code generation
//!
//! These tests compare generated TypeScript code against known-good golden files.
//! To update golden files after intentional changes, run:
//!   UPDATE_GOLDEN=1 cargo test --test golden_tests

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

use similar::TextDiff;
use utoipa::openapi::OpenApi;

use openapi_generator_typescript::TypeScriptGenerator;

/// Read a fixture file from various possible locations
fn read_fixture(fixture_path: &str) -> String {
    let possible_paths = [
        Path::new("tests/fixtures").join(fixture_path),
        Path::new("../../tests/fixtures").join(fixture_path),
        Path::new("../tests/fixtures").join(fixture_path),
    ];

    for path in &possible_paths {
        if path.exists() {
            return fs::read_to_string(path).unwrap();
        }
    }
    panic!("Could not find fixture: {}", fixture_path);
}

/// Get the golden directory path
fn get_golden_dir() -> &'static Path {
    Path::new("../tests/golden/typescript")
}

/// Generate TypeScript files from an OpenAPI specification
fn generate_typescript_files(spec_content: &str) -> HashMap<String, String> {
    let openapi: OpenApi = serde_norway::from_str(spec_content).unwrap();
    let generator = TypeScriptGenerator::new();
    let generated_files = generator.generate_files(&openapi).unwrap();

    generated_files
        .into_iter()
        .map(|f| (f.filename, f.content))
        .collect()
}

/// Update or compare golden files for a given spec
fn test_golden_files(spec_name: &str, fixture_path: &str) {
    let spec_content = read_fixture(fixture_path);
    let generated = generate_typescript_files(&spec_content);
    let update_mode = env::var("UPDATE_GOLDEN").is_ok();

    if update_mode {
        update_golden_files(spec_name, &generated);
    } else {
        compare_with_golden_files(spec_name, &generated);
    }
}

/// Update golden files with generated content
fn update_golden_files(spec_name: &str, generated: &HashMap<String, String>) {
    eprintln!("🐛 UPDATE_GOLDEN mode: updating golden files");
    let golden_dir = get_golden_dir().join(spec_name);
    fs::create_dir_all(&golden_dir).unwrap();

    for (filename, content) in generated {
        let file_path = golden_dir.join(filename);
        fs::write(&file_path, content).unwrap();
        eprintln!("  Updated: {}", file_path.display());
    }
    eprintln!("✅ Updated golden files for {}", spec_name);
}

/// Compare generated files with golden files and report differences
fn compare_with_golden_files(spec_name: &str, generated: &HashMap<String, String>) {
    let golden_dir = get_golden_dir().join(spec_name);

    for (filename, gen_content) in generated {
        let golden_path = golden_dir.join(filename);

        if !golden_path.exists() {
            panic!("Golden file not found: {}", golden_path.display());
        }

        let golden_content = fs::read_to_string(&golden_path).unwrap();

        if gen_content != &golden_content {
            show_diff(spec_name, filename, &golden_content, gen_content);
            panic!("Golden file mismatch for {}: {}", spec_name, filename);
        }
    }
    eprintln!("✅ Golden file test passed for {}", spec_name);
}

/// Show a diff when golden files don't match
fn show_diff(spec_name: &str, filename: &str, golden: &str, generated: &str) {
    eprintln!("\n❌ Content mismatch in: {}/{}", spec_name, filename);
    eprintln!("{}", "=".repeat(80));

    let diff = TextDiff::from_lines(golden, generated);
    eprintln!(
        "{}",
        diff.unified_diff()
            .context_radius(3)
            .header("golden", "generated")
    );

    eprintln!("\n💡 To update golden files, run:");
    eprintln!("   UPDATE_GOLDEN=1 cargo test --test golden_tests");
}

#[test]
fn test_petstore_golden() {
    test_golden_files("petstore", "valid/petstore.yaml");
}

#[test]
fn test_minimal_golden() {
    test_golden_files("minimal", "valid/minimal.yaml");
}
