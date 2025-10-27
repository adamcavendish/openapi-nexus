# RFD 0005: Code Generation and Type Mapping

## Summary

This RFD defines the code generation strategy and type mapping system for converting OpenAPI schemas to language-specific types. The system handles complex type relationships, nullable fields, enums, and API client generation with a focus on correctness and idiomatic code generation.

## Motivation

### Code Generation Challenges

1. **Type Mapping**: OpenAPI schemas don't map 1:1 to language types
2. **Complex Types**: Unions, intersections, and generics require careful handling
3. **Nullable Fields**: Different languages handle nullability differently
4. **API Clients**: Generate idiomatic API client code
5. **Validation**: Ensure generated code is correct and compilable

### Design Goals

- **Correctness**: Generated code should be syntactically and semantically correct
- **Idiomatic**: Generated code should follow language conventions
- **Type Safety**: Leverage language type systems for safety
- **Extensibility**: Easy to add new type mappings and generators
- **Performance**: Efficient code generation for large specifications

## Type Mapping Architecture

### Core Type Mapper Trait

```rust
// openapi-generator-core/src/type_mapping.rs
pub trait TypeMapper {
    type LanguageType;
    type Error;
    
    fn map_schema(&self, schema: &Schema) -> Result<Self::LanguageType, Self::Error>;
    fn map_primitive(&self, primitive: &PrimitiveType) -> Result<Self::LanguageType, Self::Error>;
    fn map_array(&self, item_type: &Schema) -> Result<Self::LanguageType, Self::Error>;
    fn map_object(&self, properties: &HashMap<String, Schema>) -> Result<Self::LanguageType, Self::Error>;
    fn map_union(&self, variants: &[Schema]) -> Result<Self::LanguageType, Self::Error>;
}
```

### Type Mapping Context

```rust
pub struct TypeMappingContext {
    pub openapi: &OpenApi,
    pub schema_registry: SchemaRegistry,
    pub custom_mappings: HashMap<String, String>,
    pub naming_convention: NamingConvention,
    pub nullable_strategy: NullableStrategy,
}

pub struct SchemaRegistry {
    pub schemas: HashMap<String, Schema>,
    pub references: HashMap<String, String>,
}
```

## API Client Generation Patterns

### HTTP Client Interface

```rust
pub trait ApiClientGenerator {
    type ClientType;
    type Error;
    
    fn generate_client(&self, openapi: &OpenApi) -> Result<Self::ClientType, Self::Error>;
    fn generate_methods(&self, operations: &[Operation]) -> Result<Vec<Method>, Self::Error>;
    fn generate_types(&self, schemas: &HashMap<String, Schema>) -> Result<Vec<Type>, Self::Error>;
}
```

### TypeScript API Client

```rust
impl ApiClientGenerator for TypeScriptApiClientGenerator {
    type ClientType = TsClass;
    
    fn generate_client(&self, openapi: &OpenApi) -> Result<TsClass, TypeMappingError> {
        let methods = self.generate_methods_from_operations(openapi)?;
        let properties = self.generate_client_properties()?;
        
        Ok(TsClass {
            name: "ApiClient".to_string(),
            properties,
            methods,
            extends: Some("BaseClient".to_string()),
            implements: vec![],
            generics: vec![],
            is_export: true,
            documentation: Some("Generated API client".to_string()),
        })
    }
    
    fn generate_methods(&self, operations: &[Operation]) -> Result<Vec<TsMethod>, TypeMappingError> {
        operations.iter()
            .map(|operation| self.generate_method_from_operation(operation))
            .collect()
    }
}
```

### Method Generation

```rust
impl TypeScriptApiClientGenerator {
    fn generate_method_from_operation(&self, operation: &Operation) -> Result<TsMethod, TypeMappingError> {
        let method_name = self.generate_method_name(operation);
        let parameters = self.generate_parameters(operation)?;
        let return_type = self.generate_return_type(operation)?;
        
        Ok(TsMethod {
            name: method_name,
            parameters,
            return_type: Some(return_type),
            is_async: true,
            is_static: false,
            visibility: TsVisibility::Public,
            documentation: operation.description.clone(),
        })
    }
}
```

## Code Generation Pipeline

### Generator Trait

