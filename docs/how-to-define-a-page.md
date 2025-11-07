# How to Define a Page in RHTML

Pages in RHTML are the fundamental building blocks for creating routes in your application. This guide explains how to properly define pages using the `WebPage()` component.

## Page File Structure

In RHTML, pages are defined using `.rhtml` files in the `pages/` directory. The file structure determines your URL routing:

### Preferred Structure: `page.rhtml`

The **recommended way** to define routes is using `page.rhtml` files:

```
pages/
├── users/
│   └── page.rhtml       → /users
├── products/
│   └── page.rhtml       → /products
└── about/
    └── page.rhtml       → /about
```

### Alternative Structure: Named Files

You can also use named `.rhtml` files:

```
pages/
├── users.rhtml          → /users
├── products.rhtml       → /products
└── about.rhtml          → /about
```

Both approaches work, but **`page.rhtml` is preferred** for consistency and clarity.

## Defining Pages: Three Syntaxes

RHTML supports three different syntaxes for defining pages, all of which compile to the same internal format. Choose the one that fits your style!

### 1. #[webpage] Attribute (Recommended - Most Rust-like)

The **#[webpage]** attribute provides a Rust-native syntax that feels familiar to Rust developers:

```rhtml
#[webpage]
pub fn users(props: UsersProps) {
    <div class="container">
        <h1>Users Directory</h1>
        <p>Welcome to the users page!</p>
    </div>
}
```

**Benefits:**
- 🦀 Looks and feels like native Rust code
- 📝 Clear function signature shows props type
- 🔧 Better IDE support (syntax highlighting, completion)
- ✨ Familiar to React, SvelteKit, and Rust developers

**Syntax variations:**
```rhtml
#[webpage]
pub fn home(props: PageProps) { ... }    ✅ With pub

#[webpage]
fn about(props: PageProps) { ... }       ✅ Without pub

#[webpage]
pub fn users(props: UsersProps) { ... }  ✅ Custom props type
```

### 2. WebPage() Function Syntax

The traditional function-based syntax:

```rhtml
WebPage(props: &PageProps<()>) {
    <div>
        <h1>Your page content here</h1>
    </div>
}
```

**Case Insensitive:**
```rhtml
WebPage(props: &PageProps<()>) { ... }  ✅ Recommended
webpage(props: &PageProps<()>) { ... }  ✅ Works
WEBPAGE(props: &PageProps<()>) { ... }  ✅ Works
```

### 3. Inline WebPage Syntax

The simplest syntax when you don't need props:

```rhtml
WebPage {
    <div>
        <h1>Simple page</h1>
    </div>
}
```

## Complete Page Example

Here's a complete example using the recommended **#[webpage]** syntax:

**File: `pages/users/page.rhtml`**

```rhtml
slots {
    title: "Users Directory",
    description: "Browse all users"
}

#[webpage]
pub fn users(props: UsersProps) {
    <div class="container">
        <h1>Users Directory</h1>
        <p>Welcome to the users page!</p>

        <ul>
            <li>User 1</li>
            <li>User 2</li>
            <li>User 3</li>
        </ul>
    </div>
}
```

**Route:** `/users`

## Page Components Explained

### 1. Slots (Optional)

Slots allow you to pass data to your layout template:

```rhtml
slots {
    title: "Page Title",
    description: "Page description",
    footer: "Custom footer text"
}
```

These values can be accessed in your `_layout.rhtml` file.

### 2. WebPage Component (Required)

The `WebPage()` component defines your page content:

```rhtml
WebPage(props: &PageProps<()>) {
    <div>
        <!-- Your HTML content here -->
    </div>
}
```

**Props Parameter:**
- `props: &PageProps<()>` - Standard props for pages
- The `()` can be replaced with your custom data type if needed

## Dynamic Routes

RHTML supports dynamic route parameters:

### Single Parameter

**File: `pages/users/[id]/page.rhtml`**

```rhtml
WebPage(props: &PageProps<()>) {
    <div>
        <h1>User Profile #{id}</h1>
        <p>Viewing user with ID: {id}</p>
    </div>
}
```

**Route:** `/users/123` → `id = "123"`

### Optional Parameter

**File: `pages/posts/[id?]/page.rhtml`**

```rhtml
WebPage(props: &PageProps<()>) {
    <div r-if="id">
        <h1>Post #{id}</h1>
    </div>
    <div r-else>
        <h1>All Posts</h1>
    </div>
}
```

