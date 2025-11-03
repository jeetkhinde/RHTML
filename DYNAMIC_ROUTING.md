# Dynamic Routing Features

This document describes the advanced dynamic routing features available in RHTML.

## Overview

RHTML now supports three powerful routing patterns:

1. **Catch-all routes** `[...slug]` - Match multiple path segments
2. **Optional parameters** `[id?]` - Make route segments optional
3. **Custom error pages** `_error.rhtml` - Beautiful custom error handling

## 1. Catch-All Routes `[...slug]`

Catch-all routes allow you to match any number of path segments after a certain point in the URL.

### File Naming Convention

```
pages/docs/[...slug].rhtml
```

### What It Matches

| URL | Slug Value |
|-----|------------|
| `/docs` | `""` (empty string) |
| `/docs/intro` | `"intro"` |
| `/docs/guide/getting-started` | `"guide/getting-started"` |
| `/docs/api/v1/users/create` | `"api/v1/users/create"` |

### Example Template

```html
<h1>Documentation: {slug}</h1>

{if slug}
  <p>Viewing: {slug}</p>
{else}
  <p>Browse all documentation</p>
{/if}
```

### Use Cases

- **Documentation sites** with nested pages
- **File browsers** or directory listings
- **Blog archives** (year/month/day/slug)
- **Wildcard API proxies**
- **Multi-level category pages**

### Route Priority

Catch-all routes have the **lowest priority** (priority = 1000+). They will only match after all static and dynamic routes have been tried.

```
/docs/api.rhtml         → matches first (static)
/docs/[category].rhtml  → matches second (dynamic)
/docs/[...slug].rhtml   → matches last (catch-all)
```

## 2. Optional Parameters `[id?]`

Optional parameters allow a single route segment to be present or absent, and both will match the same template.

### File Naming Convention

```
pages/posts/[id?].rhtml
```

### What It Matches

| URL | ID Value |
|-----|----------|
| `/posts` | `undefined` (not set) |
| `/posts/123` | `"123"` |
| `/posts/hello-world` | `"hello-world"` |

### Example Template

```html
{if id}
  <!-- Single post view -->
  <h1>Post #{id}</h1>
  <article>Content for post {id}</article>
{else}
  <!-- All posts list view -->
  <h1>All Posts</h1>
  <ul>
    <li><a href="/posts/1">Post 1</a></li>
    <li><a href="/posts/2">Post 2</a></li>
  </ul>
{/if}
```

### Use Cases

- **List + Detail views** in the same template
- **Optional filter parameters**
- **Pagination** where page 1 is the default
- **Profile pages** (show current user by default, or specific user)

### Route Priority

Optional parameters have **higher priority** than required dynamic parameters but **lower priority** than static routes:

```
/posts/new.rhtml     → priority 0 (static, highest)
/posts/[id?].rhtml   → priority 3 (optional)
/posts/[id].rhtml    → priority 4 (required dynamic)
```

### Combining Optional Params with Static Routes

If you have both an optional parameter route and a static route, the static route will always match first:

```
pages/posts/new.rhtml      → matches /posts/new
pages/posts/[id?].rhtml    → matches /posts and /posts/123
```

## 3. Custom Error Pages `_error.rhtml`

Create beautiful, branded error pages that match your application's design.

### File Locations

```
pages/_error.rhtml          → Root error page (404, 500, etc.)
pages/api/_error.rhtml      → Section-specific error page
```

### Error Page Variables

Your error template has access to these variables:

| Variable | Type | Description |
|----------|------|-------------|
| `status` | Number | HTTP status code (404, 500, etc.) |
| `title` | String | Error title ("Page Not Found") |
| `message` | String | Detailed error message |

### Example Error Template

```html
<!DOCTYPE html>
<html>
<head>
    <title>Error {status}</title>
</head>
<body>
    <h1>{status}</h1>
    <h2>{title}</h2>
    <p>{message}</p>
    <a href="/">Go Home</a>
</body>
</html>
```

### Section-Specific Error Pages

Create different error pages for different sections of your site:

```
pages/_error.rhtml          → Default error page
pages/api/_error.rhtml      → JSON error responses for /api/*
pages/admin/_error.rhtml    → Admin-styled errors for /admin/*
```

When an error occurs in `/api/users`, it will:
1. Look for `pages/api/_error.rhtml`
2. Fall back to `pages/_error.rhtml`
3. Fall back to the built-in error page

### Fallback Behavior

If no custom error page is found, RHTML will use the built-in default error page with Tailwind CSS styling.

## Route Priority System

