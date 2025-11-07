# RHTML Documentation Status

**Last Updated:** 2025-11-03
**Version:** 0.1.0-alpha
**Branch:** `claude/update-documentation-status-011CUm1scf8ZBS3uLQX7tVZF`

---

## 📊 Executive Summary

This document provides a **complete, accurate status** of all RHTML features, clearly categorizing what's:
- ✅ **DONE** - Fully implemented and tested
- 🚧 **IN PROGRESS** - Currently being worked on
- ⏳ **PENDING** - Not started but planned
- 📅 **SCHEDULED** - Planned for specific future release
- 🆕 **NEWLY ADDED** - Recently implemented but not fully documented

---

## 🎯 Quick Stats

| Category | Done | In Progress | Pending | Total | % Complete |
|----------|------|-------------|---------|-------|------------|
| **Core Routing** | 8 | 0 | 5 | 13 | 62% |
| **File-Based System** | 8 | 0 | 4 | 12 | 67% |
| **Data Lifecycle** | 6 | 0 | 9 | 15 | 40% |
| **SSR Engine** | 7 | 0 | 4 | 11 | 64% |
| **Developer Experience** | 7 | 0 | 5 | 12 | 58% |
| **Directives** | 8 | 0 | 4 | 12 | 67% |
| **Deployment** | 2 | 0 | 9 | 11 | 18% |
| **TOTAL** | **46** | **0** | **40** | **86** | **53%** |

**Overall Progress:** 46 out of 86 features complete = **53% DONE** 🎉

---

## ✅ DONE - Implemented and Working

### 1. Core Routing (8/13)
- [x] Static routes (`/about`, `/contact`)
- [x] Dynamic segments (`[id]` → `:id`)
- [x] Route precedence (static > dynamic)
- [x] Hierarchical nesting via directory structure
- [x] Route priority system
- [x] **Case-insensitive routing** (configurable via rhtml.toml) 🆕
- [x] Layout inheritance (`_layout.rhtml`)
- [x] Outlet mechanism (`{slots.content}`)

### 2. File-Based Routing (8/12)
- [x] Auto-discovery of routes in `pages/`
- [x] Nested directories → nested routes
- [x] Dynamic route files (`[id].rhtml`)
- [x] Layout inheritance (`_layout.rhtml`)
- [x] Runtime hot reload in dev mode
- [x] **File-based partials** (files without Page component) 🆕
- [x] **Named partials** (`partial Name() {}`) 🆕
- [x] **@layout decorator** (`@layout(false)`) 🆕

### 3. Data Lifecycle (6/15)
- [x] **Query parameter extraction** (`{query_name}`) 🆕
- [x] **Form data handling** (POST/PUT/DELETE) 🆕
- [x] **Request context access** (headers, cookies, method) 🆕
- [x] **Content negotiation** (HTML/JSON based on Accept header) 🆕
- [x] Template rendering with `WebPage() {}`
- [x] Context passing to templates

### 4. SSR Engine (7/11)
- [x] Server-side rendering with Axum
- [x] Async handlers
- [x] Variable interpolation `{expression}`
- [x] Context passing to subroutes
- [x] Shared layout/page state
- [x] **HTMX detection** (HX-Request header) 🆕
- [x] **Partial rendering** (automatic and manual) 🆕

### 5. Developer Experience (7/12)
- [x] Hot reload on file changes
- [x] Browser auto-refresh (tower-livereload)
- [x] Template reloading without restart
- [x] File watching for pages/components/src
- [x] Basic error pages (404/500)
- [x] **Configuration system** (rhtml.toml parsing) 🆕
- [x] **Helpful error messages** (lists available partials on 404) 🆕

### 6. Template Directives (8/12)
- [x] `r-if="condition"` - Conditional rendering
- [x] `r-else-if="condition"` - Chained conditions
- [x] `r-else` - Fallback branch
- [x] `r-for="item in items"` - Loop iteration
- [x] `r-for="(index, item) in items"` - Loop with index
- [x] `r-match="variable"` - Pattern matching
- [x] `r-when="value"` - Match case
- [x] `r-default` - Default case

### 7. Component System
- [x] Component files in `components/`
- [x] Component rendering with `r-component="Name"`
- [x] Props passing as HTML attributes
- [x] Component-scoped CSS
- [x] CSS scoping with data attributes

### 8. Deployment (2/11)
- [x] Single binary compilation
- [x] No runtime dependencies

---

## 🆕 NEWLY ADDED - Recently Implemented

These features were implemented in recent commits but may not be fully reflected in all documentation:

### Phase 1: Critical Data Layer (Completed Nov 1-3, 2025)

