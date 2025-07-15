use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::{SabaConfig, Project};

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
        let cargo_content = Self::generate_workspace_cargo_toml_content(&rust_projects);
        
        fs::write(&cargo_toml_path, cargo_content)
            .with_context(|| format!("Failed to create workspace Cargo.toml: {}", cargo_toml_path.display()))?;

        Ok(())
    }

    /// Generate Makefile.toml for cargo-make
    pub fn generate_makefile_toml<P: AsRef<Path>>(
        workspace_path: P,
        config: &SabaConfig,
    ) -> Result<()> {
        let makefile_toml_path = workspace_path.as_ref().join("Makefile.toml");
        let makefile_content = Self::generate_makefile_toml_content(config);
        
        fs::write(&makefile_toml_path, makefile_content)
            .with_context(|| format!("Failed to create Makefile.toml: {}", makefile_toml_path.display()))?;

        Ok(())
    }

    /// Generate workspace Cargo.toml content
    fn generate_workspace_cargo_toml_content(rust_projects: &[&Project]) -> String {
        let members: Vec<String> = rust_projects
            .iter()
            .map(|p| format!("\"{}\"", p.name()))
            .collect();

        format!(
            r#"[workspace]
members = [
    {}
]

[workspace.dependencies]
# Common dependencies for all workspace members
anyhow = "1.0"
serde = {{ version = "1.0", features = ["derive"] }}
tokio = {{ version = "1.0", features = ["full"] }}

[profile.release]
lto = true
codegen-units = 1
panic = "abort"
strip = true
"#,
            members.join(",\n    ")
        )
    }

    /// Generate Makefile.toml content for cargo-make
    fn generate_makefile_toml_content(config: &SabaConfig) -> String {
        let rust_projects: Vec<&Project> = config.projects()
            .iter()
            .filter(|p| p.language() == "rust")
            .collect();

        let has_other_languages = config.projects()
            .iter()
            .any(|p| p.language() != "rust");

        let mut content = String::from(
            r#"[config]
default_to_workspace = false

[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true

[tasks.build]
description = "Build all projects"
"#
        );

        if rust_projects.len() > 1 {
            content.push_str("command = \"cargo\"\nargs = [\"build\", \"--workspace\"]\n");
        } else if !rust_projects.is_empty() {
            content.push_str("command = \"cargo\"\nargs = [\"build\"]\n");
        }

        content.push_str(
            r#"
[tasks.test]
description = "Test all projects"
"#
        );

        if rust_projects.len() > 1 {
            content.push_str("command = \"cargo\"\nargs = [\"test\", \"--workspace\"]\n");
        } else if !rust_projects.is_empty() {
            content.push_str("command = \"cargo\"\nargs = [\"test\"]\n");
        }

        content.push_str(
            r#"
[tasks.clean]
description = "Clean all projects"
"#
        );

        if !rust_projects.is_empty() {
            content.push_str("command = \"cargo\"\nargs = [\"clean\"]\n");
        }

        content.push_str(
            r#"
[tasks.fmt]
description = "Format all Rust code"
command = "cargo"
args = ["fmt", "--all"]

[tasks.clippy]
description = "Run clippy on all Rust code"
command = "cargo"
args = ["clippy", "--workspace", "--all-targets", "--", "-D", "warnings"]

[tasks.check]
description = "Quick check all Rust code"
command = "cargo"
args = ["check", "--workspace"]
"#
        );

        if has_other_languages {
            content.push_str("\n[tasks.install-deps]\n");
            content.push_str("description = \"Install dependencies for all languages\"\n");
            content.push_str("script = [\n");
            content.push_str("    \"# Install Node.js dependencies if package.json exists\",\n");
            content.push_str("    \"find . -name package.json -not -path ./node_modules/\\* -execdir npm install \\\\;\",\n");
            content.push_str("    \"# Install Python dependencies if requirements.txt exists\",\n");
            content.push_str("    \"find . -name requirements.txt -execdir pip install -r requirements.txt \\\\;\",\n");
            content.push_str("    \"# Install Go dependencies if go.mod exists\",\n");
            content.push_str("    \"find . -name go.mod -execdir go mod download \\\\;\"\n");
            content.push_str("]\n");
        }

        content
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