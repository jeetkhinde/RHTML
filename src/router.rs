// File: src/router.rs
// Purpose: File-based routing with dynamic parameters and nested layouts

use std::collections::HashMap;

/// Represents a route with pattern and parameters
#[derive(Debug, Clone)]
pub struct Route {
    /// Route pattern (e.g., "/users/:id")
    pub pattern: String,
    /// Template file path
    pub template_path: String,
    /// Parameter names extracted from pattern
    pub params: Vec<String>,
    /// Route priority (lower = higher priority)
    /// Static routes have priority 0, dynamic routes have priority based on depth
    pub priority: usize,
    /// Whether this is a layout route
    pub is_layout: bool,
}

/// Route match result with extracted parameters
#[derive(Debug, Clone)]
pub struct RouteMatch {
    pub route: Route,
    pub params: HashMap<String, String>,
}

impl Route {
    /// Create a new route from a file path
    /// Examples:
    ///   "users/[id].rhtml" -> "/users/:id" with param "id"
    ///   "users/new.rhtml" -> "/users/new" (static)
    ///   "users/_layout.rhtml" -> "/users" (layout)
    pub fn from_path(file_path: &str, pages_dir: &str) -> Self {
        let relative = if let Some(stripped) = file_path.strip_prefix(pages_dir) {
            stripped.trim_start_matches('/')
        } else {
            file_path
        };

        // Remove .rhtml extension
        let without_ext = relative.strip_suffix(".rhtml").unwrap_or(relative);

        // Check if this is a layout file
        let is_layout = without_ext.ends_with("/_layout") || without_ext == "_layout";

        // Convert to route pattern
        let mut pattern = String::new();
        let mut params = Vec::new();
        let mut dynamic_count = 0;

        for segment in without_ext.split('/') {
            if segment.is_empty() {
                continue;
            }

            // Skip _layout segments in pattern
            if segment == "_layout" {
                continue;
            }

            // Handle index -> /
            if segment == "index" {
                continue;
            }

            // Handle dynamic segments [id] -> :id
            if segment.starts_with('[') && segment.ends_with(']') {
                let param_name = &segment[1..segment.len() - 1];
                pattern.push_str("/:");
                pattern.push_str(param_name);
                params.push(param_name.to_string());
                dynamic_count += 1;
            } else {
                pattern.push('/');
                pattern.push_str(segment);
            }
        }

        // Handle root index
        if pattern.is_empty() {
            pattern = "/".to_string();
        }

        // Calculate priority
        // Static routes: priority = 0
        // Dynamic routes: priority = number of dynamic segments + path depth
        let depth = pattern.matches('/').count();
        let priority = if dynamic_count > 0 {
            dynamic_count + depth
        } else {
            0
        };

        Route {
            pattern,
            template_path: file_path.to_string(),
            params,
            priority,
            is_layout,
        }
    }

    /// Check if this route matches a given path
    /// Returns Some(params) if match, None otherwise
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        let pattern_segments: Vec<&str> = self.pattern.split('/').filter(|s| !s.is_empty()).collect();
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        // Must have same number of segments
        if pattern_segments.len() != path_segments.len() {
            return None;
        }

        let mut params = HashMap::new();

        for (pattern_seg, path_seg) in pattern_segments.iter().zip(path_segments.iter()) {
            if pattern_seg.starts_with(':') {
                // Dynamic segment - extract parameter
                let param_name = &pattern_seg[1..];
                params.insert(param_name.to_string(), path_seg.to_string());
            } else if pattern_seg != path_seg {
                // Static segment must match exactly
                return None;
            }
        }

        Some(params)
    }

    /// Get the layout pattern for this route
    /// E.g., "/users/:id" -> "/users" (for users/_layout.rhtml)
    pub fn layout_pattern(&self) -> Option<String> {
        // Find the parent path
        if let Some(last_slash) = self.pattern.rfind('/') {
            if last_slash == 0 {
                // Root level - no section layout
                None
            } else {
                Some(self.pattern[..last_slash].to_string())
            }
        } else {
            None
        }
    }
}