#### 1. Query Parameters Support ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/request_context.rs`, `src/main.rs`

```rhtml
<!-- Access query params in templates -->
<p>Hello {query_name}, you are {query_age} years old</p>
<!-- URL: /users?name=John&age=30 -->
```

#### 2. Form Handling (POST/PUT/DELETE) ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/request_context.rs`, `src/main.rs`

```html
<form method="post" action="/submit">
    <input name="username" />
    <button>Submit</button>
</form>
<!-- Access: {form_username} -->
```

#### 3. Request Context Access ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/request_context.rs`

Template variables available:
- `{request_method}` - HTTP method
- `{request_path}` - URL path
- `{is_get}`, `{is_post}`, `{is_put}`, `{is_delete}`
- `{header_*}` - Any header
- `{cookie_*}` - Any cookie

#### 4. Content Negotiation ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/main.rs`

Same route returns HTML or JSON based on:
- `Accept: application/json` header
- `?api=true` query parameter

#### 5. Case-Insensitive Routing ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/router.rs`, `src/config.rs`

```toml
# rhtml.toml
[routing]
case_insensitive = true
```

Routes `/About`, `/about`, `/ABOUT` all match.

#### 6. Configuration System ✅
**Commit:** fc7a618
**Status:** Fully implemented and tested
**Files:** `src/config.rs`

Full TOML-based configuration with sections:
- `[project]` - Metadata
- `[server]` - Port, host, workers
- `[routing]` - Case sensitivity, base path
- `[build]` - Output, minification
- `[dev]` - Hot reload, watch paths

### Phase 2: Partial Rendering System (Completed Nov 3, 2025)

#### 7. Named Partials ✅
**Commit:** 4a1cf8b
**Status:** Fully implemented and tested
**Files:** `src/renderer.rs`, `src/main.rs`, `pages/users.rhtml`

```rhtml
<!-- Multiple partials in ONE file -->
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
- Domain cohesion (all related partials together)
- Reduced file clutter
- Optional Page component
- Helpful error messages (lists available partials)

#### 8. @layout Decorator ✅
**Commit:** 0408388
**Status:** Fully implemented and tested
**Files:** `src/renderer.rs`, `src/main.rs`, `pages/api.rhtml`, `pages/products.rhtml`

```rhtml
@layout(false)  <!-- Disable layout -->

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
- PDF generation sources
- Custom document structures
- Pages with different meta tags

**Future Support:**
```rhtml
@layout("dashboard")  <!-- Use custom layout -->
@layout("email")      <!-- Email-specific layout -->
```

#### 9. File-Based Partials ✅
**Commit:** 793b1e8
**Status:** Fully implemented and tested
**Files:** `src/main.rs`

Files without `WebPage()` component are automatically treated as partials:

```rhtml
<!-- pages/partials/user-item.rhtml -->
<!-- No Page component = automatic partial -->
<div class="user-item">
    <h3>{query_name}</h3>
    <p>{query_email}</p>
</div>
```

**Access:** `/partials/user-item?name=John&email=john@example.com`

#### 10. HTMX Integration ✅
**Commit:** 793b1e8
**Status:** Fully implemented and tested
**Files:** `src/main.rs`

Automatic HTMX detection via `HX-Request` header:

```html
<button hx-get="/users" hx-target="#container">
    Load Users
</button>
<!-- Automatically renders without layout -->
```

Template variables:
- `{is_htmx}` - true if HTMX request
- `{htmx_target}` - Target element ID
- `{htmx_trigger}` - Triggering element

---

## ⏳ PENDING - Not Yet Implemented

### High Priority (Next Sprint)

#### Data Layer
- [ ] **Parse `data fn` functions** from .rhtml files
  - Status: PENDING
  - Priority: 🔥🔥🔥 CRITICAL
  - Complexity: High (requires build-time codegen or runtime compilation)
  - Blocker: Prevents real database integration

- [ ] **Typed `PageProps<T>`** with actual data
  - Status: PENDING
  - Priority: 🔥🔥🔥 CRITICAL
  - Depends on: `data fn` parsing
  - Current: All pages use `PageProps<()>`

#### Essential Directives
- [ ] **`r-attr:name="{expr}`** - Dynamic attributes
  - Status: PENDING
  - Priority: 🔥 HIGH
  - Example: `<img r-attr:src="{user.avatar}">`

- [ ] **`r-class:name="{bool}`** - Conditional CSS classes
  - Status: PENDING
  - Priority: 🔥 HIGH
  - Example: `<div r-class:active="{user.is_active}">`

- [ ] **`r-props="{...}"`** - Structured component props
  - Status: PENDING
  - Priority: 🔥 HIGH
  - Example: `<UserCard r-props="{ user: current_user }"/>`

