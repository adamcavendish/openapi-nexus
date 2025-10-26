# RFD 0007: Pretty Printing with pretty.rs

## Summary

This RFD defines the pretty printing strategy using the `pretty.rs` crate for converting language-specific ASTs to well-formatted source code. The system provides both final code emission and intermediate debugging capabilities with language-specific formatting rules and style guides.

## Motivation

### Why pretty.rs?

1. **Wadler's Algorithm**: Implements the classic pretty printing algorithm
2. **Composability**: Easy to build complex formatters from simple combinators
3. **Performance**: Efficient rendering with good space utilization
4. **Flexibility**: Supports various formatting styles and constraints
5. **Rust Integration**: Native Rust implementation with excellent type safety

### Design Goals

- **Readability**: Generate clean, readable code that follows language conventions
- **Consistency**: Apply consistent formatting rules within the generator
- **External Formatting**: Generate code that works well with language-specific formatters
- **Performance**: Efficient pretty printing for large ASTs
- **Debugging**: Support for intermediate pretty-printing during development

## Pretty Printing Architecture

### Core Pretty Printing Trait

```rust
// openapi-generator-core/src/pretty_printing.rs
pub trait PrettyPrinter {
    type AstType;
    type Error;
    
    fn pretty_print(&self, ast: &Self::AstType) -> Result<String, Self::Error>;
    fn debug_print(&self, ast: &Self::AstType) -> Result<String, Self::Error>;
}

#[derive(Debug, Clone)]
pub struct PrettyConfig {
    pub indent_size: usize,
    pub max_line_width: usize,
}
```

### Pretty Document Type

```rust
use pretty::Doc;

pub type PrettyDoc = Doc<Box<dyn PrettyRender>>;

pub trait PrettyRender {
    fn render(&self, width: usize) -> String;
}

// Language-specific pretty renderers
pub struct TypeScriptRenderer;
pub struct RustRenderer;
pub struct PythonRenderer;
```

## AST to Pretty Doc Conversion

### TypeScript Pretty Printing

```rust
impl PrettyPrinter for TypeScriptPrettyPrinter {
    type AstType = TsNode;
    type Error = PrettyPrintError;
    
    fn pretty_print(&self, ast: &TsNode) -> Result<String, PrettyPrintError> {
        let doc = self.ast_to_doc(ast)?;
        Ok(doc.pretty(100)) // Fixed line width for consistency
    }
    
    fn ast_to_doc(&self, ast: &TsNode) -> Result<PrettyDoc, PrettyPrintError> {
        match ast {
            TsNode::Interface(interface) => self.interface_to_doc(interface),
            TsNode::TypeAlias(type_alias) => self.type_alias_to_doc(type_alias),
            TsNode::Enum(enum_def) => self.enum_to_doc(enum_def),
            TsNode::Function(function) => self.function_to_doc(function),
            TsNode::Class(class) => self.class_to_doc(class),
            TsNode::Import(import) => self.import_to_doc(import),
            TsNode::Export(export) => self.export_to_doc(export),
        }
    }
}
```

### Interface Pretty Printing

```rust
impl TypeScriptPrettyPrinter {
    fn interface_to_doc(&self, interface: &Interface) -> Result<PrettyDoc, PrettyPrintError> {
        let mut doc = Doc::text("interface");
        doc = doc + Doc::space() + Doc::text(&interface.name);
        
        // Add generics
        if !interface.generics.is_empty() {
            doc = doc + self.generics_to_doc(&interface.generics)?;
        }
        
        // Add extends
        if !interface.extends.is_empty() {
            doc = doc + Doc::space() + Doc::text("extends");
            doc = doc + Doc::space() + self.comma_separated(&interface.extends);
        }
        
        // Add body
        doc = doc + Doc::space() + Doc::text("{");
        doc = doc + Doc::line() + self.indent(self.properties_to_doc(&interface.properties)?);
        doc = doc + Doc::line() + Doc::text("}");
        
        Ok(doc)
    }
    
    fn properties_to_doc(&self, properties: &[Property]) -> Result<PrettyDoc, PrettyPrintError> {
        if properties.is_empty() {
            return Ok(Doc::nil());
        }
        
        let mut docs = Vec::new();
        for (i, property) in properties.iter().enumerate() {
            let mut prop_doc = self.property_to_doc(property)?;
            if i < properties.len() - 1 {
                prop_doc = prop_doc + Doc::text(",");
            }
            docs.push(prop_doc);
        }
        
        Ok(Doc::intersperse(docs, Doc::line()))
    }
    
    fn property_to_doc(&self, property: &Property) -> Result<PrettyDoc, PrettyPrintError> {
        let mut doc = Doc::text(&property.name);
        
        if property.optional {
            doc = doc + Doc::text("?");
        }
        
        doc = doc + Doc::text(":") + Doc::space() + self.type_expression_to_doc(&property.type_expr)?;
        
        Ok(doc)
    }
}
```