/// Router that manages all routes
#[derive(Clone)]
pub struct Router {
    routes: Vec<Route>,
    layouts: HashMap<String, Route>,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            layouts: HashMap::new(),
        }
    }

    /// Add a route
    pub fn add_route(&mut self, route: Route) {
        if route.is_layout {
            self.layouts.insert(route.pattern.clone(), route);
        } else {
            self.routes.push(route);
        }
    }

    /// Remove a route by pattern
    pub fn remove_route(&mut self, pattern: &str) {
        // Remove from routes
        self.routes.retain(|r| r.pattern != pattern);

        // Remove from layouts
        self.layouts.remove(pattern);
    }

    /// Sort routes by priority (lower priority number = higher priority)
    pub fn sort_routes(&mut self) {
        self.routes.sort_by_key(|r| r.priority);
    }

    /// Find a matching route for a given path
    pub fn match_route(&self, path: &str) -> Option<RouteMatch> {
        for route in &self.routes {
            if let Some(params) = route.matches(path) {
                return Some(RouteMatch {
                    route: route.clone(),
                    params,
                });
            }
        }
        None
    }

    /// Get the layout for a given route pattern
    pub fn get_layout(&self, pattern: &str) -> Option<&Route> {
        // First, try to find a layout for this exact pattern (for /users -> /users layout)
        if pattern != "/" {
            if let Some(layout) = self.layouts.get(pattern) {
                return Some(layout);
            }
        }

        // Then try to find a section layout by looking at parent path
        if let Some(last_slash) = pattern.rfind('/') {
            if last_slash > 0 {
                let section = &pattern[..last_slash];
                if let Some(layout) = self.layouts.get(section) {
                    return Some(layout);
                }
            }
        }

        // Fall back to root layout
        self.layouts.get("/")
    }

    /// Get all routes
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }

    /// Get all layouts
    pub fn layouts(&self) -> &HashMap<String, Route> {
        &self.layouts
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_from_path_static() {
        let route = Route::from_path("pages/about.rhtml", "pages");
        assert_eq!(route.pattern, "/about");
        assert_eq!(route.params.len(), 0);
        assert_eq!(route.priority, 0);
    }

    #[test]
    fn test_route_from_path_dynamic() {
        let route = Route::from_path("pages/users/[id].rhtml", "pages");
        assert_eq!(route.pattern, "/users/:id");
        assert_eq!(route.params, vec!["id"]);
        assert!(route.priority > 0);
    }

    #[test]
    fn test_route_from_path_index() {
        let route = Route::from_path("pages/index.rhtml", "pages");
        assert_eq!(route.pattern, "/");
    }

    #[test]
    fn test_route_from_path_nested_index() {
        let route = Route::from_path("pages/users/index.rhtml", "pages");
        assert_eq!(route.pattern, "/users");
    }

    #[test]
    fn test_route_matches_static() {
        let route = Route::from_path("pages/about.rhtml", "pages");
        assert!(route.matches("/about").is_some());
        assert!(route.matches("/about/").is_none());
        assert!(route.matches("/other").is_none());
    }

    #[test]
    fn test_route_matches_dynamic() {
        let route = Route::from_path("pages/users/[id].rhtml", "pages");
        let params = route.matches("/users/123").unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_route_priority() {
        let static_route = Route::from_path("pages/users/new.rhtml", "pages");
        let dynamic_route = Route::from_path("pages/users/[id].rhtml", "pages");

        assert!(static_route.priority < dynamic_route.priority);
    }

    #[test]
    fn test_router_matching() {
        let mut router = Router::new();

        router.add_route(Route::from_path("pages/users/new.rhtml", "pages"));
        router.add_route(Route::from_path("pages/users/[id].rhtml", "pages"));
        router.sort_routes();

        // Static route should match first
        let m = router.match_route("/users/new").unwrap();
        assert_eq!(m.route.pattern, "/users/new");
        assert_eq!(m.params.len(), 0);

        // Dynamic route should match for other IDs
        let m = router.match_route("/users/123").unwrap();
        assert_eq!(m.route.pattern, "/users/:id");
        assert_eq!(m.params.get("id"), Some(&"123".to_string()));
    }

    #[test]
    fn test_layout_route() {
        let route = Route::from_path("pages/users/_layout.rhtml", "pages");
        assert_eq!(route.pattern, "/users");
        assert!(route.is_layout);
    }
}
