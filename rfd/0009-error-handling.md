# RFD 0009: Error Handling and Diagnostics

## Summary

This RFD defines the comprehensive error handling and diagnostics system for the OpenAPI code generator. The system provides structured error types, source location tracking, user-friendly error messages, warning systems, and diagnostic reporting with recovery strategies.

## Motivation

### Why Comprehensive Error Handling?

1. **User Experience**: Clear, actionable error messages help users fix issues quickly
2. **Debugging**: Detailed error information aids in troubleshooting
3. **Reliability**: Robust error handling prevents crashes and data loss
4. **Maintainability**: Structured error handling makes the codebase easier to maintain
5. **Integration**: Well-defined error types enable better tool integration

### Design Goals

- **Clarity**: Error messages should be clear and actionable
- **Context**: Errors should include relevant context and source locations
- **Recovery**: Provide recovery strategies where possible
- **Consistency**: Uniform error handling across all components
- **Extensibility**: Easy to add new error types and handling strategies

## Error Type Hierarchy

### Core Error Types

```rust
#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Parse error: {}", source))]
    Parse { source: ParseError },
    
    #[snafu(display("Transform error: {}", source))]
    Transform { source: TransformError },
    
    #[snafu(display("Generation error: {}", source))]
    Generation { source: GenerationError },
    
    #[snafu(display("Emission error: {}", source))]
    Emission { source: EmissionError },
    
    #[snafu(display("Configuration error: {}", source))]
    Configuration { source: ConfigurationError },
    
    #[snafu(display("Plugin error: {}", source))]
    Plugin { source: PluginError },
    
    #[snafu(display("IO error: {}", source))]
    Io { source: std::io::Error },
    
    #[snafu(display("Serialization error: {}", source))]
    Serialization { source: serde_json::Error },
}
```

### Parse Errors

```rust
#[derive(Debug, Snafu)]
pub enum ParseError {
    #[snafu(display("Invalid OpenAPI specification: {}", message))]
    InvalidSpec { message: String, location: SourceLocation },
    
    #[snafu(display("Unsupported OpenAPI version: {}", version))]
    UnsupportedVersion { version: String, location: SourceLocation },
    
    #[snafu(display("Circular reference detected: {}", reference))]
    CircularReference { reference: String, location: SourceLocation },
    
    #[snafu(display("External reference not supported: {}", reference))]
    ExternalReference { reference: String, location: SourceLocation },
    
    #[snafu(display("Schema validation failed: {}", details))]
    SchemaValidation { details: String, location: SourceLocation },
    
    #[snafu(display("Missing required field '{}' in {}", field, context))]
    MissingRequiredField { field: String, context: String, location: SourceLocation },
    
    #[snafu(display("Invalid field value '{}' for field '{}'", value, field))]
    InvalidFieldValue { field: String, value: String, location: SourceLocation },
}
```

### Transform Errors

```rust
#[derive(Debug, Snafu)]
pub enum TransformError {
    #[snafu(display("Transform pass '{}' failed: {}", pass, error))]
    PassFailed { pass: String, error: String, location: SourceLocation },
    
    #[snafu(display("Circular dependency detected: {}", cycle))]
    CircularDependency { cycle: String, location: SourceLocation },
    
    #[snafu(display("Invalid pass configuration: {}", message))]
    InvalidConfiguration { message: String, location: SourceLocation },
    
    #[snafu(display("Pass '{}' not found", pass))]
    PassNotFound { pass: String, location: SourceLocation },
    
    #[snafu(display("Transform timeout after {} seconds", duration))]
    Timeout { duration: u64, location: SourceLocation },
}
```

### Generation Errors

```rust
#[derive(Debug, Snafu)]
pub enum GenerationError {
    #[snafu(display("Type mapping failed: {}", source))]
    TypeMapping { source: TypeMappingError },
    
    #[snafu(display("Code generation failed: {}", message))]
    GenerationFailed { message: String, location: SourceLocation },
    
    #[snafu(display("Validation failed: {}", message))]
    ValidationFailed { message: String, location: SourceLocation },
    
    #[snafu(display("Unsupported language: {}", language))]
    UnsupportedLanguage { language: String, location: SourceLocation },
    
    #[snafu(display("AST generation failed: {}", message))]
    AstGenerationFailed { message: String, location: SourceLocation },
}
```

