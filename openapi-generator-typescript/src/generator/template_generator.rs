//! TypeScript code generation using Minijinja templates

use minijinja::Environment;

/// Template-based code generator for TypeScript
pub struct TemplateGenerator {
    env: Environment<'static>,
}

impl TemplateGenerator {
    /// Create a new template generator
    pub fn new() -> Result<Self, minijinja::Error> {
        let mut env = Environment::new();

        // Load AST templates from embedded strings
        env.add_template("interface", include_str!("../../templates/interface.j2"))?;
        env.add_template("type_alias", include_str!("../../templates/type_alias.j2"))?;
        env.add_template("enum", include_str!("../../templates/enum.j2"))?;
        env.add_template("class", include_str!("../../templates/class.j2"))?;
        env.add_template("function", include_str!("../../templates/function.j2"))?;

        // Load package templates
        env.add_template("runtime", include_str!("../../templates/runtime.ts.j2"))?;
        env.add_template("readme", include_str!("../../templates/README.md.j2"))?;

        // Load method body templates
        env.add_template(
            "base_api_request",
            include_str!("../../templates/method_bodies/base_api_request.j2"),
        )?;
        env.add_template(
            "constructor",
            include_str!("../../templates/method_bodies/constructor.j2"),
        )?;
        env.add_template(
            "http_method",
            include_str!("../../templates/method_bodies/http_method.j2"),
        )?;
        env.add_template(
            "api_method",
            include_str!("../../templates/method_bodies/api_method.j2"),
        )?;
        env.add_template(
            "default_method",
            include_str!("../../templates/method_bodies/default.j2"),
        )?;

        Ok(Self { env })
    }

    /// Generate interface code
    pub fn generate_interface(&self, data: &InterfaceData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("interface")?;
        template.render(data)
    }

    /// Generate type alias code
    pub fn generate_type_alias(&self, data: &TypeAliasData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("type_alias")?;
        template.render(data)
    }

    /// Generate enum code
    pub fn generate_enum(&self, data: &EnumData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("enum")?;
        template.render(data)
    }

    /// Generate class code
    pub fn generate_class(&self, data: &ClassData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("class")?;
        template.render(data)
    }

    /// Generate function code
    pub fn generate_function(&self, data: &FunctionData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("function")?;
        template.render(data)
    }

    /// Generate runtime.ts code
    pub fn generate_runtime(&self, data: &RuntimeData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("runtime")?;
        template.render(data)
    }

    /// Generate README.md content
    pub fn generate_readme(&self, data: &ReadmeData) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("readme")?;
        template.render(data)
    }

    /// Generate method body for BaseAPI.request
    pub fn generate_base_api_request_body(&self) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("base_api_request")?;
        template.render(serde_json::Value::Null)
    }

    /// Generate constructor body
    pub fn generate_constructor_body(
        &self,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("constructor")?;
        let data = serde_json::json!({
            "class_name": class_name,
            "extends": extends
        });
        template.render(data)
    }

    /// Generate HTTP method body
    pub fn generate_http_method_body(&self, method_name: &str) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("http_method")?;
        let data = serde_json::json!({
            "method_name": method_name
        });
        template.render(data)
    }

    /// Generate API method body
    pub fn generate_api_method_body(
        &self,
        return_type: &str,
        has_body_param: bool,
        http_method: &str,
    ) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("api_method")?;
        let data = serde_json::json!({
            "return_type": return_type,
            "has_body_param": has_body_param,
            "http_method": http_method
        });
        template.render(data)
    }

    /// Generate default method body
    pub fn generate_default_method_body(&self) -> Result<String, minijinja::Error> {
        let template = self.env.get_template("default_method")?;
        template.render(serde_json::Value::Null)
    }
}

/// Data structure for interface generation
#[derive(serde::Serialize)]
pub struct InterfaceData {
    pub name: String,
    pub documentation: Option<String>,
    pub generics: Vec<String>,
    pub extends: Vec<String>,
    pub properties: Vec<PropertyData>,
}

/// Data structure for type alias generation
#[derive(serde::Serialize)]
pub struct TypeAliasData {
    pub name: String,
    pub documentation: Option<String>,
    pub generics: Vec<String>,
    pub type_expr: String,
}

/// Data structure for enum generation
#[derive(serde::Serialize)]
pub struct EnumData {
    pub name: String,
    pub documentation: Option<String>,
    pub variants: Vec<EnumVariantData>,
}

/// Data structure for class generation
#[derive(serde::Serialize)]
pub struct ClassData {
    pub name: String,
    pub documentation: Option<String>,
    pub generics: Vec<String>,
    pub properties: Vec<PropertyData>,
    pub methods: Vec<MethodData>,
}

/// Data structure for function generation
#[derive(serde::Serialize)]
pub struct FunctionData {
    pub name: String,
    pub documentation: Option<String>,
    pub is_async: bool,
    pub parameters: Vec<ParameterData>,
    pub return_type: Option<String>,
    pub body: Option<String>,
}

/// Data structure for property generation
#[derive(serde::Serialize)]
pub struct PropertyData {
    pub name: String,
    pub type_expr: String,
    pub optional: bool,
    pub documentation: Option<String>,
}

/// Data structure for method generation
#[derive(serde::Serialize)]
pub struct MethodData {
    pub name: String,
    pub documentation: Option<String>,
    pub is_async: bool,
    pub parameters: Vec<ParameterData>,
    pub return_type: Option<String>,
    pub body: Option<String>,
}

/// Data structure for parameter generation
#[derive(serde::Serialize)]
pub struct ParameterData {
    pub name: String,
    pub type_expr: Option<String>,
    pub optional: bool,
}

/// Data structure for enum variant generation
#[derive(serde::Serialize)]
pub struct EnumVariantData {
    pub name: String,
    pub value: Option<String>,
    pub documentation: Option<String>,
}

/// Data structure for runtime.ts generation
#[derive(serde::Serialize)]
pub struct RuntimeData {
    pub title: String,
    pub version: String,
    pub description: String,
}

/// Data structure for README.md generation
#[derive(serde::Serialize)]
pub struct ReadmeData {
    pub package_name: String,
    pub title: String,
    pub version: String,
    pub description: String,
    pub install_path: String,
    pub example_api_class: String,
    pub generated_date: String,
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to create template generator")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_template() {
        let generator = TemplateGenerator::new().unwrap();

        let data = InterfaceData {
            name: "Pet".to_string(),
            documentation: Some("Pet model".to_string()),
            generics: vec![],
            extends: vec![],
            properties: vec![
                PropertyData {
                    name: "id".to_string(),
                    type_expr: "number".to_string(),
                    optional: true,
                    documentation: None,
                },
                PropertyData {
                    name: "name".to_string(),
                    type_expr: "string".to_string(),
                    optional: false,
                    documentation: None,
                },
            ],
        };

        let result = generator.generate_interface(&data).unwrap();
        assert!(result.contains("export interface Pet"));
        assert!(result.contains("id?: number"));
        assert!(result.contains("name: string"));
    }
}