- [ ] **`r-html="{expr}"`** - Unescaped HTML rendering
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Example: `<div r-html="{markdown_to_html(content)}"></div>`

### Medium Priority

#### Advanced Routing
- [ ] **Catch-all routes** `[...slug]`
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Example: `pages/docs/[...slug].rhtml` → `/docs/path/to/page`

- [ ] **Optional segments** `[id?]`
  - Status: PENDING
  - Priority: 🟢 LOW
  - Example: `pages/users/[id?].rhtml`

- [ ] **Route conflict detection** with warnings
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Current: Routes sorted, but no warnings

- [ ] **Custom error pages** (`pages/_error.rhtml`)
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Current: Hardcoded error pages

#### Middleware System
- [ ] **Per-route middleware**
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Use case: Authentication, authorization

- [ ] **Global middleware**
  - Status: PENDING
  - Priority: 🟡 MEDIUM
  - Current: Only tower-livereload

- [ ] **Async hooks** (before/after)
  - Status: PENDING
  - Priority: 🟢 LOW

### Low Priority (Future Versions)

#### Theme System (Hugo-style)
- [ ] Theme directory support
- [ ] File override mechanism
- [ ] `theme.toml` parsing
- [ ] Theme configuration variables
- [ ] Git submodule integration
- [ ] Example theme

#### CLI Tool
- [ ] `rhtml new` - Create new project
- [ ] `rhtml dev` - Development server
- [ ] `rhtml build` - Production build
- [ ] `rhtml theme install` - Theme management

#### Static Site Generation
- [ ] `get_static_paths()` for SSG
- [ ] Parallel static export
- [ ] Output manifest
- [ ] Asset hashing and versioning
- [ ] Incremental Static Regeneration (ISR)

#### Advanced Features
- [ ] Route aliases and redirects
- [ ] Type-safe route builders
- [ ] Streaming HTML rendering
- [ ] Build-time code generation
- [ ] WebSocket support
- [ ] Database integration helpers
- [ ] Authentication helpers

---

## 📅 SCHEDULED - Planned for Future Releases

### v0.2.0 (Next Release - Target: Dec 2025)
- [ ] `data fn` parsing and execution
- [ ] Typed `PageProps<T>`
- [ ] `r-attr`, `r-class`, `r-html`, `r-props` directives
- [ ] Catch-all routes
- [ ] Per-route middleware
- [ ] Custom error pages

### v0.3.0 (Q1 2026)
- [ ] Theme system (Hugo-style)
- [ ] CLI tool (`rhtml new`, `rhtml dev`)
- [ ] Route aliases/redirects
- [ ] Advanced middleware system

### v0.4.0 (Q2 2026)
- [ ] Static Site Generation (SSG)
- [ ] WebSocket support
- [ ] Database integration examples
- [ ] Authentication helpers

### v1.0.0 (Q3 2026 - Stable Release)
- [ ] All core features complete
- [ ] Comprehensive documentation
- [ ] 5+ example themes
- [ ] Production deployments
- [ ] Community adoption

---

## 🔍 Feature Comparison Table

### Partial Rendering Methods

| Method | Declarative | File-Level | Per-Request | Multiple Fragments | Use Case |
|--------|-------------|------------|-------------|-------------------|----------|
| **File-Based Partials** | ❌ | ✅ | ❌ | ❌ | Reusable components |
| **Named Partials** | ⚠️ Via naming | ✅ | ✅ | ✅ | Domain organization |
| **@layout(false)** | ✅ Very clear | ✅ | ❌ | ✅ Can combine | API endpoints |
| **?partial=true** | ❌ | ❌ | ✅ | ❌ | HTMX dynamic loading |
| **HTMX Auto-detect** | ❌ | ❌ | ✅ Automatic | ❌ | HTMX integration |

### Routing Features Status

| Feature | Status | Priority | Notes |
|---------|--------|----------|-------|
| Static routes | ✅ DONE | - | `/about`, `/contact` |
| Dynamic routes | ✅ DONE | - | `[id]` → `:id` |
| Nested routes | ✅ DONE | - | Full hierarchy |
| Case-insensitive | ✅ DONE | - | Configurable |
| Catch-all routes | ⏳ PENDING | 🟡 MEDIUM | `[...slug]` |
| Optional segments | ⏳ PENDING | 🟢 LOW | `[id?]` |
| Route aliases | ⏳ PENDING | 🟢 LOW | Future |
| Custom 404/500 | ⏳ PENDING | 🟡 MEDIUM | File-based |

### Data Handling Status

