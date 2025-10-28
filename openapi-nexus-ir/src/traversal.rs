//! Traversal utilities for OpenAPI specifications

use utoipa::openapi::{
    OpenApi, Paths, RefOr, Response, Schema,
    path::{Operation, Parameter},
};

/// Visitor pattern for traversing OpenAPI specifications
pub trait OpenApiVisitor {
    type Error;

    /// Visit the root OpenAPI specification
    fn visit_openapi(&mut self, _openapi: &OpenApi) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit the paths section
    fn visit_paths(&mut self, _paths: &Paths) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit an operation
    fn visit_operation(&mut self, _path: &str, _operation: &Operation) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a schema definition
    fn visit_schema(&mut self, _name: &str, _schema: &RefOr<Schema>) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a response definition
    fn visit_response(
        &mut self,
        _name: &str,
        _response: &RefOr<Response>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    /// Visit a parameter definition
    fn visit_parameter(
        &mut self,
        _name: &str,
        _parameter: &RefOr<Parameter>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// Traverse an OpenAPI specification using the visitor pattern
pub struct OpenApiTraverser;

impl OpenApiTraverser {
    /// Traverse an OpenAPI specification with a visitor
    pub fn traverse<V: OpenApiVisitor>(openapi: &OpenApi, visitor: &mut V) -> Result<(), V::Error> {
        // Visit the root OpenAPI specification
        visitor.visit_openapi(openapi)?;

        // Visit paths
        visitor.visit_paths(&openapi.paths)?;

        // Visit all operations
        for (path, path_item) in &openapi.paths.paths {
            // Visit each HTTP method operation
            if let Some(op) = &path_item.get {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.post {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.put {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.delete {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.patch {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.head {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.options {
                visitor.visit_operation(path, op)?;
            }
            if let Some(op) = &path_item.trace {
                visitor.visit_operation(path, op)?;
            }
        }

        // Visit components if they exist
        if let Some(components) = &openapi.components {
            // Visit schemas
            for (name, schema) in &components.schemas {
                visitor.visit_schema(name, schema)?;
            }

            // Visit responses
            for (name, response) in &components.responses {
                visitor.visit_response(name, response)?;
            }

            // Note: utoipa Components doesn't have a parameters field
            // Parameters are typically defined inline in operations
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use utoipa::openapi::schema::Object;
    use utoipa::openapi::{Components, Info, OpenApi, Paths, RefOr, Schema};

    use super::*;

    fn create_test_openapi() -> OpenApi {
        let mut components = Components::new();

        // Add a simple schema
        let user_schema = Object::new();
        components
            .schemas
            .insert("User".to_string(), RefOr::T(Schema::Object(user_schema)));

        let info = Info::new("Test API", "1.0.0");
        let paths = Paths::new();

        let mut openapi = OpenApi::new(info, paths);
        openapi.components = Some(components);
        openapi
    }

    #[test]
    fn test_visitor_pattern() {
        let openapi = create_test_openapi();

        struct TestVisitor {
            schema_count: usize,
            operation_count: usize,
        }

        impl OpenApiVisitor for TestVisitor {
            type Error = crate::error::IrError;

            fn visit_schema(
                &mut self,
                name: &str,
                _schema: &RefOr<Schema>,
            ) -> Result<(), Self::Error> {
                assert_eq!(name, "User");
                self.schema_count += 1;
                Ok(())
            }

            fn visit_operation(
                &mut self,
                _path: &str,
                _operation: &utoipa::openapi::path::Operation,
            ) -> Result<(), Self::Error> {
                self.operation_count += 1;
                Ok(())
            }
        }

        let mut visitor = TestVisitor {
            schema_count: 0,
            operation_count: 0,
        };

        let result = OpenApiTraverser::traverse(&openapi, &mut visitor);
        assert!(result.is_ok());

        assert_eq!(visitor.schema_count, 1);
        assert_eq!(visitor.operation_count, 0); // No operations in test OpenAPI
    }

    #[test]
    fn test_visitor_error_handling() {
        let openapi = create_test_openapi();

        struct ErrorVisitor;

        impl OpenApiVisitor for ErrorVisitor {
            type Error = crate::error::IrError;

            fn visit_schema(
                &mut self,
                _name: &str,
                _schema: &RefOr<Schema>,
            ) -> Result<(), Self::Error> {
                Err(crate::error::IrError::AnalysisError {
                    message: "Test error".to_string(),
                    location: openapi_nexus_common::SourceLocation::new(),
                })
            }
        }

        let mut visitor = ErrorVisitor;
        let result = OpenApiTraverser::traverse(&openapi, &mut visitor);
        assert!(result.is_err());

        if let Err(crate::error::IrError::AnalysisError { message, .. }) = result {
            assert_eq!(message, "Test error");
        } else {
            panic!("Expected AnalysisError");
        }
    }
}