**Routes:**
- `/posts` → `id = None`
- `/posts/42` → `id = Some("42")`

### Catch-All Routes

**File: `pages/docs/[...slug]/page.rhtml`**

```rhtml
WebPage(props: &PageProps<()>) {
    <div>
        <h1>Documentation</h1>
        <p>Path: {slug}</p>
    </div>
}
```

**Routes:**
- `/docs/getting-started` → `slug = "getting-started"`
- `/docs/api/users` → `slug = "api/users"`

## Special Files

### Layout Files: `_layout.rhtml`

Layout files wrap your pages with common UI elements:

**File: `pages/_layout.rhtml`**

```rhtml
<!DOCTYPE html>
<html>
<head>
    <title>{title}</title>
</head>
<body>
    <header>
        <h1>My App</h1>
    </header>

    <main>
        {children}
    </main>

    <footer>
        {footer}
    </footer>
</body>
</html>
```

Layouts are hierarchical - each subdirectory can have its own `_layout.rhtml`.

### Error Pages: `_error.rhtml`

Error pages handle 404 and other errors:

**File: `pages/_error.rhtml`**

```rhtml
WebPage(props: &PageProps<()>) {
    <div class="error-page">
        <h1>404 - Page Not Found</h1>
        <p>The page you're looking for doesn't exist.</p>
        <a href="/">Go Home</a>
    </div>
}
```

## Directives in Pages

Pages support RHTML directives for dynamic content:

### Conditional Rendering

```rhtml
WebPage(props: &PageProps<()>) {
    <div r-if="is_admin">
        <h2>Admin Panel</h2>
    </div>
    <div r-else>
        <h2>User Dashboard</h2>
    </div>
}
```

### Loops

```rhtml
WebPage(props: &PageProps<()>) {
    <ul>
        <li r-for="user in users">
            {user.name}
        </li>
    </ul>
}
```

### Pattern Matching

```rhtml
WebPage(props: &PageProps<()>) {
    <div r-match="status">
        <div r-case="'active'">✅ Active</div>
        <div r-case="'pending'">⏳ Pending</div>
        <div r-case="_">❌ Unknown</div>
    </div>
}
```

## Page Partials

Files without a `WebPage` component are automatically treated as partials:

**File: `pages/partials/user-card.rhtml`**

```rhtml
<!-- No WebPage component = automatic partial -->
<div class="user-card">
    <h3>{name}</h3>
    <p>{email}</p>
</div>
```

Use partials via HTMX or by including them in other pages.

## Named Partials

You can define named partials within a page:

```rhtml
WebPage(props: &PageProps<()>) {
    <div>
        <h1>Main Content</h1>
    </div>
}

partial Header() {
    <header>
        <h1>Site Header</h1>
    </header>
}

partial Footer() {
    <footer>
        <p>© 2025 My App</p>
    </footer>
}
```

Access named partials via: `?partial=Header` or `?partial=Footer`

## Best Practices

1. **Use `page.rhtml` for consistency**: Prefer `pages/users/page.rhtml` over `pages/users.rhtml`
2. **Use `#[webpage]` syntax**: Recommended for its Rust-native feel and better IDE support
3. **Use slots for metadata**: Pass title, description, and other data to layouts
4. **Keep pages simple**: Move complex logic to components or partials
5. **Use meaningful route names**: Choose route paths that reflect your content structure

## Summary

- **File naming**: Use `page.rhtml` for route files (preferred) or `<name>.rhtml`
- **Three syntaxes available**:
  - `#[webpage] pub fn name(props: Type) { ... }` (Recommended - Rust-native)
  - `WebPage(props: &PageProps<()>) { ... }` (Traditional)
  - `WebPage { ... }` (Simple inline)
- **Case insensitive**: `WebPage`, `webpage`, `WEBPAGE` all normalize to `WebPage`
- **Slots**: Optional metadata for layouts
- **Dynamic routes**: Support `[param]`, `[param?]`, and `[...slug]` patterns
- **Special files**: `_layout.rhtml` for layouts, `_error.rhtml` for error pages
- **Partials**: Files without a page component are automatic partials

For more information, see:
- [Routing Documentation](../README.md#routing)
- [Components Guide](../FEATURES_OVERVIEW.md#components)
- [Directives Reference](../FEATURES_OVERVIEW.md#directives)
