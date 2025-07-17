use clap::Command;
use anyhow::{bail, Context};
use crate::project_management::config::{ConfigParser, ConfigValidator};
use crate::code_generation::core::generator::CodeGenerator;

pub fn spec() -> Command {
    Command::new("up")
        .about("Generate project structure from saba.yml configuration")
        .long_about(
            "Generate complete project structures from saba.yml specification. \
            Supports both single-project and multi-project configurations.\n\
            \n\
            Single Project Mode:\n  \
            Projects with 'root: true' generate directly in the current directory\n\
            \n\
            Multi-Project Mode:\n  \
            Creates separate directories for each project with workspace support\n\
            \n\
            Features:\n\
            • Multi-language support: Rust, Go, Python, TypeScript, JavaScript\n\
            • Language-specific project files (Cargo.toml, package.json, go.mod, etc.)\n\
            • Intelligent module structure with barrel exports/imports\n\
            • Workspace management for Rust multi-project setups\n\
            • File extension preservation (.tsx, .jsx, .vue, etc.)\n\
            \n\
            Prerequisites:\n\
            • saba.yml must exist (create with 'saba new')\n\
            • Valid YAML configuration with supported languages"
        )
}

pub fn action() -> anyhow::Result<()> {
    // Check if saba.yml exists
    if !ConfigParser::config_exists() {
        bail!("saba.yml not found. Run 'saba new' to create a new project configuration.");
    }

    // Parse configuration
    let config = ConfigParser::parse_default()
        .context("Failed to parse saba.yml")?;

    // Validate configuration
    ConfigValidator::validate(&config)
        .context("Configuration validation failed")?;

    // Print generating message for each project
    for project in config.projects() {
        println!("Generating project: {}", project.name());
    }

    // Generate structure using the new CodeGenerator
    CodeGenerator::generate_from_config(".", &config)
        .context("Failed to generate project structure")?;

    // Print success message for each project
    for project in config.projects() {
        println!("  ✓ Generated {} ({}) structure", project.name(), project.language());
    }

    println!("[Success] generate of saba has been completed.");
    Ok(())
}