# Component Macro System Guide

## Overview

The `#[component]` procedural macro is a powerful system for defining reusable, renderable HTML components in RHTML. Components provide encapsulation, reusability, and clear intent for UI elements.

## Component Types

RHTML supports two types of components:

### 1. Public Components (`#[component] pub fn`)

Public components are accessible via HTTP requests and can be fetched as standalone HTML fragments.

**Usage in `.rs` Template Files:**

```rhtml
partial analytics(props: &PartialProps<()>) {
    <div id="analytics" class="stats">
        <span class="label">Total Users:</span>
        <span class="value">{count}</span>
    </div>
}
```

**Access via HTTP:**
```
GET /users?partial=analytics
```

**Features:**
- Externally accessible via query parameters
- Can be used with HTMX for dynamic partial updates
- Registered in the component registry automatically
- Useful for reusable UI widgets

**Example Use Cases:**
- Analytics dashboards
- User statistics
- Notification widgets
- Modal dialogs
- Form sections

### 2. Private Components (`#[component] fn`)

Private components are file-scoped and only accessible within the same template file. They're useful for breaking down complex UIs into smaller, manageable pieces.

**Usage in `.rs` Template Files:**

```rhtml
fn user_card(props: &PartialProps<UserData>) {
    <div class="card" id="user-{props.id}">
        <h3>{props.name}</h3>
        <p>{props.email}</p>
        <button hx-delete="/users/{props.id}"
                hx-target="closest .card"
                hx-swap="outerHTML"
                hx-confirm="Delete {props.name}?">
            Delete
        </button>
    </div>
}

WebPage {
    <div class="users-list" r-for="user in users">
        <user_card user={user} />
    </div>
}
```

**Features:**
- File-scoped visibility
- Not accessible via HTTP
- Used for internal composition
- Breaking down complex templates
- Improved code organization

**Example Use Cases:**
- List item components
- Form field components
- Layout helper components
- Complex widget subcomponents

## Architecture

The component system is built on:

1. **Component Trait** (`src/component.rs`)
   - Defines the interface all components must implement
   - Methods: `name()`, `render()`, `is_public()`

2. **Component Registry** (`src/component.rs`)
   - Global registry for all public components
   - Supports component lookup and enumeration
   - Thread-safe via `Mutex`

3. **Procedural Macro** (`rhtml-macro/src/lib.rs`)
   - `#[component]` attribute macro
   - Automatically marks components
   - Detects public vs private visibility

4. **Renderer Integration** (`src/renderer.rs`)
   - Looks up components from registry
   - Renders components with provided props
   - Handles both template-based and Rust-defined components

## Usage Examples

### Example 1: Simple Public Component

**File: `pages/components/analytics.rs`**

```rhtml
// This is a public component - accessible at GET /users?partial=analytics
partial analytics(props: &PartialProps<()>) {
    <div class="analytics-dashboard">
        <h2>Analytics</h2>
        <div class="metric">
            <span class="label">Page Views:</span>
            <span class="value">15,234</span>
        </div>
        <div class="metric">
            <span class="label">Unique Visitors:</span>
            <span class="value">3,421</span>
        </div>
    </div>
}
```

**Access:**
```bash
curl "http://localhost:3000/users?partial=analytics"
```

### Example 2: Private Component with Composition

**File: `pages/dashboard.rs`**

```rhtml
// Private component - used internally
fn stat_card(props: &PartialProps<StatData>) {
    <div class="stat-card">
        <h3>{props.title}</h3>
        <div class="stat-value">{props.value}</div>
        <div class="stat-trend" class:positive={props.trend > 0}>
            {props.trend}%
        </div>
    </div>
}

// Public page that uses the private component
WebPage {
    <div class="dashboard">
        <h1>Dashboard</h1>
        <div class="stats-grid">
            <stat_card title="Revenue" value="$42,500" trend={12} />
            <stat_card title="Users" value="1,234" trend={8} />
            <stat_card title="Engagement" value="68%" trend={-3} />
        </div>
    </div>
}
```

### Example 3: Public Component with HTMX

**File: `pages/notification.rs`**

```rhtml
// Public component - can be swapped via HTMX
partial notification(props: &PartialProps<NotificationData>) {
    <div class="notification" role="alert">
        <div class="notification-icon" class:success={props.type == "success"}>
            {if props.type == "success" { "✓" } else { "!" }}
        </div>
        <div class="notification-content">
            <h4>{props.title}</h4>
            <p>{props.message}</p>
        </div>
        <button class="notification-close"
                hx-delete="/notifications/{props.id}"
                hx-swap="outerHTML swap:1s">
            ×
        </button>
    </div>
}
```

