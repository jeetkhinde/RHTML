# Partial Rendering in RHTML

## Overview

RHTML supports **partial rendering** (HTML fragments without layouts), making it perfect for:
- **HTMX responses** - Return just the HTML snippet to swap in
- **API endpoints returning HTML** - Not full pages
- **Reusable fragments** - Independent components
- **Turbo/Hotwire partial updates**

## How It Works

RHTML automatically detects when to render partials using three methods:

### 1. Automatic Detection (No Page Component)

Files **without** a `WebPage()` component are automatically treated as partials:

```rhtml
<!-- pages/partials/user-item.rs -->
<!-- No WebPage() = automatic partial -->
<div class="user-item p-4 border-b">
    <h3 class="font-semibold">{query_name}</h3>
    <p class="text-sm text-gray-600">{query_email}</p>
</div>
```

### 2. HTMX Request Detection

RHTML automatically detects HTMX requests via the `HX-Request` header:

```html
<button
    hx-get="/partials/user-item?name=John&email=john@example.com"
    hx-target="#container"
    hx-swap="innerHTML">
    Load User
</button>
```

**Result:** Returns just the partial HTML, no layout!

### 3. Manual Override

Force any page to render as a partial using:

#### Query Parameter
```html
<button hx-get="/about?partial=true" hx-target="#container">
    Load About (No Layout)
</button>
```

#### Custom Header
```bash
curl -H "X-Partial: true" http://localhost:3000/users
```

## Template Variables

Access partial/HTMX information in your templates:

```rhtml
WebPage(props: &PageProps<()>) {
  <div>
    <p r-if="is_htmx">This is an HTMX request!</p>
    <p r-if="wants_partial">Rendering as partial</p>

    <div r-if="htmx_target">
      Target: {htmx_target}
    </div>

    <div r-if="htmx_trigger">
      Triggered by: {htmx_trigger}
    </div>
  </div>
}
```

**Available Variables:**
- `{is_htmx}` - true if this is an HTMX request
- `{wants_partial}` - true if partial rendering was requested
- `{htmx_target}` - The target element ID (from `hx-target`)
- `{htmx_trigger}` - The triggering element (from `hx-trigger`)

## @layout Decorator

### Overview

The `@layout` decorator gives you **declarative control** over layout rendering at the file level. Place it at the top of your `.rs` file to specify layout behavior.

### Syntax

```rhtml
@layout(false)         // No layout
@layout("custom")      // Use specific layout (future feature)
// No decorator         // Use default _layout.rs
```

### @layout(false) - No Layout

Explicitly disable layout wrapping for the entire file:

```rhtml
@layout(false)

WebPage(props: &PageProps<()>) {
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>Custom Page</title>
        <script src="https://cdn.tailwindcss.com"></script>
    </head>
    <body class="bg-gray-900 text-white">
        <div class="container mx-auto p-8">
            <h1>No Layout Wrapper!</h1>
            <p>Full control over HTML structure</p>
        </div>
    </body>
    </html>
}
```

**Result:** Page renders without `_layout.rs` wrapper.

### Use Cases for @layout(false)

**1. API Endpoints Returning HTML**
```rhtml
@layout(false)

WebPage(props: &PageProps<()>) {
    <div class="api-response">
        <h2>User Data</h2>
        <p>Name: John Doe</p>
    </div>
}
```

**2. Email Templates**
```rhtml
@layout(false)

WebPage(props: &PageProps<()>) {
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            /* Email-safe inline CSS */
        </style>
    </head>
    <body>
        <table width="600">
            <tr><td>Email content...</td></tr>
        </table>
    </body>
    </html>
}
```

**3. Custom Document Structure**
```rhtml
@layout(false)

WebPage(props: &PageProps<()>) {
    <!DOCTYPE html>
    <html lang="en" data-theme="dark">
    <head>
        <!-- Custom meta tags, different from layout -->
        <meta name="robots" content="noindex">
    </head>
    <body>
        <!-- Your content -->
    </body>
    </html>
}
```

### Combining @layout(false) with Named Partials

**Powerful pattern:** Use `@layout(false)` + named partials for complete flexibility:

```rhtml
@layout(false)

// Named partial: ProductCard
// Access: /products?partial=ProductCard
partial ProductCard(props: &PartialProps<()>) {
    <div class="product-card">
        <h3>{query_name}</h3>
        <p>${query_price}</p>
    </div>
}

// Named partial: ProductList
// Access: /products?partial=ProductList
partial ProductList(props: &PartialProps<()>) {
    <div class="grid grid-cols-3 gap-4">
        <!-- Product grid -->
    </div>
}

// Full page component (no layout)
WebPage(props: &PageProps<()>) {
    <!DOCTYPE html>
    <html>
    <head>
        <title>Products</title>
        <script src="https://cdn.tailwindcss.com"></script>
        <script src="https://unpkg.com/htmx.org"></script>
    </head>
    <body>
        <div class="container mx-auto p-8">
            <h1>Product Catalog</h1>

            <!-- Load partials dynamically -->
            <button
                hx-get="/products?partial=ProductList"
                hx-target="#container">
                Load Products
            </button>

            <div id="container"></div>
        </div>
    </body>
    </html>
}
```