### Type Expression Pretty Printing

```rust
impl TypeScriptPrettyPrinter {
    fn type_expression_to_doc(&self, type_expr: &TypeExpression) -> Result<PrettyDoc, PrettyPrintError> {
        match type_expr {
            TypeExpression::Primitive(primitive) => self.primitive_to_doc(primitive),
            TypeExpression::Union(types) => self.union_to_doc(types),
            TypeExpression::Intersection(types) => self.intersection_to_doc(types),
            TypeExpression::Array(item_type) => self.array_to_doc(item_type),
            TypeExpression::Object(properties) => self.object_to_doc(properties),
            TypeExpression::Reference(name) => Ok(Doc::text(name)),
            TypeExpression::Generic(name) => Ok(Doc::text(name)),
            TypeExpression::Function(signature) => self.function_signature_to_doc(signature),
            TypeExpression::Literal(value) => Ok(Doc::text(format!("\"{}\"", value))),
            TypeExpression::Tuple(types) => self.tuple_to_doc(types),
            _ => Err(PrettyPrintError::UnsupportedTypeExpression),
        }
    }
    
    fn union_to_doc(&self, types: &[TypeExpression]) -> Result<PrettyDoc, PrettyPrintError> {
        let type_docs = types.iter()
            .map(|t| self.type_expression_to_doc(t))
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(Doc::intersperse(type_docs, Doc::text(" | ")))
    }
    
    fn array_to_doc(&self, item_type: &TypeExpression) -> Result<PrettyDoc, PrettyPrintError> {
        let item_doc = self.type_expression_to_doc(item_type)?;
        Ok(item_doc + Doc::text("[]"))
    }
}
```

## Language-Specific Pretty Printing Combinators

### TypeScript Combinators

```rust
impl TypeScriptPrettyPrinter {
    // Indentation helper
    fn indent(&self, doc: PrettyDoc) -> PrettyDoc {
        Doc::nest(2, doc) // Fixed 2-space indentation
    }
    
    // Comma-separated list
    fn comma_separated(&self, items: &[String]) -> PrettyDoc {
        let item_docs = items.iter().map(|item| Doc::text(item)).collect();
        Doc::intersperse(item_docs, Doc::text(", "))
    }
    
    // Semicolon termination (always add semicolons for consistency)
    fn with_semicolon(&self, doc: PrettyDoc) -> PrettyDoc {
        doc + Doc::text(";")
    }
    
    // Quote handling (always use double quotes for consistency)
    fn quoted(&self, text: &str) -> PrettyDoc {
        Doc::text(format!("\"{}\"", text))
    }
}
```

## Formatting Philosophy

### External Formatter Integration

The pretty printer generates clean, readable code that follows language conventions, but delegates final formatting to external tools:

- **TypeScript**: Generated code should be formatted with Prettier
- **Rust**: Generated code should be formatted with rustfmt
- **Python**: Generated code should be formatted with Black
- **Go**: Generated code should be formatted with gofmt

### Internal Formatting Standards

The generator uses consistent internal formatting standards:

#### TypeScript Standards

- 2-space indentation
- Double quotes for strings
- Semicolons for statement termination
- Trailing commas in objects/arrays
- Consistent bracket placement

#### Rust Standards

- 4-space indentation
- Snake_case for identifiers
- Consistent derive attribute placement
- Proper visibility modifiers

## Debugging and Intermediate Pretty-Printing

### Debug Pretty Printer

```rust
pub struct DebugPrettyPrinter {
    show_metadata: bool,
    show_annotations: bool,
}

impl DebugPrettyPrinter {
    pub fn new() -> Self {
        Self {
            show_metadata: true,
            show_annotations: true,
        }
    }
    
    pub fn debug_print_ast<T: Debug>(&self, ast: &T) -> String {
        format!("{:#?}", ast)
    }
    
    pub fn debug_print_doc(&self, doc: &PrettyDoc) -> String {
        // Pretty print with debug annotations
        doc.pretty(100)
    }
    
    pub fn debug_print_with_metadata(&self, ast: &TsNode) -> String {
        let mut result = String::new();
        
        // Add metadata
        if self.show_metadata {
            result.push_str(&format!("// AST Node: {:?}\n", std::any::type_name::<TsNode>()));
        }
        
        // Add pretty printed content
        let pretty_content = self.pretty_print(ast).unwrap_or_else(|_| "Error pretty printing".to_string());
        result.push_str(&pretty_content);
        
        // Add annotations
        if self.show_annotations {
            result.push_str("\n// End of AST Node");
        }
        
        result
    }
}
```

