//! TypeScript code generation using Minijinja templates

use minijinja::Environment;

/// Template-based code generator for TypeScript
pub struct TemplateGenerator {
    env: Environment<'static>,
}

impl TemplateGenerator {
    /// Create a new template generator
    pub fn new() -> Self {
        let mut env = Environment::new();

        // Load AST templates from embedded strings
        env.add_template("interface", include_str!("../../templates/interface.j2"))
            .unwrap();
        env.add_template("type_alias", include_str!("../../templates/type_alias.j2"))
            .unwrap();
        env.add_template("enum", include_str!("../../templates/enum.j2"))
            .unwrap();
        env.add_template("class", include_str!("../../templates/class.j2"))
            .unwrap();
        env.add_template("function", include_str!("../../templates/function.j2"))
            .unwrap();

        // Load package templates
        env.add_template("runtime", include_str!("../../templates/runtime.ts.j2"))
            .unwrap();
        env.add_template("readme", include_str!("../../templates/README.md.j2"))
            .unwrap();

        // Load method body templates
        env.add_template(
            "base_api_request",
            include_str!("../../templates/method_bodies/base_api_request.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor",
            include_str!("../../templates/method_bodies/constructor.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor_base_api",
            include_str!("../../templates/method_bodies/constructor_base_api.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor_required_error",
            include_str!("../../templates/method_bodies/constructor_required_error.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor_with_extends",
            include_str!("../../templates/method_bodies/constructor_with_extends.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor_default",
            include_str!("../../templates/method_bodies/constructor_default.j2"),
        )
        .unwrap();
        env.add_template(
            "constructor_enhanced",
            include_str!("../../templates/method_bodies/constructor_enhanced.j2"),
        )
        .unwrap();
        env.add_template(
            "http_method",
            include_str!("../../templates/method_bodies/http_method.j2"),
        )
        .unwrap();
        env.add_template(
            "api_method",
            include_str!("../../templates/method_bodies/api_method.j2"),
        )
        .unwrap();
        env.add_template(
            "api_method_get",
            include_str!("../../templates/method_bodies/api_method_get.j2"),
        )
        .unwrap();
        env.add_template(
            "api_method_post_put",
            include_str!("../../templates/method_bodies/api_method_post_put.j2"),
        )
        .unwrap();
        env.add_template(
            "api_method_delete",
            include_str!("../../templates/method_bodies/api_method_delete.j2"),
        )
        .unwrap();
        env.add_template(
            "default_method",
            include_str!("../../templates/method_bodies/default.j2"),
        )
        .unwrap();

        Self { env }
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

    // === SnippetLines Integration Methods ===

    /// Generate method body lines for BaseAPI.request
    pub fn generate_base_api_request_lines(&self) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("base_api_request")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate constructor body lines
    pub fn generate_constructor_lines(
        &self,
        class_name: &str,
        extends: &Option<String>,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor")?;
        let data = serde_json::json!({
            "class_name": class_name,
            "extends": extends
        });
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate BaseAPI constructor body lines
    pub fn generate_base_api_constructor_lines(&self) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor_base_api")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate RequiredError constructor body lines
    pub fn generate_required_error_constructor_lines(
        &self,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor_required_error")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate constructor body lines for classes with extends
    pub fn generate_extends_constructor_lines(&self) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor_with_extends")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate default constructor body lines
    pub fn generate_default_constructor_lines(&self) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor_default")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate HTTP method body lines
    pub fn generate_http_method_lines(
        &self,
        method_name: &str,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("http_method")?;
        let data = serde_json::json!({
            "method_name": method_name
        });
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate API method body lines
    pub fn generate_api_method_lines(
        &self,
        return_type: &str,
        has_body_param: bool,
        http_method: &str,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("api_method")?;
        let data = serde_json::json!({
            "return_type": return_type,
            "has_body_param": has_body_param,
            "http_method": http_method
        });
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate default method body lines
    pub fn generate_default_method_lines(&self) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("default_method")?;
        let rendered = template.render(serde_json::Value::Null)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate complex API method body lines
    pub fn generate_complex_api_method_lines(
        &self,
        data: &ApiMethodData,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("api_method")?;
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate enhanced constructor body lines
    pub fn generate_enhanced_constructor_lines(
        &self,
        data: &ConstructorData,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("constructor_enhanced")?;
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate GET method body lines
    pub fn generate_get_method_lines(
        &self,
        data: &ApiMethodData,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("api_method_get")?;
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate POST/PUT method body lines
    pub fn generate_post_put_method_lines(
        &self,
        data: &ApiMethodData,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("api_method_post_put")?;
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Generate DELETE method body lines
    pub fn generate_delete_method_lines(
        &self,
        data: &ApiMethodData,
    ) -> Result<Vec<String>, minijinja::Error> {
        let template = self.env.get_template("api_method_delete")?;
        let rendered = template.render(data)?;
        Ok(self.split_template_lines(&rendered))
    }

    /// Split template output into individual lines, trimming whitespace
    fn split_template_lines(&self, template_output: &str) -> Vec<String> {
        template_output
            .lines()
            .map(|line| line.trim_end().to_string())
            .collect()
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

/// Data structure for complex API method generation
#[derive(serde::Serialize)]
pub struct ApiMethodData {
    pub method_name: String,
    pub http_method: String,
    pub path: String,
    pub path_params: Vec<ParameterData>,
    pub query_params: Vec<ParameterData>,
    pub body_param: Option<ParameterData>,
    pub return_type: String,
    pub has_auth: bool,
    pub has_error_handling: bool,
}

/// Data structure for enhanced constructor generation
#[derive(serde::Serialize)]
pub struct ConstructorData {
    pub class_name: String,
    pub extends: Option<String>,
    pub has_configuration: bool,
    pub has_super_call: bool,
    pub additional_statements: Vec<String>,
}

impl Default for TemplateGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interface_template() {
        let generator = TemplateGenerator::new();

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
