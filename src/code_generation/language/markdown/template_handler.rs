use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use crate::project_management::config::models::Project;
use super::document_generator::MarkdownDocumentGenerator;

/// Template handler for Markdown projects
pub struct MarkdownTemplateHandler;

impl MarkdownTemplateHandler {
    /// Generate complete Markdown project structure
    pub fn generate_project<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = project_path.as_ref();

        // Generate project-level README.md
        MarkdownDocumentGenerator::generate_readme(project_path, project)?;

        // Generate project-level markdown files (only if they don't exist)
        for codefile in project.files() {
            let filename = codefile.filename_with_extension("markdown");
            let file_path = project_path.join(&filename);
            
            if !file_path.exists() {
                let content = Self::generate_project_file_content(codefile.name(), project.name());
                fs::write(&file_path, content)
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }

        // Generate module structure
        for module in project.modules() {
            MarkdownDocumentGenerator::generate_module(project_path, module, &[])?;
        }

        // Generate project configuration files
        Self::generate_docs_config(project_path, project)?;

        Ok(())
    }

    /// Generate project-level markdown file content
    fn generate_project_file_content(file_name: &str, _project_name: &str) -> String {
        let title = Self::format_title(file_name);
        
        format!("# {}\n\n", title)
    }

    /// Generate documentation configuration file (.docs.yml)
    fn generate_docs_config<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let config_path = project_path.as_ref().join(".docs.yml");
        
        // Only create config if it doesn't already exist
        if !config_path.exists() {
            let config_content = Self::generate_docs_config_content(project);
            fs::write(&config_path, config_content)
                .with_context(|| format!("Failed to create .docs.yml: {}", config_path.display()))?;
        }

        Ok(())
    }

    /// Generate documentation configuration content
    fn generate_docs_config_content(project: &Project) -> String {
        let mut config = format!(
            "# Documentation configuration for {}\n",
            project.name()
        );
        
        config.push_str("site_name: ");
        config.push_str(&Self::format_title(project.name()));
        config.push_str("\n");
        
        config.push_str("site_description: Documentation for ");
        config.push_str(project.name());
        config.push_str("\n\n");
        
        config.push_str("nav:\n");
        config.push_str("  - Home: README.md\n");
        
        // Add project files to navigation
        for file in project.files() {
            let filename = file.filename_with_extension("markdown");
            if filename != "README.md" {
                config.push_str(&format!("  - {}: {}\n", Self::format_title(file.name()), filename));
            }
        }
        
        // Add modules to navigation
        for module in project.modules() {
            config.push_str(&format!("  - {}:\n", Self::format_title(module.name())));
            config.push_str(&format!("    - Overview: {}/README.md\n", module.name()));
            
            for file in module.files() {
                let filename = file.filename_with_extension("markdown");
                config.push_str(&format!("    - {}: {}/{}\n", 
                    Self::format_title(file.name()), 
                    module.name(), 
                    filename
                ));
            }
            
            // Add submodules recursively
            Self::add_submodules_to_nav(&mut config, module, 2);
        }
        
        config.push_str("\ntheme:\n");
        config.push_str("  name: material\n");
        config.push_str("  features:\n");
        config.push_str("    - navigation.tabs\n");
        config.push_str("    - navigation.sections\n");
        config.push_str("    - toc.integrate\n");
        
        config
    }

    /// Recursively add submodules to navigation configuration
    fn add_submodules_to_nav(config: &mut String, module: &crate::project_management::config::models::Module, indent_level: usize) {
        for submodule in module.submodules() {
            let indent = "  ".repeat(indent_level);
            config.push_str(&format!("{}  - {}:\n", indent, Self::format_title(submodule.name())));
            config.push_str(&format!("{}    - Overview: {}/{}/README.md\n", indent, module.name(), submodule.name()));
            
            for file in submodule.files() {
                let filename = file.filename_with_extension("markdown");
                config.push_str(&format!("{}    - {}: {}/{}/{}\n", 
                    indent,
                    Self::format_title(file.name()), 
                    module.name(),
                    submodule.name(),
                    filename
                ));
            }
            
            // Recursive call for deeper submodules
            Self::add_submodules_to_nav(config, submodule, indent_level + 1);
        }
    }

    /// Format name as title (capitalize words, replace underscores with spaces)
    fn format_title(name: &str) -> String {
        name.replace('_', " ")
            .split_whitespace()
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                }
            })
            .collect::<Vec<_>>()
            .join(" ")
    }

    /// Check if project needs a table of contents
    pub fn should_generate_toc(project: &Project) -> bool {
        // Generate TOC if project has multiple files or modules
        project.files().len() > 1 || !project.modules().is_empty()
    }

    /// Generate table of contents file
    pub fn generate_toc<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        if !Self::should_generate_toc(project) {
            return Ok(());
        }

        let toc_path = project_path.as_ref().join("TOC.md");
        
        // Only create TOC if it doesn't already exist
        if !toc_path.exists() {
            let toc_content = Self::generate_toc_content(project);
            fs::write(&toc_path, toc_content)
                .with_context(|| format!("Failed to create TOC.md: {}", toc_path.display()))?;
        }

        Ok(())
    }

    /// Generate table of contents content
    fn generate_toc_content(project: &Project) -> String {
        let mut toc = format!("# Table of Contents\n\n**Project:** {}\n\n", project.name());
        
        toc.push_str("## Project Documentation\n\n");
        toc.push_str("- [README](README.md)\n");
        
        for file in project.files() {
            let filename = file.filename_with_extension("markdown");
            if filename != "README.md" {
                toc.push_str(&format!("- [{}]({})\n", Self::format_title(file.name()), filename));
            }
        }
        
        for module in project.modules() {
            toc.push_str(&format!("\n## {} Module\n\n", Self::format_title(module.name())));
            toc.push_str(&format!("- [Overview]({}/README.md)\n", module.name()));
            
            for file in module.files() {
                let filename = file.filename_with_extension("markdown");
                toc.push_str(&format!("- [{}]({}/{})\n", 
                    Self::format_title(file.name()), 
                    module.name(), 
                    filename
                ));
            }
            
            Self::add_submodules_to_toc(&mut toc, module, 1);
        }
        
        toc
    }

    /// Recursively add submodules to table of contents
    fn add_submodules_to_toc(toc: &mut String, module: &crate::project_management::config::models::Module, level: usize) {
        for submodule in module.submodules() {
            toc.push_str(&format!("\n{} {} Submodule\n\n", "#".repeat(level + 2), Self::format_title(submodule.name())));
            toc.push_str(&format!("- [Overview]({}/{}/README.md)\n", module.name(), submodule.name()));
            
            for file in submodule.files() {
                let filename = file.filename_with_extension("markdown");
                toc.push_str(&format!("- [{}]({}/{}/{})\n", 
                    Self::format_title(file.name()), 
                    module.name(),
                    submodule.name(),
                    filename
                ));
            }
            
            Self::add_submodules_to_toc(toc, submodule, level + 1);
        }
    }
}