### Intermediate Pretty-Printing

```rust
pub struct IntermediatePrettyPrinter {
    debug_mode: bool,
}

impl IntermediatePrettyPrinter {
    pub fn print_transformation_stage(&self, stage: &str, ast: &TsNode) -> String {
        let mut result = String::new();
        
        if self.debug_mode {
            result.push_str(&format!("// === {} ===\n", stage));
        }
        
        let pretty_content = self.pretty_print(ast).unwrap_or_else(|_| "Error".to_string());
        result.push_str(&pretty_content);
        
        if self.debug_mode {
            result.push_str(&format!("\n// === End {} ===\n", stage));
        }
        
        result
    }
    
    pub fn print_ast_comparison(&self, before: &TsNode, after: &TsNode) -> String {
        let mut result = String::new();
        
        result.push_str("// === BEFORE ===\n");
        result.push_str(&self.pretty_print(before).unwrap_or_else(|_| "Error".to_string()));
        
        result.push_str("\n\n// === AFTER ===\n");
        result.push_str(&self.pretty_print(after).unwrap_or_else(|_| "Error".to_string()));
        
        result
    }
}
```

## Performance Considerations

### Pretty Document Optimization

```rust
pub struct OptimizedPrettyPrinter {
    cache: HashMap<String, PrettyDoc>,
}

impl OptimizedPrettyPrinter {
    pub fn pretty_print_cached(&mut self, ast: &TsNode) -> Result<String, PrettyPrintError> {
        let cache_key = self.generate_cache_key(ast);
        
        if let Some(cached_doc) = self.cache.get(&cache_key) {
            return Ok(cached_doc.pretty(100));
        }
        
        let doc = self.ast_to_doc(ast)?;
        self.cache.insert(cache_key, doc.clone());
        Ok(doc.pretty(100))
    }
    
    fn generate_cache_key(&self, ast: &TsNode) -> String {
        // Generate a hash-based cache key for the AST
        format!("{:x}", md5::compute(format!("{:?}", ast)))
    }
}
```

### Lazy Pretty Printing

```rust
pub struct LazyPrettyPrinter {
    lazy_docs: HashMap<String, LazyDoc>,
}

pub enum LazyDoc {
    Computed(PrettyDoc),
    Pending(Box<dyn Fn() -> PrettyDoc>),
}

impl LazyPrettyPrinter {
    pub fn create_lazy_doc<F>(&mut self, key: String, f: F) 
    where
        F: Fn() -> PrettyDoc + 'static,
    {
        self.lazy_docs.insert(key, LazyDoc::Pending(Box::new(f)));
    }
    
    pub fn get_lazy_doc(&mut self, key: &str) -> Option<PrettyDoc> {
        match self.lazy_docs.get(key) {
            Some(LazyDoc::Computed(doc)) => Some(doc.clone()),
            Some(LazyDoc::Pending(f)) => {
                let doc = f();
                self.lazy_docs.insert(key.to_string(), LazyDoc::Computed(doc.clone()));
                Some(doc)
            }
            None => None,
        }
    }
}
```

## Examples of Pretty Doc Construction

### Complex TypeScript Interface

```rust
fn complex_interface_to_doc(&self, interface: &Interface) -> Result<PrettyDoc, PrettyPrintError> {
    let mut doc = Doc::text("interface");
    doc = doc + Doc::space() + Doc::text(&interface.name);
    
    // Add generics
    if !interface.generics.is_empty() {
        doc = doc + Doc::text("<");
        let generic_docs = interface.generics.iter()
            .map(|g| self.generic_to_doc(g))
            .collect::<Result<Vec<_>, _>>()?;
        doc = doc + Doc::intersperse(generic_docs, Doc::text(", "));
        doc = doc + Doc::text(">");
    }
    
    // Add extends
    if !interface.extends.is_empty() {
        doc = doc + Doc::space() + Doc::text("extends");
        doc = doc + Doc::space() + Doc::intersperse(
            interface.extends.iter().map(|e| Doc::text(e)).collect(),
            Doc::text(", ")
        );
    }
    
    // Add body
    doc = doc + Doc::space() + Doc::text("{");
    
    if !interface.properties.is_empty() {
        doc = doc + Doc::line() + self.indent(
            Doc::intersperse(
                interface.properties.iter()
                    .map(|p| self.property_to_doc(p))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .enumerate()
                    .map(|(i, mut prop_doc)| {
                        if i < interface.properties.len() - 1 {
                            prop_doc = prop_doc + Doc::text(",");
                        }
                        prop_doc
                    })
                    .collect(),
                Doc::line()
            )
        );
    }
    
    doc = doc + Doc::line() + Doc::text("}");
    
    Ok(doc)
}
```

