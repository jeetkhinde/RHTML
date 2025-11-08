# RHTML Getting Started Guide

Welcome to RHTML! This guide covers everything you need to build modern server-side rendered web applications with Rust.

## Table of Contents

1. [Installation & Setup](#installation--setup)
2. [Project Structure](#project-structure)
3. [Creating Your First Page](#creating-your-first-page)
4. [Understanding Layouts](#understanding-layouts)
5. [Building Components](#building-components)
6. [Handling Forms with Actions](#handling-forms-with-actions)
7. [Form Validation](#form-validation)
8. [Database Integration](#database-integration)
9. [Template Directives](#template-directives)
10. [Development Workflow](#development-workflow)
11. [Best Practices](#best-practices)

---

## Installation & Setup

### Prerequisites

- Rust 1.70+ installed
- Basic understanding of HTML and web concepts

### Creating a New Project

```bash
# Clone the RHTML template (or use this repo)
cargo new my-app
cd my-app

# Add RHTML as a dependency (if using as library)
# Or use the workspace structure from this repository
```

### Project Structure

```
my-app/
├── src/
│   ├── main.rs              # Server setup
│   ├── lib.rs               # Re-exports
│   ├── renderer.rs          # Template rendering
│   ├── template_loader.rs   # Template discovery
│   ├── request_context.rs   # Request data
│   ├── actions.rs           # Action responses
│   ├── action_handlers.rs   # Handler registry
│   ├── action_executor.rs   # Handler execution
│   ├── database.rs          # Database layer
│   ├── validation/          # Validation module
│   └── validation_pipeline.rs
├── pages/                   # Page templates
│   ├── _layout.rhtml        # Root layout
│   ├── index.rhtml          # Home page
│   ├── about.rhtml          # About page
│   └── users/               # Nested section
│       ├── _layout.rhtml    # Users layout
│       ├── index.rhtml      # Users list
│       └── [id].rhtml       # User detail
├── components/              # Reusable components
│   ├── Header.rhtml
│   ├── Footer.rhtml
│   ├── Button.rhtml
│   └── Card.rhtml
├── Cargo.toml               # Dependencies
└── rhtml.toml               # RHTML config
```

### Running Your App

```bash
# Development with hot reload
cargo run

# Production build
cargo build --release
./target/release/rhtml_app

# Visit http://localhost:3000
```

---

## Project Structure

### Pages Directory

The `pages/` directory structure maps directly to URL routes:

```
pages/
├── index.rhtml           → /
├── about.rhtml           → /about
├── contact.rhtml         → /contact
├── blog/
│   ├── index.rhtml       → /blog
│   ├── [id].rhtml        → /blog/123 (dynamic)
│   └── _layout.rhtml     → Blog-specific layout
└── admin/
    ├── _layout.rhtml     → Admin layout
    ├── dashboard.rhtml   → /admin/dashboard
    └── users/
        ├── index.rhtml   → /admin/users
        └── [id].rhtml    → /admin/users/123
```

### Special Filenames

- **`_layout.rhtml`** - Layout wrapper for the directory and subdirectories
- **`_error.rhtml`** - Error page handler (future feature)
- **`[param].rhtml`** - Dynamic route with parameter named `param`

---

## Creating Your First Page

### Simple Static Page

**File: `pages/home.rhtml`**

```html
<div class="hero">
  <h1>Welcome to My App!</h1>
  <p>This is a simple RHTML page.</p>
  <a href="/about" class="btn">Learn More</a>
</div>

<style scoped>
  .hero {
    padding: 2rem;
    text-align: center;
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    color: white;
  }

  h1 {
    font-size: 3rem;
    margin-bottom: 1rem;
  }
</style>
```

**Access at:** `http://localhost:3000/home`

### Page with Dynamic Data

**File: `pages/users/index.rhtml`**

```html
<div class="users-container">
  <h1>Users</h1>

  @if users.is_empty()
    <p class="empty">No users found.</p>
  @else
    <ul class="user-list">
      @for user in users
        <li class="user-item">
          <a href="/users/@{user.id}">
            @{user.name}
            <span class="email">@{user.email}</span>
          </a>
        </li>
      @end
    </ul>
  @end
</div>

<style scoped>
  .users-container { padding: 2rem; }
  .user-list { list-style: none; padding: 0; }
  .user-item {
    padding: 1rem;
    border-bottom: 1px solid #eee;
  }
  .email { color: #666; font-size: 0.9rem; }
</style>
```

**Access at:** `http://localhost:3000/users`

### Page with Route Parameter

**File: `pages/users/[id].rhtml`**

```html
<div class="user-detail">
  <h1>@{user.name}</h1>

  <div class="info">
    <p>
      <strong>Email:</strong>
      <a href="mailto:@{user.email}">@{user.email}</a>
    </p>

    @if user.bio.is_some()
      <p>
        <strong>Bio:</strong>
        @{user.bio}
      </p>
    @end

    <p>
      <strong>Age:</strong>
      @{user.age}
    </p>
  </div>

  <a href="/users" class="back-link">← Back to Users</a>
</div>
```

**Access at:** `http://localhost:3000/users/1`

---

## Understanding Layouts

### Creating a Root Layout

**File: `pages/_layout.rhtml`**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width, initial-scale=1.0">
  <title>@{page_title.unwrap_or("My App")}</title>
  <link rel="stylesheet" href="/styles.css">
</head>
<body>
  <header>
    <nav class="navbar">
      <a href="/" class="logo">My App</a>
      <ul>
        <li><a href="/home">Home</a></li>
        <li><a href="/users">Users</a></li>
        <li><a href="/about">About</a></li>
      </ul>
    </nav>
  </header>

  <main class="container">
    @{content}
  </main>

  <footer>
    <p>&copy; 2024 My App. All rights reserved.</p>
  </footer>

  <script>
    // Your scripts here
  </script>
</body>
</html>

<style scoped>
  * { margin: 0; padding: 0; box-sizing: border-box; }

  body { font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif; }

  header { background: #333; color: white; padding: 1rem; }
  .navbar { display: flex; align-items: center; justify-content: space-between; }
  .logo { font-weight: bold; font-size: 1.5rem; color: white; text-decoration: none; }

  nav ul { display: flex; list-style: none; gap: 2rem; }
  nav a { color: white; text-decoration: none; }
  nav a:hover { text-decoration: underline; }

  .container { max-width: 1200px; margin: 0 auto; padding: 2rem; }

  footer { background: #f5f5f5; padding: 2rem; text-align: center; margin-top: 4rem; }
</style>
```

### Section-Specific Layout

**File: `pages/admin/_layout.rhtml`**

```html
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <title>Admin - @{page_title.unwrap_or("Dashboard")}</title>
</head>
<body>
  <div class="admin-layout">
    <aside class="sidebar">
      <h2>Admin</h2>
      <nav>
        <a href="/admin/dashboard">Dashboard</a>
        <a href="/admin/users">Users</a>
        <a href="/admin/settings">Settings</a>
      </nav>
    </aside>

    <main class="admin-content">
      <div class="admin-header">
        <h1>@{page_title.unwrap_or("Admin Panel")}</h1>
      </div>
      @{content}
    </main>
  </div>

  <style scoped>
    .admin-layout { display: flex; min-height: 100vh; }
    .sidebar { width: 250px; background: #2c3e50; color: white; padding: 2rem; }
    .sidebar nav { display: flex; flex-direction: column; gap: 1rem; }
    .sidebar a { color: white; text-decoration: none; padding: 0.5rem 1rem; border-radius: 4px; }
    .sidebar a:hover { background: #34495e; }
    .admin-content { flex: 1; padding: 2rem; }
    .admin-header { margin-bottom: 2rem; border-bottom: 2px solid #eee; padding-bottom: 1rem; }
  </style>
</body>
</html>
```

### Disabling Layout for Specific Pages

**File: `pages/api-response.rhtml`**

```html
@layout(false)

<div class="json-response">
  {
    "status": "success",
    "data": @{json_data}
  }
</div>
```

---

## Building Components

### Creating a Button Component

**File: `components/Button.rhtml`**

```html
@component
@slot(content) {
  <button class="btn btn-@{variant.unwrap_or("primary")}">
    @{content}
  </button>
}

<style scoped>
  .btn {
    padding: 0.75rem 1.5rem;
    border: none;
    border-radius: 4px;
    cursor: pointer;
    font-size: 1rem;
    transition: all 0.3s ease;
  }

  .btn-primary {
    background: #667eea;
    color: white;
  }

  .btn-primary:hover {
    background: #5568d3;
  }

  .btn-secondary {
    background: #e0e0e0;
    color: #333;
  }

  .btn-secondary:hover {
    background: #d0d0d0;
  }
</style>
```

### Creating a Card Component

**File: `components/Card.rhtml`**

```html
@component
@prop(title: String)
@prop(description: Option<String>)

<div class="card">
  <div class="card-header">
    <h3>@{title}</h3>
  </div>
  <div class="card-body">
    @if description.is_some()
      <p>@{description}</p>
    @end
    @{slot}
  </div>
</div>

<style scoped>
  .card {
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    overflow: hidden;
    box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  }

  .card-header {
    background: #f5f5f5;
    padding: 1rem;
    border-bottom: 1px solid #ddd;
  }

  .card-header h3 {
    margin: 0;
    font-size: 1.25rem;
  }

  .card-body {
    padding: 1.5rem;
  }
</style>
```

### Using Components in Pages

**File: `pages/products.rhtml`**

```html
<div class="products">
  <h1>Products</h1>

  <div class="grid">
    @for product in products
      @partial("ProductCard")
        <div class="card">
          <h3>@{product.name}</h3>
          <p class="price">$@{product.price}</p>
          <p>@{product.description}</p>
          <button class="btn">Add to Cart</button>
        </div>
      @end
    @end
  </div>
</div>

<style scoped>
  .products { padding: 2rem; }
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
    gap: 2rem;
    margin-top: 2rem;
  }

  .card {
    background: white;
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1.5rem;
    transition: transform 0.3s ease;
  }

  .card:hover {
    transform: translateY(-4px);
    box-shadow: 0 8px 16px rgba(0,0,0,0.1);
  }

  .price {
    font-size: 1.5rem;
    color: #667eea;
    font-weight: bold;
    margin: 0.5rem 0;
  }
</style>
```

---

## Handling Forms with Actions

### Creating a Form Page

**File: `pages/contact.rhtml`**

```html
<div class="contact-container">
  <h1>Contact Us</h1>

  <form method="post" action="/contact" class="contact-form">
    <div class="form-group">
      <label for="name">Name *</label>
      <input
        type="text"
        id="name"
        name="name"
        required
        placeholder="Your name"
        value="@{form.get_value("name").unwrap_or("")}"
      />
      @if form.has_error("name")
        <span class="error">@{form.get_error("name")}</span>
      @end
    </div>

    <div class="form-group">
      <label for="email">Email *</label>
      <input
        type="email"
        id="email"
        name="email"
        required
        placeholder="your@email.com"
        value="@{form.get_value("email").unwrap_or("")}"
      />
      @if form.has_error("email")
        <span class="error">@{form.get_error("email")}</span>
      @end
    </div>

    <div class="form-group">
      <label for="message">Message *</label>
      <textarea
        id="message"
        name="message"
        required
        rows="5"
        placeholder="Your message..."
      >@{form.get_value("message").unwrap_or("")}</textarea>
      @if form.has_error("message")
        <span class="error">@{form.get_error("message")}</span>
      @end
    </div>

    <button type="submit" class="btn btn-primary">Send Message</button>
  </form>
</div>

<style scoped>
  .contact-container { max-width: 600px; margin: 2rem auto; padding: 2rem; }

  .contact-form { display: flex; flex-direction: column; gap: 1.5rem; }

  .form-group {
    display: flex;
    flex-direction: column;
  }

  label {
    font-weight: 600;
    margin-bottom: 0.5rem;
    color: #333;
  }

  input, textarea {
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
    font-family: inherit;
  }

  input:focus, textarea:focus {
    outline: none;
    border-color: #667eea;
    box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
  }

  .error {
    color: #e74c3c;
    font-size: 0.875rem;
    margin-top: 0.25rem;
  }

  .btn {
    padding: 0.75rem 1.5rem;
    background: #667eea;
    color: white;
    border: none;
    border-radius: 4px;
    font-size: 1rem;
    cursor: pointer;
  }

  .btn:hover {
    background: #5568d3;
  }
</style>
```

### Creating an Action Handler

**File: `src/example_actions.rs`** (add to this file)

```rust
use crate::{
    action_executor::ActionResult,
    request_context::RequestContext,
    validation_pipeline::{validate_request, ValidationPipelineResult},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl crate::validation::Validate for ContactRequest {
    fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors = HashMap::new();

        if self.name.trim().is_empty() {
            errors.insert("name".to_string(), "Name is required".to_string());
        }

        if !self.email.contains('@') {
            errors.insert("email".to_string(), "Invalid email format".to_string());
        }

        if self.message.trim().len() < 10 {
            errors.insert(
                "message".to_string(),
                "Message must be at least 10 characters".to_string(),
            );
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub async fn post_contact(ctx: RequestContext) -> ActionResult {
    // Validate the form data
    let validation_result = validate_request::<ContactRequest>(&ctx.form);

    match validation_result {
        ValidationPipelineResult::Valid(contact) => {
            // TODO: Send email or save to database
            println!("Contact from {}: {}", contact.name, contact.message);

            // Return success response
            ActionResult::Html {
                content: format!(
                    r#"<div class="success"><p>Thank you, {}! We'll get back to you soon.</p></div>"#,
                    contact.name
                ),
                headers: Default::default(),
            }
        }
        ValidationPipelineResult::Invalid(form_context) => {
            // Return form with errors for re-display
            ActionResult::Html {
                content: format!(
                    r#"<div class="form-errors">
                        <p>Please fix the following errors:</p>
                        <ul>
                        {}</ul>
                        </div>"#,
                    form_context
                        .errors
                        .iter()
                        .map(|(k, v)| format!("<li><strong>{}:</strong> {}</li>", k, v))
                        .collect::<Vec<_>>()
                        .join("")
                ),
                headers: Default::default(),
            }
        }
    }
}
```

### Registering the Action Handler

**File: `src/action_handlers.rs`** (in `register_built_in_handlers`)

```rust
pub fn register_built_in_handlers(registry: &mut ActionHandlerRegistry) {
    use crate::example_actions;

    // ... existing registrations ...

    registry.register(
        "/contact",
        "POST",
        |ctx| Box::pin(example_actions::post_contact(ctx)),
    );
}
```

---

## Form Validation

### Built-in Validators

RHTML provides validators for common patterns:

```rust
use crate::validation::Validate;
use std::collections::HashMap;

#[derive(serde::Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_confirm: String,
    pub age: i32,
}

impl Validate for SignupForm {
    fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors = HashMap::new();

        // Username validation
        if self.username.len() < 3 {
            errors.insert(
                "username".to_string(),
                "Username must be at least 3 characters".to_string(),
            );
        }

        if self.username.len() > 20 {
            errors.insert(
                "username".to_string(),
                "Username must be at most 20 characters".to_string(),
            );
        }

        // Email validation
        if !self.email.contains('@') {
            errors.insert("email".to_string(), "Invalid email format".to_string());
        }

        // Password validation
        if self.password.len() < 8 {
            errors.insert(
                "password".to_string(),
                "Password must be at least 8 characters".to_string(),
            );
        }

        if self.password != self.password_confirm {
            errors.insert(
                "password_confirm".to_string(),
                "Passwords do not match".to_string(),
            );
        }

        // Age validation
        if self.age < 18 {
            errors.insert("age".to_string(), "You must be 18 or older".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
```

### Custom Validators

**File: `src/validation/custom.rs`**

```rust
use regex::Regex;
use once_cell::sync::Lazy;

// URL validator
static URL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap()
});

pub fn is_valid_url(url: &str) -> bool {
    URL_REGEX.is_match(url)
}

// Phone validator
static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\+?1?\d{9,15}$").unwrap()
});

pub fn is_valid_phone(phone: &str) -> bool {
    PHONE_REGEX.is_match(phone)
}
```

---

## Database Integration

### Defining Models

**File: `src/database.rs`**

```rust
use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub bio: Option<String>,
    pub username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
    pub created_at: String,
}
```

### Database Queries

```rust
use sqlx::SqlitePool;

// Get all users
pub async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, age, bio, username FROM users ORDER BY id"
    )
    .fetch_all(pool)
    .await
}

// Get user by ID
pub async fn get_user(pool: &SqlitePool, id: i32) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, name, email, age, bio, username FROM users WHERE id = ? LIMIT 1"
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

// Create user
pub async fn create_user(
    pool: &SqlitePool,
    name: String,
    email: String,
    age: i32,
    username: String,
) -> Result<i32, sqlx::Error> {
    let result = sqlx::query(
        "INSERT INTO users (name, email, age, username) VALUES (?, ?, ?, ?)"
    )
    .bind(&name)
    .bind(&email)
    .bind(age)
    .bind(&username)
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid() as i32)
}

// Update user
pub async fn update_user(
    pool: &SqlitePool,
    id: i32,
    name: String,
    email: String,
    age: i32,
    bio: Option<String>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE users SET name = ?, email = ?, age = ?, bio = ? WHERE id = ?"
    )
    .bind(&name)
    .bind(&email)
    .bind(age)
    .bind(&bio)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

// Delete user
pub async fn delete_user(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;

    Ok(())
}
```

### Using Database in Pages

**File: `pages/users/index.rhtml`**

```html
<div class="users-container">
  <h1>Users</h1>

  <a href="/admin/users/new" class="btn btn-primary">Add User</a>

  @if users.is_empty()
    <p>No users found.</p>
  @else
    <table>
      <thead>
        <tr>
          <th>Name</th>
          <th>Email</th>
          <th>Age</th>
          <th>Actions</th>
        </tr>
      </thead>
      <tbody>
        @for user in users
          <tr>
            <td>@{user.name}</td>
            <td>@{user.email}</td>
            <td>@{user.age}</td>
            <td>
              <a href="/admin/users/@{user.id}/edit">Edit</a>
              <button onclick="deleteUser(@{user.id})">Delete</button>
            </td>
          </tr>
        @end
      </tbody>
    </table>
  @end
</div>

<script>
  function deleteUser(id) {
    if (confirm('Are you sure?')) {
      fetch(`/admin/users/${id}`, { method: 'DELETE' })
        .then(() => location.reload());
    }
  }
</script>

<style scoped>
  .users-container { padding: 2rem; }
  table { width: 100%; border-collapse: collapse; margin-top: 2rem; }
  th, td { padding: 1rem; text-align: left; border-bottom: 1px solid #ddd; }
  th { background: #f5f5f5; font-weight: 600; }
  button { background: #e74c3c; color: white; border: none; padding: 0.5rem 1rem; cursor: pointer; }
</style>
```

---

## Template Directives

### @if / @else

```html
@if user.is_active
  <span class="badge active">Active</span>
@else
  <span class="badge inactive">Inactive</span>
@end
```

### @for

```html
<ul>
  @for item in items
    <li>
      @{item.name}
      @if item.featured
        <span class="featured">Featured</span>
      @end
    </li>
  @end
</ul>
```

### @match

```html
@match user.role
  "admin" => {
    <div>Administrator</div>
  }
  "moderator" => {
    <div>Moderator</div>
  }
  _ => {
    <div>User</div>
  }
@end
```

### @partial

```html
@partial("Featured Items")
  <div class="featured">
    @for item in featured_items
      <div>@{item.name}</div>
    @end
  </div>
@end

@partial("Recent Items")
  <div class="recent">
    @for item in recent_items
      <div>@{item.name}</div>
    @end
  </div>
@end
```

Request specific partial: `?partial=Featured%20Items`

### @layout

```html
@layout(false)        <!-- No layout -->
@layout("admin")      <!-- Custom layout -->
<!-- default: uses _layout.rhtml -->
```

---

## Development Workflow

### Hot Reload

RHTML automatically reloads templates when you make changes:

```bash
# Terminal 1: Start dev server
cargo run

# Terminal 2: Edit your pages/
# Save a file → browser automatically refreshes
```

Set `HOT_RELOAD=false` to disable:

```bash
HOT_RELOAD=false cargo run
```

### Testing Pages

**Manual Testing:**

```bash
# Test GET request
curl http://localhost:3000/users

# Test POST request
curl -X POST http://localhost:3000/contact \
  -d "name=John&email=john@example.com&message=Hello"

# Request JSON response
curl -H "Accept: application/json" http://localhost:3000/users

# Request partial
curl "http://localhost:3000/users?partial=true"
```

### Debugging

Enable logging in your app:

```rust
// In main.rs
fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Your code...
}
```

Then view logs:

```bash
RUST_LOG=debug cargo run
```

---

## Best Practices

### 1. Organize Pages Logically

```
pages/
├── _layout.rhtml         # Root layout
├── index.rhtml           # Home
├── auth/
│   ├── login.rhtml
│   ├── signup.rhtml
│   └── logout.rhtml
├── dashboard/
│   ├── _layout.rhtml     # Dashboard layout
│   ├── index.rhtml
│   └── settings.rhtml
└── api/                  # API endpoints
    ├── users.rhtml
    └── posts.rhtml
```

### 2. Reuse Components

Create a `components/` directory:

```
components/
├── Alert.rhtml
├── Button.rhtml
├── Card.rhtml
├── Form.rhtml
├── Header.rhtml
└── Pagination.rhtml
```

### 3. Keep Validation Close

Define validation with your form handlers:

```rust
// src/forms/contact.rs
#[derive(Deserialize)]
pub struct ContactForm { ... }

impl Validate for ContactForm { ... }

pub async fn handle_contact(ctx: RequestContext) -> ActionResult { ... }
```

### 4. Use Scoped CSS

Keep styles with their templates:

```html
<div class="card">Content</div>

<style scoped>
  .card { /* Only applies to this template */ }
</style>
```

### 5. Meaningful Database Queries

Keep queries organized and documented:

```rust
/// Get user with their recent posts
pub async fn get_user_with_posts(
    pool: &SqlitePool,
    user_id: i32,
) -> Result<(User, Vec<Post>), sqlx::Error> {
    let user = get_user(pool, user_id).await?;
    let posts = get_user_posts(pool, user_id).await?;
    Ok((user.unwrap(), posts))
}
```

### 6. Handle Errors Gracefully

```html
@if error.is_some()
  <div class="alert alert-error">
    @{error}
  </div>
@end
```

### 7. Secure Forms

- Always validate input
- Use CSRF tokens (future feature)
- Escape output automatically (RHTML does this)
- Use parameterized queries (SQLx does this)

---

## Examples

### Blog Application

**File: `pages/blog/index.rhtml`**

```html
<div class="blog">
  <h1>Blog</h1>

  <div class="posts">
    @for post in posts
      <article class="post-card">
        <h2><a href="/blog/@{post.id}">@{post.title}</a></h2>
        <p class="meta">
          By <strong>@{post.author}</strong>
          on <time>@{post.created_at}</time>
        </p>
        <p>@{post.excerpt}</p>
        <a href="/blog/@{post.id}" class="read-more">Read More →</a>
      </article>
    @end
  </div>
</div>

<style scoped>
  .blog { max-width: 800px; margin: 0 auto; padding: 2rem; }
  .posts { display: flex; flex-direction: column; gap: 2rem; margin-top: 2rem; }
  .post-card {
    border: 1px solid #ddd;
    padding: 1.5rem;
    border-radius: 8px;
  }
  .post-card:hover { box-shadow: 0 4px 12px rgba(0,0,0,0.1); }
  .meta { color: #666; font-size: 0.9rem; }
  .read-more { color: #667eea; text-decoration: none; font-weight: 600; }
</style>
```

### Shopping Cart

```html
<div class="cart">
  <h1>Shopping Cart</h1>

  @if cart_items.is_empty()
    <p>Your cart is empty</p>
    <a href="/products">Continue Shopping</a>
  @else
    <table class="cart-items">
      <tr>
        <th>Product</th>
        <th>Qty</th>
        <th>Price</th>
        <th>Total</th>
        <th></th>
      </tr>
      @for item in cart_items
        <tr>
          <td>@{item.product_name}</td>
          <td>
            <input type="number" value="@{item.quantity}"
              onchange="updateQuantity(@{item.id}, this.value)">
          </td>
          <td>$@{item.price}</td>
          <td>$@{item.total}</td>
          <td>
            <button onclick="removeItem(@{item.id})">Remove</button>
          </td>
        </tr>
      @end
    </table>

    <div class="summary">
      <p><strong>Subtotal:</strong> $@{subtotal}</p>
      <p><strong>Tax:</strong> $@{tax}</p>
      <p><strong>Total:</strong> $@{total}</p>
      <button class="checkout-btn">Proceed to Checkout</button>
    </div>
  @end
</div>
```

---

## Next Steps

1. **Read the Full Documentation:**
   - [Layout System](./layout-system.md)
   - [Actions & Validation](./actions-and-validation.md)
   - [Configuration Guide](./configuration.md)

2. **Explore Examples:**
   - Check `pages/` for example templates
   - Review `components/` for reusable UI patterns
   - Study `src/example_actions.rs` for handler patterns

3. **Build Your App:**
   - Create your pages directory structure
   - Add your layouts
   - Build your components
   - Set up your database
   - Implement your business logic

4. **Deploy:**
   - Build release binary: `cargo build --release`
   - Set environment variables: `DATABASE_URL`, `PORT`
   - Run with production settings

---

## Getting Help

- **Check existing pages** in `pages/` for syntax examples
- **Review tests** in `src/` for usage patterns
- **Read error messages** - they're designed to help
- **Look at components** in `components/` for patterns

Happy coding! 🚀
