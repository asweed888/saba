use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{SabaConfig, Project};
use crate::shared::utils::content_updater::ContentUpdater;

/// Rust workspace handler for generating workspace Cargo.toml and Makefile.toml
pub struct WorkspaceHandler;

impl WorkspaceHandler {
    /// Generate workspace Cargo.toml for multi-project configuration with Rust projects
    pub fn generate_workspace_cargo_toml<P: AsRef<Path>>(
        workspace_path: P,
        config: &SabaConfig,
    ) -> Result<()> {
        let rust_projects: Vec<&Project> = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .collect();

        if rust_projects.is_empty() {
            return Ok(()); // No Rust projects, no workspace needed
        }

        let cargo_toml_path = workspace_path.as_ref().join("Cargo.toml");
        let members: Vec<String> = rust_projects
            .iter()
            .map(|p| p.name().to_string())
            .collect();
        
        ContentUpdater::update_workspace_cargo_toml(&cargo_toml_path, &members)
            .with_context(|| format!("Failed to update workspace Cargo.toml: {}", cargo_toml_path.display()))?;

        Ok(())
    }

    /// Generate Makefile.toml for cargo-make
    pub fn generate_makefile_toml<P: AsRef<Path>>(
        workspace_path: P,
        config: &SabaConfig,
    ) -> Result<()> {
        let makefile_toml_path = workspace_path.as_ref().join("Makefile.toml");
        
        // Only create if file doesn't exist (protect existing configuration)
        if !makefile_toml_path.exists() {
            let makefile_content = Self::generate_makefile_toml_content(config);
            fs::write(&makefile_toml_path, makefile_content)
                .with_context(|| format!("Failed to create Makefile.toml: {}", makefile_toml_path.display()))?;
        }

        Ok(())
    }


    /// Generate Makefile.toml content for cargo-make
    fn generate_makefile_toml_content(config: &SabaConfig) -> String {
        let rust_projects: Vec<&Project> = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .collect();

        // Use the first Rust project as default for dev task
        let default_project = rust_projects.first().map(|p| p.name()).unwrap_or("app_1");

        format!(
            r#"[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
# CARGO_MAKE_LOAD_SCRIPT = ".env"

[tasks.dev]
description = "Start development environment."
workspace = false
cwd = "{}"
command = "cargo"
args = ["watch", "-x", "run"]
"#,
            default_project
        )
    }

    /// Check if workspace generation is needed (multi-project with Rust)
    pub fn should_generate_workspace(config: &SabaConfig) -> bool {
        let rust_project_count = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .count();

        // Generate workspace if there are multiple projects with at least one Rust project
        // OR if there's a single Rust project but other language projects exist
        rust_project_count > 1 || 
        (rust_project_count > 0 && config.projects().len() > rust_project_count)
    }
}