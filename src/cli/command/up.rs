use clap::Command;
use anyhow::{bail, Context};
use crate::project_management::config::{ConfigParser, ConfigValidator};
use crate::code_generation::core::{DirectoryBuilder, FileBuilder};

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

    // Generate structure for each project
    for project in config.projects() {
        println!("Generating project: {}", project.name());
        
        // Build directory structure
        DirectoryBuilder::build_project_structure(".", project)
            .with_context(|| format!("Failed to build directory structure for project: {}", project.name()))?;

        // Build files
        FileBuilder::build_project_files(".", project)
            .with_context(|| format!("Failed to build files for project: {}", project.name()))?;

        println!("  ✓ Generated {} ({}) structure", project.name(), project.language());
    }

    println!("[Success] generate of saba has been completed.");
    Ok(())
}