# RHTML - Rust HTML Framework

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
cmp layout(slots: &Slots) {
  <!DOCTYPE html>
  <html>
  <head>
    <title>{slots.get("title").unwrap_or("RHTML App")}</title>
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
      {slots.get("footer").unwrap_or("© 2024")}
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

Pages define routes and their content.

```rhtml
<!-- pages/users/index.rhtml -->

slots {
  title: "Users Directory",
  footer: "User Management"
}

cmp Page(props: &PageProps<()>) {
  <div class="container mx-auto p-4">
    <h1 class="text-3xl font-bold mb-6">Users</h1>

    <!-- Demo: Eventually this will use data from data fn -->
    <div class="grid gap-4 md:grid-cols-2 lg:grid-cols-3">
      <div r-for="(index, item) in users">
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

cmp Page(props: &PageProps<()>) {
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

### Current Status (v0.0.1)
- ✅ File-based routing with dynamic params
- ✅ Layout inheritance
- ✅ Component system
- ✅ Core directives (if/for/match)
- ✅ Hot reload
- ✅ CSS scoping

### Next Up (v0.1.0) - MVP
- [ ] Data fetching in pages (`data fn` functions)
- [ ] Query parameter support
- [ ] Form handling (POST/PUT/DELETE)
- [ ] Request context (cookies, headers)
- [ ] Content negotiation (HTML/JSON)
- [ ] Additional directives (r-attr, r-class, r-props)

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

cmp Page(props: &PageProps<()>) {
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

- [TODO.md](TODO.md) - Feature tracking
- [FEATURE_AUDIT.md](FEATURE_AUDIT.md) - Comprehensive feature audit
- [ROADMAP.md](ROADMAP.md) - Implementation roadmap
- [Vision.pdf](Vision.pdf) - Vision document

---

## ❓ FAQ

### Why not use leptos/dioxus/yew?

RHTML is SSR-only with HTMX for interactivity, not a WASM framework. Different use case.

### Can I use this in production?

Not yet. Core data layer features are still in development. See [TODO.md](TODO.md).

### How do I fetch data?

Coming soon! `data fn` functions are the #1 priority. See Sprint 1 in [TODO.md](TODO.md).

### Does it support TypeScript/JSX?

No. RHTML uses `.rhtml` files with embedded Rust code, not JavaScript.

### Can I use it with databases?

Yes! Once `data fn` is implemented, you can use any Rust database crate (sqlx, diesel, etc).

---

**Built with ❤️ in Rust**
