# RFD 0010: Testing and Validation Strategy

## Summary

This RFD defines the comprehensive testing and validation strategy for the OpenAPI code generator. The system includes unit testing, integration testing, golden file testing, AST validation, transform pass testing, performance benchmarking, and test fixtures with examples.

## Motivation

### Why Comprehensive Testing?

1. **Reliability**: Ensure the code generator produces correct, compilable code
2. **Regression Prevention**: Catch bugs before they reach users
3. **Refactoring Safety**: Enable confident refactoring and improvements
4. **Documentation**: Tests serve as living documentation of expected behavior
5. **Quality Assurance**: Maintain high code quality standards

### Design Goals

- **Coverage**: Comprehensive test coverage across all components
- **Maintainability**: Easy to write, maintain, and update tests
- **Performance**: Fast test execution for rapid feedback
- **Reliability**: Tests should be deterministic and stable
- **Documentation**: Tests should clearly document expected behavior

## Testing Architecture

### Test Organization

```
tests/
├── unit/
│   ├── parser/
│   ├── transforms/
│   ├── generators/
│   └── emitters/
├── integration/
│   ├── full_pipeline/
│   ├── plugin_system/
│   └── error_handling/
├── golden/
│   ├── typescript/
│   ├── rust/
│   └── python/
├── performance/
│   ├── benchmarks/
│   └── stress_tests/
└── fixtures/
    ├── valid_specs/
    ├── invalid_specs/
    └── edge_cases/
```

### Test Framework

```rust
// tests/common/mod.rs
use openapi_generator_core::*;
use std::path::PathBuf;

pub struct TestContext {
    pub fixtures_dir: PathBuf,
    pub output_dir: PathBuf,
    pub temp_dir: tempfile::TempDir,
}

impl TestContext {
    pub fn new() -> Self {
        let temp_dir = tempfile::tempdir().unwrap();
        Self {
            fixtures_dir: PathBuf::from("tests/fixtures"),
            output_dir: temp_dir.path().to_path_buf(),
            temp_dir,
        }
    }
    
    pub fn load_fixture(&self, name: &str) -> String {
        let path = self.fixtures_dir.join(name);
        std::fs::read_to_string(path).unwrap()
    }
    
    pub fn load_openapi_fixture(&self, name: &str) -> OpenApi {
        let content = self.load_fixture(name);
        serde_yaml::from_str(&content).unwrap()
    }
}
```

## Unit Testing Approach

### Parser Unit Tests

```rust
// tests/unit/parser/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::TestContext;
    
    #[test]
    fn test_parse_valid_openapi() {
        let context = TestContext::new();
        let parser = OpenApiParser::new();
        let content = context.load_fixture("valid_specs/petstore.yaml");
        
        let result = parser.parse_content(&content);
        assert!(result.is_ok());
        
        let openapi = result.unwrap();
        assert_eq!(openapi.info.title, "Petstore API");
        assert!(openapi.paths.len() > 0);
    }
    
    #[test]
    fn test_parse_invalid_openapi() {
        let context = TestContext::new();
        let parser = OpenApiParser::new();
        let content = context.load_fixture("invalid_specs/missing_title.yaml");
        
        let result = parser.parse_content(&content);
        assert!(result.is_err());
        
        if let Err(Error::Parse { source: ParseError::MissingRequiredField { field, .. } }) = result {
            assert_eq!(field, "title");
        } else {
            panic!("Expected MissingRequiredField error");
        }
    }
    
    #[test]
    fn test_parse_circular_reference() {
        let context = TestContext::new();
        let parser = OpenApiParser::new();
        let content = context.load_fixture("invalid_specs/circular_ref.yaml");
        
        let result = parser.parse_content(&content);
        assert!(result.is_err());
        
        if let Err(Error::Parse { source: ParseError::CircularReference { .. } }) = result {
            // Expected error
        } else {
            panic!("Expected CircularReference error");
        }
    }
}
```

### Transform Pass Unit Tests

