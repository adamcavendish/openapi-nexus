//! Package file generators for npm package structure

use crate::config::{PackageConfig, TypeScriptModule};
use crate::emission::file_generator::{FileType, GeneratedFile};
use utoipa::openapi::OpenApi;

/// Generator for npm package files
pub struct PackageFilesGenerator {
    config: PackageConfig,
}

impl PackageFilesGenerator {
    /// Create a new package files generator
    pub fn new(config: PackageConfig) -> Self {
        Self { config }
    }

    /// Generate package.json file
    pub fn generate_package_json(&self, _openapi: &OpenApi) -> GeneratedFile {
        let title = "openapi-client";
        let version = "1.0.0";
        let description = "OpenAPI TypeScript client";

        // Convert title to kebab-case for package name
        let package_name = self.to_kebab_case(title);
        let scoped_name = if let Some(scope) = &self.config.scope {
            format!("{}/{}", scope, package_name)
        } else {
            package_name
        };

        let mut package_json = serde_json::json!({
            "name": scoped_name,
            "version": version,
            "description": description,
            "type": "module",
            "main": "./src/index.ts",
            "types": "./src/index.ts",
            "exports": {
                ".": "./src/index.ts"
            }
        });

        // Add build scripts if configured
        if self.config.include_build_scripts {
            package_json["scripts"] = serde_json::json!({
                "build": "tsc",
                "build:esm": "tsc -p tsconfig.esm.json",
                "prepublishOnly": "npm run build"
            });
        }

        let content =
            serde_json::to_string_pretty(&package_json).unwrap_or_else(|_| "{}".to_string());

        GeneratedFile {
            filename: "package.json".to_string(),
            content,
            file_type: FileType::PackageJson,
        }
    }

    /// Generate tsconfig.json file
    pub fn generate_tsconfig(&self, _openapi: &OpenApi) -> GeneratedFile {
        let module_str = match self.config.typescript_module {
            TypeScriptModule::CommonJS => "commonjs",
            TypeScriptModule::ESNext => "esnext",
            TypeScriptModule::ES2020 => "es2020",
            TypeScriptModule::ES2022 => "es2022",
        };

        let tsconfig = serde_json::json!({
            "compilerOptions": {
                "declaration": true,
                "target": self.config.typescript_target,
                "module": module_str,
                "moduleResolution": "node",
                "outDir": "dist",
                "typeRoots": [
                    "node_modules/@types"
                ]
            },
            "exclude": [
                "dist",
                "node_modules"
            ]
        });

        let content = serde_json::to_string_pretty(&tsconfig).unwrap_or_else(|_| "{}".to_string());

        GeneratedFile {
            filename: "tsconfig.json".to_string(),
            content,
            file_type: FileType::TsConfig,
        }
    }

    /// Generate tsconfig.esm.json file
    pub fn generate_tsconfig_esm(&self, _openapi: &OpenApi) -> GeneratedFile {
        let tsconfig_esm = serde_json::json!({
            "extends": "./tsconfig.json",
            "compilerOptions": {
                "module": "esnext",
                "outDir": "dist/esm"
            }
        });

        let content =
            serde_json::to_string_pretty(&tsconfig_esm).unwrap_or_else(|_| "{}".to_string());

        GeneratedFile {
            filename: "tsconfig.esm.json".to_string(),
            content,
            file_type: FileType::TsConfigEsm,
        }
    }

    /// Generate README.md file
    pub fn generate_readme(&self, _openapi: &OpenApi) -> GeneratedFile {
        let title = "OpenAPI Client";
        let version = "1.0.0";
        let description = "OpenAPI TypeScript client";

        let package_name = if let Some(scope) = &self.config.scope {
            format!("{}/{}", scope, self.to_kebab_case(title))
        } else {
            self.to_kebab_case(title)
        };

        // Determine install path based on scope
        let install_path = if self.config.scope.is_some() {
            format!("file:path/to/{}", package_name.replace('/', "-"))
        } else {
            format!("file:path/to/{}", package_name)
        };

        // Example API class name (will be improved when we have actual API classes)
        let example_api_class = "DefaultApi";

        // Get current date
        let generated_date = chrono::Utc::now().format("%Y-%m-%d").to_string();

        // Load the template
        let template_str = include_str!("../../templates/README.md.j2");

        // Create minijinja environment
        let mut env = minijinja::Environment::new();
        env.add_template("README.md", template_str)
            .expect("Failed to add README template");

        // Prepare template context
        let ctx = minijinja::context! {
            package_name => package_name,
            title => title,
            version => version,
            description => description,
            install_path => install_path,
            example_api_class => example_api_class,
            generated_date => generated_date,
        };

        // Render the template
        let tmpl = env
            .get_template("README.md")
            .expect("Failed to get README template");
        let content = tmpl.render(ctx).expect("Failed to render README template");

        GeneratedFile {
            filename: "README.md".to_string(),
            content,
            file_type: FileType::Readme,
        }
    }

    /// Convert a string to kebab-case
    fn to_kebab_case(&self, s: &str) -> String {
        let mut result = String::new();
        for (i, c) in s.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('-');
            }
            result.push(c.to_lowercase().next().unwrap());
        }
        result
    }
}

impl Default for PackageFilesGenerator {
    fn default() -> Self {
        Self::new(PackageConfig::default())
    }
}
