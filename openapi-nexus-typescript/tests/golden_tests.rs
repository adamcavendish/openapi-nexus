//! Golden file tests for TypeScript code generation
//!
//! These tests compare generated TypeScript code against known-good golden files.
//! To update golden files after intentional changes, run:
//!   UPDATE_GOLDEN=1 cargo test --test golden_tests

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
use std::process;

use similar::TextDiff;
use tracing::{error, info};
use utoipa::openapi::OpenApi;

use openapi_nexus_core::traits::file_writer::FileWriter;
use openapi_nexus_typescript::TypeScriptGenerator;

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
fn generate_typescript_files(spec_content: &str) -> Result<HashMap<String, String>, Box<dyn std::error::Error + Send + Sync>> {
    let openapi: OpenApi = serde_norway::from_str(spec_content)?;
    let generator = TypeScriptGenerator::new();
    let generated_files = match generator.generate_files(&openapi) {
        Ok(files) => {
            info!("Successfully generated {} files", files.len());
            files
        },
        Err(e) => {
            error!("Error generating files: {}", e);
            return Err(Box::new(e));
        }
    };

    // Create a unique temporary directory to write files with proper directory structure
    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let temp_dir = std::env::temp_dir().join(format!(
        "openapi_nexus_test_{}_{}",
        std::process::id(),
        timestamp
    ));
    fs::create_dir_all(&temp_dir).unwrap();

    // Use the FileWriter trait to write files with proper directory organization
    if let Err(e) = generator.write_files(&temp_dir, &generated_files) {
        error!("Error writing files: {}", e);
        error!("Temp directory: {}", temp_dir.display());
        error!("Generated files count: {}", generated_files.len());
        for (i, file) in generated_files.iter().enumerate() {
            error!("  File {}: {} (category: {:?})", i, file.filename, file.category);
        }
        return Err(e);
    }

    // Read all files recursively from the temporary directory
    let mut result = HashMap::new();
    read_directory_recursive(&temp_dir, &temp_dir, &mut result);

    // Clean up temporary directory
    fs::remove_dir_all(&temp_dir).unwrap();

    Ok(result)
}

/// Recursively read all files from a directory
fn read_directory_recursive(
    base_dir: &Path,
    current_dir: &Path,
    result: &mut HashMap<String, String>,
) {
    for entry in fs::read_dir(current_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        if path.is_dir() {
            read_directory_recursive(base_dir, &path, result);
        } else if path.is_file() {
            let relative_path = path.strip_prefix(base_dir).unwrap();
            // Normalize path separators to forward slashes for consistency
            let filename = relative_path.to_string_lossy().replace('\\', "/");
            let content = fs::read_to_string(&path).unwrap();
            result.insert(filename, content);
        }
    }
}

/// Update or compare golden files for a given spec
fn test_golden_files(spec_name: &str, fixture_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let spec_content = read_fixture(fixture_path);
    let generated = match generate_typescript_files(&spec_content) {
        Ok(files) => files,
        Err(e) => {
            error!("Failed to generate TypeScript files for {}: {}", spec_name, e);
            return Err(e);
        }
    };
    let update_mode = env::var("UPDATE_GOLDEN").is_ok();

    if update_mode {
        update_golden_files(spec_name, &generated)?;
    } else {
        compare_with_golden_files(spec_name, &generated)?;
    }
    
    Ok(())
}

/// Update golden files with generated content
fn update_golden_files(spec_name: &str, generated: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    info!("UPDATE_GOLDEN mode: updating golden files for {}", spec_name);
    let golden_dir = get_golden_dir().join(spec_name);

    // Clean up existing files before updating
    if golden_dir.exists() {
        info!("Cleaning up existing files in: {}", golden_dir.display());
        fs::remove_dir_all(&golden_dir)?;
    }

    fs::create_dir_all(&golden_dir)?;

    for (filename, content) in generated {
        let file_path = golden_dir.join(filename);

        // Create parent directories if they don't exist
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)?;
        info!("Updated: {}", file_path.display());
    }
    info!("Updated golden files for {}", spec_name);
    Ok(())
}

/// Compare generated files with golden files and report differences
fn compare_with_golden_files(spec_name: &str, generated: &HashMap<String, String>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let golden_dir = get_golden_dir().join(spec_name);

    // Recursively compare directories
    compare_directories_recursive(&golden_dir, &golden_dir, generated, spec_name)?;

    info!("Golden file test passed for {}", spec_name);
    Ok(())
}

/// Recursively compare directories and files
fn compare_directories_recursive(
    base_dir: &Path,
    current_dir: &Path,
    generated: &HashMap<String, String>,
    spec_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !current_dir.exists() {
        error!("Golden directory not found: {}", current_dir.display());
        return Err(format!("Golden directory not found: {}", current_dir.display()).into());
    }

    // Walk through the golden directory recursively
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively compare subdirectories
            compare_directories_recursive(base_dir, &path, generated, spec_name)?;
        } else if path.is_file() {
            // Compare individual files
            let relative_path = path.strip_prefix(base_dir)?;
            // Normalize path separators to forward slashes for consistency
            let filename = relative_path.to_string_lossy().replace('\\', "/");

            if let Some(generated_content) = generated.get(&filename) {
                let golden_content = fs::read_to_string(&path)?;

                if generated_content != &golden_content {
                    show_diff(spec_name, &filename, &golden_content, generated_content);
                    return Err(format!("Golden file mismatch for {}: {}", spec_name, filename).into());
                }
            } else {
                error!("Generated file not found for golden file: {}", filename);
                return Err(format!("Generated file not found for golden file: {}", filename).into());
            }
        }
    }
    
    Ok(())
}

/// Show a diff when golden files don't match
fn show_diff(spec_name: &str, filename: &str, golden: &str, generated: &str) {
    error!("Content mismatch in: {}/{}", spec_name, filename);
    error!("{}", "=".repeat(80));

    let diff = TextDiff::from_lines(golden, generated);
    error!(
        "{}",
        diff.unified_diff()
            .context_radius(3)
            .header("golden", "generated")
    );

    error!("To update golden files, run:");
    error!("   UPDATE_GOLDEN=1 cargo test --test golden_tests");
}

#[test]
fn test_petstore_golden() {
    if let Err(e) = test_golden_files("petstore", "valid/petstore.yaml") {
        error!("Petstore golden test failed: {}", e);
        process::exit(1);
    }
}

#[test]
fn test_minimal_golden() {
    if let Err(e) = test_golden_files("minimal", "valid/minimal.yaml") {
        error!("Minimal golden test failed: {}", e);
        process::exit(1);
    }
}

#[test]
fn test_comprehensive_schemas_golden() {
    if let Err(e) = test_golden_files("comprehensive-schemas", "valid/comprehensive-schemas.yaml") {
        error!("Comprehensive schemas golden test failed: {}", e);
        process::exit(1);
    }
}