### Rust Struct with Derives

```rust
fn rust_struct_to_doc(&self, struct_def: &Struct) -> Result<PrettyDoc, PrettyPrintError> {
    let mut doc = Doc::nil();
    
    // Add derives
    if !struct_def.derives.is_empty() {
        doc = doc + self.derives_to_doc(&struct_def.derives) + Doc::line();
    }
    
    // Add documentation
    if let Some(docs) = &struct_def.documentation {
        doc = doc + self.documentation_to_doc(docs) + Doc::line();
    }
    
    // Add visibility
    doc = doc + self.visibility_to_doc(&struct_def.visibility);
    doc = doc + Doc::space() + Doc::text("struct");
    doc = doc + Doc::space() + Doc::text(&struct_def.name);
    
    // Add generics
    if !struct_def.generics.is_empty() {
        doc = doc + self.generics_to_doc(&struct_def.generics)?;
    }
    
    // Add body
    doc = doc + Doc::space() + Doc::text("{");
    
    if !struct_def.fields.is_empty() {
        doc = doc + Doc::line() + self.indent(
            Doc::intersperse(
                struct_def.fields.iter()
                    .map(|f| self.field_to_doc(f))
                    .collect::<Result<Vec<_>, _>>()?
                    .into_iter()
                    .enumerate()
                    .map(|(i, mut field_doc)| {
                        if i < struct_def.fields.len() - 1 {
                            field_doc = field_doc + Doc::text(",");
                        }
                        field_doc
                    })
                    .collect(),
                Doc::line()
            )
        );
    }
    
    doc = doc + Doc::line() + Doc::text("}");
    
    Ok(doc)
}
```

## Error Handling

### Pretty Print Errors

```rust
#[derive(Debug, Snafu)]
pub enum PrettyPrintError {
    #[snafu(display("Unsupported AST node: {}", node_type))]
    UnsupportedAstNode { node_type: String },
    
    #[snafu(display("Invalid type expression: {}", expr))]
    InvalidTypeExpression { expr: String },
    
    #[snafu(display("Pretty printing failed: {}", message))]
    PrettyPrintFailed { message: String },
    
    #[snafu(display("Configuration error: {}", message))]
    ConfigurationError { message: String },
}
```

### Error Recovery

```rust
impl PrettyPrinter for TypeScriptPrettyPrinter {
    fn pretty_print(&self, ast: &TsNode) -> Result<String, PrettyPrintError> {
        match self.ast_to_doc(ast) {
            Ok(doc) => Ok(doc.pretty(self.config.max_line_width)),
            Err(e) => {
                // Fallback to basic pretty printing
                self.fallback_pretty_print(ast)
            }
        }
    }
    
    fn fallback_pretty_print(&self, ast: &TsNode) -> Result<String, PrettyPrintError> {
        // Simple fallback that just converts to string
        Ok(format!("{:#?}", ast))
    }
}
```

## Testing Strategy

### Unit Tests

Test individual pretty printing functions:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interface_pretty_printing() {
        let printer = TypeScriptPrettyPrinter::new();
        let interface = create_test_interface();
        
        let result = printer.pretty_print(&TsNode::Interface(interface));
        assert!(result.is_ok());
        
        let pretty_code = result.unwrap();
        assert!(pretty_code.contains("interface"));
        assert!(pretty_code.contains("{"));
        assert!(pretty_code.contains("}"));
    }
}
```

### Integration Tests

Test full pretty printing pipeline:

```rust
#[test]
fn test_full_pretty_printing() {
    let openapi = load_test_spec();
    let generator = TypeScriptGenerator::new();
    let ast = generator.generate_ast(&openapi).unwrap();
    
    let printer = TypeScriptPrettyPrinter::new();
    let result = printer.pretty_print(&ast[0]);
    assert!(result.is_ok());
}
```

## Conclusion

The pretty printing system using `pretty.rs` provides a robust and flexible foundation for generating well-formatted source code. The composable nature of the pretty document combinators makes it easy to build complex formatters, while the language-specific combinators ensure idiomatic code generation.

The debugging and intermediate pretty-printing capabilities provide valuable tooling for development and debugging, while the performance optimizations ensure efficient pretty printing for large ASTs.

The system focuses on generating clean, readable code that follows language conventions, delegating final formatting to external tools like Prettier, rustfmt, and other language-specific formatters. This approach provides the best of both worlds: consistent internal formatting and the flexibility of external formatting tools.

## Related RFDs

- [RFD 0003: Language-Specific AST Design](./0003-language-ast-design.md)
- [RFD 0005: Code Generation and Type Mapping](./0005-code-generation.md)
- [RFD 0006: Multi-File Emission Strategy](./0006-multi-file-emission.md)