```rust
// tests/unit/transforms/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    use openapi_generator_transforms::*;
    
    #[test]
    fn test_reference_resolution_pass() {
        let mut openapi = create_test_openapi_with_refs();
        let pass = ReferenceResolutionPass::new();
        
        let result = pass.transform(&mut openapi);
        assert!(result.is_ok());
        
        // Verify references are resolved
        assert!(openapi.components.as_ref().unwrap().schemas.is_empty());
    }
    
    #[test]
    fn test_schema_normalization_pass() {
        let mut openapi = create_test_openapi_with_unnormalized_schemas();
        let pass = SchemaNormalizationPass::new();
        
        let result = pass.transform(&mut openapi);
        assert!(result.is_ok());
        
        // Verify schemas are normalized
        // Add specific assertions based on normalization rules
    }
    
    #[test]
    fn test_pass_dependency_resolution() {
        let pipeline = TransformPipeline::new()
            .add_openapi_pass(ReferenceResolutionPass::new())
            .add_openapi_pass(SchemaNormalizationPass::new());
        
        // Test that passes are executed in correct order
        let mut openapi = create_test_openapi();
        let result = pipeline.execute(openapi, "typescript");
        assert!(result.is_ok());
    }
}
```

### Generator Unit Tests

```rust
// tests/unit/generators/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    use openapi_generator_typescript::*;
    
    #[test]
    fn test_typescript_interface_generation() {
        let generator = TypeScriptGenerator::new();
        let openapi = create_test_openapi_with_schemas();
        
        let result = generator.generate_ast(&openapi);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert!(!ast.is_empty());
        
        // Verify specific interface generation
        let interfaces: Vec<_> = ast.iter()
            .filter_map(|node| match node {
                TsNode::Interface(interface) => Some(interface),
                _ => None,
            })
            .collect();
        
        assert!(!interfaces.is_empty());
        assert!(interfaces.iter().any(|i| i.name == "User"));
    }
    
    #[test]
    fn test_typescript_type_mapping() {
        let mapper = TypeScriptTypeMapper::new();
        let schema = create_string_schema();
        
        let result = mapper.map_schema(&schema);
        assert!(result.is_ok());
        
        if let Ok(TsTypeExpression::Primitive(TsPrimitiveType::String)) = result {
            // Expected result
        } else {
            panic!("Expected string type");
        }
    }
    
    #[test]
    fn test_rust_struct_generation() {
        let generator = RustGenerator::new();
        let openapi = create_test_openapi_with_schemas();
        
        let result = generator.generate_ast(&openapi);
        assert!(result.is_ok());
        
        let ast = result.unwrap();
        assert!(!ast.is_empty());
        
        // Verify specific struct generation
        let structs: Vec<_> = ast.iter()
            .filter_map(|node| match node {
                RustNode::Struct(struct_def) => Some(struct_def),
                _ => None,
            })
            .collect();
        
        assert!(!structs.is_empty());
        assert!(structs.iter().any(|s| s.name == "User"));
    }
}
```

## Integration Testing

### Full Pipeline Testing

```rust
// tests/integration/full_pipeline/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    use openapi_generator::*;
    
    #[test]
    fn test_full_typescript_generation() {
        let context = TestContext::new();
        let openapi = context.load_openapi_fixture("valid_specs/petstore.yaml");
        
        let config = GeneratorConfig::new()
            .languages(vec!["typescript".to_string()])
            .output_dir(context.output_dir.clone());
        
        let generator = OpenApiGenerator::new(config);
        let result = generator.generate(&openapi);
        assert!(result.is_ok());
        
        // Verify generated files exist
        let models_file = context.output_dir.join("models.ts");
        let api_file = context.output_dir.join("api.ts");
        let client_file = context.output_dir.join("client.ts");
        
        assert!(models_file.exists());
        assert!(api_file.exists());
        assert!(client_file.exists());
        
        // Verify generated code compiles
        let models_content = std::fs::read_to_string(&models_file).unwrap();
        assert!(models_content.contains("interface"));
        assert!(models_content.contains("export"));
    }
    
    #[test]
    fn test_full_rust_generation() {
        let context = TestContext::new();
        let openapi = context.load_openapi_fixture("valid_specs/petstore.yaml");
        
        let config = GeneratorConfig::new()
            .languages(vec!["rust".to_string()])
            .output_dir(context.output_dir.clone());
        
        let generator = OpenApiGenerator::new(config);
        let result = generator.generate(&openapi);
        assert!(result.is_ok());
        
        // Verify generated files exist
        let lib_file = context.output_dir.join("lib.rs");
        let models_file = context.output_dir.join("models.rs");
        let api_file = context.output_dir.join("api.rs");
        
        assert!(lib_file.exists());
        assert!(models_file.exists());
        assert!(api_file.exists());
        
        // Verify generated code compiles
        let models_content = std::fs::read_to_string(&models_file).unwrap();
        assert!(models_content.contains("struct"));
        assert!(models_content.contains("pub"));
    }
    
    #[test]
    fn test_multi_language_generation() {
        let context = TestContext::new();
        let openapi = context.load_openapi_fixture("valid_specs/petstore.yaml");
        
        let config = GeneratorConfig::new()
            .languages(vec!["typescript".to_string(), "rust".to_string()])
            .output_dir(context.output_dir.clone())
            .create_subdirs(true);
        
        let generator = OpenApiGenerator::new(config);
        let result = generator.generate(&openapi);
        assert!(result.is_ok());
        
        // Verify language-specific directories exist
        let ts_dir = context.output_dir.join("typescript");
        let rust_dir = context.output_dir.join("rust");
        
        assert!(ts_dir.exists());
        assert!(rust_dir.exists());
        
        // Verify files in each directory
        assert!(ts_dir.join("models.ts").exists());
        assert!(rust_dir.join("models.rs").exists());
    }
}
```

