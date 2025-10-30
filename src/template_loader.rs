// File: src/template_loader.rs
// Purpose: Loads RHTML templates from the pages/ directory

use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a loaded RHTML template
#[derive(Debug, Clone)]
pub struct Template {
    pub path: PathBuf,
    pub content: String,
}

/// Template loader that reads and caches RHTML files
pub struct TemplateLoader {
    pages_dir: PathBuf,
    templates: HashMap<String, Template>,
}

impl TemplateLoader {
    /// Create a new template loader
    pub fn new(pages_dir: impl Into<PathBuf>) -> Self {
        Self {
            pages_dir: pages_dir.into(),
            templates: HashMap::new(),
        }
    }

    /// Load all templates from the pages directory
    pub fn load_all(&mut self) -> Result<()> {
        self.load_directory(&self.pages_dir.clone())?;
        Ok(())
    }

    /// Recursively load templates from a directory
    fn load_directory(&mut self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                // Recursively load subdirectories
                self.load_directory(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("rhtml") {
                // Load .rhtml files
                self.load_template(&path)?;
            }
        }

        Ok(())
    }

    /// Load a single template file
    fn load_template(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read template: {:?}", path))?;

        // Generate route key from file path
        let route = self.path_to_route(path);

        let template = Template {
            path: path.to_path_buf(),
            content,
        };

        self.templates.insert(route.clone(), template);

        println!(
            "📄 Loaded template: {} -> {:?}",
            route,
            path.file_name().unwrap()
        );

        Ok(())
    }

    /// Convert file path to route (e.g., pages/index.rhtml -> "/")
    fn path_to_route(&self, path: &Path) -> String {
        let relative = path.strip_prefix(&self.pages_dir).unwrap_or(path);

        let route = relative
            .with_extension("")
            .to_string_lossy()
            .replace('\\', "/");

        // Convert "index" to "/"
        if route == "index" || route.is_empty() {
            "/".to_string()
        } else if route.starts_with('/') {
            route
        } else {
            format!("/{}", route)
        }
    }

    /// Get a template by route
    pub fn get(&self, route: &str) -> Option<&Template> {
        self.templates.get(route)
    }

    /// Get the layout template
    pub fn get_layout(&self) -> Option<&Template> {
        self.templates.get("/_layout")
    }

    /// List all loaded templates
    pub fn list_routes(&self) -> Vec<String> {
        let mut routes: Vec<_> = self.templates.keys().cloned().collect();
        routes.sort();
        routes
    }

    /// Get total number of loaded templates
    pub fn count(&self) -> usize {
        self.templates.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_to_route() {
        let loader = TemplateLoader::new("pages");

        // Test cases
        assert_eq!(loader.path_to_route(Path::new("pages/index.rhtml")), "/");
        assert_eq!(
            loader.path_to_route(Path::new("pages/about.rhtml")),
            "/about"
        );
        assert_eq!(
            loader.path_to_route(Path::new("pages/users/profile.rhtml")),
            "/users/profile"
        );
    }
}