Understanding route priority is crucial for predictable routing behavior:

| Route Type | Priority | Example | Matches |
|------------|----------|---------|---------|
| Static | 0 (highest) | `/users/new` | Exact match only |
| Optional Param | dynamic_count + depth | `/posts/:id?` | With or without param |
| Required Dynamic | dynamic_count + depth + 1 | `/users/:id` | Only with param |
| Catch-all | 1000+ (lowest) | `/docs/*slug` | Everything else |

### Priority Calculation Examples

```rust
// Static routes always have priority 0
"/users/new"           → priority = 0

// Dynamic routes: count + depth
"/users/:id"           → priority = 1 + 2 = 3
"/posts/:slug"         → priority = 1 + 1 = 2  (higher than users)

// Optional params
"/posts/:id?"          → priority = 1 + 1 + 0 = 2

// Catch-all routes
"/docs/*slug"          → priority = 1000 + 1 = 1001
```

## Complete Examples

### Example 1: Documentation Site

```
pages/
├── docs/
│   ├── index.rhtml              → /docs
│   ├── api.rhtml                → /docs/api (static, matches first)
│   └── [...slug].rhtml          → /docs/* (catch-all)
```

**URL Matching:**
- `/docs` → `index.rhtml`
- `/docs/api` → `api.rhtml` (static wins)
- `/docs/guide/intro` → `[...slug].rhtml` (slug = "guide/intro")

### Example 2: Blog with Optional Pagination

```
pages/
├── blog/
│   ├── [page?].rhtml            → /blog or /blog/2
│   └── [slug].rhtml             → /blog/my-post-title
```

**URL Matching:**
- `/blog` → `[page?].rhtml` (page is undefined)
- `/blog/2` → Could match either! Use more specific routing:

```
pages/
├── blog/
│   ├── index.rhtml              → /blog (list)
│   ├── page/
│   │   └── [num].rhtml          → /blog/page/2
│   └── [slug].rhtml             → /blog/my-post-title
```

### Example 3: E-commerce Site

```
pages/
├── _error.rhtml                 → Global errors
├── shop/
│   ├── _error.rhtml            → Shop-specific errors
│   ├── products/
│   │   ├── new.rhtml           → /shop/products/new (static)
│   │   ├── [id].rhtml          → /shop/products/123
│   │   └── [id]/
│   │       ├── edit.rhtml      → /shop/products/123/edit
│   │       └── reviews.rhtml   → /shop/products/123/reviews
│   └── categories/
│       └── [...path].rhtml     → /shop/categories/electronics/phones/iphone
```

## Testing Your Routes

Use the router tests to verify your routing logic:

```rust
#[test]
fn test_my_routes() {
    let mut router = Router::new();

    router.add_route(Route::from_path("pages/docs/[...slug].rhtml", "pages"));
    router.add_route(Route::from_path("pages/posts/[id?].rhtml", "pages"));
    router.sort_routes();

    // Test catch-all
    let m = router.match_route("/docs/guide/intro").unwrap();
    assert_eq!(m.params.get("slug"), Some(&"guide/intro".to_string()));

    // Test optional param
    let m = router.match_route("/posts").unwrap();
    assert_eq!(m.params.get("id"), None);
}
```

## Performance Considerations

- **Route Sorting**: Routes are sorted once at startup by priority
- **Matching Algorithm**: O(n) where n = number of routes at the same priority level
- **Static Routes**: Always checked first (priority 0)
- **Catch-all Routes**: Always checked last (priority 1000+)

For best performance:
1. Use static routes when possible
2. Limit the number of dynamic routes at the same path level
3. Use catch-all routes sparingly (they're powerful but have lower priority)

## Troubleshooting

### My catch-all route isn't matching

**Check:**
- Is there a static route that matches first?
- Is there a more specific dynamic route?
- Did you call `router.sort_routes()`?

### My optional parameter is always undefined

**Check:**
- Are you using the correct syntax? `[id?]` not `[id]`
- Is there a competing static route?
- Check the route priority order

### My error page isn't showing

**Check:**
- Is the file named exactly `_error.rhtml`?
- Is it in the correct directory?
- Does your template have syntax errors?

## Migration Guide

If you're upgrading from an older version:

1. **No Breaking Changes**: All existing routes continue to work
2. **New Features**: Start adding optional params and catch-all routes
3. **Error Pages**: Replace built-in errors with custom `_error.rhtml`
4. **Test**: Run your test suite to verify routing behavior

## See Also

- [File-based Routing](docs/routing.md)
- [Template Syntax](docs/templates.md)
- [API Reference](docs/api.md)
