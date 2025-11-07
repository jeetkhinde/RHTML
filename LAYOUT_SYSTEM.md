# RHTML Layout and Slotting System

## Overview

RHTML supports a modern, type-safe layout and slotting system that provides:

- **Type-safe slot contracts** using `LayoutSlots` structs
- **Clean, declarative syntax** with `#[layout]` and `slot!` macros
- **Optional slots** with `Option<T>` types
- **Improved developer experience** with clear, typed interfaces

## Architecture

The layout system uses:

1. **`#[layout]` macro** - Marks layout functions
2. **`slot!` macro** - Declares slot values for pages
3. **LayoutSlots struct** - Defines the type-safe slot contract
4. **Runtime renderer** - Processes the slot values and renders layouts

### Components

#### 1. `rhtml-macro` Crate

Procedural macros for layout definitions:

- **`#[layout]`** - Marks layout functions
- **`slot!`** - Declares slot values for pages
- **`#[component]`** - Marks reusable components

Located in: `rhtml-macro/src/`

#### 2. Renderer

The renderer handles slot values through the `__rhtml_slots__` internal marker:

- Processes `slot!` macro output
- Extracts slot values
- Renders layouts with provided slots

Located in: `src/renderer.rs`

#### 3. Layout Registry (Future)

Infrastructure for enhanced compile-time validation:

- `layout_registry.rs` - Stores layout metadata
- `layout_resolver.rs` - Finds `_layout.rhtml` files
- Currently not actively used but ready for future enhancements

## Syntax

### Defining Layouts

**File**: `pages/_layout.rhtml`

```rust
// Define the slot contract
pub struct LayoutSlots {
    pub content: String,           // Required - auto-filled with page body
    pub title: Option<String>,      // Optional - with default fallback
    pub footer: Option<String>,     // Optional
}

#[layout]
pub fn layout(slots: LayoutSlots) {
<!DOCTYPE html>
<html lang="en">
<head>
  <title>{slots.title.unwrap_or("My App".to_string())}</title>
</head>
<body>
  <nav><!-- Navigation --></nav>
  <main>{slots.content}</main>
  <footer>{slots.footer.unwrap_or("© 2024".to_string())}</footer>
</body>
</html>
}
```

### Using Layouts in Pages

**File**: `pages/home.rhtml`

```rust
slot! {
    title: "Home Page",
    footer: "Welcome to our site"
}

#[webpage]
pub fn page(props: PageProps) {
  <div class="container">
    <h1>Welcome!</h1>
    <p>This content goes into the layout's content slot.</p>
  </div>
}
```

## How It Works

### Processing Flow

1. **Slot Macro Expansion**: `slot!` macro expands to internal `__rhtml_slots__` marker
2. **Parser Processing**: RHTML parser recognizes the slots marker
3. **Layout Resolution**: System finds appropriate `_layout.rhtml` file
4. **Rendering**: Layout is rendered with slot values, page content fills `content` slot

### Key Features

✅ **Type Safety**: `LayoutSlots` struct defines clear slot contracts
✅ **Optional Slots**: Use `Option<T>` for optional slots with defaults
✅ **Clean Syntax**: Declarative `slot!` macro for slot values
✅ **IDE Support**: Type definitions enable autocomplete and validation

## File Structure

```
pages/
├── _layout.rhtml          # Root layout
├── index.rhtml            # Uses slot! for slot values
├── about.rhtml
│
├── users/
│   ├── _layout.rhtml      # Users-specific layout
│   ├── index.rhtml        # Uses users layout
│   ├── new.rhtml
│   └── [id].rhtml
│
└── test/
    ├── _layout.rhtml      # Test layout example
    └── index.rhtml        # Test page example
```

## Implementation Files

### Core Files

1. **`rhtml-macro/src/`**
   - `lib.rs` - Exports `#[layout]`, `slot!`, `#[component]` macros
   - `layout.rs` - Implements `#[layout]` macro
   - `slot.rs` - Implements `slot!` macro
   - `layout_registry.rs` - Layout metadata storage (future)
   - `layout_resolver.rs` - Layout file discovery (future)

2. **`src/renderer.rs`**
   - `find_slots_block()` - Locates slot declarations
   - Slot value extraction and processing

3. **Example Files**
   - `pages/_layout.rhtml` - Root layout with new syntax
   - `pages/users/_layout.rhtml` - Nested layout example
   - `pages/test/_layout.rhtml` - Example layout
   - `pages/test/index.rhtml` - Example page with `slot!`

## Testing

### Running Tests

```bash
# Test the macro compilation
cd rhtml-macro && cargo test

# Test the full application
cargo run
curl http://localhost:3000
curl http://localhost:3000/users
```

### Test Coverage

✅ `slot!` macro compilation
✅ `#[layout]` macro compilation
✅ Layout rendering with slots
✅ Nested layouts
✅ Optional slots with defaults

## Best Practices

### 1. Define Clear Slot Contracts

```rust
pub struct LayoutSlots {
    pub content: String,        // Always include content
    pub title: Option<String>,   // Use Option for optional slots
    pub meta_description: Option<String>,
}
```

### 2. Provide Sensible Defaults

```rust
<title>{slots.title.unwrap_or("My App".to_string())}</title>
<meta name="description" content="{slots.meta_description.unwrap_or("Default description".to_string())}" />
```

### 3. Document Slot Purpose

```rust
pub struct LayoutSlots {
    /// Main content of the page (automatically filled)
    pub content: String,

    /// Page title shown in browser tab
    pub title: Option<String>,

    /// Footer text (defaults to copyright notice)
    pub footer: Option<String>,
}
```

## Future Enhancements

1. **Enhanced Compile-Time Validation**
   - Validate slot requirements at compile time
   - Provide compile errors for missing required slots
   - Type checking for slot values

2. **Advanced Slot Types**
   - Support for component slots
   - Function slots for dynamic content
   - Slot inheritance

3. **Better Error Messages**
   - Specific errors for missing required slots
   - Suggestions for typos in slot names
   - Type mismatch explanations

4. **IDE Integration**
   - Autocomplete for slot names
   - Type hints for slot values
   - Jump-to-definition for layouts

## Notes

- The `LayoutSlots` struct defines the contract for layout slots
- Slot values are passed through the `slot!` macro
- The `content` slot is automatically filled with the page body
- Use `Option<T>` for optional slots and provide defaults with `unwrap_or()`
- Nested layouts inherit from parent layouts up the directory tree

## Questions & Support

For questions or issues with the layout system:
- Check existing layouts in `pages/` for examples
- Review macro implementations in `rhtml-macro/src/`
- See renderer implementation in `src/renderer.rs`