**Benefits:**
- ✅ No layout wrapper - custom HTML structure
- ✅ Multiple partials in one file
- ✅ Perfect for HTMX
- ✅ Domain cohesion (all product-related fragments together)

### @layout vs Other Methods

| Method | When Layout Skipped | Declarative | File-Level |
|--------|---------------------|-------------|------------|
| **@layout(false)** | Always (explicit) | ✅ Yes | ✅ Yes |
| **No Page component** | Always (implicit) | ❌ No | ✅ Yes |
| **?partial=true** | Per request | ❌ No | ❌ No |
| **HX-Request header** | Per request | ❌ No | ❌ No |

**When to use @layout(false):**
- ✅ Page should NEVER use layout
- ✅ Custom HTML structure needed
- ✅ API endpoints, email templates, PDFs
- ✅ Clear, declarative intent

**When to use other methods:**
- ✅ Dynamic partial requests (HTMX)
- ✅ File-based partials (no Page component)
- ✅ Conditional partial rendering

### Examples

**Example 1: API Endpoint**
```rhtml
<!-- pages/api.rs -->
@layout(false)

WebPage(props: &PageProps<()>) {
    <!DOCTYPE html>
    <html>
    <head>
        <meta charset="UTF-8">
        <title>API Response</title>
    </head>
    <body>
        <div class="api-response">
            <h2>Data</h2>
            <pre>{request_path}</pre>
        </div>
    </body>
    </html>
}
```
**Access:** `/api` → Custom HTML, no layout

**Example 2: Combined with Partials**
```rhtml
<!-- pages/dashboard.rs -->
@layout(false)

partial Metrics(...) { /* KPIs */ }
partial Charts(...) { /* Analytics */ }

WebPage(...) {
    <!DOCTYPE html>
    <html>
    <head><title>Dashboard</title></head>
    <body>
        <div id="metrics"
             hx-get="/dashboard?partial=Metrics"
             hx-trigger="load"></div>
    </body>
    </html>
}
```
**Access:**
- `/dashboard` → Full page, no layout
- `/dashboard?partial=Metrics` → Just metrics

## Named Partials (Multiple Partials in One File)

For better organization, RHTML supports **named partials** - multiple partials defined in a single file and accessed via `?partial=Name`.

### Why Named Partials?

**Problem with File-Based Partials:**
```
pages/
  users/
    partials/
      stats.rs
      active-users.rs
      recent-activity.rs
    index.rs
```
Too many files, domain logic scattered!

**Solution with Named Partials:**
```
pages/
  users.rs  ← All user-related partials in ONE file!
```

### Defining Named Partials

```rhtml
<!-- pages/users.rs -->

// Named partial: Stats
// Access: /users?partial=Stats
partial Stats(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">User Statistics</h2>
        <div class="grid grid-cols-3 gap-4">
            <div class="text-center p-4 bg-blue-50 rounded">
                <div class="text-3xl font-bold text-blue-600">1,234</div>
                <div class="text-sm text-gray-600">Total Users</div>
            </div>
        </div>
    </div>
}

// Named partial: ActiveUsers
// Access: /users?partial=ActiveUsers
partial ActiveUsers(props: &PartialProps<()>) {
    <div class="bg-white rounded-lg shadow-lg p-6">
        <h2 class="text-2xl font-bold mb-4">Active Users</h2>
        <div class="space-y-3">
            <div class="flex items-center gap-3 p-3 hover:bg-gray-50 rounded">
                <div class="font-semibold">John Doe</div>
                <span class="ml-auto text-xs bg-green-100 text-green-800 rounded px-2 py-1">
                    Online
                </span>
            </div>
        </div>
    </div>
}

// Optional: Full page component
WebPage(props: &PageProps<()>) {
    <div class="container mx-auto p-8">
        <h1 class="text-4xl font-bold mb-8">User Dashboard</h1>

        <!-- Load partials dynamically with HTMX -->
        <div id="stats-section">
            <button
                hx-get="/users?partial=Stats"
                hx-target="#stats-section"
                hx-swap="innerHTML"
                class="bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
                Load User Stats
            </button>
        </div>

        <div id="active-users-section">
            <button
                hx-get="/users?partial=ActiveUsers"
                hx-target="#active-users-section"
                hx-swap="innerHTML"
                class="bg-green-600 text-white px-4 py-2 rounded hover:bg-green-700">
                Load Active Users
            </button>
        </div>
    </div>
}
```

