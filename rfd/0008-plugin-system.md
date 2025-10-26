# RFD 0008: Plugin System and Extensibility

## Summary

This RFD defines the plugin system architecture that enables extensibility and customization of the OpenAPI code generator. The system supports language generator plugins, transform pass plugins, custom type mapping plugins, and provides a robust plugin discovery and registration mechanism.

## Motivation

### Why a Plugin System?

1. **Extensibility**: Allow users to add new languages and features
2. **Customization**: Enable project-specific customizations
3. **Modularity**: Keep core functionality separate from extensions
4. **Community**: Enable community contributions and third-party plugins
5. **Maintainability**: Reduce core complexity by moving specialized features to plugins

### Design Goals

- **Type Safety**: Leverage Rust's type system for plugin safety
- **Performance**: Minimal overhead for plugin system
- **Discoverability**: Easy plugin discovery and registration
- **Isolation**: Plugin failures don't affect core functionality
- **Documentation**: Clear plugin development guidelines

## Plugin System Architecture

### Core Plugin Traits

```rust
// openapi-generator-plugin/src/traits.rs
pub trait Plugin {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    fn description(&self) -> &str;
    fn dependencies(&self) -> Vec<PluginDependency>;
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;
    fn shutdown(&self) -> Result<(), PluginError>;
}

#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

#[derive(Debug, Clone)]
pub struct PluginContext {
    pub config: PluginConfig,
    pub logger: Box<dyn PluginLogger>,
    pub registry: Arc<PluginRegistry>,
}

pub trait PluginLogger {
    fn info(&self, message: &str);
    fn warn(&self, message: &str);
    fn error(&self, message: &str);
    fn debug(&self, message: &str);
}
```

### Language Generator Plugin Trait

```rust
pub trait LanguageGeneratorPlugin: Plugin {
    type AstType;
    type Error;
    
    fn generate_ast(&self, openapi: &OpenApi) -> Result<Vec<Self::AstType>, Self::Error>;
    fn generate_code(&self, ast: &[Self::AstType]) -> Result<Vec<GeneratedFile>, Self::Error>;
    fn get_language_name(&self) -> &str;
    fn get_file_extensions(&self) -> Vec<&str>;
    fn get_config_schema(&self) -> Option<serde_json::Value>;
}

// Example implementation
pub struct PythonGeneratorPlugin {
    config: PythonConfig,
    logger: Box<dyn PluginLogger>,
}

impl Plugin for PythonGeneratorPlugin {
    fn name(&self) -> &str { "python-generator" }
    fn version(&self) -> &str { "1.0.0" }
    fn description(&self) -> &str { "Python code generator plugin" }
    fn dependencies(&self) -> Vec<PluginDependency> { vec![] }
    
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        self.logger = context.logger.clone();
        Ok(())
    }
    
    fn shutdown(&self) -> Result<(), PluginError> {
        Ok(())
    }
}

impl LanguageGeneratorPlugin for PythonGeneratorPlugin {
    type AstType = PythonNode;
    type Error = PythonGeneratorError;
    
    fn generate_ast(&self, openapi: &OpenApi) -> Result<Vec<PythonNode>, PythonGeneratorError> {
        // Generate Python AST from OpenAPI spec
        Ok(vec![])
    }
    
    fn generate_code(&self, ast: &[PythonNode]) -> Result<Vec<GeneratedFile>, PythonGeneratorError> {
        // Generate Python code from AST
        Ok(vec![])
    }
    
    fn get_language_name(&self) -> &str { "python" }
    fn get_file_extensions(&self) -> Vec<&str> { vec!["py"] }
    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(PythonConfig::default()).unwrap())
    }
}
```

### Transform Pass Plugin Trait

