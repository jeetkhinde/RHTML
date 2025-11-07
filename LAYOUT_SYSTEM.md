# RHTML Layout and Slotting System

## Overview

RHTML now supports a modern, type-safe layout and slotting system that provides:

- **Type-safe slot contracts** using `LayoutSlots` structs
- **Compile-time syntax validation** with `#[layout]` and `slot!` macros
- **Runtime compatibility** through automatic transformation
- **Improved developer experience** with clear, declarative syntax

## Architecture

The layout system uses a **hybrid approach**:

1. **Compile-time macros** (`#[layout]`, `slot!`) provide syntax validation and IDE support
2. **Runtime transformation** converts new syntax to existing runtime format
3. **Backward compatibility** - existing `cmp layout` and `slots {}` syntax still works

### Components

#### 1. `rhtml-macro` Crate

New procedural macros for layout definitions:

- **`#[layout]`** - Marks layout functions and transforms syntax
- **`slot!`** - Declares slot values for pages
- **`#[component]`** - Marks reusable components

Located in: `rhtml-macro/src/`

#### 2. Runtime Parser Extensions

The `FunctionComponentParser` now handles:

- Transformation of `slot! { }` to `slots { }`
- Transformation of `#[layout]` to `cmp layout`
- Removal of `LayoutSlots` struct definitions (documentation only)

Located in: `rhtml-parser/src/function_component.rs`

#### 3. Layout Registry (Future)

Infrastructure for compile-time validation:

- `layout_registry.rs` - Stores layout metadata
- `layout_resolver.rs` - Finds `_layout.rhtml` files
- Currently not actively used but ready for future compile-time validation

## New Syntax

### Defining Layouts

**File**: `pages/_layout.rhtml`

```rust
// Define the slot contract (documentation + future validation)
pub struct LayoutSlots {
    pub content: String,           // Required - auto-filled with page body
    pub title: String,              // Required - must be provided
    pub description: Option<String>, // Optional
}

#[layout]
pub fn layout(slots: LayoutSlots) {
<!DOCTYPE html>
<html lang="en">
<head>
  <title>{slots.get("title").unwrap_or("My App")}</title>
  <meta name="description" content="{slots.get("description").unwrap_or("")}" />
</head>
<body>
  <nav><!-- Navigation --></nav>
  <main>{slots.content}</main>
  <footer>© 2024</footer>
</body>
</html>
}
```

### Using Layouts in Pages

**File**: `pages/home.rhtml`

```rust
slot! {
    title: "Home Page",
    description: "Welcome to our site"
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

### Transformation Process

1. **File is loaded** by template loader at runtime
2. **Parser transforms** new syntax to old syntax:
   ```rust
   // Input (new syntax)
   slot! { title: "Home" }

   // Output (runtime syntax)
   slots { title: "Home" }
   ```

3. **Layout macro transforms**:
   ```rust
   // Input
   #[layout]
   pub fn layout(slots: LayoutSlots) { ... }

   // Output
   cmp layout(slots: &Slots) { ... }
   ```

4. **LayoutSlots struct is removed** (it's just documentation)
5. **Existing renderer** processes the transformed code

### Benefits

✅ **Developer Experience**: Clean, typed syntax with IDE support
✅ **Backward Compatible**: Old syntax still works
✅ **No Breaking Changes**: Gradual migration path
✅ **Future-Ready**: Infrastructure for true compile-time validation

## Migration Guide

### From Old to New Syntax

**Old Style** (`pages/_layout.rhtml`):
```rust
cmp layout(slots: &Slots) {
<!DOCTYPE html>
<html>
  <head><title>{slots.get("title").unwrap_or("App")}</title></head>
  <body>{slots.content}</body>
</html>
}
```

**New Style**:
```rust
pub struct LayoutSlots {
    pub content: String,
    pub title: String,
}

#[layout]
pub fn layout(slots: LayoutSlots) {
<!DOCTYPE html>
<html>
  <head><title>{slots.get("title").unwrap_or("App")}</title></head>
  <body>{slots.content}</body>
</html>
}
```

**Old Style** (`pages/home.rhtml`):
```rust
slots {
  title: "Home"
}

#[webpage]
pub fn page(props: PageProps) { ... }
```

**New Style**:
```rust
slot! {
    title: "Home"
}

#[webpage]
pub fn page(props: PageProps) { ... }
```

## File Structure

```
pages/
├── _layout.rhtml          # Root layout (uses #[layout] syntax)
├── index.rhtml            # Uses slot! for slot values
├── about.rhtml
│
├── dashboard/
│   ├── _layout.rhtml      # Dashboard-specific layout
│   └── index.rhtml        # Uses dashboard layout
│
└── test/
    ├── _layout.rhtml      # Example of new syntax
    └── index.rhtml        # Example page with slot!
```

## Implementation Files

### Core Files Changed/Added

1. **`rhtml-macro/src/`**
   - `lib.rs` - Exports `#[layout]`, `slot!`, `#[component]` macros
   - `layout.rs` - Implements `#[layout]` transformation
   - `slot.rs` - Implements `slot!` transformation
   - `layout_registry.rs` - Layout metadata storage (future)
   - `layout_resolver.rs` - Layout file discovery (future)

2. **`rhtml-parser/src/function_component.rs`**
   - Added `transform_new_syntax()` - Main transformation function
   - Added `transform_slot_macro()` - Transforms `slot!` to `slots`
   - Added `transform_layout_macro()` - Transforms `#[layout]` to `cmp`
   - Added `remove_layout_slots_struct()` - Removes struct definitions
   - Added tests for all transformations

3. **`rhtml-macro/Cargo.toml`**
   - Added `once_cell` dependency for registry

4. **Example Files**
   - `pages/test/_layout.rhtml` - Example layout with new syntax
   - `pages/test/index.rhtml` - Example page with `slot!`

## Testing

### Running Tests

```bash
# Test the parser transformations
cargo test function_component

# Test the macro compilation
cd rhtml-macro && cargo test

# Test the full application
cargo run
curl http://localhost:3000/test
```

### Test Coverage

✅ `slot!` macro transformation
✅ `#[layout]` macro transformation
✅ `LayoutSlots` struct removal
✅ Full syntax transformation pipeline
✅ Backward compatibility with old syntax

## Future Enhancements

1. **True Compile-Time Validation**
   - Use layout registry to validate slot requirements
   - Provide compile errors for missing/wrong slots
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

- The `LayoutSlots` struct is currently **documentation only** - it gets removed at runtime
- Slot values are still accessed via `slots.get("name")` at runtime
- Future versions will add true compile-time validation using the struct definition
- Both old and new syntax work simultaneously - no migration pressure

## Questions & Support

For questions or issues with the layout system:
- Check existing layouts in `pages/` for examples
- Review tests in `rhtml-parser/src/function_component.rs`
- See macro implementations in `rhtml-macro/src/`
