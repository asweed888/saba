use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::Project;

/// Rust Cargo handler for generating individual project Cargo.toml
pub struct CargoHandler;

impl CargoHandler {
    /// Generate Cargo.toml for individual Rust project
    pub fn generate_cargo_toml<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let cargo_toml_path = project_path.as_ref().join("Cargo.toml");
        let cargo_content = Self::generate_cargo_toml_content(project);
        
        fs::write(&cargo_toml_path, cargo_content)
            .with_context(|| format!("Failed to create Cargo.toml: {}", cargo_toml_path.display()))?;

        Ok(())
    }

    /// Generate Cargo.toml content
    fn generate_cargo_toml_content(project: &Project) -> String {
        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]

"#,
            project.name()
        )
    }

    /// Determine if project should be a library based on presence of lib.rs
    fn is_library_project(project: &Project) -> bool {
        // Check if there's a lib.rs file defined in the project
        let has_lib_in_project = project.files().iter()
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        let has_lib_in_src = project.modules().iter()
            .filter(|m| m.name() == "src")
            .flat_map(|m| m.files())
            .any(|f| f.name() == "lib" || f.filename_with_extension("rust") == "lib.rs");
        
        has_lib_in_project || has_lib_in_src
    }

    /// Sanitize crate name for Rust (replace hyphens with underscores)
    fn sanitize_crate_name(name: &str) -> String {
        name.replace('-', "_")
    }
}