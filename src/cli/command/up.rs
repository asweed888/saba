use clap::Command;
use anyhow::{bail, Context};
use crate::project_management::config::{ConfigParser, ConfigValidator};
use crate::code_generation::core::generator::CodeGenerator;

pub fn spec() -> Command {
    Command::new("up")
        .about("Generate code structure from saba.yml")
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