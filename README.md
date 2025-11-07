# RHTML - Rust HTML Framework
# Keep it original
> **A Rust-first SSR framework that brings functional programming patterns to web development.**

Write real HTML with minimal directives, keep business logic in Rust, and compile everything to a single binary.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)

---

## 🌟 Core Philosophy

- **HTML files, not Rust macros** - Write actual HTML with minimal directives
- **Functional patterns** - Embrace immutability and pattern matching
- **SSR-only** - Use HTMX/Alpine.js for client interactivity
- **Single binary** - Deploy one executable, no Node.js required
- **Tailwind CSS** - First-class support for utility-first CSS
- **File-based routing** - Routes generated from file structure

---

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/yourusername/rhtml.git
cd rhtml

# Run development server
cargo run

# Visit http://localhost:3000
```

**Note:** CLI tool (`rhtml new`, `rhtml dev`) is coming soon. For now, use `cargo run` directly.

---

## 📂 Project Structure

```
my-app/
├── pages/
│   ├── _layout.rhtml         # Root layout
│   ├── index.rhtml           # Home page (/)
│   ├── about.rhtml           # About page (/about)
│   └── users/
│       ├── _layout.rhtml     # Users section layout
│       ├── index.rhtml       # Users list (/users)
│       ├── [id].rhtml        # User detail (/users/:id)
│       └── new.rhtml         # New user form (/users/new)
├── components/
│   ├── Header.rhtml
│   ├── Footer.rhtml
│   └── Button.rhtml
├── static/
│   └── favicon.ico
├── src/
│   └── main.rs              # Server entry point
├── rhtml.toml               # Configuration
└── Cargo.toml
```

---

## 📝 File Types

### Layout Files (`_layout.rhtml`)

Layouts wrap page content with common elements like headers and footers.

```rhtml
pub struct LayoutSlots {
    pub content: String,
    pub title: Option<String>,
    pub footer: Option<String>,
}

#[layout]
pub fn layout(slots: LayoutSlots) {
  <!DOCTYPE html>
  <html>
  <head>
    <title>{slots.title.unwrap_or("RHTML App".to_string())}</title>
    <script src="https://unpkg.com/htmx.org@1.9.0"></script>
    <script src="https://cdn.tailwindcss.com"></script>
  </head>
  <body>
    <nav>
      <a href="/">Home</a>
      <a href="/users">Users</a>
    </nav>

    <main>
      {slots.content}  <!-- Page content inserted here -->
    </main>

    <footer>
      {slots.footer.unwrap_or("© 2024".to_string())}
    </footer>
  </body>
  </html>
}

css layout {
  nav {
    background: #333;
    padding: 1rem;
  }
  nav a {
    color: white;
    margin-right: 1rem;
  }
}
```

---

### Page Files (`*.rhtml`)

Pages define routes and their content. RHTML supports multiple syntaxes - use **#[webpage]** for a Rust-native feel:

```rhtml
<!-- pages/users/index.rhtml -->

slot! {
  title: "Users Directory",
  footer: "User Management"
}

#[webpage]
pub fn users(props: UsersProps) {
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">Users</h1>

    <!-- Demo: Eventually this will use data from data fn -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div r-for="(index, item) in props.users">
        <div class="card p-4 border rounded">
          <h3>User #{index}</h3>
          <p>{item}</p>
        </div>
      </div>
    </div>
  </div>
}

css Page {
  .container {
    max-width: 1200px;
  }
}
```

**The `#[webpage]` attribute** is the ONLY way to define pages in RHTML - Rust-native syntax with better IDE support!

---

### Component Files (`components/*.rhtml`)

Reusable UI components.

```rhtml
<!-- components/Button.rhtml -->

cmp Button {
  <button class="btn">
    {text}
  </button>
}

css Button {
  .btn {
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
    font-weight: 500;
    cursor: pointer;
  }

  .btn:hover {
    transform: translateY(-2px);
    box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  }
}
```

**Usage in pages:**

```rhtml
<Button r-component="Button" text="Click Me" />
```

---

## 🎯 Named Partials & Layouts

### Named Partials

Define multiple partials in a single file for better domain organization:

```rhtml
<!-- pages/users.rhtml -->

partial Stats(props: &PartialProps<()>) {
  <div>User Statistics</div>
}

partial ActiveUsers(props: &PartialProps<()>) {
  <div>Active Users List</div>
}

WebPage(props: &PageProps<()>) {
  <button hx-get="/users?partial=Stats">Load Stats</button>
}
```

**Access:**
- `/users` → Full page
- `/users?partial=Stats` → Just Stats partial
- `/users?partial=ActiveUsers` → Just ActiveUsers partial

**Benefits:**
- Keep related partials together
- Reduce file clutter
- Perfect for HTMX integration

### @layout Decorator

Control layout rendering declaratively:

```rhtml
@layout(false)  <!-- Disable layout for this page -->

WebPage(props: &PageProps<()>) {
  <!DOCTYPE html>
  <html>
  <head><title>Custom Page</title></head>
  <body>
    <!-- Full control over HTML structure -->
  </body>
  </html>
}
```