## Source Location Tracking

### Source Location Structure

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub file_path: Option<PathBuf>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub openapi_path: Option<String>,
    pub context: Option<String>,
}

impl SourceLocation {
    pub fn new() -> Self {
        Self {
            file_path: None,
            line: None,
            column: None,
            openapi_path: None,
            context: None,
        }
    }
    
    pub fn file(file_path: PathBuf) -> Self {
        Self {
            file_path: Some(file_path),
            line: None,
            column: None,
            openapi_path: None,
            context: None,
        }
    }
    
    pub fn openapi(openapi_path: String) -> Self {
        Self {
            file_path: None,
            line: None,
            column: None,
            openapi_path: Some(openapi_path),
            context: None,
        }
    }
    
    pub fn with_line(mut self, line: u32) -> Self {
        self.line = Some(line);
        self
    }
    
    pub fn with_column(mut self, column: u32) -> Self {
        self.column = Some(column);
        self
    }
    
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}
```

### Location Tracking in Parsing

```rust
pub struct LocationTracker {
    current_file: Option<PathBuf>,
    current_line: u32,
    current_column: u32,
    openapi_path: Vec<String>,
}

impl LocationTracker {
    pub fn new() -> Self {
        Self {
            current_file: None,
            current_line: 1,
            current_column: 1,
            openapi_path: Vec::new(),
        }
    }
    
    pub fn set_file(&mut self, file_path: PathBuf) {
        self.current_file = Some(file_path);
        self.current_line = 1;
        self.current_column = 1;
    }
    
    pub fn advance_line(&mut self) {
        self.current_line += 1;
        self.current_column = 1;
    }
    
    pub fn advance_column(&mut self) {
        self.current_column += 1;
    }
    
    pub fn enter_openapi_path(&mut self, path: String) {
        self.openapi_path.push(path);
    }
    
    pub fn exit_openapi_path(&mut self) {
        self.openapi_path.pop();
    }
    
    pub fn current_location(&self) -> SourceLocation {
        SourceLocation {
            file_path: self.current_file.clone(),
            line: Some(self.current_line),
            column: Some(self.current_column),
            openapi_path: if self.openapi_path.is_empty() {
                None
            } else {
                Some(self.openapi_path.join("."))
            },
            context: None,
        }
    }
}
```

## User-Friendly Error Messages

### Error Message Formatting

```rust
pub struct ErrorFormatter {
    colorize: bool,
    include_context: bool,
    include_suggestions: bool,
}

impl ErrorFormatter {
    pub fn new() -> Self {
        Self {
            colorize: true,
            include_context: true,
            include_suggestions: true,
        }
    }
    
    pub fn format_error(&self, error: &Error) -> String {
        let mut message = String::new();
        
        // Add error type and message
        message.push_str(&self.format_error_header(error));
        
        // Add source location
        if let Some(location) = self.extract_location(error) {
            message.push_str(&self.format_location(&location));
        }
        
        // Add context
        if self.include_context {
            if let Some(context) = self.extract_context(error) {
                message.push_str(&self.format_context(&context));
            }
        }
        
        // Add suggestions
        if self.include_suggestions {
            if let Some(suggestions) = self.extract_suggestions(error) {
                message.push_str(&self.format_suggestions(&suggestions));
            }
        }
        
        // Add help text
        message.push_str(&self.format_help(error));
        
        message
    }
    
    fn format_error_header(&self, error: &Error) -> String {
        let color = if self.colorize { "\x1b[31m" } else { "" };
        let reset = if self.colorize { "\x1b[0m" } else { "" };
        
        format!("{}{}Error:{} {}\n", color, "error", reset, error)
    }
    
    fn format_location(&self, location: &SourceLocation) -> String {
        let mut location_str = String::new();
        
        if let Some(file) = &location.file_path {
            location_str.push_str(&format!("  --> {}\n", file.display()));
        }
        
        if let Some(openapi_path) = &location.openapi_path {
            location_str.push_str(&format!("  OpenAPI path: {}\n", openapi_path));
        }
        
        if let (Some(line), Some(column)) = (location.line, location.column) {
            location_str.push_str(&format!("  Line {}, Column {}\n", line, column));
        }
        
        location_str
    }
}
```

### Error Context and Suggestions

```rust
pub struct ErrorContext {
    pub source_code: Option<String>,
    pub surrounding_lines: Option<Vec<String>>,
    pub related_definitions: Vec<String>,
    pub suggestions: Vec<String>,
}

