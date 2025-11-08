# RHTML Quick Reference

Fast lookup for common tasks in RHTML development.

## File Organization

```
pages/          → Routes (directory structure = URL routes)
components/     → Reusable UI components
src/            → Rust backend code
Cargo.toml      → Dependencies
rhtml.toml      → Configuration
```

## Creating Routes

| File | URL |
|------|-----|
| `pages/index.rhtml` | `/` |
| `pages/about.rhtml` | `/about` |
| `pages/blog/[id].rhtml` | `/blog/123` |
| `pages/admin/_layout.rhtml` | Layout for `/admin/*` |

## Template Directives

### Control Flow

```html
@if condition
  <p>True</p>
@else
  <p>False</p>
@end

@for item in items
  <li>@{item.name}</li>
@end

@match value
  "option1" => { <p>One</p> }
  "option2" => { <p>Two</p> }
  _ => { <p>Other</p> }
@end
```

### Slots & Components

```html
@partial("ComponentName")
  <div>Content</div>
@end

<!-- Use with: ?partial=ComponentName -->
```

### Layout Control

```html
@layout(false)           <!-- No layout -->
@layout("admin")         <!-- Custom layout -->
<!-- Default: nearest _layout.rhtml -->
```

## Variable Syntax

```html
<!-- Output variable -->
@{variable}

<!-- With default -->
@{variable.unwrap_or("default")}

<!-- Method calls -->
@{list.len()}
@{string.to_uppercase()}

<!-- Object properties -->
@{user.name}
@{user.email}

<!-- Conditions -->
@if user.is_active
@if items.is_empty()
```

## Scoped Styles

```html
<style scoped>
  /* Only applies to this template */
  .local-class { color: red; }
</style>
```

## Forms & Validation

```html
<!-- Form submission -->
<form method="post" action="/submit">
  <input name="field" value="@{form.get_value("field").unwrap_or("")}" />
  @if form.has_error("field")
    <span class="error">@{form.get_error("field")}</span>
  @end
  <button>Submit</button>
</form>
```

## Form Handling in Rust

```rust
use serde::Deserialize;
use crate::validation::Validate;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct MyForm {
    pub field: String,
}

impl Validate for MyForm {
    fn validate(&self) -> Result<(), HashMap<String, String>> {
        let mut errors = HashMap::new();

        if self.field.is_empty() {
            errors.insert("field".to_string(), "Required".to_string());
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
```

## Actions (Form Handlers)

```rust
// In src/example_actions.rs
pub async fn post_form_name(ctx: RequestContext) -> ActionResult {
    let result = validate_request::<FormType>(&ctx.form);

    match result {
        ValidationPipelineResult::Valid(data) => {
            // Process data
            ActionResult::Html {
                content: "Success!".to_string(),
                headers: Default::default(),
            }
        }
        ValidationPipelineResult::Invalid(form_ctx) => {
            // Return form with errors
            ActionResult::Html {
                content: format!("Error: {:?}", form_ctx.errors),
                headers: Default::default(),
            }
        }
    }
}

// Register in action_handlers.rs
registry.register("/route", "POST", |ctx| {
    Box::pin(example_actions::post_form_name(ctx))
});
```

## Database Queries

```rust
use sqlx::SqlitePool;

// Get all
pub async fn list(pool: &SqlitePool) -> Result<Vec<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT * FROM items")
        .fetch_all(pool)
        .await
}

// Get one
pub async fn get(pool: &SqlitePool, id: i32) -> Result<Option<Item>, sqlx::Error> {
    sqlx::query_as::<_, Item>("SELECT * FROM items WHERE id = ?")
        .bind(id)
        .fetch_optional(pool)
        .await
}

// Create
pub async fn create(pool: &SqlitePool, name: String) -> Result<i32, sqlx::Error> {
    let result = sqlx::query("INSERT INTO items (name) VALUES (?)")
        .bind(&name)
        .execute(pool)
        .await?;
    Ok(result.last_insert_rowid() as i32)
}

// Update
pub async fn update(pool: &SqlitePool, id: i32, name: String) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE items SET name = ? WHERE id = ?")
        .bind(&name)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}

// Delete
pub async fn delete(pool: &SqlitePool, id: i32) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM items WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
```