```rust
pub trait TransformPassPlugin: Plugin {
    type Error;
    
    fn create_openapi_pass(&self) -> Option<Box<dyn OpenApiTransformPass>>;
    fn create_ir_pass(&self) -> Option<Box<dyn IrTransformPass>>;
    fn create_ast_pass<T>(&self) -> Option<Box<dyn AstTransformPass<T>>>;
    fn get_pass_config_schema(&self) -> Option<serde_json::Value>;
}

// Example implementation
pub struct CustomValidationPlugin {
    config: ValidationConfig,
}

impl TransformPassPlugin for CustomValidationPlugin {
    type Error = ValidationError;
    
    fn create_openapi_pass(&self) -> Option<Box<dyn OpenApiTransformPass>> {
        Some(Box::new(CustomValidationPass::new(self.config.clone())))
    }
    
    fn create_ir_pass(&self) -> Option<Box<dyn IrTransformPass>> {
        Some(Box::new(CustomIrValidationPass::new(self.config.clone())))
    }
    
    fn create_ast_pass<T>(&self) -> Option<Box<dyn AstTransformPass<T>>> {
        Some(Box::new(CustomAstValidationPass::new(self.config.clone())))
    }
    
    fn get_pass_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(ValidationConfig::default()).unwrap())
    }
}
```

### Type Mapping Plugin Trait

```rust
pub trait TypeMappingPlugin: Plugin {
    type LanguageType;
    type Error;
    
    fn create_type_mapper(&self) -> Box<dyn TypeMapper<LanguageType = Self::LanguageType, Error = Self::Error>>;
    fn get_supported_languages(&self) -> Vec<&str>;
    fn get_mapping_rules(&self) -> Vec<TypeMappingRule>;
}

// Example implementation
pub struct CustomTypeMappingPlugin {
    config: TypeMappingConfig,
}

impl TypeMappingPlugin for CustomTypeMappingPlugin {
    type LanguageType = CustomType;
    type Error = TypeMappingError;
    
    fn create_type_mapper(&self) -> Box<dyn TypeMapper<LanguageType = CustomType, Error = TypeMappingError>> {
        Box::new(CustomTypeMapper::new(self.config.clone()))
    }
    
    fn get_supported_languages(&self) -> Vec<&str> {
        vec!["typescript", "rust", "python"]
    }
    
    fn get_mapping_rules(&self) -> Vec<TypeMappingRule> {
        vec![
            TypeMappingRule {
                openapi_type: "string".to_string(),
                custom_type: "CustomString".to_string(),
                conditions: vec!["format:email".to_string()],
            },
        ]
    }
}
```

## Plugin Registry

### Plugin Registry Implementation

```rust
// openapi-generator-plugin/src/registry.rs
pub struct PluginRegistry {
    plugins: HashMap<String, Box<dyn Plugin>>,
    language_generators: HashMap<String, Box<dyn LanguageGeneratorPlugin>>,
    transform_passes: Vec<Box<dyn TransformPassPlugin>>,
    type_mappers: HashMap<String, Box<dyn TypeMappingPlugin>>,
    context: PluginContext,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
            language_generators: HashMap::new(),
            transform_passes: Vec::new(),
            type_mappers: HashMap::new(),
            context: PluginContext::default(),
        }
    }
    
    pub fn register_plugin<P: Plugin + 'static>(&mut self, plugin: P) -> Result<(), PluginError> {
        let name = plugin.name().to_string();
        
        // Check dependencies
        self.check_dependencies(&plugin.dependencies())?;
        
        // Initialize plugin
        let mut plugin_box = Box::new(plugin);
        plugin_box.initialize(&self.context)?;
        
        // Register based on plugin type
        if let Some(lang_plugin) = plugin_box.as_any().downcast_ref::<dyn LanguageGeneratorPlugin>() {
            self.language_generators.insert(name.clone(), lang_plugin.clone());
        }
        
        if let Some(transform_plugin) = plugin_box.as_any().downcast_ref::<dyn TransformPassPlugin>() {
            self.transform_passes.push(transform_plugin.clone());
        }
        
        if let Some(type_plugin) = plugin_box.as_any().downcast_ref::<dyn TypeMappingPlugin>() {
            self.type_mappers.insert(name.clone(), type_plugin.clone());
        }
        
        self.plugins.insert(name, plugin_box);
        Ok(())
    }
    
    pub fn get_language_generator(&self, language: &str) -> Option<&dyn LanguageGeneratorPlugin> {
        self.language_generators.get(language).map(|p| p.as_ref())
    }
    
    pub fn get_transform_passes(&self) -> &[Box<dyn TransformPassPlugin>] {
        &self.transform_passes
    }
    
    pub fn get_type_mapper(&self, name: &str) -> Option<&dyn TypeMappingPlugin> {
        self.type_mappers.get(name).map(|p| p.as_ref())
    }
}
```

