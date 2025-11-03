// File: src/request_context.rs
// Purpose: Request context with query params, headers, cookies, and form data

use axum::http::{HeaderMap, Method};
use serde_json::Value as JsonValue;
use std::collections::HashMap;

/// Request context passed to data functions and templates
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// HTTP method (GET, POST, PUT, DELETE, etc.)
    pub method: Method,

    /// Query parameters from URL (?key=value)
    pub query: QueryParams,

    /// Form data from POST/PUT requests
    pub form: FormData,

    /// Request headers
    pub headers: HeaderMap,

    /// Parsed cookies
    pub cookies: HashMap<String, String>,

    /// Request path
    pub path: String,
}

impl RequestContext {
    /// Create a new request context
    pub fn new(
        method: Method,
        path: String,
        query: QueryParams,
        form: FormData,
        headers: HeaderMap,
    ) -> Self {
        // Parse cookies from headers
        let cookies = Self::parse_cookies(&headers);

        Self {
            method,
            query,
            form,
            headers,
            cookies,
            path,
        }
    }

    /// Parse cookies from Cookie header
    fn parse_cookies(headers: &HeaderMap) -> HashMap<String, String> {
        let mut cookies = HashMap::new();

        if let Some(cookie_header) = headers.get("cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                for cookie in cookie_str.split(';') {
                    let cookie = cookie.trim();
                    if let Some((key, value)) = cookie.split_once('=') {
                        cookies.insert(key.to_string(), value.to_string());
                    }
                }
            }
        }

        cookies
    }

    /// Get a cookie value
    pub fn get_cookie(&self, name: &str) -> Option<&String> {
        self.cookies.get(name)
    }

    /// Get a header value
    pub fn get_header(&self, name: &str) -> Option<&str> {
        self.headers.get(name)?.to_str().ok()
    }

    /// Check if request accepts JSON
    pub fn accepts_json(&self) -> bool {
        if let Some(accept) = self.get_header("accept") {
            accept.contains("application/json") || accept.contains("json")
        } else {
            false
        }
    }

    /// Check if request wants a partial/fragment response (without layout)
    /// Returns true if:
    /// - Query parameter ?partial=true is present
    /// - HX-Request header is present (HTMX request)
    /// - X-Partial header is present
    pub fn wants_partial(&self) -> bool {
        // Check query parameter
        if self.query.get("partial") == Some(&"true".to_string()) {
            return true;
        }

        // Check HTMX header
        if self.get_header("hx-request").is_some() {
            return true;
        }

        // Check X-Partial header
        if self.get_header("x-partial").is_some() {
            return true;
        }

        false
    }

    /// Check if this is an HTMX request
    pub fn is_htmx(&self) -> bool {
        self.get_header("hx-request").is_some()
    }

    /// Get HTMX target element (if present)
    pub fn htmx_target(&self) -> Option<&str> {
        self.get_header("hx-target")
    }

    /// Get HTMX trigger element (if present)
    pub fn htmx_trigger(&self) -> Option<&str> {
        self.get_header("hx-trigger")
    }

    /// Check if this is a specific method
    pub fn is_get(&self) -> bool {
        self.method == Method::GET
    }

    pub fn is_post(&self) -> bool {
        self.method == Method::POST
    }

    pub fn is_put(&self) -> bool {
        self.method == Method::PUT
    }

    pub fn is_delete(&self) -> bool {
        self.method == Method::DELETE
    }
}

/// Query parameters from URL
#[derive(Debug, Clone, Default)]
pub struct QueryParams {
    params: HashMap<String, String>,
}

impl QueryParams {
    /// Create from HashMap
    pub fn new(params: HashMap<String, String>) -> Self {
        Self { params }
    }

    /// Get a query parameter value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }

    /// Get a query parameter as a specific type
    pub fn get_as<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        self.params.get(key)?.parse().ok()
    }

    /// Check if a parameter exists
    pub fn has(&self, key: &str) -> bool {
        self.params.contains_key(key)
    }

    /// Get all parameter names
    pub fn keys(&self) -> Vec<&String> {
        self.params.keys().collect()
    }

    /// Get as HashMap
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.params
    }
}

/// Form data from POST/PUT requests
#[derive(Debug, Clone, Default)]
pub struct FormData {
    fields: HashMap<String, String>,
    raw_json: Option<JsonValue>,
}

impl FormData {
    /// Create empty form data
    pub fn new() -> Self {
        Self {
            fields: HashMap::new(),
            raw_json: None,
        }
    }

    /// Create from form fields
    pub fn from_fields(fields: HashMap<String, String>) -> Self {
        Self {
            fields,
            raw_json: None,
        }
    }

    /// Create from JSON
    pub fn from_json(json: JsonValue) -> Self {
        let mut fields = HashMap::new();

        // If JSON is an object, extract fields
        if let JsonValue::Object(map) = &json {
            for (key, value) in map {
                if let Some(s) = value.as_str() {
                    fields.insert(key.clone(), s.to_string());
                } else {
                    fields.insert(key.clone(), value.to_string());
                }
            }
        }

        Self {
            fields,
            raw_json: Some(json),
        }
    }

    /// Get a form field value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.fields.get(key)
    }

    /// Get a form field as a specific type
    pub fn get_as<T: std::str::FromStr>(&self, key: &str) -> Option<T> {
        self.fields.get(key)?.parse().ok()
    }

    /// Check if a field exists
    pub fn has(&self, key: &str) -> bool {
        self.fields.contains_key(key)
    }

    /// Get all field names
    pub fn keys(&self) -> Vec<&String> {
        self.fields.keys().collect()
    }

    /// Get raw JSON if available
    pub fn json(&self) -> Option<&JsonValue> {
        self.raw_json.as_ref()
    }

    /// Get as HashMap
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.fields
    }

    /// Check if form is empty
    pub fn is_empty(&self) -> bool {
        self.fields.is_empty() && self.raw_json.is_none()
    }
}