## HTTP Requests

### GET Request

```bash
curl http://localhost:3000/path
```

### POST Request

```bash
curl -X POST http://localhost:3000/path \
  -d "field1=value1&field2=value2"
```

### JSON Request

```bash
curl -X POST http://localhost:3000/path \
  -H "Content-Type: application/json" \
  -d '{"field":"value"}'
```

### Request Specific Partial

```bash
curl "http://localhost:3000/page?partial=ComponentName"
```

### Request JSON Response

```bash
curl -H "Accept: application/json" http://localhost:3000/path
```

## Configuration

**File: `rhtml.toml`**

```toml
[server]
port = 3000
host = "127.0.0.1"

[routing]
pages_dir = "pages"
components_dir = "components"
case_insensitive = true

[dev]
hot_reload = true

[database]
url = "sqlite:app.db"
```

## Common Patterns

### Display List with Pagination

```html
@for item in paginated_items
  <div class="item">@{item.name}</div>
@end

<div class="pagination">
  @if page > 1
    <a href="?page=@{page - 1}">← Previous</a>
  @end

  <span>Page @{page}</span>

  @if has_next_page
    <a href="?page=@{page + 1}">Next →</a>
  @end
</div>
```

### Conditional Display

```html
@if user.is_admin()
  <div class="admin-panel">
    <a href="/admin">Admin</a>
  </div>
@end

@if cart_items.is_empty()
  <p>No items in cart</p>
@else
  <table>
    @for item in cart_items
      <tr><td>@{item.name}</td></tr>
    @end
  </table>
@end
```

### Form with Error Display

```html
<form method="post">
  @if form.has_errors()
    <div class="alert alert-error">
      Please fix the following errors:
      <ul>
        @for (field, error) in form.errors.iter()
          <li>@{field}: @{error}</li>
        @end
      </ul>
    </div>
  @end

  <input name="email" value="@{form.get_value("email").unwrap_or("")}" />
  @if form.has_error("email")
    <span class="field-error">@{form.get_error("email")}</span>
  @end
</form>
```

### Conditional CSS

```html
<div class="item @{if item.featured { "featured" } else { "" }}">
  @{item.name}
</div>

<style scoped>
  .item { padding: 1rem; }
  .item.featured { background: gold; }
</style>
```

## Environment Variables

```bash
# Run with custom settings
PORT=8080 HOT_RELOAD=false cargo run

# Enable debug logging
RUST_LOG=debug cargo run

# Set database
DATABASE_URL="sqlite:mydb.db" cargo run
```

## Development Commands

```bash
# Run dev server
cargo run

# Build release
cargo build --release

# Run tests
cargo test

# Check without building
cargo check

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Debugging Tips

1. **Check page loads:** Visit `http://localhost:3000/your-page`
2. **Check form submission:** Look at browser console + server logs
3. **Check database:** Use SQLite browser to inspect `.db` file
4. **Enable logging:** `RUST_LOG=debug cargo run`
5. **Test routes:** Use `curl` to test different methods
6. **Inspect HTML:** Use browser DevTools

## Performance Tips

1. **Cache templates** - TemplateLoader caches automatically
2. **Use database indexes** - Index frequently queried columns
3. **Reuse layouts** - Avoid duplicating HTML
4. **Optimize queries** - Select only needed columns
5. **Use partial rendering** - For AJAX requests
6. **Enable compression** - For production

## Security Checklist

- [ ] Validate all form inputs
- [ ] Use parameterized queries (SQLx does this)
- [ ] Escape output (RHTML does this automatically)
- [ ] Use HTTPS in production
- [ ] Validate file uploads
- [ ] Implement CSRF tokens (future feature)
- [ ] Use secure password hashing
- [ ] Implement rate limiting

---

For more details, see:
- [Getting Started Guide](./getting-started.md)
- [Layout System](./layout-system.md)
- [Actions & Validation](./actions-and-validation.md)
