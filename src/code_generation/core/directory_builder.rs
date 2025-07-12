use std::fs;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use crate::project_management::config::models::{Project, Module};

/// Generic directory structure builder
pub struct DirectoryBuilder;

impl DirectoryBuilder {
    /// Build complete project directory structure
    pub fn build_project_structure<P: AsRef<Path>>(
        base_path: P,
        project: &Project,
    ) -> Result<()> {
        let project_path = if project.is_root() {
            // Root project creates files in current directory
            base_path.as_ref().to_path_buf()
        } else {
            // Non-root project creates subdirectory
            let project_dir = base_path.as_ref().join(project.name());
            fs::create_dir_all(&project_dir)
                .with_context(|| format!("Failed to create project directory: {}", project_dir.display()))?;
            project_dir
        };

        // Build module structure recursively
        for module in project.modules() {
            Self::build_module_structure(&project_path, module)?;
        }

        Ok(())
    }

    /// Build module directory structure recursively
    pub fn build_module_structure<P: AsRef<Path>>(
        base_path: P,
        module: &Module,
    ) -> Result<()> {
        let module_path = base_path.as_ref().join(module.name());
        
        // Create directory for this module
        fs::create_dir_all(&module_path)
            .with_context(|| format!("Failed to create module directory: {}", module_path.display()))?;

        // Recursively build submodule structures
        for submodule in module.submodules() {
            Self::build_module_structure(&module_path, submodule)?;
        }

        Ok(())
    }

    /// Get all directory paths that will be created for a project
    pub fn get_project_directories(project: &Project) -> Vec<PathBuf> {
        let mut directories = Vec::new();
        
        let base_path = if project.is_root() {
            PathBuf::new()
        } else {
            PathBuf::from(project.name())
        };

        for module in project.modules() {
            Self::collect_module_directories(&base_path, module, &mut directories);
        }

        directories
    }

    /// Recursively collect all directory paths for a module
    fn collect_module_directories(
        base_path: &Path,
        module: &Module,
        directories: &mut Vec<PathBuf>,
    ) {
        let module_path = base_path.join(module.name());
        directories.push(module_path.clone());

        for submodule in module.submodules() {
            Self::collect_module_directories(&module_path, submodule, directories);
        }
    }

    /// Check if directory structure already exists
    pub fn structure_exists<P: AsRef<Path>>(
        base_path: P,
        project: &Project,
    ) -> bool {
        let directories = Self::get_project_directories(project);
        
        directories.iter().all(|dir| {
            let full_path = base_path.as_ref().join(dir);
            full_path.exists() && full_path.is_dir()
        })
    }

    /// Clean up project directory structure (for testing or rebuilding)
    pub fn clean_project_structure<P: AsRef<Path>>(
        base_path: P,
        project: &Project,
    ) -> Result<()> {
        if project.is_root() {
            // For root projects, only clean specific module directories
            for module in project.modules() {
                let module_path = base_path.as_ref().join(module.name());
                if module_path.exists() {
                    fs::remove_dir_all(&module_path)
                        .with_context(|| format!("Failed to remove directory: {}", module_path.display()))?;
                }
            }
        } else {
            // For non-root projects, remove the entire project directory
            let project_path = base_path.as_ref().join(project.name());
            if project_path.exists() {
                fs::remove_dir_all(&project_path)
                    .with_context(|| format!("Failed to remove project directory: {}", project_path.display()))?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use crate::project_management::config::models::*;

    #[test]
    fn test_build_simple_project_structure() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let project = Project {
            name: "app".to_string(),
            root: true,
            lang: "rust".to_string(),
            upstream: vec![
                Module {
                    name: "src".to_string(),
                    upstream: vec![
                        Module {
                            name: "domain".to_string(),
                            upstream: vec![],
                            codefile: vec![],
                        },
                    ],
                    codefile: vec![],
                },
            ],
        };

        DirectoryBuilder::build_project_structure(base_path, &project).unwrap();

        // Check directories exist
        assert!(base_path.join("src").exists());
        assert!(base_path.join("src/domain").exists());
    }

    #[test]
    fn test_build_multi_project_structure() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let project = Project {
            name: "backend".to_string(),
            root: false,
            lang: "rust".to_string(),
            upstream: vec![
                Module {
                    name: "src".to_string(),
                    upstream: vec![],
                    codefile: vec![],
                },
            ],
        };

        DirectoryBuilder::build_project_structure(base_path, &project).unwrap();

        // Check project directory and subdirectories exist
        assert!(base_path.join("backend").exists());
        assert!(base_path.join("backend/src").exists());
    }

    #[test]
    fn test_get_project_directories() {
        let project = Project {
            name: "app".to_string(),
            root: true,
            lang: "rust".to_string(),
            upstream: vec![
                Module {
                    name: "src".to_string(),
                    upstream: vec![
                        Module {
                            name: "domain".to_string(),
                            upstream: vec![
                                Module {
                                    name: "model".to_string(),
                                    upstream: vec![],
                                    codefile: vec![],
                                },
                            ],
                            codefile: vec![],
                        },
                    ],
                    codefile: vec![],
                },
            ],
        };

        let directories = DirectoryBuilder::get_project_directories(&project);
        
        assert_eq!(directories.len(), 3);
        assert!(directories.contains(&PathBuf::from("src")));
        assert!(directories.contains(&PathBuf::from("src/domain")));
        assert!(directories.contains(&PathBuf::from("src/domain/model")));
    }

    #[test]
    fn test_structure_exists() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let project = Project {
            name: "app".to_string(),
            root: true,
            lang: "rust".to_string(),
            upstream: vec![
                Module {
                    name: "src".to_string(),
                    upstream: vec![],
                    codefile: vec![],
                },
            ],
        };

        // Initially doesn't exist
        assert!(!DirectoryBuilder::structure_exists(base_path, &project));

        // After building, should exist
        DirectoryBuilder::build_project_structure(base_path, &project).unwrap();
        assert!(DirectoryBuilder::structure_exists(base_path, &project));
    }

    #[test]
    fn test_clean_project_structure() {
        let temp_dir = TempDir::new().unwrap();
        let base_path = temp_dir.path();

        let project = Project {
            name: "backend".to_string(),
            root: false,
            lang: "rust".to_string(),
            upstream: vec![
                Module {
                    name: "src".to_string(),
                    upstream: vec![],
                    codefile: vec![],
                },
            ],
        };

        // Build and verify structure exists
        DirectoryBuilder::build_project_structure(base_path, &project).unwrap();
        assert!(base_path.join("backend").exists());

        // Clean and verify structure is removed
        DirectoryBuilder::clean_project_structure(base_path, &project).unwrap();
        assert!(!base_path.join("backend").exists());
    }
}