impl ErrorContext {
    pub fn new() -> Self {
        Self {
            source_code: None,
            surrounding_lines: None,
            related_definitions: Vec::new(),
            suggestions: Vec::new(),
        }
    }
    
    pub fn with_source_code(mut self, source: String) -> Self {
        self.source_code = Some(source);
        self
    }
    
    pub fn with_surrounding_lines(mut self, lines: Vec<String>) -> Self {
        self.surrounding_lines = Some(lines);
        self
    }
    
    pub fn add_suggestion(mut self, suggestion: String) -> Self {
        self.suggestions.push(suggestion);
        self
    }
}

// Example error with context
pub fn create_parse_error_with_context(
    message: String,
    location: SourceLocation,
    context: ErrorContext,
) -> ParseError {
    ParseError::InvalidSpec {
        message,
        location,
        context: Some(context),
    }
}
```

## Warning and Lint System

### Warning Types

```rust
#[derive(Debug, Clone)]
pub enum Warning {
    #[snafu(display("Deprecated feature used: {}", feature))]
    DeprecatedFeature { feature: String, location: SourceLocation },
    
    #[snafu(display("Non-standard OpenAPI extension: {}", extension))]
    NonStandardExtension { extension: String, location: SourceLocation },
    
    #[snafu(display("Unused definition: {}", definition))]
    UnusedDefinition { definition: String, location: SourceLocation },
    
    #[snafu(display("Potential performance issue: {}", issue))]
    PerformanceIssue { issue: String, location: SourceLocation },
    
    #[snafu(display("Code style warning: {}", message))]
    CodeStyle { message: String, location: SourceLocation },
}

#[derive(Debug, Clone)]
pub enum LintLevel {
    Error,
    Warning,
    Info,
    Hint,
}
```

### Warning Collector

```rust
pub struct WarningCollector {
    warnings: Vec<Warning>,
    max_warnings: Option<usize>,
    lint_levels: HashMap<String, LintLevel>,
}

impl WarningCollector {
    pub fn new() -> Self {
        Self {
            warnings: Vec::new(),
            max_warnings: None,
            lint_levels: HashMap::new(),
        }
    }
    
    pub fn add_warning(&mut self, warning: Warning) -> Result<(), WarningError> {
        if let Some(max) = self.max_warnings {
            if self.warnings.len() >= max {
                return Err(WarningError::TooManyWarnings { max });
            }
        }
        
        self.warnings.push(warning);
        Ok(())
    }
    
    pub fn get_warnings(&self) -> &[Warning] {
        &self.warnings
    }
    
    pub fn clear_warnings(&mut self) {
        self.warnings.clear();
    }
    
    pub fn set_lint_level(&mut self, rule: String, level: LintLevel) {
        self.lint_levels.insert(rule, level);
    }
}
```

## Diagnostic Reporting Format

### Diagnostic Structure

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub location: SourceLocation,
    pub code: Option<String>,
    pub source: Option<String>,
    pub suggestions: Vec<String>,
    pub related_information: Vec<RelatedInformation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedInformation {
    pub location: SourceLocation,
    pub message: String,
}
```

### Diagnostic Reporter

