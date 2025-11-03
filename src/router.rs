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
    /// Whether this route has a catch-all parameter
    pub has_catch_all: bool,
    /// Optional parameter names (ending with ?)
    pub optional_params: Vec<String>,
    /// Whether this is an error page
    pub is_error_page: bool,
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
    ///   "docs/[...slug].rhtml" -> "/docs/*slug" (catch-all)
    ///   "posts/[id?].rhtml" -> "/posts/:id?" (optional)
    ///   "_error.rhtml" -> special error page
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

        // Check if this is an error page
        let is_error_page = without_ext.ends_with("/_error") || without_ext == "_error";

        // Convert to route pattern
        let mut pattern = String::new();
        let mut params = Vec::new();
        let mut optional_params = Vec::new();
        let mut dynamic_count = 0;
        let mut has_catch_all = false;

        for segment in without_ext.split('/') {
            if segment.is_empty() {
                continue;
            }

            // Skip _layout and _error segments in pattern
            if segment == "_layout" || segment == "_error" {
                continue;
            }

            // Handle index -> /
            if segment == "index" {
                continue;
            }

            // Handle dynamic segments
            if segment.starts_with('[') && segment.ends_with(']') {
                let inner = &segment[1..segment.len() - 1];

                // Check for catch-all [...slug]
                if inner.starts_with("...") {
                    let param_name = &inner[3..];
                    pattern.push_str("/*");
                    pattern.push_str(param_name);
                    params.push(param_name.to_string());
                    has_catch_all = true;
                    dynamic_count += 100; // Very low priority
                }
                // Check for optional [id?]
                else if inner.ends_with('?') {
                    let param_name = &inner[..inner.len() - 1];
                    pattern.push_str("/:");
                    pattern.push_str(param_name);
                    pattern.push('?');
                    params.push(param_name.to_string());
                    optional_params.push(param_name.to_string());
                    dynamic_count += 1;
                }
                // Regular dynamic segment [id]
                else {
                    pattern.push_str("/:");
                    pattern.push_str(inner);
                    params.push(inner.to_string());
                    dynamic_count += 1;
                }
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
        // Optional param routes: priority = dynamic_count + depth + 1
        // Required dynamic routes: priority = dynamic_count + depth + 2
        // Catch-all routes: priority = 1000 (lowest priority)
        let depth = pattern.matches('/').count();
        let priority = if has_catch_all {
            1000 + depth
        } else if dynamic_count > 0 {
            let optional_bonus = if !optional_params.is_empty() { 0 } else { 1 };
            dynamic_count + depth + optional_bonus
        } else {
            0
        };

        Route {
            pattern,
            template_path: file_path.to_string(),
            params,
            priority,
            is_layout,
            has_catch_all,
            optional_params,
            is_error_page,
        }
    }

    /// Check if this route matches a given path (case-sensitive)
    /// Returns Some(params) if match, None otherwise
    pub fn matches(&self, path: &str) -> Option<HashMap<String, String>> {
        self.matches_with_options(path, false)
    }

    /// Check if this route matches a given path with options
    /// Returns Some(params) if match, None otherwise
    pub fn matches_with_options(&self, path: &str, case_insensitive: bool) -> Option<HashMap<String, String>> {
        let pattern_segments: Vec<&str> = self.pattern.split('/').filter(|s| !s.is_empty()).collect();
        let path_segments: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        let mut params = HashMap::new();
        let mut pattern_idx = 0;
        let mut path_idx = 0;

        while pattern_idx < pattern_segments.len() {
            let pattern_seg = pattern_segments[pattern_idx];

            // Handle catch-all segment (e.g., *slug)
            if pattern_seg.starts_with('*') {
                let param_name = &pattern_seg[1..];
                // Collect all remaining path segments
                let remaining: Vec<&str> = path_segments[path_idx..].to_vec();
                params.insert(param_name.to_string(), remaining.join("/"));
                return Some(params);
            }

            // Handle optional parameter (e.g., :id?)
            if pattern_seg.ends_with('?') {
                let param_name = &pattern_seg[1..pattern_seg.len() - 1];

                // If there are more path segments, consume one
                if path_idx < path_segments.len() {
                    // Check if next pattern segment matches current path segment
                    // If yes, skip this optional param. If no, consume it.
                    let should_consume = if pattern_idx + 1 < pattern_segments.len() {
                        let next_pattern = pattern_segments[pattern_idx + 1];
                        if next_pattern.starts_with(':') || next_pattern.starts_with('*') {
                            // Next is also dynamic, consume this one
                            true
                        } else {
                            // Next is static, check if current path matches it
                            let matches = if case_insensitive {
                                !next_pattern.eq_ignore_ascii_case(path_segments[path_idx])
                            } else {
                                next_pattern != path_segments[path_idx]
                            };
                            matches
                        }
                    } else {
                        // No more pattern segments, consume if there are path segments left
                        true
                    };

                    if should_consume && path_idx < path_segments.len() {
                        params.insert(param_name.to_string(), path_segments[path_idx].to_string());
                        path_idx += 1;
                    }
                }
                pattern_idx += 1;
                continue;
            }

            // Handle regular dynamic segment (e.g., :id)
            if pattern_seg.starts_with(':') {
                if path_idx >= path_segments.len() {
                    return None;
                }
                let param_name = &pattern_seg[1..];
                params.insert(param_name.to_string(), path_segments[path_idx].to_string());
                path_idx += 1;
                pattern_idx += 1;
                continue;
            }

            // Handle static segment
            if path_idx >= path_segments.len() {
                return None;
            }

            let matches = if case_insensitive {
                pattern_seg.eq_ignore_ascii_case(path_segments[path_idx])
            } else {
                pattern_seg == path_segments[path_idx]
            };

            if !matches {
                return None;
            }

            path_idx += 1;
            pattern_idx += 1;
        }

        // All pattern segments matched, check if all path segments consumed
        if path_idx == path_segments.len() {
            Some(params)
        } else {
            None
        }
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
    error_pages: HashMap<String, Route>,
    case_insensitive: bool,
}

impl Router {
    /// Create a new router
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            layouts: HashMap::new(),
            error_pages: HashMap::new(),
            case_insensitive: false,
        }
    }

    /// Create a new router with case-insensitive matching
    pub fn with_case_insensitive(case_insensitive: bool) -> Self {
        Self {
            routes: Vec::new(),
            layouts: HashMap::new(),
            error_pages: HashMap::new(),
            case_insensitive,
        }
    }

    /// Set case-insensitive mode
    pub fn set_case_insensitive(&mut self, case_insensitive: bool) {
        self.case_insensitive = case_insensitive;
    }

    /// Add a route
    pub fn add_route(&mut self, route: Route) {
        if route.is_layout {
            self.layouts.insert(route.pattern.clone(), route);
        } else if route.is_error_page {
            self.error_pages.insert(route.pattern.clone(), route);
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

        // Remove from error pages
        self.error_pages.remove(pattern);
    }

    /// Sort routes by priority (lower priority number = higher priority)
    pub fn sort_routes(&mut self) {
        self.routes.sort_by_key(|r| r.priority);
    }

    /// Find a matching route for a given path
    pub fn match_route(&self, path: &str) -> Option<RouteMatch> {
        for route in &self.routes {
            if let Some(params) = route.matches_with_options(path, self.case_insensitive) {
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

    /// Get the error page for a given route pattern
    /// Looks for section-specific error page first, then root error page
    pub fn get_error_page(&self, pattern: &str) -> Option<&Route> {
        // First, try to find an error page for this exact pattern
        if pattern != "/" {
            if let Some(error_page) = self.error_pages.get(pattern) {
                return Some(error_page);
            }
        }

        // Then try to find a section error page by looking at parent path
        if let Some(last_slash) = pattern.rfind('/') {
            if last_slash > 0 {
                let section = &pattern[..last_slash];
                if let Some(error_page) = self.error_pages.get(section) {
                    return Some(error_page);
                }
            }
        }

        // Fall back to root error page
        self.error_pages.get("/")
    }

    /// Get all error pages
    pub fn error_pages(&self) -> &HashMap<String, Route> {
        &self.error_pages
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
        // Trailing slashes are normalized (common web framework behavior)
        assert!(route.matches("/about/").is_some());
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

    #[test]
    fn test_catch_all_route() {
        let route = Route::from_path("pages/docs/[...slug].rhtml", "pages");
        assert_eq!(route.pattern, "/docs/*slug");
        assert_eq!(route.params, vec!["slug"]);
        assert!(route.has_catch_all);
        assert!(route.priority > 100); // Very low priority
    }

    #[test]
    fn test_catch_all_matches() {
        let route = Route::from_path("pages/docs/[...slug].rhtml", "pages");

        // Should match multiple segments
        let params = route.matches("/docs/guide/getting-started").unwrap();
        assert_eq!(params.get("slug"), Some(&"guide/getting-started".to_string()));

        // Should match single segment
        let params = route.matches("/docs/intro").unwrap();
        assert_eq!(params.get("slug"), Some(&"intro".to_string()));

        // Should match empty (just /docs)
        let params = route.matches("/docs").unwrap();
        assert_eq!(params.get("slug"), Some(&"".to_string()));
    }

    #[test]
    fn test_optional_param_route() {
        let route = Route::from_path("pages/posts/[id?].rhtml", "pages");
        assert_eq!(route.pattern, "/posts/:id?");
        assert_eq!(route.params, vec!["id"]);
        assert_eq!(route.optional_params, vec!["id"]);
        assert!(!route.has_catch_all);
    }

    #[test]
    fn test_optional_param_matches() {
        let route = Route::from_path("pages/posts/[id?].rhtml", "pages");

        // Should match with parameter
        let params = route.matches("/posts/123").unwrap();
        assert_eq!(params.get("id"), Some(&"123".to_string()));

        // Should match without parameter
        let params = route.matches("/posts").unwrap();
        assert_eq!(params.get("id"), None);
    }

    #[test]
    fn test_error_page_route() {
        let route = Route::from_path("pages/_error.rhtml", "pages");
        assert_eq!(route.pattern, "/");
        assert!(route.is_error_page);
        assert!(!route.is_layout);
    }

    #[test]
    fn test_section_error_page() {
        let route = Route::from_path("pages/api/_error.rhtml", "pages");
        assert_eq!(route.pattern, "/api");
        assert!(route.is_error_page);
    }

    #[test]
    fn test_route_priority_ordering() {
        let static_route = Route::from_path("pages/users/new.rhtml", "pages");
        let optional_route = Route::from_path("pages/users/[id?].rhtml", "pages");
        let dynamic_route = Route::from_path("pages/users/[id].rhtml", "pages");
        let catchall_route = Route::from_path("pages/users/[...rest].rhtml", "pages");

        // Static should have highest priority (lowest number)
        assert!(static_route.priority < optional_route.priority);
        assert!(optional_route.priority < dynamic_route.priority);
        assert!(dynamic_route.priority < catchall_route.priority);
    }

    #[test]
    fn test_router_with_all_route_types() {
        let mut router = Router::new();

        router.add_route(Route::from_path("pages/docs/[...slug].rhtml", "pages"));
        router.add_route(Route::from_path("pages/docs/api.rhtml", "pages"));
        router.add_route(Route::from_path("pages/posts/[id?].rhtml", "pages"));
        router.add_route(Route::from_path("pages/posts/new.rhtml", "pages"));
        router.sort_routes();

        // Static should match first
        let m = router.match_route("/docs/api").unwrap();
        assert_eq!(m.route.pattern, "/docs/api");

        // Catch-all should match everything else
        let m = router.match_route("/docs/guide/intro").unwrap();
        assert_eq!(m.route.pattern, "/docs/*slug");
        assert_eq!(m.params.get("slug"), Some(&"guide/intro".to_string()));

        // Static posts/new should match
        let m = router.match_route("/posts/new").unwrap();
        assert_eq!(m.route.pattern, "/posts/new");

        // Optional param with value
        let m = router.match_route("/posts/123").unwrap();
        assert_eq!(m.route.pattern, "/posts/:id?");
        assert_eq!(m.params.get("id"), Some(&"123".to_string()));

        // Optional param without value
        let m = router.match_route("/posts").unwrap();
        assert_eq!(m.route.pattern, "/posts/:id?");
        assert_eq!(m.params.get("id"), None);
    }

    #[test]
    fn test_error_page_lookup() {
        let mut router = Router::new();

        router.add_route(Route::from_path("pages/_error.rhtml", "pages"));
        router.add_route(Route::from_path("pages/api/_error.rhtml", "pages"));

        // Root error page
        let error_page = router.get_error_page("/").unwrap();
        assert_eq!(error_page.pattern, "/");

        // Section error page
        let error_page = router.get_error_page("/api").unwrap();
        assert_eq!(error_page.pattern, "/api");

        // Non-existent section should fall back to root
        let error_page = router.get_error_page("/users").unwrap();
        assert_eq!(error_page.pattern, "/");
    }
}