**Use Cases:**
- API endpoints returning HTML fragments
- Email templates
- Custom document structures
- Pages with different meta tags

---

## 🎨 Directives

RHTML provides 8 core directives for dynamic content:

### Conditional Rendering

```rhtml
<div r-if="user.is_premium">
  <span class="badge-premium">Premium Member</span>
</div>
<div r-else-if="user.is_active">
  <span class="badge-active">Active</span>
</div>
<div r-else>
  <span class="badge-inactive">Inactive</span>
</div>
```

### Loops

```rhtml
<!-- Simple iteration -->
<ul>
  <li r-for="item in items">
    {item.name}
  </li>
</ul>

<!-- With index -->
<div r-for="(index, item) in items">
  <span>{index + 1}. {item.name}</span>
</div>
```

### Pattern Matching

```rhtml
<div r-match="user.role">
  <div r-when="admin">
    <p>Admin Dashboard</p>
  </div>
  <div r-when="user">
    <p>User Profile</p>
  </div>
  <div r-default>
    <p>Guest View</p>
  </div>
</div>
```

### Components

```rhtml
<Button r-component="Button" text="Submit" variant="primary" />
```

### Interpolation

```rhtml
<h1>Welcome, {user.name}!</h1>
<p>You have {messages.len()} new messages</p>
<p>Total: ${(price * quantity).round()}</p>
```

---

## 🔥 Hot Reload

RHTML includes built-in hot reload for rapid development:

- **Template changes** → Browser refreshes automatically
- **Component changes** → Instant reload
- **Source code changes** → Restart server manually

```bash
# Enable/disable hot reload
HOT_RELOAD=true cargo run   # Enabled (default)
HOT_RELOAD=false cargo run  # Disabled
```

---

## 🗂️ File-based Routing

Routes are automatically generated from your file structure:

| File Path | Route | Params |
|-----------|-------|--------|
| `pages/index.rhtml` | `/` | - |
| `pages/about.rhtml` | `/about` | - |
| `pages/users/index.rhtml` | `/users` | - |
| `pages/users/new.rhtml` | `/users/new` | - |
| `pages/users/[id].rhtml` | `/users/:id` | `id` |
| `pages/blog/[slug].rhtml` | `/blog/:slug` | `slug` |

### Dynamic Routes

Use bracket notation for dynamic segments:

```rhtml
<!-- pages/users/[id].rhtml -->

WebPage(props: &PageProps<()>) {
  <div>
    <h1>User Profile #{id}</h1>
    <p>Viewing user: {id}</p>
  </div>
}
```

Visit `/users/42` → `id = "42"`

### Route Priority

Static routes always match before dynamic routes:

- `/users/new` (static) has higher priority than `/users/:id` (dynamic)
- This prevents conflicts

---

## 🎨 CSS Scoping

All CSS is automatically scoped to prevent conflicts:

```rhtml
cmp Button {
  <button class="btn">Click</button>
}

css Button {
  .btn { color: blue; }
}
```

**Generated HTML:**

```html
<button class="btn" data-rhtml="Button">Click</button>

<style>
[data-rhtml="Button"] .btn { color: blue; }
</style>
```

Components can't accidentally override each other's styles!

---

## 🔧 Configuration

Coming soon: `rhtml.toml` configuration file.

```toml
[project]
name = "my-app"
version = "0.1.0"

[server]
port = 3000
host = "0.0.0.0"

[dev]
hot_reload = true
port = 3000

[build]
output_dir = "dist"
minify_html = true
```

---

## 🚧 Roadmap

See [TODO.md](TODO.md) for detailed feature tracking.

### Current Status (v0.1.0-alpha) - 53% Complete! 🎉
- ✅ File-based routing with dynamic params
- ✅ Layout inheritance
- ✅ Component system
- ✅ Core directives (if/for/match)
- ✅ Hot reload
- ✅ CSS scoping
- ✅ **Query parameter support** (`{query_name}`) 🆕
- ✅ **Form handling** (POST/PUT/DELETE) 🆕
- ✅ **Request context** (cookies, headers, method) 🆕
- ✅ **Content negotiation** (HTML/JSON) 🆕
- ✅ **Named partials** (`partial Name() {}`) 🆕
- ✅ **@layout decorator** (`@layout(false)`) 🆕
- ✅ **HTMX integration** (automatic detection) 🆕
- ✅ **Configuration system** (rhtml.toml) 🆕
- ✅ **Case-insensitive routing** (configurable) 🆕

### Next Up (v0.2.0) - Feature Complete
- [ ] Data fetching in pages (`data fn` functions)
- [ ] Typed PageProps<T> with actual data
- [ ] Additional directives (r-attr, r-class, r-props)
- [ ] Catch-all routes ([...slug])

### Future (v0.2.0+)
- [ ] Catch-all routes
- [ ] Theme system (Hugo-style)
- [ ] Middleware system
- [ ] CLI tool (`rhtml new`, `rhtml dev`)
- [ ] SSG/ISR support

---