### Accessing Named Partials

**Direct URL Access:**
```bash
# Full page (if Page component exists)
curl http://localhost:3000/users

# Just the Stats partial
curl http://localhost:3000/users?partial=Stats

# Just the ActiveUsers partial
curl http://localhost:3000/users?partial=ActiveUsers
```

**With HTMX:**
```html
<button
    hx-get="/users?partial=Stats"
    hx-target="#container"
    hx-swap="innerHTML">
    Load Stats
</button>
```

### Error Handling

If you request a partial that doesn't exist, RHTML shows a helpful error:

```bash
curl http://localhost:3000/users?partial=Invalid
```

**Response:**
```
404 Partial Not Found

Partial 'Invalid' not found in /users
Available partials: Stats, ActiveUsers, RecentActivity
```

### When to Use Named Partials vs File-Based

**Use Named Partials When:**
- ✅ Multiple partials belong to the same domain (e.g., user-related fragments)
- ✅ Partials share context and data
- ✅ You want better organization and less file clutter
- ✅ Building a dashboard with multiple dynamic sections

**Use File-Based Partials When:**
- ✅ Partials are truly independent (e.g., generic components)
- ✅ Partials are reused across different pages
- ✅ Keeping partials in separate files improves clarity

**Best Practice:** Use named partials for domain-specific fragments, file-based partials for reusable components.

## Complete Example

### 1. Create a Partial File

```rhtml
<!-- pages/partials/todo-item.rs -->
<li class="flex items-center gap-2 p-2">
    <input type="checkbox" r-attr:checked="{query_completed}" />
    <span r-class:line-through="{query_completed}">
        {query_title}
    </span>
    <button
        hx-delete="/todos/{query_id}"
        hx-target="closest li"
        hx-swap="outerHTML"
        class="text-red-500 ml-auto">
        Delete
    </button>
</li>
```

### 2. Create the Full Page

```rhtml
<!-- pages/todos.rs -->
slots {
    title: "Todo List"
}

WebPage(props: &PageProps<()>) {
  <div class="container mx-auto p-8">
    <h1 class="text-3xl font-bold mb-6">My Todos</h1>

    <!-- HTMX Form to Add Todos -->
    <form
        hx-post="/todos"
        hx-target="#todo-list"
        hx-swap="beforeend"
        class="mb-6 flex gap-2">
        <input
            name="title"
            placeholder="New todo..."
            class="flex-1 px-3 py-2 border rounded" />
        <button type="submit" class="bg-blue-600 text-white px-4 py-2 rounded">
            Add Todo
        </button>
    </form>

    <!-- Todo List -->
    <ul id="todo-list" class="space-y-2">
        <!-- Todos will be loaded here via HTMX -->
    </ul>

    <!-- Load Initial Todos -->
    <div hx-get="/partials/todo-item?id=1&title=Buy groceries&completed=false"
         hx-trigger="load"
         hx-target="#todo-list"
         hx-swap="beforeend"></div>
  </div>
}
```

### 3. Handle Form Submission

```rhtml
<!-- pages/todos.rs - Add this at the end -->

<!-- Handle POST request - return partial only -->
<div r-if="is_post">
    <!-- This will be returned as a partial when form is submitted -->
    <li class="flex items-center gap-2 p-2">
        <input type="checkbox" />
        <span>{form_title}</span>
        <button
            hx-delete="/todos/new"
            hx-target="closest li"
            hx-swap="outerHTML"
            class="text-red-500 ml-auto">
            Delete
        </button>
    </li>
</div>
```

## Advanced Patterns

### Conditional Rendering Based on Request Type

```rhtml
WebPage(props: &PageProps<()>) {
  <!-- Full page for regular requests -->
  <div r-if="!is_htmx">
    <header>
        <h1>My App</h1>
        <nav>...</nav>
    </header>
  </div>

  <!-- Content (rendered for both full page and HTMX) -->
  <div id="main-content">
    <h2>Users</h2>
    <div r-for="user in users">
        {user.name}
    </div>
  </div>

  <!-- Footer only for full page -->
  <footer r-if="!is_htmx">
    © 2024 My App
  </footer>
}
```

### Progressive Enhancement

```rhtml
<!-- Works with and without JavaScript -->
<form action="/search" method="get"
      hx-get="/search?partial=true"
      hx-target="#results"
      hx-swap="innerHTML">
    <input name="q" placeholder="Search..." />
    <button type="submit">Search</button>
</form>

<div id="results">
    <!-- Results loaded here via HTMX -->
    <!-- Falls back to full page navigation without JS -->
</div>
```