```rust
pub struct DiagnosticReporter {
    diagnostics: Vec<Diagnostic>,
    format: DiagnosticFormat,
}

#[derive(Debug, Clone)]
pub enum DiagnosticFormat {
    Human,
    Json,
    Xml,
    Lsp,
}

impl DiagnosticReporter {
    pub fn new(format: DiagnosticFormat) -> Self {
        Self {
            diagnostics: Vec::new(),
            format,
        }
    }
    
    pub fn add_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
    
    pub fn report(&self) -> String {
        match self.format {
            DiagnosticFormat::Human => self.format_human(),
            DiagnosticFormat::Json => self.format_json(),
            DiagnosticFormat::Xml => self.format_xml(),
            DiagnosticFormat::Lsp => self.format_lsp(),
        }
    }
    
    fn format_human(&self) -> String {
        let mut output = String::new();
        
        for diagnostic in &self.diagnostics {
            output.push_str(&format!("{:?}: {}\n", diagnostic.severity, diagnostic.message));
            
            if let Some(location) = &diagnostic.location {
                output.push_str(&format!("  at {}\n", location));
            }
            
            if !diagnostic.suggestions.is_empty() {
                output.push_str("  Suggestions:\n");
                for suggestion in &diagnostic.suggestions {
                    output.push_str(&format!("    - {}\n", suggestion));
                }
            }
            
            output.push_str("\n");
        }
        
        output
    }
    
    fn format_json(&self) -> String {
        serde_json::to_string_pretty(&self.diagnostics).unwrap_or_else(|_| "[]".to_string())
    }
}
```

## Recovery Strategies

### Error Recovery Trait

```rust
pub trait ErrorRecovery {
    type Error;
    type Recovered;
    
    fn can_recover(&self, error: &Self::Error) -> bool;
    fn recover(&self, error: &Self::Error) -> Result<Self::Recovered, Self::Error>;
    fn get_recovery_suggestions(&self, error: &Self::Error) -> Vec<String>;
}

pub struct ParseErrorRecovery {
    auto_fix: bool,
    suggestions: HashMap<ParseError, Vec<String>>,
}

impl ErrorRecovery for ParseErrorRecovery {
    type Error = ParseError;
    type Recovered = OpenApi;
    
    fn can_recover(&self, error: &ParseError) -> bool {
        match error {
            ParseError::MissingRequiredField { .. } => true,
            ParseError::InvalidFieldValue { .. } => true,
            ParseError::SchemaValidation { .. } => false,
            ParseError::CircularReference { .. } => false,
            _ => false,
        }
    }
    
    fn recover(&self, error: &ParseError) -> Result<OpenApi, ParseError> {
        match error {
            ParseError::MissingRequiredField { field, .. } => {
                // Try to provide default value
                self.provide_default_value(field)
            }
            ParseError::InvalidFieldValue { field, value, .. } => {
                // Try to fix the value
                self.fix_field_value(field, value)
            }
            _ => Err(error.clone()),
        }
    }
    
    fn get_recovery_suggestions(&self, error: &ParseError) -> Vec<String> {
        self.suggestions.get(error).cloned().unwrap_or_default()
    }
}
```

### Automatic Recovery

```rust
pub struct AutoRecovery {
    recovery_strategies: Vec<Box<dyn ErrorRecovery>>,
    max_recovery_attempts: usize,
}

impl AutoRecovery {
    pub fn new() -> Self {
        Self {
            recovery_strategies: Vec::new(),
            max_recovery_attempts: 3,
        }
    }
    
    pub fn add_recovery_strategy<R: ErrorRecovery + 'static>(&mut self, strategy: R) {
        self.recovery_strategies.push(Box::new(strategy));
    }
    
    pub fn attempt_recovery<E>(&self, error: &E) -> Option<E::Recovered>
    where
        E: Clone,
        Box<dyn ErrorRecovery<Error = E>>: ErrorRecovery<Recovered = E::Recovered>,
    {
        for strategy in &self.recovery_strategies {
            if strategy.can_recover(error) {
                if let Ok(recovered) = strategy.recover(error) {
                    return Some(recovered);
                }
            }
        }
        None
    }
}
```

## Error Context Propagation

### Error Context Stack

```rust
pub struct ErrorContextStack {
    contexts: Vec<ErrorContext>,
}

impl ErrorContextStack {
    pub fn new() -> Self {
        Self {
            contexts: Vec::new(),
        }
    }
    
    pub fn push_context(&mut self, context: ErrorContext) {
        self.contexts.push(context);
    }
    
    pub fn pop_context(&mut self) -> Option<ErrorContext> {
        self.contexts.pop()
    }
    
    pub fn current_context(&self) -> Option<&ErrorContext> {
        self.contexts.last()
    }
    
    pub fn add_to_current_context(&mut self, key: String, value: String) {
        if let Some(context) = self.contexts.last_mut() {
            context.add_metadata(key, value);
        }
    }
}
```

### Context-Aware Error Creation

