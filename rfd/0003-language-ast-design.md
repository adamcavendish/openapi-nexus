# RFD 0003: Language-Specific AST Design

## Summary

This RFD defines the design principles and structure for language-specific Abstract Syntax Trees (ASTs) used in the OpenAPI code generator. We design ASTs that are simple, type-safe, serializable, and extensible for multiple target languages.

## Motivation

### Why Language-Specific ASTs?

1. **Language Idioms**: Each language has unique constructs and patterns
2. **Type Safety**: Language-specific types provide better compile-time guarantees
3. **Code Quality**: ASTs can enforce language-specific best practices
4. **Extensibility**: Easy to add new languages without affecting existing ones
5. **Tooling**: Language-specific ASTs enable better IDE support and tooling

### Design Goals

- **Simplicity**: AST nodes should be easy to understand and manipulate
- **Type Safety**: Leverage Rust's type system for compile-time guarantees
- **Serializability**: Support serialization for debugging and caching
- **Extensibility**: Easy to add new node types and languages
- **Performance**: Efficient memory usage and traversal

## AST Design Principles

### 1. Node-Based Design

Each AST is a tree of nodes, where each node represents a language construct:

```rust
// TypeScript AST example
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
    Import(Import),
    Export(Export),
}
```

### 2. Composition Over Inheritance

Use composition to build complex structures:

```rust
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}
```

### 3. Optional Metadata

Include optional metadata for enhanced functionality:

```rust
pub struct Property {
    pub name: String,
    pub type_expr: TypeExpression,
    pub optional: bool,
    pub documentation: Option<String>,
    pub attributes: Vec<Attribute>, // For decorators, etc.
}
```

### 4. Reference-Based Types

Use references for type expressions to avoid deep nesting:

```rust
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Union(Vec<TypeExpression>),
    Array(Box<TypeExpression>),
    Reference(String), // Reference to another type
    Generic(String),   // Generic type parameter
}
```

## TypeScript AST Structure

### Core Node Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TsNode {
    Interface(Interface),
    TypeAlias(TypeAlias),
    Enum(Enum),
    Function(Function),
    Class(Class),
    Import(Import),
    Export(Export),
}
```

### Interface Definition

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub properties: Vec<Property>,
    pub extends: Vec<String>,
    pub generics: Vec<Generic>,
    pub documentation: Option<String>,
}
```

### Type System

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TypeExpression {
    Primitive(PrimitiveType),
    Union(Vec<TypeExpression>),
    Intersection(Vec<TypeExpression>),
    Array(Box<TypeExpression>),
    Object(HashMap<String, TypeExpression>),
    Reference(String),
    Generic(String),
    Function(Box<FunctionSignature>),
    Literal(String),
    IndexSignature(String, Box<TypeExpression>),
    Tuple(Vec<TypeExpression>),
}
```

### Primitive Types

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveType {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
    Any,
    Unknown,
    Void,
    Never,
}
```

## Common Patterns Across Language ASTs

### 1. Documentation Support

All AST nodes support documentation:

```rust
pub struct SomeNode {
    // ... other fields
    pub documentation: Option<String>,
}
```

### 2. Generic Support

Generic types are handled consistently:

```rust
pub struct Generic {
    pub name: String,
    pub constraint: Option<TypeExpression>,
    pub default: Option<TypeExpression>,
}
```

### 3. Visibility/Export Control

Each language has its own visibility system:

```rust
// TypeScript
pub enum Visibility {
    Public,
    Private,
    Protected,
}

// Rust
pub enum Visibility {
    Public,
    Private,
    Crate,
    Super,
    In(String),
}
```

### 4. Function Signatures

Functions follow a common pattern:

```rust
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub return_type: Option<TypeExpression>,
    pub is_async: bool,
    pub documentation: Option<String>,
    // Language-specific fields
}
```

## AST Validation and Well-Formedness

### Validation Traits

```rust
pub trait AstValidator {
    type Node;
    type Error;
    
    fn validate(&self, node: &Self::Node) -> Result<(), Self::Error>;
    fn validate_tree(&self, root: &Self::Node) -> Result<(), Self::Error>;
}
```

