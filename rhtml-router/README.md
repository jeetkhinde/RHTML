# rhtml-router

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

A zero-dependency file-based router for Rust web frameworks.

## Features

- 🚀 **Zero dependencies** - Only uses Rust standard library
- 📁 **File-based routing** - Routes automatically generated from file structure
- 🎯 **Smart prioritization** - Static routes match before dynamic routes
- 🔀 **Dynamic segments** - `/users/:id`, `/posts/:slug`
- 🌟 **Catch-all routes** - `/docs/*path` matches any path depth
- ❓ **Optional parameters** - `/posts/:id?` matches with or without ID
- 🎨 **Layout support** - Special `_layout` files for nested layouts
- ⚠️ **Error pages** - Custom error pages with `_error` files
- 🔤 **Case-insensitive** - Optional case-insensitive matching
- ⚡ **Framework-agnostic** - Works with Axum, Actix, Rocket, Warp, etc.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rhtml-router = "0.1.0"
```

Or use cargo add:

```bash
cargo add rhtml-router
```

## Quick Start

```rust
use rhtml_router::{Router, Route};

fn main() {
    let mut router = Router::new();

    // Add routes from file paths
    router.add_route(Route::from_path("pages/index.rhtml", "pages"));
    router.add_route(Route::from_path("pages/about.rhtml", "pages"));
    router.add_route(Route::from_path("pages/users/[id].rhtml", "pages"));
    router.add_route(Route::from_path("pages/docs/[...slug].rhtml", "pages"));

    // Sort routes by priority (call this after adding all routes)
    router.sort_routes();

    // Match incoming requests
    if let Some(route_match) = router.match_route("/users/123") {
        println!("Matched: {}", route_match.route.pattern);
        println!("User ID: {}", route_match.params["id"]);
    }
}
```

## File Naming Convention

| File Path | Route Pattern | Description |
|-----------|---------------|-------------|
| `pages/index.rhtml` | `/` | Root page |
| `pages/about.rhtml` | `/about` | Static route |
| `pages/users/index.rhtml` | `/users` | Section index |
| `pages/users/[id].rhtml` | `/users/:id` | Dynamic segment |
| `pages/docs/[...slug].rhtml` | `/docs/*slug` | Catch-all |
| `pages/posts/[id?].rhtml` | `/posts/:id?` | Optional param |
| `pages/_layout.rhtml` | `/` | Root layout |
| `pages/users/_layout.rhtml` | `/users` | Section layout |
| `pages/_error.rhtml` | `/` | Root error page |

## Examples

### Basic Routing

```rust
use rhtml_router::{Router, Route};

let mut router = Router::new();
router.add_route(Route::from_path("pages/about.rhtml", "pages"));
router.sort_routes();

let result = router.match_route("/about").unwrap();
assert_eq!(result.route.pattern, "/about");
```

### Dynamic Routes

```rust
use rhtml_router::{Router, Route};

let mut router = Router::new();
router.add_route(Route::from_path("pages/users/[id].rhtml", "pages"));
router.sort_routes();

let result = router.match_route("/users/42").unwrap();
assert_eq!(result.params["id"], "42");
```

### Catch-all Routes

```rust
use rhtml_router::{Router, Route};

let mut router = Router::new();
router.add_route(Route::from_path("pages/docs/[...slug].rhtml", "pages"));
router.sort_routes();

let result = router.match_route("/docs/guide/intro").unwrap();
assert_eq!(result.params["slug"], "guide/intro");
```

### Optional Parameters

```rust
use rhtml_router::{Router, Route};

let mut router = Router::new();
router.add_route(Route::from_path("pages/posts/[id?].rhtml", "pages"));
router.sort_routes();

// Matches with parameter
let result = router.match_route("/posts/123").unwrap();
assert_eq!(result.params["id"], "123");

// Matches without parameter
let result = router.match_route("/posts").unwrap();
assert!(result.params.get("id").is_none());
```

### Case-Insensitive Routing

```rust
use rhtml_router::{Router, Route};

let mut router = Router::with_case_insensitive(true);
router.add_route(Route::from_path("pages/about.rhtml", "pages"));
router.sort_routes();

// All match the same route
assert!(router.match_route("/about").is_some());
assert!(router.match_route("/About").is_some());
assert!(router.match_route("/ABOUT").is_some());
```

### Integration with Axum

```rust
use axum::{Router as AxumRouter, routing::get, extract::Path};
use rhtml_router::{Router, Route};
use std::collections::HashMap;

async fn handle_route(
    Path(path): Path<String>,
) -> String {
    let mut router = Router::new();
    router.add_route(Route::from_path("pages/users/[id].rhtml", "pages"));
    router.sort_routes();

    if let Some(route_match) = router.match_route(&format!("/{}", path)) {
        format!("User ID: {}", route_match.params["id"])
    } else {
        "Not found".to_string()
    }
}

#[tokio::main]
async fn main() {
    let app = AxumRouter::new()
        .route("/*path", get(handle_route));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

## Route Priority

Routes are matched in order of priority:

1. **Static routes** (priority = 0) - Exact matches like `/about`, `/users/new`
2. **Optional parameters** (priority = depth + params) - `/posts/:id?`
3. **Required dynamic routes** (priority = depth + params + 1) - `/users/:id`
4. **Catch-all routes** (priority = 1000+) - `/docs/*path`

### Example Priority Order

```rust
let static_route = Route::from_path("pages/users/new.rhtml", "pages");
// Priority: 0 (static always wins)

let optional_route = Route::from_path("pages/users/[id?].rhtml", "pages");
// Priority: ~3

let dynamic_route = Route::from_path("pages/users/[id].rhtml", "pages");
// Priority: ~4

let catchall_route = Route::from_path("pages/users/[...rest].rhtml", "pages");
// Priority: 1000+
```

When matching `/users/new`:
- `static_route` matches first ✅
- Other routes never checked

When matching `/users/123`:
- `static_route` doesn't match
- `optional_route` matches ✅

## Special Files

### Layout Files (`_layout.rhtml`)

Layout files define nested layouts. The router tracks them separately:

```rust
let mut router = Router::new();
router.add_route(Route::from_path("pages/_layout.rhtml", "pages"));
router.add_route(Route::from_path("pages/users/_layout.rhtml", "pages"));

// Get layout for a route
let layout = router.get_layout("/users/123").unwrap();
assert_eq!(layout.pattern, "/users");
```

### Error Pages (`_error.rhtml`)

Error pages handle 404s and other errors:

```rust
let mut router = Router::new();
router.add_route(Route::from_path("pages/_error.rhtml", "pages"));
router.add_route(Route::from_path("pages/api/_error.rhtml", "pages"));

// Get error page for a route
let error = router.get_error_page("/api/users").unwrap();
assert_eq!(error.pattern, "/api"); // Section-specific error page
```

## Performance

- **Route sorting**: O(n log n) at startup
- **Route matching**: O(n) worst case, typically O(1) for static routes
- **Memory**: Minimal overhead, only stores route metadata
- **Zero allocations** during matching (except for parameter extraction)

## Testing

Run tests:

```bash
cargo test
```

Run with coverage:

```bash
cargo test --all-features
```

## Use Cases

- **Web frameworks** - Add file-based routing to any Rust web framework
- **Static site generators** - Map file structure to URLs
- **API gateways** - Route requests based on file structure
- **Documentation sites** - Perfect for nested documentation
- **Content management** - File-based content routing

## Comparison with Other Routers

| Feature | rhtml-router | matchit | path-tree |
|---------|--------------|---------|-----------|
| File-based | ✅ | ❌ | ❌ |
| Zero deps | ✅ | ❌ | ❌ |
| Catch-all | ✅ | ✅ | ✅ |
| Optional params | ✅ | ❌ | ❌ |
| Layouts | ✅ | ❌ | ❌ |
| Error pages | ✅ | ❌ | ❌ |
| Case insensitive | ✅ | ❌ | ❌ |

## Contributing

Contributions welcome! Please check out the [RHTML repository](https://github.com/jeetkhinde/RHTML).

## License

MIT License - see [LICENSE](../LICENSE) file for details.

## Changelog

### 0.1.0 (2025-01-04)

- Initial release
- Static routes
- Dynamic segments
- Catch-all routes
- Optional parameters
- Layout support
- Error page support
- Case-insensitive routing
- Zero dependencies

## Acknowledgments

Part of the [RHTML project](https://github.com/jeetkhinde/RHTML) - a Rust-first SSR framework.