### Plugin Discovery

```rust
pub struct PluginDiscovery {
    search_paths: Vec<PathBuf>,
    plugin_patterns: Vec<String>,
}

impl PluginDiscovery {
    pub fn new() -> Self {
        Self {
            search_paths: vec![
                PathBuf::from("./plugins"),
                PathBuf::from("~/.openapi-generator/plugins"),
                PathBuf::from("/usr/local/lib/openapi-generator/plugins"),
            ],
            plugin_patterns: vec![
                "libopenapi_*_plugin.*".to_string(),
                "openapi-*-plugin.*".to_string(),
            ],
        }
    }
    
    pub fn discover_plugins(&self) -> Result<Vec<PluginInfo>, PluginError> {
        let mut plugins = Vec::new();
        
        for search_path in &self.search_paths {
            if search_path.exists() {
                let discovered = self.scan_directory(search_path)?;
                plugins.extend(discovered);
            }
        }
        
        Ok(plugins)
    }
    
    fn scan_directory(&self, path: &Path) -> Result<Vec<PluginInfo>, PluginError> {
        let mut plugins = Vec::new();
        
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if self.is_plugin_file(&path) {
                    let plugin_info = self.load_plugin_info(&path)?;
                    plugins.push(plugin_info);
                }
            } else if path.is_dir() {
                let sub_plugins = self.scan_directory(&path)?;
                plugins.extend(sub_plugins);
            }
        }
        
        Ok(plugins)
    }
    
    fn is_plugin_file(&self, path: &Path) -> bool {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            self.plugin_patterns.iter().any(|pattern| {
                glob::Pattern::new(pattern).unwrap().matches(file_name)
            })
        } else {
            false
        }
    }
}
```

## Plugin Configuration

### Plugin Configuration Schema

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub priority: i32,
    pub parameters: HashMap<String, serde_json::Value>,
    pub dependencies: Vec<PluginDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub name: String,
    pub version: String,
    pub optional: bool,
}

// Example plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonConfig {
    pub indent_size: usize,
    pub use_type_hints: bool,
    pub generate_async_code: bool,
    pub include_pydantic: bool,
    pub custom_imports: Vec<String>,
}
```

### Configuration Loading

```rust
pub struct PluginConfigLoader {
    config_path: PathBuf,
}

impl PluginConfigLoader {
    pub fn load_plugin_configs(&self) -> Result<HashMap<String, PluginConfig>, PluginError> {
        let config_file = self.config_path.join("plugins.json");
        
        if !config_file.exists() {
            return Ok(HashMap::new());
        }
        
        let content = std::fs::read_to_string(config_file)?;
        let configs: HashMap<String, PluginConfig> = serde_json::from_str(&content)?;
        
        Ok(configs)
    }
    
    pub fn save_plugin_configs(&self, configs: &HashMap<String, PluginConfig>) -> Result<(), PluginError> {
        let config_file = self.config_path.join("plugins.json");
        let content = serde_json::to_string_pretty(configs)?;
        std::fs::write(config_file, content)?;
        Ok(())
    }
}
```

## Plugin Development Guidelines

### Plugin Structure

```text
my-openapi-plugin/
├── Cargo.toml
├── src/
│   ├── lib.rs
│   ├── plugin.rs
│   ├── generator.rs
│   └── config.rs
├── README.md
└── plugin.json
```

### Plugin Cargo.toml

```toml
[package]
name = "my-openapi-plugin"
version = "0.1.0"
edition = "2021"

[dependencies]
openapi-generator-plugin = "0.1.0"
openapi-generator-core = "0.1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[lib]
crate-type = ["cdylib"]
```

### Plugin Implementation Example

```rust
// src/lib.rs
use openapi_generator_plugin::*;
use std::collections::HashMap;