### Plugin System Integration Tests

```rust
// tests/integration/plugin_system/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    use openapi_generator_plugin::*;
    
    #[test]
    fn test_plugin_registration() {
        let mut registry = PluginRegistry::new();
        let plugin = TestPlugin::new();
        
        let result = registry.register_plugin(plugin);
        assert!(result.is_ok());
        
        let registered_plugin = registry.get_plugin("test-plugin");
        assert!(registered_plugin.is_some());
    }
    
    #[test]
    fn test_plugin_dependency_resolution() {
        let mut registry = PluginRegistry::new();
        
        // Register dependency first
        let dependency = DependencyPlugin::new();
        registry.register_plugin(dependency).unwrap();
        
        // Register plugin that depends on it
        let plugin = DependentPlugin::new();
        let result = registry.register_plugin(plugin);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_plugin_language_generation() {
        let mut registry = PluginRegistry::new();
        let plugin = TestLanguagePlugin::new();
        registry.register_plugin(plugin).unwrap();
        
        let generator = registry.get_language_generator("test-language");
        assert!(generator.is_some());
        
        let openapi = create_test_openapi();
        let result = generator.unwrap().generate_ast(&openapi);
        assert!(result.is_ok());
    }
}
```

## Golden File Testing

### Golden File Structure

```
tests/golden/
├── typescript/
│   ├── petstore/
│   │   ├── models.ts
│   │   ├── api.ts
│   │   ├── client.ts
│   │   └── index.ts
│   └── github-api/
│       ├── models.ts
│       ├── api.ts
│       └── client.ts
├── rust/
│   ├── petstore/
│   │   ├── lib.rs
│   │   ├── models.rs
│   │   ├── api.rs
│   │   └── Cargo.toml
│   └── github-api/
│       ├── lib.rs
│       ├── models.rs
│       └── api.rs
└── python/
    ├── petstore/
    │   ├── models.py
    │   ├── api.py
    │   └── client.py
    └── github-api/
        ├── models.py
        ├── api.py
        └── client.py
```

### Golden File Test Framework

