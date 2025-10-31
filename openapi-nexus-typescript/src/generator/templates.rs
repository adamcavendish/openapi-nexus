use rust_embed::RustEmbed;

/// Embedded template files
#[derive(RustEmbed)]
#[folder = "templates/"]
#[include = "*.j2"]
pub struct Templates;

impl Templates {
    /// Get all template names
    pub fn template_names() -> Vec<String> {
        Templates::iter().map(|file| file.to_string()).collect()
    }

    /// Get template content by name
    pub fn get_template_bytes(name: &str) -> Option<Vec<u8>> {
        Templates::get(name).map(|file| file.data.to_vec())
    }

    /// Get template content as string by name
    pub fn get_template_str(name: &str) -> Option<String> {
        Templates::get(name).and_then(|file| String::from_utf8(file.data.to_vec()).ok())
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
    fn test_templates_embedded() {
        init_test_logging();
        let template_names = Templates::template_names();
        assert!(!template_names.is_empty(), "No templates found");

        // Check that we have the expected templates
        let expected_templates = [
            "README.md.j2",
            "api/method_bodies/constructor_base_api.j2",
            "api/method_bodies/api_method_get.j2",
            "api/method_bodies/api_method_post_put_patch.j2",
            "api/method_bodies/api_method_delete.j2",
            "api/method_bodies/api_method_convenience.j2",
            "api/method_bodies/default.j2",
        ];

        for expected in &expected_templates {
            assert!(
                template_names.contains(&expected.to_string()),
                "Template {} not found in embedded templates",
                expected
            );
        }
    }

    #[test]
    fn test_get_template_content() {
        init_test_logging();
        let content = Templates::get_template_str("README.md.j2");
        assert!(content.is_some(), "README.md.j2 template not found");

        let content = content.unwrap();
        assert!(!content.is_empty(), "README.md.j2 template is empty");
        assert!(
            content.contains("package_name"),
            "README.md.j2 should contain 'package_name'"
        );
    }
}
