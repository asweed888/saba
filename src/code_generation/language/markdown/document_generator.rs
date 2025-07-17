use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::project_management::config::models::{Module, CodeFile, Project};
use crate::shared::utils::content_updater::ContentUpdater;

/// Markdown-specific document generator
pub struct MarkdownDocumentGenerator;

impl MarkdownDocumentGenerator {
    /// Generate Markdown module structure recursively
    pub fn generate_module<P: AsRef<Path>>(
        base_path: P,
        module: &Module,
        parent_modules: &[String],
    ) -> Result<()> {
        let module_path = base_path.as_ref().join(&module.name);
        
        // Create directory
        fs::create_dir_all(&module_path)
            .with_context(|| format!("Failed to create directory: {}", module_path.display()))?;

        // Generate markdown files in this module
        for codefile in module.files() {
            let filename = codefile.filename_with_extension("markdown");
            let file_path = module_path.join(&filename);
            
            // Create markdown file with basic template (only if it doesn't exist)
            if !file_path.exists() {
                let content = Self::generate_markdown_content(codefile.name(), &module.name, parent_modules);
                fs::write(&file_path, content)
                    .with_context(|| format!("Failed to create file: {}", file_path.display()))?;
            }
        }

        // Generate/update README.md for the module using ContentUpdater
        let readme_path = module_path.join("README.md");
        Self::generate_module_readme(&readme_path, &module.name, module, parent_modules)?;

        // Process submodules recursively
        for submodule in module.submodules() {
            let mut new_parent_modules = parent_modules.to_vec();
            new_parent_modules.push(module.name().to_string());
            
            Self::generate_module(&module_path, submodule, &new_parent_modules)?;
        }

        Ok(())
    }

    /// Generate project-level README.md
    pub fn generate_readme<P: AsRef<Path>>(
        project_path: P,
        project: &Project,
    ) -> Result<()> {
        let current_dir = std::env::current_dir()?;
        std::env::set_current_dir(project_path.as_ref())?;
        
        let result = Self::generate_readme_content(project);
        
        std::env::set_current_dir(current_dir)?;
        result
    }

    /// Generate markdown file content with basic template
    fn generate_markdown_content(file_name: &str, _module_name: &str, _parent_modules: &[String]) -> String {
        let title = Self::format_title(file_name);
        
        format!("# {}\n\n", title)
    }

    /// Generate/update README.md for a module using ContentUpdater
    fn generate_module_readme<P: AsRef<Path>>(
        readme_path: P,
        module_name: &str,
        module: &Module,
        _parent_modules: &[String],
    ) -> Result<()> {
        let title = Self::format_title(module_name);
        
        // Create base content if file doesn't exist
        let readme_path = readme_path.as_ref();
        if !readme_path.exists() {
            fs::write(readme_path, format!("# {}\n\n", title))
                .with_context(|| format!("Failed to create README.md: {}", readme_path.display()))?;
        }

        // Generate managed content for files and modules
        let mut managed_content = String::new();
        
        // Add files section (exclude README from self-reference)
        let other_files: Vec<_> = module.files().iter()
            .filter(|file| file.name() != "README")
            .collect();
        if !other_files.is_empty() {
            managed_content.push_str("\n## Documents\n\n");
            for file in other_files {
                let filename = file.filename_with_extension("markdown");
                managed_content.push_str(&format!("- [{}]({})\n", Self::format_title(file.name()), filename));
            }
        }

        // Add submodules section
        if !module.submodules().is_empty() {
            managed_content.push_str("\n## Modules\n\n");
            for submodule in module.submodules() {
                managed_content.push_str(&format!("- [{}]({}/)\n", 
                    Self::format_title(submodule.name()),
                    submodule.name()
                ));
            }
        }

        // Update managed section using ContentUpdater (append at end for markdown)
        if !managed_content.is_empty() {
            ContentUpdater::append_managed_section(
                readme_path,
                &managed_content,
                "<!-- start auto exported by saba. -->",
                "<!-- end auto exported by saba. -->",
            )?;
        }

        Ok(())
    }

    /// Generate/update README.md content for project
    fn generate_readme_content(project: &Project) -> Result<()> {
        let readme_path = "README.md";
        let title = Self::format_title(project.name());
        
        // Create base content if file doesn't exist
        if !std::path::Path::new(readme_path).exists() {
            fs::write(readme_path, format!("# {}\n\n", title))
                .with_context(|| "Failed to create project README.md")?;
        }

        // Generate managed content
        let mut managed_content = String::new();
        
        // Add project files section (exclude README from self-reference)
        let other_files: Vec<_> = project.files().iter()
            .filter(|file| file.name() != "README")
            .collect();
        if !other_files.is_empty() {
            managed_content.push_str("\n## Documents\n\n");
            for file in other_files {
                let filename = file.filename_with_extension("markdown");
                managed_content.push_str(&format!("- [{}]({})\n", Self::format_title(file.name()), filename));
            }
        }

        // Add modules section
        if !project.modules().is_empty() {
            managed_content.push_str("\n## Modules\n\n");
            for module in project.modules() {
                managed_content.push_str(&format!("- [{}]({}/)\n", 
                    Self::format_title(module.name()),
                    module.name()
                ));
            }
        }

        // Update managed section using ContentUpdater (append at end for markdown)
        if !managed_content.is_empty() {
            ContentUpdater::append_managed_section(
                readme_path,
                &managed_content,
                "<!-- start auto exported by saba. -->",
                "<!-- end auto exported by saba. -->",
            )?;
        }

        Ok(())
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

    /// Generate breadcrumb navigation
    fn generate_breadcrumb(parent_modules: &[String], current_module: &str) -> String {
        if parent_modules.is_empty() && current_module.is_empty() {
            return String::new();
        }

        let mut breadcrumb = String::from("**Navigation:** ");
        
        if !parent_modules.is_empty() {
            for (i, module) in parent_modules.iter().enumerate() {
                if i > 0 {
                    breadcrumb.push_str(" > ");
                }
                breadcrumb.push_str(&Self::format_title(module));
            }
            
            if !current_module.is_empty() {
                breadcrumb.push_str(" > ");
                breadcrumb.push_str(&Self::format_title(current_module));
            }
        } else if !current_module.is_empty() {
            breadcrumb.push_str(&Self::format_title(current_module));
        }

        breadcrumb
    }
}