```rust
// tests/golden/mod.rs
use std::path::PathBuf;
use std::fs;

pub struct GoldenFileTester {
    golden_dir: PathBuf,
    output_dir: PathBuf,
}

impl GoldenFileTester {
    pub fn new() -> Self {
        Self {
            golden_dir: PathBuf::from("tests/golden"),
            output_dir: tempfile::tempdir().unwrap().path().to_path_buf(),
        }
    }
    
    pub fn test_generation(&self, spec_name: &str, language: &str) -> Result<(), GoldenFileError> {
        let openapi = self.load_spec(spec_name)?;
        let generated_files = self.generate_files(&openapi, language)?;
        let golden_files = self.load_golden_files(spec_name, language)?;
        
        self.compare_files(generated_files, golden_files)
    }
    
    fn load_spec(&self, spec_name: &str) -> Result<OpenApi, GoldenFileError> {
        let spec_path = self.golden_dir.join("specs").join(format!("{}.yaml", spec_name));
        let content = fs::read_to_string(spec_path)?;
        let openapi = serde_yaml::from_str(&content)?;
        Ok(openapi)
    }
    
    fn generate_files(&self, openapi: &OpenApi, language: &str) -> Result<Vec<GeneratedFile>, GoldenFileError> {
        let config = GeneratorConfig::new()
            .languages(vec![language.to_string()])
            .output_dir(self.output_dir.clone());
        
        let generator = OpenApiGenerator::new(config);
        let result = generator.generate(openapi)?;
        Ok(result)
    }
    
    fn load_golden_files(&self, spec_name: &str, language: &str) -> Result<Vec<GoldenFile>, GoldenFileError> {
        let golden_dir = self.golden_dir.join(language).join(spec_name);
        let mut files = Vec::new();
        
        for entry in fs::read_dir(golden_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                let content = fs::read_to_string(&path)?;
                files.push(GoldenFile {
                    name: path.file_name().unwrap().to_string_lossy().to_string(),
                    content,
                });
            }
        }
        
        Ok(files)
    }
    
    fn compare_files(&self, generated: Vec<GeneratedFile>, golden: Vec<GoldenFile>) -> Result<(), GoldenFileError> {
        if generated.len() != golden.len() {
            return Err(GoldenFileError::FileCountMismatch {
                generated: generated.len(),
                golden: golden.len(),
            });
        }
        
        for (gen_file, gold_file) in generated.iter().zip(golden.iter()) {
            if gen_file.name != gold_file.name {
                return Err(GoldenFileError::FileNameMismatch {
                    generated: gen_file.name.clone(),
                    golden: gold_file.name.clone(),
                });
            }
            
            if gen_file.content != gold_file.content {
                return Err(GoldenFileError::FileContentMismatch {
                    file: gen_file.name.clone(),
                    generated: gen_file.content.clone(),
                    golden: gold_file.content.clone(),
                });
            }
        }
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct GoldenFile {
    pub name: String,
    pub content: String,
}

#[derive(Debug, Snafu)]
pub enum GoldenFileError {
    #[snafu(display("File count mismatch: generated {}, golden {}", generated, golden))]
    FileCountMismatch { generated: usize, golden: usize },
    
    #[snafu(display("File name mismatch: generated '{}', golden '{}'", generated, golden))]
    FileNameMismatch { generated: String, golden: String },
    
    #[snafu(display("File content mismatch in '{}'", file))]
    FileContentMismatch { file: String, generated: String, golden: String },
    
    #[snafu(display("IO error: {}", source))]
    Io { source: std::io::Error },
    
    #[snafu(display("Parse error: {}", source))]
    Parse { source: serde_yaml::Error },
}
```

### Golden File Test Cases

```rust
// tests/golden/typescript/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_petstore_typescript_generation() {
        let tester = GoldenFileTester::new();
        let result = tester.test_generation("petstore", "typescript");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_github_api_typescript_generation() {
        let tester = GoldenFileTester::new();
        let result = tester.test_generation("github-api", "typescript");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_complex_spec_typescript_generation() {
        let tester = GoldenFileTester::new();
        let result = tester.test_generation("complex-spec", "typescript");
        assert!(result.is_ok());
    }
}

// tests/golden/rust/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_petstore_rust_generation() {
        let tester = GoldenFileTester::new();
        let result = tester.test_generation("petstore", "rust");
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_github_api_rust_generation() {
        let tester = GoldenFileTester::new();
        let result = tester.test_generation("github-api", "rust");
        assert!(result.is_ok());
    }
}
```

## AST Validation Tests

### AST Validation Framework

```rust
// tests/validation/ast/mod.rs
use openapi_generator_core::*;

pub struct AstValidator {
    rules: Vec<Box<dyn AstValidationRule>>,
}

impl AstValidator {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
        }
    }
    
    pub fn add_rule<R: AstValidationRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }
    
    pub fn validate<T>(&self, ast: &T) -> Result<(), AstValidationError> {
        for rule in &self.rules {
            rule.validate(ast)?;
        }
        Ok(())
    }
}

pub trait AstValidationRule {
    fn validate<T>(&self, ast: &T) -> Result<(), AstValidationError>;
    fn name(&self) -> &str;
}

pub struct TypeScriptAstValidator {
    rules: Vec<Box<dyn AstValidationRule>>,
}

impl TypeScriptAstValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            rules: Vec::new(),
        };
        
        validator.add_rule(UniqueNameRule::new());
        validator.add_rule(ValidTypeExpressionRule::new());
        validator.add_rule(ValidImportRule::new());
        validator.add_rule(ValidExportRule::new());
        
        validator
    }
    
    pub fn add_rule<R: AstValidationRule + 'static>(&mut self, rule: R) {
        self.rules.push(Box::new(rule));
    }
    
    pub fn validate(&self, ast: &[TsNode]) -> Result<(), AstValidationError> {
        for rule in &self.rules {
            rule.validate(ast)?;
        }
        Ok(())
    }
}

// Example validation rules
pub struct UniqueNameRule;

impl AstValidationRule for UniqueNameRule {
    fn validate<T>(&self, ast: &T) -> Result<(), AstValidationError> {
        // Implement unique name validation
        Ok(())
    }
    
    fn name(&self) -> &str { "unique-name" }
}

pub struct ValidTypeExpressionRule;

impl AstValidationRule for ValidTypeExpressionRule {
    fn validate<T>(&self, ast: &T) -> Result<(), AstValidationError> {
        // Implement type expression validation
        Ok(())
    }
    
    fn name(&self) -> &str { "valid-type-expression" }
}
```