## 📚 Examples

### HTMX Integration

```rhtml
<!-- pages/todos.rhtml -->

WebPage(props: &PageProps<()>) {
  <div class="max-w-2xl mx-auto p-4">
    <h1>Todo List</h1>

    <!-- HTMX form -->
    <form hx-post="/todos"
          hx-target="#todo-list"
          hx-swap="beforeend"
          class="mb-4 flex gap-2">
      <input name="title"
             placeholder="New todo..."
             class="flex-1 px-3 py-2 border rounded" />
      <button type="submit" class="btn btn-primary">
        Add Todo
      </button>
    </form>

    <!-- Todo list -->
    <ul id="todo-list">
      <li r-for="todo in todos" class="flex items-center gap-2 p-2">
        <input type="checkbox"
               hx-patch="/todos/{todo.id}/toggle"
               hx-target="closest li"
               hx-swap="outerHTML" />
        <span>{todo.title}</span>
        <button hx-delete="/todos/{todo.id}"
                hx-target="closest li"
                hx-swap="outerHTML"
                class="ml-auto text-red-500">
          Delete
        </button>
      </li>
    </ul>
  </div>
}
```

---

## 🛠️ Development

### Prerequisites

- Rust 1.70 or higher
- Cargo

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Run with hot reload
cargo run
```

### Project Structure

```
src/
├── main.rs              # Server entry point
├── lib.rs               # Public API
├── router.rs            # File-based routing engine
├── renderer.rs          # Template rendering
├── template_loader.rs   # Template discovery & loading
├── hot_reload.rs        # Hot reload system
└── parser/
    ├── directive.rs     # Directive parsing
    ├── expression.rs    # Expression evaluation
    └── css.rs           # CSS scoping
```

---

## 🚀 Deployment

### Single Binary

```bash
# Build release binary
cargo build --release

# Run anywhere (no dependencies)
./target/release/rhtml_app
```

### Docker

```dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/rhtml_app /usr/local/bin/
EXPOSE 3000
CMD ["rhtml_app"]
```

### Environment Variables

```bash
PORT=8080               # Server port (default: 3000)
HOT_RELOAD=false        # Disable hot reload in production
RUST_LOG=info           # Logging level
```

---

## 🤝 Contributing

Contributions are welcome! Please see [TODO.md](TODO.md) for current priorities.

### High-Impact Areas

1. **Data Layer** - Implement `data fn` functions
2. **Query Parameters** - Add query param extraction
3. **Directives** - Implement `r-attr`, `r-class`, `r-props`
4. **Theme System** - Hugo-style theme support
5. **Documentation** - More examples and guides

---

## 📄 License

MIT License - see [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

Inspired by:
- **Hugo** - Theme system and file-based routing
- **Next.js** - Developer experience
- **Leptos** - Rust SSR patterns
- **HTMX** - HTML-first interactivity

---

## 📖 Documentation

- [DOCUMENTATION_STATUS.md](DOCUMENTATION_STATUS.md) - **Complete status of all features** ⭐
- [TODO.md](TODO.md) - Feature tracking (53% complete)
- [FEATURE_AUDIT.md](FEATURE_AUDIT.md) - Comprehensive feature audit
- [FEATURES_OVERVIEW.md](FEATURES_OVERVIEW.md) - Complete feature overview
- [PARTIAL_RENDERING.md](PARTIAL_RENDERING.md) - Partial rendering guide (600+ lines)
- [NAMED_PARTIALS_SUMMARY.md](NAMED_PARTIALS_SUMMARY.md) - Named partials implementation
- [IMPLEMENTATION_SUMMARY.md](IMPLEMENTATION_SUMMARY.md) - Technical implementation details
- [Vision.pdf](Vision.pdf) - Vision document

---

## ❓ FAQ

### Why not use leptos/dioxus/yew?

RHTML is SSR-only with HTMX for interactivity, not a WASM framework. Different use case.

### Can I use this in production?

**YES, for 75% of use cases!** The framework is now production-ready with:
- ✅ Full request handling (GET/POST/PUT/DELETE)
- ✅ Query parameters and form data
- ✅ HTMX integration
- ✅ Named partials and @layout decorator

**Limitations:**
- `data fn` parsing not yet implemented (use external Rust functions as workaround)
- All pages use `PageProps<()>` (typed props coming in v0.2.0)

See [DOCUMENTATION_STATUS.md](DOCUMENTATION_STATUS.md) for complete status.

### How do I fetch data?

**Current approach (v0.1.0-alpha):**
- Use external Rust functions in `src/main.rs` to fetch data
- Pass data to renderer via context variables
- Access in templates via `{variable_name}`

**Coming in v0.2.0:**
- `data fn` functions in .rhtml files
- Typed `PageProps<T>` with actual data

### Does it support TypeScript/JSX?

No. RHTML uses `.rhtml` files with embedded Rust code, not JavaScript.

### Can I use it with databases?

Yes! Once `data fn` is implemented, you can use any Rust database crate (sqlx, diesel, etc).

---

**Built with ❤️ in Rust**
