//! TypeScript code generation using Minijinja templates

use std::fmt;

use minijinja::Environment;
use rust_embed::RustEmbed;

use crate::generator::templates::Templates;

/// Embedded template files
#[derive(RustEmbed)]
#[folder = "templates/"]
#[include = "*.j2"]
#[include = "*.ts"]
struct AllTemplates;

/// Available templates for TypeScript code generation
#[derive(Debug, Clone)]
pub enum Template {
    /// README.md template
    Readme(ReadmeData),
    /// Base API request method body
    BaseApiRequest,
    /// Base API constructor body
    ConstructorBaseApi,
    /// Required error constructor body
    ConstructorRequiredError,
    /// Constructor body for classes with extends
    ConstructorWithExtends,
    /// Default constructor body
    ConstructorDefault,
    /// GET method body
    ApiMethodGet(ApiMethodData),
    /// POST/PUT/PATCH method body
    ApiMethodPostPutPatch(ApiMethodData),
    /// DELETE method body
    ApiMethodDelete(ApiMethodData),
    /// Default method body
    DefaultMethod,
}

/// Template file path mapping
const TEMPLATE_PATHS: &[(&str, &str)] = &[
    ("readme", "README.md.j2"),
    (
        "constructor_base_api",
        "api/method_bodies/constructor_base_api.j2",
    ),
    ("api_method_get", "api/method_bodies/api_method_get.j2"),
    (
        "api_method_post_put",
        "api/method_bodies/api_method_post_put.j2",
    ),
    (
        "api_method_delete",
        "api/method_bodies/api_method_delete.j2",
    ),
    (
        "api_method_convenience",
        "api/method_bodies/api_method_convenience.j2",
    ),
    ("default_method", "api/method_bodies/default.j2"),
];

/// Template-based code generator for TypeScript
#[derive(Debug, Clone)]
pub struct TemplateGenerator {
    env: Environment<'static>,
}

impl TemplateGenerator {
    /// Create a new empty template generator with no templates loaded
    fn new_empty() -> Self {
        Self {
            env: Environment::new(),
        }
    }

    /// Create a new template generator
    pub fn new() -> Self {
        let mut t = Self::new_empty();

        // Load all templates using for loop
        for (template_name, file_path) in TEMPLATE_PATHS {
            if let Some(content) = Templates::get_template_str(file_path) {
                t.env
                    .add_template_owned(template_name.to_string(), content)
                    .unwrap_or_else(|e| {
                        panic!("Failed to add template '{}': {}", template_name, e)
                    });
            } else {
                panic!(
                    "Template file '{}' not found in embedded resources",
                    file_path
                );
            }
        }

        t
    }

    /// Generate code using the specified template
    pub fn generate(&self, template: &Template) -> Result<String, minijinja::Error> {
        let tmpl = self.env.get_template(template.to_string().as_str())?;

        match template {
            Template::Readme(data) => tmpl.render(data),
            Template::ApiMethodGet(data) => tmpl.render(data),
            Template::ApiMethodPostPutPatch(data) => tmpl.render(data),
            Template::ApiMethodDelete(data) => tmpl.render(data),
            _ => tmpl.render(serde_json::Value::Null),
        }
    }

    /// Generate code lines using the specified template
    pub fn generate_lines(&self, template: &Template) -> Result<Vec<String>, minijinja::Error> {
        let rendered = self.generate(template)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Split template output into individual lines, trimming whitespace
    fn split_template_lines(&self, template_output: &str) -> Vec<String> {
        template_output
            .lines()
            .map(|line| line.trim_end().to_string())
            .collect()
    }

    /// Get all runtime static files (ending with .ts in runtime/ directory) with content
    pub fn get_runtime_static_files() -> Vec<(String, String)> {
        AllTemplates::iter()
            .filter(|file| file.starts_with("runtime/") && file.ends_with(".ts"))
            .filter_map(|file| {
                AllTemplates::get(&file)
                    .and_then(|file_data| String::from_utf8(file_data.data.to_vec()).ok())
                    .map(|content| {
                        // Extract filename without runtime/ prefix but keep .ts extension
                        let filename = file.strip_prefix("runtime/").unwrap_or(&file).to_string();
                        (filename, content)
                    })
            })
            .collect()
    }

    /// Get runtime static files organized by category
    pub fn get_runtime_files_by_category()
    -> std::collections::HashMap<String, Vec<(String, String)>> {
        let mut categories = std::collections::HashMap::new();

        for (filename, content) in Self::get_runtime_static_files() {
            let category = if filename.starts_with("interfaces/") {
                "interfaces"
            } else if filename.starts_with("classes/") {
                "classes"
            } else if filename.starts_with("functions/") {
                "functions"
            } else if filename.starts_with("constants/") {
                "constants"
            } else if filename.starts_with("types/") {
                "types"
            } else {
                "core"
            };

            categories
                .entry(category.to_string())
                .or_insert_with(Vec::new)
                .push((filename, content));
        }

        categories
    }

    /// Get a specific runtime file by name
    pub fn get_runtime_file(filename: &str) -> Option<String> {
        let full_path = format!("runtime/{}", filename);
        AllTemplates::get(&full_path)
            .and_then(|file_data| String::from_utf8(file_data.data.to_vec()).ok())
    }
}

impl fmt::Display for Template {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Template::Readme(_) => write!(f, "readme"),
            Template::BaseApiRequest => write!(f, "base_api_request"),
            Template::ConstructorBaseApi => write!(f, "constructor_base_api"),
            Template::ConstructorRequiredError => write!(f, "constructor_required_error"),
            Template::ConstructorWithExtends => write!(f, "constructor_with_extends"),
            Template::ConstructorDefault => write!(f, "constructor_default"),
            Template::ApiMethodGet(_) => write!(f, "api_method_get"),
            Template::ApiMethodPostPutPatch(_) => write!(f, "api_method_post_put"),
            Template::ApiMethodDelete(_) => write!(f, "api_method_delete"),
            Template::DefaultMethod => write!(f, "default_method"),
        }
    }
}