### AST Validation Test Cases

```rust
// tests/validation/ast/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_typescript_ast_validation() {
        let validator = TypeScriptAstValidator::new();
        let ast = create_test_typescript_ast();
        
        let result = validator.validate(&ast);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_rust_ast_validation() {
        let validator = RustAstValidator::new();
        let ast = create_test_rust_ast();
        
        let result = validator.validate(&ast);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_ast_validation_failure() {
        let validator = TypeScriptAstValidator::new();
        let ast = create_invalid_typescript_ast();
        
        let result = validator.validate(&ast);
        assert!(result.is_err());
        
        if let Err(AstValidationError::ValidationFailed { rule, message }) = result {
            assert_eq!(rule, "unique-name");
            assert!(message.contains("duplicate"));
        } else {
            panic!("Expected validation failure");
        }
    }
}
```

## Transform Pass Testing

### Transform Pass Test Framework

```rust
// tests/transforms/mod.rs
use openapi_generator_transforms::*;

pub struct TransformPassTester {
    openapi: OpenApi,
    expected_result: Option<OpenApi>,
}

impl TransformPassTester {
    pub fn new(openapi: OpenApi) -> Self {
        Self {
            openapi,
            expected_result: None,
        }
    }
    
    pub fn with_expected_result(mut self, expected: OpenApi) -> Self {
        self.expected_result = Some(expected);
        self
    }
    
    pub fn test_pass<P: OpenApiTransformPass>(&self, pass: P) -> Result<(), TransformTestError> {
        let mut openapi = self.openapi.clone();
        let result = pass.transform(&mut openapi);
        
        if result.is_err() {
            return Err(TransformTestError::PassFailed {
                pass: pass.name().to_string(),
                error: result.unwrap_err().to_string(),
            });
        }
        
        if let Some(expected) = &self.expected_result {
            if !self.compare_openapi(&openapi, expected) {
                return Err(TransformTestError::ResultMismatch {
                    pass: pass.name().to_string(),
                });
            }
        }
        
        Ok(())
    }
    
    fn compare_openapi(&self, actual: &OpenApi, expected: &OpenApi) -> bool {
        // Implement OpenAPI comparison logic
        actual.info.title == expected.info.title
    }
}

#[derive(Debug, Snafu)]
pub enum TransformTestError {
    #[snafu(display("Transform pass '{}' failed: {}", pass, error))]
    PassFailed { pass: String, error: String },
    
    #[snafu(display("Transform result mismatch for pass '{}'", pass))]
    ResultMismatch { pass: String },
}
```

### Transform Pass Test Cases

```rust
// tests/transforms/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reference_resolution_pass() {
        let openapi = create_openapi_with_refs();
        let expected = create_openapi_with_resolved_refs();
        
        let tester = TransformPassTester::new(openapi)
            .with_expected_result(expected);
        
        let pass = ReferenceResolutionPass::new();
        let result = tester.test_pass(pass);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_schema_normalization_pass() {
        let openapi = create_openapi_with_unnormalized_schemas();
        let expected = create_openapi_with_normalized_schemas();
        
        let tester = TransformPassTester::new(openapi)
            .with_expected_result(expected);
        
        let pass = SchemaNormalizationPass::new();
        let result = tester.test_pass(pass);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_pass_chain() {
        let openapi = create_openapi_with_refs_and_unnormalized_schemas();
        let expected = create_openapi_with_resolved_and_normalized_schemas();
        
        let tester = TransformPassTester::new(openapi)
            .with_expected_result(expected);
        
        let pass1 = ReferenceResolutionPass::new();
        let pass2 = SchemaNormalizationPass::new();
        
        let result1 = tester.test_pass(pass1);
        assert!(result1.is_ok());
        
        let result2 = tester.test_pass(pass2);
        assert!(result2.is_ok());
    }
}
```

