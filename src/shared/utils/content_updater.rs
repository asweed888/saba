use std::fs;
use std::path::Path;
use anyhow::{Context, Result};
use regex::Regex;

/// Utility for updating managed content sections while preserving user code
pub struct ContentUpdater;

impl ContentUpdater {
    /// Update content between header and footer markers, preserving everything else
    pub fn update_managed_section<P: AsRef<Path>>(
        file_path: P,
        new_managed_content: &str,
        header: &str,
        footer: &str,
    ) -> Result<()> {
        let file_path = file_path.as_ref();
        
        // Read existing content if file exists
        let existing_content = if file_path.exists() {
            fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read file: {}", file_path.display()))?
        } else {
            String::new()
        };

        // Create regex pattern to match header...footer section
        let escaped_header = regex::escape(header);
        let escaped_footer = regex::escape(footer);
        let pattern = format!(r"{}[\s\S]*{}", escaped_header, escaped_footer);
        let regex = Regex::new(&pattern)
            .with_context(|| "Failed to create regex pattern")?;

        // Create new managed section
        let new_section = format!("{}{}\n{}", header, new_managed_content, footer);

        // Replace or append the managed section
        let updated_content = if regex.is_match(&existing_content) {
            // Replace existing managed section
            regex.replace(&existing_content, new_section.as_str()).to_string()
        } else {
            // Append new managed section (file doesn't have managed section yet)
            if existing_content.is_empty() {
                format!("{}\n\n", new_section)
            } else {
                format!("{}\n{}\n\n", existing_content.trim(), new_section)
            }
        };

        // Write updated content
        fs::write(file_path, updated_content)
            .with_context(|| format!("Failed to write file: {}", file_path.display()))?;

        Ok(())
    }

    /// Update Rust module files (mod.rs, main.rs, lib.rs) with module declarations
    pub fn update_rust_module_file<P: AsRef<Path>>(
        file_path: P,
        module_declarations: &[String],
        additional_content: Option<&str>,
    ) -> Result<()> {
        let managed_content = if module_declarations.is_empty() {
            "\n".to_string()
        } else {
            format!("\n{}\n", module_declarations.join("\n"))
        };

        Self::update_managed_section(
            &file_path,
            &managed_content,
            "// start auto exported by saba.\n",
            "// end auto exported by saba.",
        )?;

        // Add additional content (like main function) only if this is a new file that didn't exist before
        if let Some(extra) = additional_content {
            let file_path = file_path.as_ref();
            let current_content = fs::read_to_string(file_path)
                .with_context(|| format!("Failed to read file: {}", file_path.display()))?;
            
            // Only add additional content if:
            // 1. The file is essentially empty (only contains managed section)
            // 2. The additional content is not already present
            let is_essentially_empty = current_content.trim().lines()
                .filter(|line| !line.trim().is_empty())
                .all(|line| line.contains("start auto exported by saba") 
                    || line.contains("end auto exported by saba")
                    || line.trim().starts_with("mod "));
            
            if is_essentially_empty && !current_content.contains(extra.trim()) {
                let updated_content = format!("{}{}\n", current_content, extra);
                fs::write(file_path, updated_content)
                    .with_context(|| format!("Failed to write file: {}", file_path.display()))?;
            }
        }

        Ok(())
    }

    /// Update Python __init__.py files with import statements
    pub fn update_python_init_file<P: AsRef<Path>>(
        file_path: P,
        import_statements: &[String],
    ) -> Result<()> {
        let managed_content = if import_statements.is_empty() {
            "\n".to_string()
        } else {
            format!("\n{}\n", import_statements.join("\n"))
        };

        Self::update_managed_section(
            &file_path,
            &managed_content,
            "# start auto exported by saba.\n",
            "# end auto exported by saba.",
        )
    }

    /// Update JavaScript/TypeScript index files with export statements
    pub fn update_js_index_file<P: AsRef<Path>>(
        file_path: P,
        export_statements: &[String],
    ) -> Result<()> {
        let managed_content = if export_statements.is_empty() {
            "\n".to_string()
        } else {
            format!("\n{}\n", export_statements.join("\n"))
        };

        Self::update_managed_section(
            &file_path,
            &managed_content,
            "// start auto exported by saba.\n",
            "// end auto exported by saba.",
        )
    }
}