use serde::{Deserialize, Serialize};

/// v2 saba.yml configuration root
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SabaConfig {
    #[serde(rename = "$value")]
    pub projects: Vec<Project>,
}

/// Individual project configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub name: String,
    #[serde(default)]
    pub root: bool,
    pub lang: String,
    #[serde(default)]
    pub codefile: Vec<CodeFile>,
    #[serde(default)]
    pub upstream: Vec<Module>,
}

/// Module or directory structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Module {
    pub name: String,
    #[serde(default)]
    pub r#pub: Option<String>,
    #[serde(default)]
    pub upstream: Vec<Module>,
    #[serde(default)]
    pub codefile: Vec<CodeFile>,
}

/// Individual code file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeFile {
    pub name: String,
    #[serde(default)]
    pub r#pub: Option<String>,
}

impl SabaConfig {
    /// Get all projects
    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    /// Get root project (single project mode)
    pub fn root_project(&self) -> Option<&Project> {
        self.projects.iter().find(|p| p.root)
    }

    /// Get non-root projects (multi-project mode)
    pub fn sub_projects(&self) -> Vec<&Project> {
        self.projects.iter().filter(|p| !p.root).collect()
    }

    /// Check if this is a single project configuration
    pub fn is_single_project(&self) -> bool {
        self.root_project().is_some()
    }
}

impl Project {
    /// Get project name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Check if this is a root project
    pub fn is_root(&self) -> bool {
        self.root
    }

    /// Get project language
    pub fn language(&self) -> &str {
        &self.lang
    }

    /// Get top-level modules
    pub fn modules(&self) -> &[Module] {
        &self.upstream
    }

    /// Get all code files at project level
    pub fn files(&self) -> &[CodeFile] {
        &self.codefile
    }
}

impl Module {
    /// Get module name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get sub-modules
    pub fn submodules(&self) -> &[Module] {
        &self.upstream
    }

    /// Get code files in this module
    pub fn files(&self) -> &[CodeFile] {
        &self.codefile
    }

    /// Check if this module has sub-modules
    pub fn has_submodules(&self) -> bool {
        !self.upstream.is_empty()
    }

    /// Check if this module has code files
    pub fn has_files(&self) -> bool {
        !self.codefile.is_empty()
    }

    /// Get pub visibility setting for the module
    pub fn pub_setting(&self) -> Option<&str> {
        self.r#pub.as_deref()
    }
}

impl CodeFile {
    /// Get file name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get file name with extension based on language
    pub fn filename_with_extension(&self, language: &str) -> String {
        if self.name.contains('.') {
            // Already has extension
            self.name.clone()
        } else {
            // Add language-specific extension
            let extension = match language {
                "rust" => "rs",
                "go" => "go",
                "python" => "py",
                "javascript" => "js",
                "typescript" => "ts",
                _ => "txt", // fallback
            };
            format!("{}.{}", self.name, extension)
        }
    }

    /// Get pub visibility setting
    pub fn pub_setting(&self) -> Option<&str> {
        self.r#pub.as_deref()
    }
}