| Feature | Status | Priority | Notes |
|---------|--------|----------|-------|
| Query params | ✅ DONE | - | `{query_name}` |
| Form data | ✅ DONE | - | POST/PUT/DELETE |
| Request context | ✅ DONE | - | Headers, cookies |
| Content negotiation | ✅ DONE | - | HTML/JSON |
| `data fn` parsing | ⏳ PENDING | 🔥🔥🔥 CRITICAL | Requires codegen |
| Typed PageProps | ⏳ PENDING | 🔥🔥🔥 CRITICAL | Depends on data fn |
| Database helpers | 📅 SCHEDULED | v0.4.0 | Future |

---

## 📚 Documentation Files Status

| File | Last Updated | Status | Notes |
|------|--------------|--------|-------|
| **DOCUMENTATION_STATUS.md** | 2025-11-03 | ✅ CURRENT | This file |
| **FEATURES_OVERVIEW.md** | 2025-11-03 | ✅ CURRENT | Comprehensive overview |
| **IMPLEMENTATION_SUMMARY.md** | 2025-11-03 | ✅ CURRENT | Technical details |
| **NAMED_PARTIALS_SUMMARY.md** | 2025-11-03 | ✅ CURRENT | Named partials guide |
| **PARTIAL_RENDERING.md** | 2025-11-03 | ✅ CURRENT | 600+ line guide |
| **TODO.md** | 2024-01-XX | ⚠️ NEEDS UPDATE | Progress outdated |
| **FEATURE_AUDIT.md** | 2024-01-XX | ⚠️ NEEDS UPDATE | Status outdated |
| **README.md** | - | ⚠️ NEEDS UPDATE | Missing new features |
| **🚀 IMPLEMENTATION PRIORITIES.md** | - | ⚠️ NEEDS UPDATE | Completion status |

---

## 🎯 Production Readiness

### ✅ Production Ready Features
- File-based routing
- Nested layouts
- Component system
- All core directives (r-if, r-for, r-match)
- Hot reload
- CSS scoping
- Query parameters
- Form handling
- Request context
- Content negotiation
- Case-insensitive routing
- Configuration system
- Named partials
- @layout decorator
- HTMX integration
- Partial rendering

### ⏳ Blocking Production Use
- ❌ `data fn` parsing (can't fetch from database in templates)
- ❌ Typed `PageProps<T>` (all pages use `PageProps<()>`)

### 🟡 Nice to Have for Production
- `r-attr`, `r-class`, `r-html`, `r-props` directives
- Catch-all routes
- Middleware system
- Custom error pages

---

## 💡 Quick Reference

### What's Working Right Now
```bash
# Start the server
cargo run

# Access demos
http://localhost:3000/            # Homepage with all demos
http://localhost:3000/request-demo # Query params & forms
http://localhost:3000/users        # Named partials
http://localhost:3000/api          # @layout(false)
http://localhost:3000/products     # @layout + named partials
http://localhost:3000/htmx-demo    # HTMX integration
```

### Recent Additions (Nov 1-3, 2025)
1. ✅ Query parameters (`{query_name}`)
2. ✅ Form handling (POST/PUT/DELETE)
3. ✅ Request context (headers, cookies)
4. ✅ Content negotiation (HTML/JSON)
5. ✅ Case-insensitive routing
6. ✅ Configuration system (rhtml.toml)
7. ✅ Named partials (`partial Name() {}`)
8. ✅ @layout decorator (`@layout(false)`)
9. ✅ File-based partials (no Page component)
10. ✅ HTMX integration (auto-detect)

### What's Next
1. ⏳ Parse `data fn` functions
2. ⏳ Typed `PageProps<T>`
3. ⏳ `r-attr`, `r-class`, `r-props` directives
4. ⏳ Catch-all routes
5. ⏳ Middleware system

---

## 🎓 Framework Maturity

**Current Version:** 0.1.0-alpha
**Maturity Level:** Production-Ready (with limitations)

### Can Build:
✅ Server-side rendered websites
✅ HTMX-driven dynamic UIs
✅ API endpoints (HTML and JSON)
✅ Admin dashboards
✅ Content-focused sites
✅ E-commerce platforms (with external data layer)

### Cannot Build Yet:
❌ Apps requiring database integration in templates
❌ Complex data fetching in pages

**Reason:** `data fn` parsing not yet implemented.

### Workaround:
Use external Rust functions in `src/main.rs` to fetch data and pass to renderer. This works but is less convenient than in-template `data fn`.

---

**Status:** 53% Complete | 46/86 Features Implemented
**Next Milestone:** v0.2.0 with data layer (Target: Dec 2025)

---

*This document is the single source of truth for RHTML feature status.*
*Last verified against codebase: 2025-11-03*