```rust
pub struct ContextAwareErrorBuilder {
    context_stack: ErrorContextStack,
    location_tracker: LocationTracker,
}

impl ContextAwareErrorBuilder {
    pub fn new() -> Self {
        Self {
            context_stack: ErrorContextStack::new(),
            location_tracker: LocationTracker::new(),
        }
    }
    
    pub fn create_parse_error(&self, message: String) -> ParseError {
        ParseError::InvalidSpec {
            message,
            location: self.location_tracker.current_location(),
            context: self.context_stack.current_context().cloned(),
        }
    }
    
    pub fn create_transform_error(&self, pass: String, error: String) -> TransformError {
        TransformError::PassFailed {
            pass,
            error,
            location: self.location_tracker.current_location(),
        }
    }
}
```

## Testing Strategy

### Error Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let location = SourceLocation::file(PathBuf::from("test.yaml"))
            .with_line(10)
            .with_column(5);
        
        let error = ParseError::InvalidSpec {
            message: "Invalid OpenAPI spec".to_string(),
            location: location.clone(),
        };
        
        assert_eq!(error.location(), Some(location));
    }
    
    #[test]
    fn test_error_formatting() {
        let formatter = ErrorFormatter::new();
        let error = ParseError::InvalidSpec {
            message: "Test error".to_string(),
            location: SourceLocation::file(PathBuf::from("test.yaml")),
        };
        
        let formatted = formatter.format_error(&Error::Parse { source: error });
        assert!(formatted.contains("Test error"));
        assert!(formatted.contains("test.yaml"));
    }
    
    #[test]
    fn test_warning_collection() {
        let mut collector = WarningCollector::new();
        let warning = Warning::DeprecatedFeature {
            feature: "old-feature".to_string(),
            location: SourceLocation::new(),
        };
        
        collector.add_warning(warning).unwrap();
        assert_eq!(collector.get_warnings().len(), 1);
    }
}
```

### Integration Testing

```rust
#[test]
fn test_error_recovery() {
    let mut recovery = AutoRecovery::new();
    recovery.add_recovery_strategy(ParseErrorRecovery::new());
    
    let error = ParseError::MissingRequiredField {
        field: "title".to_string(),
        context: "info".to_string(),
        location: SourceLocation::new(),
    };
    
    let recovered = recovery.attempt_recovery(&error);
    assert!(recovered.is_some());
}
```

## Performance Considerations

### Error Caching

```rust
pub struct ErrorCache {
    cache: HashMap<String, CachedError>,
}

pub struct CachedError {
    error: Error,
    timestamp: Instant,
    usage_count: u64,
}

impl ErrorCache {
    pub fn get_cached_error(&self, key: &str) -> Option<&Error> {
        self.cache.get(key).map(|cached| &cached.error)
    }
    
    pub fn cache_error(&mut self, key: String, error: Error) {
        self.cache.insert(key, CachedError {
            error,
            timestamp: Instant::now(),
            usage_count: 0,
        });
    }
}
```

### Lazy Error Evaluation

```rust
pub struct LazyError {
    error_fn: Box<dyn Fn() -> Error>,
    evaluated: Option<Error>,
}

impl LazyError {
    pub fn new<F>(error_fn: F) -> Self
    where
        F: Fn() -> Error + 'static,
    {
        Self {
            error_fn: Box::new(error_fn),
            evaluated: None,
        }
    }
    
    pub fn get_error(&mut self) -> &Error {
        if self.evaluated.is_none() {
            self.evaluated = Some((self.error_fn)());
        }
        self.evaluated.as_ref().unwrap()
    }
}
```

## Conclusion

The comprehensive error handling and diagnostics system provides a robust foundation for reliable OpenAPI code generation. The structured error types, source location tracking, and user-friendly error messages ensure a good developer experience, while the warning system and recovery strategies help users fix issues quickly.

The diagnostic reporting format enables integration with various tools and IDEs, while the performance optimizations ensure efficient error handling even for large specifications.

## Related RFDs

- [RFD 0001: Overall Architecture and Design Philosophy](./0001-architecture-overview.md)
- [RFD 0002: OpenAPI Parsing with utoipa](./0002-openapi-parsing.md)
- [RFD 0004: Multi-Level Transformation Passes](./0004-transformation-passes.md)
