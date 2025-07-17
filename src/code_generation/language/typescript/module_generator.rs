use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::project_management::config::models::{Module, CodeFile, Project};
use crate::shared::utils::content_updater::ContentUpdater;

/// TypeScript-specific module generator
pub struct TypeScriptModuleGenerator;

impl TypeScriptModuleGenerator {
    /// Generate TypeScript module structure recursively
    pub fn generate_module<P: AsRef<Path>>(
        base_path: P,
        module: &Module,
        parent_modules: &[String],
    ) -> Result<()> {
        let module_path = base_path.as_ref().join(&module.name);
        
        // Create directory
        fs::create_dir_all(&module_path)
            .with_context(|| format!("Failed to create directory: {}", module_path.display()))?;

        // Collect all submodule names and code file names for index.ts
        let mut export_declarations = Vec::new();

        // Generate code files in this module
        for codefile in module.files() {
            let filename = codefile.filename_with_extension("typescript");
            let file_path = module_path.join(&filename);
            
            // Create empty TypeScript file (only if it doesn't exist)
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }

            // Add to export declarations if it's not index.ts
            if filename != "index.ts" {
                // Use name without extension for TypeScript module resolution
                let module_name = Self::get_module_name_for_export(codefile);
                export_declarations.push(format!("export * from './{}';", module_name));
            }
        }

        // Process submodules recursively
        for submodule in module.submodules() {
            let mut new_parent_modules = parent_modules.to_vec();
            new_parent_modules.push(module.name().to_string());
            
            Self::generate_module(&module_path, submodule, &new_parent_modules)?;
            
            // Add submodule export declaration
            export_declarations.push(format!("export * from './{}';", submodule.name()));
        }

        // Generate index.ts for all modules
        let index_ts_path = module_path.join("index.ts");
        ContentUpdater::update_js_index_file(&index_ts_path, &export_declarations)?;

        Ok(())
    }

    /// Get module name for export statement (removes extension)
    fn get_module_name_for_export(codefile: &CodeFile) -> String {
        let name = codefile.name();
        // Remove common TypeScript extensions for proper module resolution
        if name.ends_with(".ts") || name.ends_with(".tsx") || name.ends_with(".vue") || name.ends_with(".js") || name.ends_with(".jsx") {
            let dot_pos = name.rfind('.').unwrap();
            name[..dot_pos].to_string()
        } else {
            name.to_string()
        }
    }

    /// Generate package.json for TypeScript project
    pub fn generate_package_json<P: AsRef<Path>>(
        project_path: P,
        project_name: &str,
    ) -> Result<()> {
        let package_json_path = project_path.as_ref().join("package.json");
        let package_content = Self::generate_package_json_content(project_name);
        
        // Only create package.json if it doesn't already exist
        if !package_json_path.exists() {
            fs::write(&package_json_path, package_content)
                .with_context(|| format!("Failed to create package.json: {}", package_json_path.display()))?;
        }

        Ok(())
    }

    /// Generate tsconfig.json for TypeScript project
    pub fn generate_tsconfig_json<P: AsRef<Path>>(
        project_path: P,
    ) -> Result<()> {
        let tsconfig_path = project_path.as_ref().join("tsconfig.json");
        let tsconfig_content = Self::generate_tsconfig_content();
        
        // Only create tsconfig.json if it doesn't already exist
        if !tsconfig_path.exists() {
            fs::write(&tsconfig_path, tsconfig_content)
                .with_context(|| format!("Failed to create tsconfig.json: {}", tsconfig_path.display()))?;
        }

        Ok(())
    }

    /// Generate index.ts file content
    fn generate_index_ts_content(export_declarations: &[String]) -> String {
        format!(
            "// start auto exported by saba.\n{}\n// end auto exported by saba.\n\n",
            export_declarations.join("\n")
        )
    }

    /// Generate package.json content
    fn generate_package_json_content(project_name: &str) -> String {
        format!(
            r#"{{
  "name": "{}",
  "version": "1.0.0",
  "description": "",
  "main": "dist/index.js",
  "scripts": {{
    "build": "tsc",
    "dev": "tsc --watch",
    "start": "node dist/index.js"
  }},
  "devDependencies": {{
    "typescript": "^5.0.0",
    "@types/node": "^18.0.0"
  }},
  "keywords": [],
  "author": "",
  "license": "ISC"
}}
"#,
            project_name
        )
    }

    /// Generate tsconfig.json content
    fn generate_tsconfig_content() -> String {
        r#"{
  "compilerOptions": {
    "target": "ES2020",
    "module": "commonjs",
    "outDir": "./dist",
    "rootDir": "./src",
    "strict": true,
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true,
    "declaration": true,
    "declarationMap": true,
    "sourceMap": true
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
"#.to_string()
    }
}