/// Data structure for parameter generation
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ParameterData {
    pub name: String,
    pub type_expr: Option<String>,
    pub optional: bool,
}

/// Data structure for README.md generation
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ReadmeData {
    pub package_name: String,
    pub title: String,
    pub version: String,
    pub description: String,
    pub install_path: String,
    pub example_api_class: String,
    pub generated_date: String,
}

/// Data structure for complex API method generation
#[derive(Debug, Default, Clone, serde::Serialize)]
pub struct ApiMethodData {
    pub method_name: String,
    pub http_method: String,
    pub path: String,
    pub path_params: Vec<ParameterData>,
    pub query_params: Vec<ParameterData>,
    pub header_params: Vec<ParameterData>,
    pub body_param: Option<ParameterData>,
    pub return_type: String,
    pub has_auth: bool,
    pub has_error_handling: bool,
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_test_logging() {
        let _ = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::WARN)
            .try_init();
    }

    #[test]
    fn test_new_empty() {
        init_test_logging();
        let _t = TemplateGenerator::new_empty();
        // Test that the generator can be created successfully with no templates
    }

    #[test]
    fn test_template_display() {
        init_test_logging();
        let readme_data = ReadmeData {
            package_name: "test-package".to_string(),
            title: "Test Package".to_string(),
            version: "1.0.0".to_string(),
            description: "Test description".to_string(),
            install_path: "test-path".to_string(),
            example_api_class: "TestApi".to_string(),
            generated_date: "2024-01-01".to_string(),
        };

        assert_eq!(Template::Readme(readme_data.clone()).to_string(), "readme");
        assert_eq!(Template::BaseApiRequest.to_string(), "base_api_request");
        assert_eq!(
            Template::ConstructorBaseApi.to_string(),
            "constructor_base_api"
        );
        assert_eq!(
            Template::ConstructorRequiredError.to_string(),
            "constructor_required_error"
        );
        assert_eq!(
            Template::ConstructorWithExtends.to_string(),
            "constructor_with_extends"
        );
        assert_eq!(
            Template::ConstructorDefault.to_string(),
            "constructor_default"
        );
        assert_eq!(
            Template::ApiMethodGet(ApiMethodData {
                method_name: "test".to_string(),
                http_method: "GET".to_string(),
                path: "/test".to_string(),
                path_params: vec![],
                query_params: vec![],
                header_params: vec![],
                body_param: None,
                return_type: "Promise<any>".to_string(),
                has_auth: false,
                has_error_handling: false,
            })
            .to_string(),
            "api_method_get"
        );
        assert_eq!(
            Template::ApiMethodPostPutPatch(ApiMethodData {
                method_name: "test".to_string(),
                http_method: "POST".to_string(),
                path: "/test".to_string(),
                path_params: vec![],
                query_params: vec![],
                header_params: vec![],
                body_param: None,
                return_type: "Promise<any>".to_string(),
                has_auth: false,
                has_error_handling: false,
            })
            .to_string(),
            "api_method_post_put"
        );
        assert_eq!(
            Template::ApiMethodDelete(ApiMethodData {
                method_name: "test".to_string(),
                http_method: "DELETE".to_string(),
                path: "/test".to_string(),
                path_params: vec![],
                query_params: vec![],
                header_params: vec![],
                body_param: None,
                return_type: "Promise<any>".to_string(),
                has_auth: false,
                has_error_handling: false,
            })
            .to_string(),
            "api_method_delete"
        );
        assert_eq!(Template::DefaultMethod.to_string(), "default_method");
    }

    #[test]
    fn test_template_generator_creation() {
        init_test_logging();
        let _generator = TemplateGenerator::new();
        // Test that the generator can be created successfully
    }
}