pub struct MyCustomPlugin {
    config: MyPluginConfig,
}

impl Plugin for MyCustomPlugin {
    fn name(&self) -> &str { "my-custom-plugin" }
    fn version(&self) -> &str { "0.1.0" }
    fn description(&self) -> &str { "My custom OpenAPI plugin" }
    fn dependencies(&self) -> Vec<PluginDependency> { vec![] }
    
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError> {
        // Initialize plugin with context
        Ok(())
    }
    
    fn shutdown(&self) -> Result<(), PluginError> {
        // Cleanup resources
        Ok(())
    }
}

impl LanguageGeneratorPlugin for MyCustomPlugin {
    type AstType = MyCustomNode;
    type Error = MyCustomError;
    
    fn generate_ast(&self, openapi: &OpenApi) -> Result<Vec<MyCustomNode>, MyCustomError> {
        // Generate custom AST
        Ok(vec![])
    }
    
    fn generate_code(&self, ast: &[MyCustomNode]) -> Result<Vec<GeneratedFile>, MyCustomError> {
        // Generate custom code
        Ok(vec![])
    }
    
    fn get_language_name(&self) -> &str { "my-custom-language" }
    fn get_file_extensions(&self) -> Vec<&str> { vec!["mcl"] }
    fn get_config_schema(&self) -> Option<serde_json::Value> {
        Some(serde_json::to_value(MyPluginConfig::default()).unwrap())
    }
}

// Plugin entry point
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyCustomPlugin {
        config: MyPluginConfig::default(),
    }))
}
```

## Plugin Loading and Management

### Dynamic Plugin Loading

```rust
pub struct DynamicPluginLoader {
    loaded_plugins: HashMap<String, PluginHandle>,
}

pub struct PluginHandle {
    library: libloading::Library,
    plugin: Box<dyn Plugin>,
}

impl DynamicPluginLoader {
    pub fn load_plugin(&mut self, path: &Path) -> Result<(), PluginError> {
        let library = unsafe { libloading::Library::new(path)? };
        
        let create_plugin: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn Plugin> = 
            unsafe { library.get(b"create_plugin")? };
        
        let plugin_ptr = unsafe { create_plugin() };
        let plugin = unsafe { Box::from_raw(plugin_ptr) };
        
        let name = plugin.name().to_string();
        let handle = PluginHandle { library, plugin };
        
        self.loaded_plugins.insert(name, handle);
        Ok(())
    }
    
    pub fn unload_plugin(&mut self, name: &str) -> Result<(), PluginError> {
        if let Some(handle) = self.loaded_plugins.remove(name) {
            handle.plugin.shutdown()?;
            // Library will be dropped here, unloading it
        }
        Ok(())
    }
}
```

### Plugin Lifecycle Management

```rust
pub struct PluginManager {
    registry: PluginRegistry,
    loader: DynamicPluginLoader,
    config_loader: PluginConfigLoader,
}

impl PluginManager {
    pub fn new() -> Self {
        Self {
            registry: PluginRegistry::new(),
            loader: DynamicPluginLoader::new(),
            config_loader: PluginConfigLoader::new(),
        }
    }
    
    pub fn initialize_plugins(&mut self) -> Result<(), PluginError> {
        // Load plugin configurations
        let configs = self.config_loader.load_plugin_configs()?;
        
        // Discover available plugins
        let discovery = PluginDiscovery::new();
        let plugin_infos = discovery.discover_plugins()?;
        
        // Load and register plugins
        for plugin_info in plugin_infos {
            if let Some(config) = configs.get(&plugin_info.name) {
                if config.enabled {
                    self.load_and_register_plugin(&plugin_info, config)?;
                }
            }
        }
        
        Ok(())
    }
    
    fn load_and_register_plugin(&mut self, info: &PluginInfo, config: &PluginConfig) -> Result<(), PluginError> {
        // Load plugin library
        self.loader.load_plugin(&info.path)?;
        
        // Get plugin instance
        let plugin = self.loader.get_plugin(&info.name)?;
        
        // Register with registry
        self.registry.register_plugin(plugin)?;
        
        Ok(())
    }
}
```

## Error Handling

### Plugin Errors

```rust
#[derive(Debug, Snafu)]
pub enum PluginError {
    #[snafu(display("Plugin '{}' not found", name))]
    PluginNotFound { name: String },
    