### TypeScript Validation

```rust
impl AstValidator for TypeScriptValidator {
    type Node = TsNode;
    type Error = TsValidationError;
    
    fn validate(&self, node: &TsNode) -> Result<(), TsValidationError> {
        match node {
            TsNode::Interface(interface) => {
                // Validate interface name
                // Validate property names are unique
                // Validate generic constraints
                // Validate extends references exist
            }
            // ... other node types
        }
    }
}
```

### Common Validation Rules

1. **Name Uniqueness**: No duplicate names in same scope
2. **Reference Validity**: All references must be resolvable
3. **Type Consistency**: Type expressions must be well-formed
4. **Generic Constraints**: Generic constraints must be valid
5. **Documentation**: Documentation strings must be valid

## AST Traversal and Manipulation

### Visitor Pattern

```rust
pub trait AstVisitor {
    type Node;
    
    fn visit(&mut self, node: &mut Self::Node) -> Result<(), Error>;
    fn visit_children(&mut self, node: &mut Self::Node) -> Result<(), Error>;
}
```

### Transformer Pattern

```rust
pub trait AstTransformer {
    type Node;
    
    fn transform(&mut self, node: &mut Self::Node) -> Result<(), Error>;
    fn transform_children(&mut self, node: &mut Self::Node) -> Result<(), Error>;
}
```

### Example Transformations

```rust
// Rename all interfaces with a prefix
pub struct InterfaceRenamer {
    prefix: String,
}

impl AstTransformer for InterfaceRenamer {
    type Node = TsNode;
    
    fn transform(&mut self, node: &mut TsNode) -> Result<(), Error> {
        if let TsNode::Interface(interface) = node {
            interface.name = format!("{}{}", self.prefix, interface.name);
        }
        Ok(())
    }
}
```

## Performance Considerations

### Memory Efficiency

1. **Reference-Based**: Use `String` references instead of owned strings where possible
2. **Boxed Types**: Use `Box<T>` for recursive types to avoid stack overflow
3. **Vec Reuse**: Reuse vectors where possible to avoid allocations
4. **Lazy Evaluation**: Defer expensive computations until needed

### Traversal Efficiency

1. **Depth-First**: Use depth-first traversal for most operations
2. **Early Exit**: Stop traversal when possible
3. **Caching**: Cache expensive computations
4. **Parallel Processing**: Process independent subtrees in parallel

## Serialization and Debugging

### Serialization Support

All AST nodes implement `Serialize` and `Deserialize`:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    // ... fields
}
```

### Debugging Support

```rust
// Pretty-print AST for debugging
pub fn debug_print_ast<T: Debug>(ast: &T) {
    println!("{:#?}", ast);
}

// Serialize AST to JSON for inspection
pub fn serialize_ast<T: Serialize>(ast: &T) -> Result<String, Error> {
    serde_json::to_string_pretty(ast)
}
```

## Testing Strategy

### Unit Tests

Test individual AST node creation and manipulation:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interface_creation() {
        let interface = Interface {
            name: "User".to_string(),
            properties: vec![],
            extends: vec![],
            generics: vec![],
            documentation: None,
        };
        
        assert_eq!(interface.name, "User");
    }
}
```

### Integration Tests

Test AST generation from OpenAPI specs:

```rust
#[test]
fn test_openapi_to_ts_ast() {
    let openapi = load_test_spec();
    let generator = TypeScriptGenerator::new();
    let ast = generator.generate_ast(&openapi).unwrap();
    
    // Verify AST structure
    assert!(!ast.nodes.is_empty());
}
```

## Conclusion

The language-specific AST design provides a solid foundation for generating high-quality code in multiple languages. The common patterns across languages make the system maintainable, while language-specific features ensure idiomatic code generation.

The validation and traversal systems provide robust tooling for AST manipulation, and the serialization support enables debugging and caching.

## Related RFDs

- [RFD 0001: Overall Architecture and Design Philosophy](./0001-architecture-overview.md)
- [RFD 0005: Code Generation and Type Mapping](./0005-code-generation.md)
- [RFD 0007: Pretty Printing with pretty.rs](./0007-pretty-printing.md)
