// File: src/template_loader.rs
// Purpose: Loads RHTML templates from the pages/ directory

use crate::parser::css::{CssParser, ScopedCss};
use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a loaded RHTML template
#[derive(Debug, Clone)]
pub struct Template {
    pub path: PathBuf,
    pub content: String,
    pub scoped_css: Option<ScopedCss>,
}

/// Template loader that reads and caches RHTML files
pub struct TemplateLoader {
    pages_dir: PathBuf,
    components_dir: PathBuf,
    templates: HashMap<String, Template>,
    components: HashMap<String, Template>,
}

impl TemplateLoader {
    /// Create a new template loader
    pub fn new(pages_dir: impl Into<PathBuf>) -> Self {
        Self {
            pages_dir: pages_dir.into(),
            components_dir: "components".into(),
            templates: HashMap::new(),
            components: HashMap::new(),
        }
    }

    /// Load all templates from the pages directory
    pub fn load_all(&mut self) -> Result<()> {
        self.load_directory(&self.pages_dir.clone())?;
        self.load_components()?;
        Ok(())
    }

    /// Load all components from the components directory
    fn load_components(&mut self) -> Result<()> {
        let components_dir = self.components_dir.clone();
        if !components_dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(&components_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("rhtml") {
                // Load component file
                self.load_component(&path)?;
            }
        }

        Ok(())
    }

    /// Load a single component file
    fn load_component(&mut self, path: &Path) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read component: {:?}", path))?;

        // Component name is the file name without extension
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();

        // Process CSS
        let (content_without_css, scoped_css) = CssParser::process_template(&content);

        let template = Template {
            path: path.to_path_buf(),
            content: content_without_css,
            scoped_css,
        };

        self.components.insert(name.clone(), template);

        println!("🧩 Loaded component: {} -> {:?}", name, path.file_name().unwrap());

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

        // Process CSS
        let (content_without_css, scoped_css) = CssParser::process_template(&content);

        let template = Template {
            path: path.to_path_buf(),
            content: content_without_css,
            scoped_css,
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

    /// Get a component by name
    pub fn get_component(&self, name: &str) -> Option<&Template> {
        self.components.get(name)
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