    #[snafu(display("Plugin '{}' failed to initialize: {}", name, error))]
    PluginInitializationFailed { name: String, error: String },
    
    #[snafu(display("Plugin dependency '{}' not satisfied", dependency))]
    DependencyNotSatisfied { dependency: String },
    
    #[snafu(display("Plugin configuration error: {}", message))]
    ConfigurationError { message: String },
    
    #[snafu(display("Plugin loading failed: {}", message))]
    LoadingFailed { message: String },
}
```

### Plugin Isolation

```rust
pub struct IsolatedPluginRunner {
    sandbox: PluginSandbox,
}

pub struct PluginSandbox {
    // Sandbox configuration
    max_memory: usize,
    max_execution_time: Duration,
    allowed_system_calls: Vec<String>,
}

impl IsolatedPluginRunner {
    pub fn run_plugin<T, F>(&self, plugin: &dyn Plugin, operation: F) -> Result<T, PluginError>
    where
        F: FnOnce() -> Result<T, PluginError>,
    {
        // Set up sandbox
        self.sandbox.enter()?;
        
        // Run plugin operation
        let result = operation();
        
        // Clean up sandbox
        self.sandbox.exit()?;
        
        result
    }
}
```

## Testing Strategy

### Plugin Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
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
}
```

### Integration Testing

```rust
#[test]
fn test_plugin_integration() {
    let mut manager = PluginManager::new();
    manager.initialize_plugins().unwrap();
    
    // Test language generator plugin
    let generator = manager.get_language_generator("python");
    assert!(generator.is_some());
    
    // Test transform pass plugin
    let passes = manager.get_transform_passes();
    assert!(!passes.is_empty());
}
```

## Performance Considerations

### Plugin Caching

```rust
pub struct PluginCache {
    cache: HashMap<String, CachedPlugin>,
}

pub struct CachedPlugin {
    plugin: Box<dyn Plugin>,
    last_used: Instant,
    usage_count: u64,
}

impl PluginCache {
    pub fn get_plugin(&mut self, name: &str) -> Option<&mut dyn Plugin> {
        if let Some(cached) = self.cache.get_mut(name) {
            cached.last_used = Instant::now();
            cached.usage_count += 1;
            Some(cached.plugin.as_mut())
        } else {
            None
        }
    }
}
```

### Lazy Plugin Loading

```rust
pub struct LazyPluginLoader {
    plugin_paths: HashMap<String, PathBuf>,
    loaded_plugins: HashMap<String, Box<dyn Plugin>>,
}

impl LazyPluginLoader {
    pub fn get_plugin(&mut self, name: &str) -> Result<&mut dyn Plugin, PluginError> {
        if !self.loaded_plugins.contains_key(name) {
            if let Some(path) = self.plugin_paths.get(name) {
                let plugin = self.load_plugin_from_path(path)?;
                self.loaded_plugins.insert(name.to_string(), plugin);
            } else {
                return Err(PluginError::PluginNotFound { name: name.to_string() });
            }
        }
        
        Ok(self.loaded_plugins.get_mut(name).unwrap().as_mut())
    }
}
```

## Conclusion

The plugin system provides a powerful and flexible foundation for extending the OpenAPI code generator. The type-safe plugin traits ensure reliability, while the dynamic loading system enables runtime extensibility. The comprehensive error handling and isolation mechanisms ensure that plugin failures don't affect core functionality.

The plugin development guidelines and testing strategies make it easy for developers to create and maintain plugins, while the performance optimizations ensure efficient plugin execution.

## Related RFDs

- [RFD 0001: Overall Architecture and Design Philosophy](./0001-architecture-overview.md)
- [RFD 0004: Multi-Level Transformation Passes](./0004-transformation-passes.md)
- [RFD 0005: Code Generation and Type Mapping](./0005-code-generation.md)
