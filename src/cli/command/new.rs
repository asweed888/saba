use clap::{Arg, ArgMatches, Command};
use anyhow::{bail, Context, Result};
use inquire::Select;
use std::fs;

pub fn spec() -> Command {
    Command::new("new")
        .about("Initialize a new project with saba.yml configuration")
        .long_about(
            "Initialize a new project by creating or updating saba.yml. \
            Supports two modes:\n\
            \n\
            Human Mode (Interactive):\n  \
            saba new  # Prompts for language selection\n\
            \n\
            AI Mode (Direct):\n  \
            saba new --lang rust  # Direct language specification\n\
            \n\
            Features:\n\
            • Auto-generates sequential project names (app_1, app_2, etc.)\n\
            • Smart multi-project handling (removes root: true from existing projects)\n\
            • Language-specific directory structures (Rust uses src/, others use root-level)\n\
            • Supports: rust, go, python, typescript, javascript"
        )
        .arg(
            Arg::new("lang")
                .short('l')
                .long("lang")
                .help("Programming language for direct specification (AI mode)")
                .long_help(
                    "Specify the programming language directly without interactive prompts. \
                    Supported languages: rust, go, python, typescript, javascript. \
                    When omitted, enters interactive mode for human users."
                )
                .value_name("LANGUAGE")
                .required(false)
        )
}

pub fn action(matches: &ArgMatches) -> Result<()> {
    let language = if let Some(lang) = matches.get_one::<String>("lang") {
        // AI mode - language specified via --lang option
        let supported_languages = ["rust", "go", "python", "typescript", "javascript"];
        if !supported_languages.contains(&lang.as_str()) {
            bail!("Unsupported language: {}. Supported languages: {}", lang, supported_languages.join(", "));
        }
        lang.clone()
    } else {
        // Human mode - interactive language selection
        let languages = vec!["rust", "go", "python", "typescript", "javascript"];
        Select::new("Programming language:", languages)
            .prompt()
            .context("Failed to get programming language")?.
            to_string()
    };

    // Generate sequential project name
    let project_name = generate_sequential_project_name()?;

    // Check if saba.yml already exists
    if fs::metadata("saba.yml").is_ok() {
        // Existing saba.yml - append new project
        append_to_existing_saba_yml(&project_name, &language)?;
        println!("✓ Added {} ({}) project to existing saba.yml", project_name, language);
    } else {
        // No existing saba.yml - create new one with root: true
        let saba_content = generate_new_saba_yml(&project_name, &language)?;
        fs::write("saba.yml", saba_content)
            .context("Failed to write saba.yml")?;
        println!("✓ Generated new saba.yml for {} ({}) project", project_name, language);
    }
    
    println!("[Success] saba.yml initialization completed.");
    println!("Run 'saba up' to generate your project structure.");
    
    Ok(())
}

fn generate_new_saba_yml(project_name: &str, language: &str) -> Result<String> {
    let main_file = get_main_file_name(language);
    
    if language == "rust" {
        // Rust uses src/ structure
        Ok(format!(
            r#"- name: {}
  root: true
  lang: {}
  upstream:
    - name: src
  codefile:
    - name: {}
"#,
            project_name, language, main_file
        ))
    } else {
        // Other languages use root-level structure
        Ok(format!(
            r#"- name: {}
  root: true
  lang: {}
  codefile:
    - name: {}
"#,
            project_name, language, main_file
        ))
    }
}

fn append_to_existing_saba_yml(project_name: &str, language: &str) -> Result<()> {
    // Read existing saba.yml
    let existing_content = fs::read_to_string("saba.yml")
        .context("Failed to read existing saba.yml")?;
    
    // Remove root: true from existing content
    let updated_content = existing_content.replace("  root: true\n", "");
    
    // Generate new project YAML
    let main_file = get_main_file_name(language);
    let new_project_yaml = if language == "rust" {
        format!(
            r#"

- name: {}
  lang: {}
  upstream:
    - name: src
  codefile:
    - name: {}
"#,
            project_name, language, main_file
        )
    } else {
        format!(
            r#"

- name: {}
  lang: {}
  codefile:
    - name: {}
"#,
            project_name, language, main_file
        )
    };
    
    // Combine and write back
    let final_content = format!("{}{}", updated_content, new_project_yaml);
    fs::write("saba.yml", final_content)
        .context("Failed to write updated saba.yml")?;
    
    Ok(())
}

fn generate_sequential_project_name() -> Result<String> {
    let mut counter = 1;
    
    // If saba.yml exists, check for existing app_X projects
    if fs::metadata("saba.yml").is_ok() {
        let content = fs::read_to_string("saba.yml")
            .context("Failed to read existing saba.yml")?;
        
        // Find the highest app_X number
        for line in content.lines() {
            if let Some(name_part) = line.strip_prefix("- name: app_") {
                if let Ok(num) = name_part.parse::<i32>() {
                    if num >= counter {
                        counter = num + 1;
                    }
                }
            }
        }
    }
    
    Ok(format!("app_{}", counter))
}

fn get_main_file_name(language: &str) -> &str {
    match language {
        "rust" => "main",
        "go" => "main", 
        "python" => "main",
        "typescript" => "index",
        "javascript" => "index",
        _ => "main",
    }
}