## Performance Benchmarking

### Benchmark Framework

```rust
// tests/performance/benchmarks/mod.rs
use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use openapi_generator::*;

pub fn benchmark_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("parser");
    
    for spec in &["petstore", "github-api", "complex-spec"] {
        let openapi = load_test_spec(spec);
        let content = serde_yaml::to_string(&openapi).unwrap();
        
        group.bench_with_input(BenchmarkId::new("parse", spec), &content, |b, content| {
            b.iter(|| {
                let parser = OpenApiParser::new();
                parser.parse_content(content)
            })
        });
    }
    
    group.finish();
}

pub fn benchmark_generator(c: &mut Criterion) {
    let mut group = c.benchmark_group("generator");
    
    for language in &["typescript", "rust", "python"] {
        for spec in &["petstore", "github-api", "complex-spec"] {
            let openapi = load_test_spec(spec);
            
            group.bench_with_input(
                BenchmarkId::new("generate", format!("{}-{}", language, spec)),
                &openapi,
                |b, openapi| {
                    b.iter(|| {
                        let config = GeneratorConfig::new()
                            .languages(vec![language.to_string()]);
                        let generator = OpenApiGenerator::new(config);
                        generator.generate(openapi)
                    })
                }
            );
        }
    }
    
    group.finish();
}

pub fn benchmark_transform_passes(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform_passes");
    
    for spec in &["petstore", "github-api", "complex-spec"] {
        let openapi = load_test_spec(spec);
        
        group.bench_with_input(BenchmarkId::new("reference_resolution", spec), &openapi, |b, openapi| {
            b.iter(|| {
                let mut openapi = openapi.clone();
                let pass = ReferenceResolutionPass::new();
                pass.transform(&mut openapi)
            })
        });
        
        group.bench_with_input(BenchmarkId::new("schema_normalization", spec), &openapi, |b, openapi| {
            b.iter(|| {
                let mut openapi = openapi.clone();
                let pass = SchemaNormalizationPass::new();
                pass.transform(&mut openapi)
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_parser, benchmark_generator, benchmark_transform_passes);
criterion_main!(benches);
```

### Stress Testing

```rust
// tests/performance/stress/mod.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_large_spec_parsing() {
        let large_spec = create_large_openapi_spec(1000); // 1000 schemas
        let parser = OpenApiParser::new();
        
        let start = std::time::Instant::now();
        let result = parser.parse_content(&serde_yaml::to_string(&large_spec).unwrap());
        let duration = start.elapsed();
        
        assert!(result.is_ok());
        assert!(duration.as_secs() < 10); // Should complete within 10 seconds
    }
    
    #[test]
    fn test_memory_usage() {
        let large_spec = create_large_openapi_spec(5000); // 5000 schemas
        let parser = OpenApiParser::new();
        
        let result = parser.parse_content(&serde_yaml::to_string(&large_spec).unwrap());
        assert!(result.is_ok());
        
        // Check memory usage doesn't exceed reasonable limits
        // This would require integration with a memory profiler
    }
    
    #[test]
    fn test_concurrent_generation() {
        use std::thread;
        use std::sync::Arc;
        
        let openapi = Arc::new(load_test_spec("petstore"));
        let mut handles = Vec::new();
        
        for i in 0..10 {
            let openapi = Arc::clone(&openapi);
            let handle = thread::spawn(move || {
                let config = GeneratorConfig::new()
                    .languages(vec!["typescript".to_string()]);
                let generator = OpenApiGenerator::new(config);
                generator.generate(&openapi)
            });
            handles.push(handle);
        }
        
        for handle in handles {
            let result = handle.join().unwrap();
            assert!(result.is_ok());
        }
    }
}
```

## Test Fixtures and Examples

### Test Fixture Structure

```
tests/fixtures/
├── valid_specs/
│   ├── petstore.yaml
│   ├── github-api.yaml
│   ├── complex-spec.yaml
│   └── minimal-spec.yaml
├── invalid_specs/
│   ├── missing-title.yaml
│   ├── circular-refs.yaml
│   ├── broken-refs.yaml
│   └── invalid-schema.yaml
├── edge_cases/
│   ├── empty-spec.yaml
│   ├── no-paths.yaml
│   ├── no-schemas.yaml
│   └── external-refs.yaml
└── stress_tests/
    ├── large-spec.yaml
    ├── deep-nesting.yaml
    └── many-refs.yaml
```