**Usage with HTMX:**
```html
<button hx-get="/notifications?partial=notification&id=123"
        hx-target="#notification-area"
        hx-swap="beforeend">
    Load Notification
</button>
```

## Component Props

Components receive props through the `PartialProps<T>` generic struct:

```rust
pub struct PartialProps<T> {
    pub data: T,
    // Additional context available to all components
}
```

### Accessing Props

In templates, access props using Rust-like syntax:

```rhtml
partial user_info(props: &PartialProps<UserData>) {
    <div>
        <h2>{props.data.name}</h2>
        <p>{props.data.email}</p>
    </div>
}
```

## Component Registry

The component registry is a global, thread-safe store of all public components:

```rust
use rhtml_app::{get_component, register_component};

// Register a component (done automatically by the macro)
register_component(Arc::new(MyComponent));

// Get a component by name
if let Some(component) = get_component("my-component") {
    let html = component.render(props)?;
}
```

## Migration Guide

### From Old Syntax

**Old (cmp syntax):**
```rhtml
cmp Button {
    <button class="btn">{text}</button>
}
```

**New (with #[component] macro in Rust):**
```rust
#[component]
pub fn button(props: ButtonProps) {
    // Return HTML string
}
```

**Or (in .rs files):**
```rhtml
partial button(props: &PartialProps<ButtonProps>) {
    <button class="btn">{props.data.text}</button>
}
```

### From @partial Annotation

**Old syntax:**
```rhtml
@partial
fn my_partial() { ... }
```

**New syntax:**
```rhtml
// Just use partial keyword
partial my_partial(props: &PartialProps<()>) {
    // ...
}
```

## Best Practices

1. **Use Public Components for:**
   - Reusable widgets
   - Features that might be accessed independently
   - HTMX dynamic updates
   - API endpoints for UI fragments

2. **Use Private Components for:**
   - Breaking down large templates
   - Internal implementation details
   - Helper components
   - Reducing nesting complexity

3. **Component Naming:**
   - Use snake_case for component names
   - Use descriptive names that indicate purpose
   - Keep names concise but clear
   - Example: `user_card`, `stat_widget`, `notification_alert`

4. **Props Structure:**
   - Keep props focused and simple
   - Group related data into structs
   - Use Option<T> for optional fields
   - Validate props at component entry

5. **Composability:**
   - Prefer component composition over inheritance
   - Keep components single-responsibility
   - Make components reusable across contexts
   - Use slots for component customization (future feature)

## Testing Components

Test components in isolation:

```rust
#[test]
fn test_analytics_component() {
    let props = PartialProps::default();
    let html = analytics_component(&props).unwrap();
    assert!(html.contains("Analytics"));
}
```

## Performance Considerations

- **Caching:** Component HTML can be cached if props haven't changed
- **Lazy Loading:** Use `hx-trigger="load delay:1000"` for deferred rendering
- **Streaming:** Components can be streamed progressively
- **Batching:** Multiple component requests can be batched into single HTTP request

## Accessibility

All components should follow accessibility best practices:

```rhtml
partial notification(props: &PartialProps<NotificationData>) {
    <div class="notification"
         role="alert"
         aria-live="polite"
         aria-atomic="true">
        {props.data.message}
    </div>
}
```

## Future Enhancements

Planned features for the component system:

1. **Component Slots:** Allow content projection
   ```rhtml
   partial card(props: &PartialProps<()>) {
       <div class="card">
           <slot name="header" />
           <slot name="body" />
       </div>
   }
   ```

2. **Props Validation:** Runtime type checking
   ```rust
   #[component]
   #[validate]
   pub fn my_component(props: MyProps) { }
   ```

3. **Component Scoped CSS:** Automatic style scoping
   ```css
   @component my_component {
       .card { /* auto-scoped to component */ }
   }
   ```

4. **Async Components:** Non-blocking async rendering
   ```rust
   #[component]
   async fn async_component(props: Props) { }
   ```

## See Also

- [RHTML Documentation](README.md)
- [Template Syntax Guide](TEMPLATE_SYNTAX.md)
- [Routing Guide](ROUTING.md)
- [Directive Reference](DIRECTIVES.md)
