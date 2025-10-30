use pretty::RcDoc;
use serde::{Deserialize, Serialize};

use crate::emission::error::EmitError;
use openapi_nexus_core::traits::{EmissionContext, ToRcDocWithContext};

use super::ts_class_import_specifier::TsClassImportSpecifier;

/// Import statement for template rendering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TsImportStatement {
    pub module_path: String,
    pub imports: Vec<TsClassImportSpecifier>,
    pub is_type_only: bool,
}

impl TsImportStatement {
    /// Create a new import statement
    pub fn new(module_path: String) -> Self {
        Self {
            module_path,
            imports: Vec::new(),
            is_type_only: false,
        }
    }

    /// Add import specifier
    pub fn with_import(mut self, name: String, alias: Option<String>) -> Self {
        self.imports.push(TsClassImportSpecifier {
            name,
            alias,
            is_type: false,
        });
        self
    }

    /// Add type import specifier
    pub fn with_type_import(mut self, name: String, alias: Option<String>) -> Self {
        self.imports.push(TsClassImportSpecifier {
            name,
            alias,
            is_type: true,
        });
        self
    }

    /// Make type-only import
    pub fn with_type_only(mut self) -> Self {
        self.is_type_only = true;
        self
    }

    /// Format import statement for template rendering
    pub fn to_typescript_string(&self) -> String {
        if self.imports.is_empty() {
            return format!("import '{}';", self.module_path);
        }

        let mut import_parts = Vec::new();

        // Type-only imports
        if self.is_type_only {
            import_parts.push("type".to_string());
        }

        // Import specifiers
        let specifiers: Vec<String> = self
            .imports
            .iter()
            .map(|spec| {
                let mut s = String::new();
                if spec.is_type && !self.is_type_only {
                    s.push_str("type ");
                }
                s.push_str(&spec.name);
                if let Some(alias) = &spec.alias {
                    s.push_str(" as ");
                    s.push_str(alias);
                }
                s
            })
            .collect();

        if specifiers.len() == 1 {
            import_parts.push(format!("{{ {} }}", specifiers[0]));
        } else {
            import_parts.push(format!("{{ {} }}", specifiers.join(", ")));
        }

        import_parts.push("from".to_string());
        import_parts.push(format!("'{}'", self.module_path));

        format!("import {};", import_parts.join(" "))
    }
}

impl ToRcDocWithContext for TsImportStatement {
    type Error = EmitError;

    fn to_rcdoc_with_context(
        &self,
        _context: &EmissionContext,
    ) -> Result<RcDoc<'static, ()>, EmitError> {
        // Side-effect only import
        if self.imports.is_empty() {
            return Ok(RcDoc::text(format!("import '{}';", self.module_path)));
        }

        let mut parts = vec![RcDoc::text("import")];

        // Type-only import
        if self.is_type_only {
            parts.push(RcDoc::space());
            parts.push(RcDoc::text("type"));
        }

        // Format specifiers
        let specifier_docs: Vec<RcDoc<()>> = self
            .imports
            .iter()
            .map(|spec| {
                let mut spec_parts = Vec::new();
                if spec.is_type && !self.is_type_only {
                    spec_parts.push(RcDoc::text("type"));
                    spec_parts.push(RcDoc::space());
                }
                spec_parts.push(RcDoc::text(spec.name.clone()));
                if let Some(alias) = &spec.alias {
                    spec_parts.push(RcDoc::space());
                    spec_parts.push(RcDoc::text("as"));
                    spec_parts.push(RcDoc::space());
                    spec_parts.push(RcDoc::text(alias.clone()));
                }
                RcDoc::concat(spec_parts)
            })
            .collect();

        parts.push(RcDoc::space());
        parts.push(RcDoc::text("{"));
        parts.push(RcDoc::space());
        parts.push(RcDoc::intersperse(
            specifier_docs,
            RcDoc::text(",").append(RcDoc::space()),
        ));
        parts.push(RcDoc::space());
        parts.push(RcDoc::text("}"));
        parts.push(RcDoc::space());
        parts.push(RcDoc::text("from"));
        parts.push(RcDoc::space());
        parts.push(RcDoc::text(format!("'{}'", self.module_path)));
        parts.push(RcDoc::text(";"));

        Ok(RcDoc::concat(parts))
    }
}