### Test Fixture Loading

```rust
// tests/fixtures/mod.rs
use std::path::PathBuf;
use std::collections::HashMap;

pub struct TestFixtures {
    fixtures_dir: PathBuf,
    cache: HashMap<String, OpenApi>,
}

impl TestFixtures {
    pub fn new() -> Self {
        Self {
            fixtures_dir: PathBuf::from("tests/fixtures"),
            cache: HashMap::new(),
        }
    }
    
    pub fn load_spec(&mut self, name: &str) -> OpenApi {
        if let Some(cached) = self.cache.get(name) {
            return cached.clone();
        }
        
        let path = self.fixtures_dir.join("valid_specs").join(format!("{}.yaml", name));
        let content = std::fs::read_to_string(path).unwrap();
        let openapi = serde_yaml::from_str(&content).unwrap();
        
        self.cache.insert(name.to_string(), openapi.clone());
        openapi
    }
    
    pub fn load_invalid_spec(&self, name: &str) -> String {
        let path = self.fixtures_dir.join("invalid_specs").join(format!("{}.yaml", name));
        std::fs::read_to_string(path).unwrap()
    }
    
    pub fn load_edge_case_spec(&mut self, name: &str) -> OpenApi {
        let path = self.fixtures_dir.join("edge_cases").join(format!("{}.yaml", name));
        let content = std::fs::read_to_string(path).unwrap();
        serde_yaml::from_str(&content).unwrap()
    }
}
```

## Workflow Management with Justfile

### Justfile Configuration

The project uses `just` (a command runner) to manage common workflows and tasks:

```justfile
# justfile
# OpenAPI Generator Workflow Management

# Default recipe - show available commands
default:
    @just --list

# Development workflow
dev: install
    @echo "Starting development environment..."
    cargo watch -x "check --all"

# Run all tests
test:
    @echo "Running all tests..."
    cargo test --all
    cargo test --test golden
    cargo test --test integration
    cargo test --test performance

# Run tests with coverage
test-coverage:
    @echo "Running tests with coverage..."
    cargo test --all --no-run
    cargo tarpaulin --all --out Html --output-dir coverage

# Run benchmarks
bench:
    @echo "Running benchmarks..."
    cargo bench --all

# Generate code from test specs
generate-test:
    @echo "Generating code from test specifications..."
    cargo run --bin openapi-generator -- --input tests/fixtures/valid_specs/petstore.yaml --output test-output --languages typescript rust

# Format generated code
format-generated:
    @echo "Formatting generated code..."
    prettier --write test-output/typescript/**/*.ts
    rustfmt test-output/rust/**/*.rs

# Validate generated code
validate-generated:
    @echo "Validating generated TypeScript..."
    cd test-output/typescript && npm install && npm run build
    @echo "Validating generated Rust..."
    cd test-output/rust && cargo check

# Run golden file tests
golden:
    @echo "Running golden file tests..."
    cargo test --test golden

# Update golden files
update-golden:
    @echo "Updating golden files..."
    cargo run --bin openapi-generator -- --input tests/fixtures/valid_specs/petstore.yaml --output tests/golden/typescript/petstore --languages typescript
    cargo run --bin openapi-generator -- --input tests/fixtures/valid_specs/petstore.yaml --output tests/golden/rust/petstore --languages rust
    @echo "Golden files updated. Review changes before committing."

# Run integration tests
integration:
    @echo "Running integration tests..."
    cargo test --test integration

# Run performance tests
perf:
    @echo "Running performance tests..."
    cargo test --test performance

# Run linting
lint:
    @echo "Running lints..."
    cargo clippy --all -- -D warnings
    cargo fmt --all -- --check

# Fix linting issues
lint-fix:
    @echo "Fixing linting issues..."
    cargo clippy --all --fix --allow-dirty
    cargo fmt --all

# Build all crates
build:
    @echo "Building all crates..."
    cargo build --all

# Build release
build-release:
    @echo "Building release..."
    cargo build --all --release

# Clean build artifacts
clean:
    @echo "Cleaning build artifacts..."
    cargo clean
    rm -rf test-output

# Run documentation generation
docs:
    @echo "Generating documentation..."
    cargo doc --all --no-deps --open

# Run security audit
audit:
    @echo "Running security audit..."
    cargo audit

# Check for unused dependencies
unused-deps:
    @echo "Checking for unused dependencies..."
    cargo install cargo-udeps
    cargo +nightly udeps

# Run full CI pipeline locally
ci: lint test bench audit
    @echo "Full CI pipeline completed successfully!"

# Generate example from petstore
example-petstore:
    @echo "Generating example from petstore spec..."
    cargo run --bin openapi-generator -- --input petstore-api.yaml --output examples/petstore-generated --languages typescript rust
    @echo "Example generated in examples/petstore-generated/"

# Generate example from GitHub API
example-github:
    @echo "Generating example from GitHub API spec..."
    cargo run --bin openapi-generator -- --input github-api.yaml --output examples/github-generated --languages typescript rust
    @echo "Example generated in examples/github-generated/"

# Run all examples
examples: example-petstore example-github
    @echo "All examples generated!"

# Plugin development workflow
plugin-dev:
    @echo "Setting up plugin development environment..."
    cargo build --package openapi-generator-plugin
    @echo "Plugin development environment ready!"

# Create new language generator
new-language language:
    @echo "Creating new language generator for {{language}}..."
    cargo new --lib openapi-generator-{{language}}
    @echo "New language generator created: openapi-generator-{{language}}"

# Run specific test file
test-file file:
    @echo "Running test file: {{file}}"
    cargo test --test {{file}}

# Run tests matching pattern
test-pattern pattern:
    @echo "Running tests matching pattern: {{pattern}}"
    cargo test --all -- {{pattern}}

# Profile build performance
profile-build:
    @echo "Profiling build performance..."
    cargo build --timings

# Check for breaking changes
check-breaking:
    @echo "Checking for breaking changes..."
    cargo install cargo-breaking
    cargo breaking

# Update dependencies
update-deps:
    @echo "Updating dependencies..."
    cargo update

# Check dependency vulnerabilities
check-vulns:
    @echo "Checking for vulnerabilities..."
    cargo audit

# Run memory usage analysis
mem-usage:
    @echo "Running memory usage analysis..."
    cargo install cargo-valgrind
    cargo valgrind test --all

# Generate test report
test-report:
    @echo "Generating test report..."
    cargo test --all -- --nocapture > test-report.txt
    @echo "Test report saved to test-report.txt"

# Setup development environment
setup: install
    @echo "Setting up development environment..."
    cargo build --all
    cargo test --all
    @echo "Development environment ready!"

# Help
help:
    @echo "Available commands:"
    @just --list
```

### Workflow Usage Examples

```bash
# Setup development environment
just setup

# Run all tests
just test

# Run specific test pattern
just test-pattern "parser"

# Generate and validate test code
just generate-test
just format-generated
just validate-generated

# Update golden files
just update-golden

# Run full CI pipeline locally
just ci

# Generate examples
just examples

# Create new language generator
just new-language python

# Profile build performance
just profile-build
```

## Continuous Integration

### CI Configuration

```yaml
# .github/workflows/test.yml
name: Test

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    strategy:
      matrix:
        rust: [stable, beta, nightly]
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: ${{ matrix.rust }}
        override: true
    
    - name: Install just
      run: cargo install just
    
    - name: Setup development environment
      run: just setup
    
    - name: Run full CI pipeline
      run: just ci
    
    - name: Generate test report
      run: just test-report
    
    - name: Upload test report
      uses: actions/upload-artifact@v3
      with:
        name: test-report
        path: test-report.txt
```

## Conclusion

The comprehensive testing and validation strategy provides a robust foundation for ensuring the reliability and correctness of the OpenAPI code generator. The multi-layered approach covers unit testing, integration testing, golden file testing, AST validation, transform pass testing, and performance benchmarking.

The test fixtures and examples make it easy to add new test cases, while the CI integration ensures that all tests are run automatically. The performance benchmarking helps identify and prevent performance regressions.

## Related RFDs

- [RFD 0001: Overall Architecture and Design Philosophy](./0001-architecture-overview.md)
- [RFD 0003: Language-Specific AST Design](./0003-language-ast-design.md)
- [RFD 0004: Multi-Level Transformation Passes](./0004-transformation-passes.md)
- [RFD 0009: Error Handling and Diagnostics](./0009-error-handling.md)