### Infinite Scroll

```rhtml
<div id="user-list">
    <div r-for="user in users">
        <userCard r-props="{user: user}" />
    </div>

    <!-- Load More Trigger -->
    <div hx-get="/users?page=2&partial=true"
         hx-trigger="intersect once"
         hx-target="#user-list"
         hx-swap="beforeend">
        Loading more...
    </div>
</div>
```

## API Reference

### Request Context Methods

```rust
// In Rust (request handler)
request_context.wants_partial()  // -> bool
request_context.is_htmx()        // -> bool
request_context.htmx_target()    // -> Option<&str>
request_context.htmx_trigger()   // -> Option<&str>
```

### Renderer Methods

```rust
// In Rust (rendering)
renderer.is_partial(&content)       // -> bool (checks if file has Page component)
renderer.render_partial(&content)   // -> Result<String> (render without layout)
renderer.render_with_layout(&layout, &page)  // -> Result<String> (normal rendering)
```

## URL Patterns

All of these return partials when requested:

```bash
# File without Page component
GET /partials/user-item?name=John

# Query parameter override
GET /about?partial=true

# HTMX request (automatic)
curl -H "HX-Request: true" http://localhost:3000/users

# X-Partial header
curl -H "X-Partial: true" http://localhost:3000/users
```

## Best Practices

### 1. Organize Partials

```
pages/
├── _layout.rs
├── index.rs
├── users.rs
└── partials/
    ├── user-item.rs
    ├── user-form.rs
    └── user-stats.rs
```

### 2. Use Descriptive Names

```
✅ Good: partials/user-item.rs
✅ Good: partials/todo-form.rs
❌ Bad: partials/temp.rs
❌ Bad: partials/fragment1.rs
```

### 3. Keep Partials Small and Focused

```rhtml
<!-- ✅ Good: Single responsibility -->
<!-- partials/user-avatar.rs -->
<div class="avatar">
    <img src="{query_avatar}" alt="{query_name}" />
</div>

<!-- ❌ Bad: Too much logic -->
<!-- partials/dashboard.rs -->
<div>
    <!-- 500 lines of mixed content -->
</div>
```

### 4. Use Template Variables

```rhtml
<!-- Pass data via query params -->
<button hx-get="/partials/user?name=John&role=admin" ...>

<!-- Access in partial -->
<div class="user">
    <span>{query_name}</span>
    <span class="role">{query_role}</span>
</div>
```

### 5. Handle Errors Gracefully

```rhtml
<!-- pages/partials/user-item.rs -->
<div r-if="query_name">
    <h3>{query_name}</h3>
    <p>{query_email}</p>
</div>
<div r-else class="error">
    <p>Error: User data missing</p>
</div>
```

## Performance Tips

1. **Keep partials small** - Faster parsing and rendering
2. **Use query parameters** - Pass data without database calls
3. **Cache partials** - Use HTTP caching headers for static partials
4. **Minimize nesting** - Flat structure renders faster

## Migration Guide

### From Full Pages to Partials

**Before:**
```rhtml
<!-- pages/user-card.rs -->
slots { title: "User Card" }

WebPage(props: &PageProps<()>) {
  <div class="user-card">
    <h3>{query_name}</h3>
  </div>
}
```

**After:**
```rhtml
<!-- pages/partials/user-card.rs -->
<!-- Remove slots and Page component -->
<div class="user-card">
    <h3>{query_name}</h3>
</div>
```

## Troubleshooting

### Issue: Partial renders with layout

**Cause:** File has a `WebPage()` component
**Solution:** Remove the `WebPage()` component or use `?partial=true`

### Issue: HTMX request not detected

**Cause:** HTMX not setting headers
**Solution:** Ensure HTMX is loaded: `<script src="https://unpkg.com/htmx.org"></script>`

### Issue: Variables not accessible

**Cause:** Variables passed as route params instead of query params
**Solution:** Use query params: `/partial?name=John` not `/partial/John`

## Examples in Action

Visit these demo pages:
- `/htmx-demo` - Interactive HTMX demonstrations
- `/partials/user-item` - Sample partial file
- `/request-demo?partial=true` - Force partial mode

## Related Documentation

- [Request Context](IMPLEMENTATION_SUMMARY.md#3-request-context-access)
- [HTMX Integration](https://htmx.org/docs/)
- [Content Negotiation](IMPLEMENTATION_SUMMARY.md#4-content-negotiation-htmljson)

---

**Last Updated:** 2025-11-01
**Status:** ✅ Production Ready
**Feature:** Partial/Fragment Rendering Support