```rust
pub trait LanguageCodeGenerator {
    type AstType;
    type Error;
    
    fn generate(&self, openapi: &OpenApi) -> Result<Self::AstType, Self::Error>;
    fn generate_models(&self, schemas: &HashMap<String, Schema>) -> Result<Vec<Self::AstType>, Self::Error>;
    fn generate_api(&self, operations: &[Operation]) -> Result<Vec<Self::AstType>, Self::Error>;
    fn generate_client(&self, openapi: &OpenApi) -> Result<Self::AstType, Self::Error>;
}
```

### TypeScript Generator

```rust
impl LanguageCodeGenerator for TypeScriptGenerator {
    type AstType = TsNode;
    type Error = GeneratorError;
    
    fn generate(&self, openapi: &OpenApi) -> Result<Vec<TsNode>, GeneratorError> {
        let mut nodes = Vec::new();
        
        // Generate models
        if let Some(components) = &openapi.components {
            if let Some(schemas) = &components.schemas {
                let model_nodes = self.generate_models(schemas)?;
                nodes.extend(model_nodes);
            }
        }
        
        // Generate API methods
        let api_nodes = self.generate_api_from_paths(&openapi.paths)?;
        nodes.extend(api_nodes);
        
        // Generate client
        let client_node = self.generate_client(openapi)?;
        nodes.push(client_node);
        
        Ok(nodes)
    }
}
```

## Error Handling

### Type Mapping Errors

```rust
#[derive(Debug, Snafu)]
pub enum TypeMappingError {
    #[snafu(display("Unsupported schema type: {}", schema_type))]
    UnsupportedSchemaType { schema_type: String },
    
    #[snafu(display("Circular reference detected: {}", reference))]
    CircularReference { reference: String },
    
    #[snafu(display("Invalid type mapping: {}", message))]
    InvalidMapping { message: String },
    
    #[snafu(display("Missing required field: {}", field))]
    MissingRequiredField { field: String },
}
```

### Generator Errors

```rust
#[derive(Debug, Snafu)]
pub enum GeneratorError {
    #[snafu(display("Type mapping failed: {}", source))]
    TypeMapping { source: TypeMappingError },
    
    #[snafu(display("Code generation failed: {}", message))]
    GenerationFailed { message: String },
    
    #[snafu(display("Validation failed: {}", message))]
    ValidationFailed { message: String },
}
```

## Testing Strategy

### Unit Tests

Test individual type mappings:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_string_schema_mapping() {
        let mapper = TypeScriptTypeMapper::new(TypeScriptConfig::default());
        let schema = Schema::String(StringSchema::default());
        
        let result = mapper.map_openapi_schema(&schema);
        assert!(result.is_ok());
        
        if let Ok(TsTypeExpression::Primitive(TsPrimitiveType::String)) = result {
            // Test passed
        } else {
            panic!("Expected string type");
        }
    }
}
```

### Integration Tests

Test full code generation:

```rust
#[test]
fn test_full_generation() {
    let openapi = load_test_spec();
    let generator = TypeScriptGenerator::new();
    
    let result = generator.generate(&openapi);
    assert!(result.is_ok());
    
    let nodes = result.unwrap();
    assert!(!nodes.is_empty());
}
```

## Performance Considerations

### Caching

```rust
pub struct TypeMappingCache {
    cache: HashMap<String, TsTypeExpression>,
}

impl TypeMappingCache {
    pub fn get_cached_type(&self, schema_hash: &str) -> Option<&TsTypeExpression> {
        self.cache.get(schema_hash)
    }
    
    pub fn cache_type(&mut self, schema_hash: String, type_expr: TsTypeExpression) {
        self.cache.insert(schema_hash, type_expr);
    }
}
```

### Parallel Processing

```rust
impl LanguageCodeGenerator for TypeScriptGenerator {
    fn generate_models(&self, schemas: &HashMap<String, Schema>) -> Result<Vec<TsNode>, GeneratorError> {
        // Process schemas in parallel
        schemas.par_iter()
            .map(|(name, schema)| self.generate_model(name, schema))
            .collect()
    }
}
```

## Conclusion

The code generation and type mapping system provides a robust foundation for converting OpenAPI schemas to language-specific types. The modular design allows for easy extension to new languages and type systems, while the comprehensive error handling ensures reliable code generation.

The focus on idiomatic code generation and type safety makes the generated code production-ready and maintainable.

## Related RFDs

- [RFD 0003: Language-Specific AST Design](./0003-language-ast-design.md)
- [RFD 0006: Multi-File Emission Strategy](./0006-multi-file-emission.md)
- [RFD 0007: Pretty Printing with pretty.rs](./0007-pretty-printing.md)
