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

Files **without** a `cmp Page()` component are automatically treated as partials:

```rhtml
<!-- pages/partials/user-item.rhtml -->
<!-- No cmp Page() = automatic partial -->
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
cmp Page(props: &PageProps<()>) {
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

## Complete Example

### 1. Create a Partial File

```rhtml
<!-- pages/partials/todo-item.rhtml -->
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
<!-- pages/todos.rhtml -->
slots {
    title: "Todo List"
}

cmp Page(props: &PageProps<()>) {
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
<!-- pages/todos.rhtml - Add this at the end -->

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
cmp Page(props: &PageProps<()>) {
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
├── _layout.rhtml
├── index.rhtml
├── users.rhtml
└── partials/
    ├── user-item.rhtml
    ├── user-form.rhtml
    └── user-stats.rhtml
```

### 2. Use Descriptive Names

```
✅ Good: partials/user-item.rhtml
✅ Good: partials/todo-form.rhtml
❌ Bad: partials/temp.rhtml
❌ Bad: partials/fragment1.rhtml
```

### 3. Keep Partials Small and Focused

```rhtml
<!-- ✅ Good: Single responsibility -->
<!-- partials/user-avatar.rhtml -->
<div class="avatar">
    <img src="{query_avatar}" alt="{query_name}" />
</div>

<!-- ❌ Bad: Too much logic -->
<!-- partials/dashboard.rhtml -->
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
<!-- pages/partials/user-item.rhtml -->
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
<!-- pages/user-card.rhtml -->
slots { title: "User Card" }

cmp Page(props: &PageProps<()>) {
  <div class="user-card">
    <h3>{query_name}</h3>
  </div>
}
```

**After:**
```rhtml
<!-- pages/partials/user-card.rhtml -->
<!-- Remove slots and Page component -->
<div class="user-card">
    <h3>{query_name}</h3>
</div>
```

## Troubleshooting

### Issue: Partial renders with layout

**Cause:** File has a `cmp Page()` component
**Solution:** Remove the `cmp Page()` component or use `?partial=true`

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
