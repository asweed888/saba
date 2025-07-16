use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::project_management::config::models::{Module, CodeFile, Project};
use crate::shared::utils::content_updater::ContentUpdater;

/// Rust-specific module generator
pub struct RustModuleGenerator;

impl RustModuleGenerator {
    /// Convert pub setting to Rust visibility prefix
    /// target_type: "main", "lib", or "mod"
    fn get_visibility_prefix(pub_setting: Option<&str>, target_type: &str) -> &'static str {
        match pub_setting {
            Some("yes") => "pub ",
            Some("no") => "",
            Some("crate") => "pub(crate) ",
            Some("super") => "pub(super) ",
            None => {
                // Use defaults based on target type
                match target_type {
                    "main" => "", // main.rs defaults to private
                    "lib" | "mod" => "pub ", // lib.rs and mod.rs default to public
                    _ => "pub ",
                }
            }
            _ => "pub ", // fallback to public for unknown values
        }
    }
    /// Generate Rust module structure recursively
    pub fn generate_module<P: AsRef<Path>>(
        base_path: P,
        module: &Module,
        parent_modules: &[String],
    ) -> Result<()> {
        let module_path = base_path.as_ref().join(&module.name);
        
        // Create directory
        fs::create_dir_all(&module_path)
            .with_context(|| format!("Failed to create directory: {}", module_path.display()))?;

        // Collect all submodule names and code file names for mod.rs
        let mut module_declarations = Vec::new();

        // Generate code files in this module
        for codefile in module.files() {
            let filename = codefile.filename_with_extension("rust");
            let file_path = module_path.join(&filename);
            
            // Create empty Rust file (only if it doesn't exist)
            if !file_path.exists() {
                fs::write(&file_path, "")
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }

            // Add to module declarations if it's not mod.rs
            if filename != "mod.rs" {
                let module_name = codefile.name();
                let visibility = Self::get_visibility_prefix(codefile.pub_setting(), "mod");
                module_declarations.push(format!("{}mod {};", visibility, module_name));
            }
        }

        // Process submodules recursively
        for submodule in module.submodules() {
            let mut new_parent_modules = parent_modules.to_vec();
            new_parent_modules.push(module.name().to_string());
            
            Self::generate_module(&module_path, submodule, &new_parent_modules)?;
            
            // Add submodule declaration using the submodule's pub setting
            let visibility = Self::get_visibility_prefix(submodule.pub_setting(), "mod");
            module_declarations.push(format!("{}mod {};", visibility, submodule.name()));
        }

        // Generate mod.rs if there are module declarations and this is not a src module
        // (src modules use main.rs or lib.rs instead of mod.rs)
        if !module_declarations.is_empty() && module.name() != "src" {
            let mod_rs_path = module_path.join("mod.rs");
            ContentUpdater::update_rust_module_file(&mod_rs_path, &module_declarations, None)?;
        }

        Ok(())
    }

    /// Generate main.rs content for root project
    pub fn generate_main_rs<P: AsRef<Path>>(
        project_path: P,
        src_modules: &[Module],
    ) -> Result<()> {
        let main_rs_path = project_path.as_ref().join("src").join("main.rs");
        
        // Collect module declarations from src module contents
        let mut module_declarations = Vec::new();
        
        for src_module in src_modules {
            if src_module.name() == "src" {
                // Add declarations for submodules and code files within src
                for codefile in src_module.files() {
                    let filename = codefile.filename_with_extension("rust");
                    if filename != "mod.rs" && filename != "main.rs" && filename != "lib.rs" {
                        let visibility = Self::get_visibility_prefix(codefile.pub_setting(), "main");
                        module_declarations.push(format!("{}mod {};", visibility, codefile.name()));
                    }
                }
                
                for submodule in src_module.submodules() {
                    let visibility = Self::get_visibility_prefix(submodule.pub_setting(), "main");
                    module_declarations.push(format!("{}mod {};", visibility, submodule.name()));
                }
            }
        }

        let main_function = "fn main() {\n    println!(\"Hello, world!\");\n}";
        ContentUpdater::update_rust_module_file(&main_rs_path, &module_declarations, Some(main_function))?;

        Ok(())
    }

    /// Generate lib.rs content for library project
    pub fn generate_lib_rs<P: AsRef<Path>>(
        project_path: P,
        src_modules: &[Module],
    ) -> Result<()> {
        let lib_rs_path = project_path.as_ref().join("src").join("lib.rs");
        
        // Collect module declarations from src module contents
        let mut module_declarations = Vec::new();
        
        for src_module in src_modules {
            if src_module.name() == "src" {
                // Add declarations for submodules and code files within src
                for codefile in src_module.files() {
                    let filename = codefile.filename_with_extension("rust");
                    if filename != "mod.rs" && filename != "main.rs" && filename != "lib.rs" {
                        let visibility = Self::get_visibility_prefix(codefile.pub_setting(), "lib");
                        module_declarations.push(format!("{}mod {};", visibility, codefile.name()));
                    }
                }
                
                for submodule in src_module.submodules() {
                    let visibility = Self::get_visibility_prefix(submodule.pub_setting(), "lib");
                    module_declarations.push(format!("{}mod {};", visibility, submodule.name()));
                }
            }
        }

        let lib_comment = if module_declarations.is_empty() { Some("// Library root") } else { None };
        ContentUpdater::update_rust_module_file(&lib_rs_path, &module_declarations, lib_comment)?;

        Ok(())
    }

    /// Generate mod.rs file content
    fn generate_mod_rs_content(module_declarations: &[String]) -> String {
        format!(
            "// start auto exported by saba.\n{}\n// end auto exported by saba.\n\n",
            module_declarations.join("\n")
        )
    }

    /// Generate main.rs file content
    fn generate_main_rs_content(module_declarations: &[String]) -> String {
        let mod_section = if module_declarations.is_empty() {
            String::new()
        } else {
            format!(
                "// start auto exported by saba.\n{}\n// end auto exported by saba.\n\n",
                module_declarations.join("\n")
            )
        };

        format!(
            "{}fn main() {{\n    println!(\"Hello, world!\");\n}}\n",
            mod_section
        )
    }

    /// Generate lib.rs file content
    fn generate_lib_rs_content(module_declarations: &[String]) -> String {
        if module_declarations.is_empty() {
            "// Library root\n".to_string()
        } else {
            format!(
                "// start auto exported by saba.\n{}\n// end auto exported by saba.\n\n",
                module_declarations.join("\n")
            )
        }
    }

    /// Determine if project should have main.rs or lib.rs
    pub fn should_generate_main_rs(project: &Project) -> bool {
        // Generate main.rs if there's no explicit lib.rs file defined in the project
        let has_lib_in_project = project.files().iter()
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        let has_lib_in_src = project.modules().iter()
            .filter(|m| m.name() == "src")
            .flat_map(|m| m.files())
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        !has_lib_in_project && !has_lib_in_src
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::project_management::config::models::*;

    #[test]
    fn test_generate_simple_module() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let module = Module {
            name: "domain".to_string(),
            upstream: vec![],
            codefile: vec![
                CodeFile { name: "model".to_string() },
                CodeFile { name: "repository".to_string() },
            ],
        };

        RustModuleGenerator::generate_module(base_path, &module, &[]).unwrap();

        // Check directory exists
        assert!(base_path.join("domain").exists());
        
        // Check files exist
        assert!(base_path.join("domain/model.rs").exists());
        assert!(base_path.join("domain/repository.rs").exists());
        assert!(base_path.join("domain/mod.rs").exists());

        // Check mod.rs content
        let mod_content = fs::read_to_string(base_path.join("domain/mod.rs")).unwrap();
        assert!(mod_content.contains("pub mod model;"));
        assert!(mod_content.contains("pub mod repository;"));
    }

    #[test]
    fn test_generate_nested_modules() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let module = Module {
            name: "src".to_string(),
            upstream: vec![
                Module {
                    name: "domain".to_string(),
                    upstream: vec![],
                    codefile: vec![CodeFile { name: "model".to_string() }],
                },
            ],
            codefile: vec![],
        };

        RustModuleGenerator::generate_module(base_path, &module, &[]).unwrap();

        // Check nested structure
        assert!(base_path.join("src").exists());
        assert!(base_path.join("src/domain").exists());
        assert!(base_path.join("src/domain/model.rs").exists());
        assert!(base_path.join("src/domain/mod.rs").exists());
        assert!(!base_path.join("src/mod.rs").exists()); // src should not have mod.rs

        // Check domain mod.rs content
        let domain_mod_content = fs::read_to_string(base_path.join("src/domain/mod.rs")).unwrap();
        assert!(domain_mod_content.contains("pub mod model;"));
    }

    #[test]
    fn test_main_rs_generation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path();
        
        // Create src directory
        fs::create_dir_all(project_path.join("src")).unwrap();

        let modules = vec![
            Module {
                name: "src".to_string(),
                upstream: vec![
                    Module {
                        name: "domain".to_string(),
                        upstream: vec![],
                        codefile: vec![],
                    },
                ],
                codefile: vec![],
            },
        ];

        RustModuleGenerator::generate_main_rs(project_path, &modules).unwrap();

        let main_rs_path = project_path.join("src/main.rs");
        assert!(main_rs_path.exists());

        let main_content = fs::read_to_string(main_rs_path).unwrap();
        assert!(main_content.contains("mod domain;"));
        assert!(main_content.contains("fn main()"));
    }
}