# RHTML Feature Tracking

**Last Updated:** 2024-01-XX
**Total Progress:** 28/99 features (28%)

## 📊 Progress by Category

| Category | Done | Pending | Total | % |
|----------|------|---------|-------|---|
| Routing Core | 5 | 8 | 13 | 38% |
| File-based System | 5 | 7 | 12 | 42% |
| Data Lifecycle | 2 | 13 | 15 | 13% ⚠️ |
| SSR Engine | 4 | 7 | 11 | 36% |
| Developer Experience | 3 | 9 | 12 | 25% |
| Directives | 8 | 4 | 12 | 67% |
| Deployment | 1 | 10 | 11 | 9% |
| New Requirements | 0 | 13 | 13 | 0% ⚠️ |

---

## 🔥 CRITICAL PRIORITIES (Sprint 1-2)

### Data Layer & Request Handling
- [ ] **#1** Parse `data fn` functions from .rhtml files
- [ ] **#2** Extract query parameters in handlers (`query.get("filter")`)
- [ ] **#3** Form handling (POST/PUT/DELETE methods)
- [ ] **#4** Typed `PageProps<T>` with actual data (replace `PageProps<()>`)
- [ ] **#5** Request context access (cookies, headers, session)
- [ ] **#6** Content negotiation (same file returns HTML or JSON)
- [ ] **#7** Case-insensitive routing (configurable)
- [ ] **#8** `rhtml.toml` configuration parsing

**Target:** Week 1-3
**Status:** 🔴 Blocked - Cannot build real apps without these

---

## 🟡 HIGH PRIORITY (Sprint 3-4)

### Essential Directives
- [ ] **#9** `r-attr:name="{expr}"` - Dynamic attributes
- [ ] **#10** `r-class:name="{bool}"` - Conditional CSS classes
- [ ] **#11** `r-props="{...}"` - Structured component props
- [ ] **#12** `r-html="{expr}"` - Unescaped HTML rendering

### Advanced Routing
- [ ] **#13** Catch-all routes `[...slug]`
- [ ] **#14** Optional segments `[id?]`
- [ ] **#15** Route conflict detection with warnings
- [ ] **#16** Custom error pages (`pages/_error.rhtml`)

**Target:** Week 4-5
**Status:** 🟡 Important for feature parity with modern frameworks

---

## 🟢 MEDIUM PRIORITY (Sprint 5-6)

### Theme System (Hugo-style)
- [ ] **#17** Theme directory support in TemplateLoader
- [ ] **#18** File override mechanism (user files > theme files)
- [ ] **#19** `theme.toml` parsing
- [ ] **#20** Theme configuration variables
- [ ] **#21** Git submodule integration documentation
- [ ] **#22** Create example theme

### Developer Experience
- [ ] **#23** Dedicated CLI tool (`rhtml dev`, `rhtml build`)
- [ ] **#24** Improved error messages with line numbers
- [ ] **#25** Route listing command
- [ ] **#26** Middleware system (per-route + global)

**Target:** Week 6-8
**Status:** 🟢 Enhances ecosystem and DX

---

## ⏸️ LOW PRIORITY (Future)

### Advanced Features
- [ ] **#27** Route aliases and redirects
- [ ] **#28** Type-safe route builders
- [ ] **#29** Streaming HTML rendering
- [ ] **#30** Build-time code generation
- [ ] **#31** Theme CLI commands (`rhtml theme install`)
- [ ] **#32** Theme registry website

### SSG/ISR (Far Future)
- [ ] **#33** `get_static_paths()` for SSG
- [ ] **#34** Parallel static export
- [ ] **#35** Asset hashing and versioning
- [ ] **#36** Incremental Static Regeneration
- [ ] **#37** Cache invalidation strategies

**Status:** 🔵 Nice to have, not blocking

---

## ✅ COMPLETED FEATURES

### 1. Routing Core
- [x] Static routes (`/about`, `/contact`)
- [x] Dynamic segments (`[id]` → `:id`)
- [x] Route precedence (static > dynamic)
- [x] Hierarchical nesting via directory structure
- [x] Route priority system

### 2. File-based Routing
- [x] Auto-discovery of routes in `pages/`
- [x] Nested directories → nested routes
- [x] Dynamic route files (`[id].rhtml`)
- [x] Layout inheritance (`_layout.rhtml`)
- [x] Runtime hot reload in dev mode

### 3. Layout System
- [x] Root layout (`pages/_layout.rhtml`)
- [x] Section layouts (`pages/users/_layout.rhtml`)
- [x] Slot system (`{slots.content}`, `{slots.get("key")}`)
- [x] Layout hierarchy resolution

### 4. Directives
- [x] `r-if="condition"` - Conditional rendering
- [x] `r-else-if="condition"` - Chained conditions
- [x] `r-else` - Fallback branch
- [x] `r-for="item in items"` - Loop iteration
- [x] `r-for="(index, item) in items"` - Loop with index
- [x] `r-match="variable"` - Pattern matching
- [x] `r-when="value"` - Match case
- [x] `r-default` - Default case

### 5. Component System
- [x] Component files in `components/`
- [x] Component rendering with `r-component="Name"`
- [x] Props passing as HTML attributes
- [x] Component-scoped CSS
- [x] CSS scoping with data attributes

### 6. SSR & Server
- [x] Server-side rendering with Axum
- [x] Async handlers
- [x] Variable interpolation `{expression}`
- [x] Context passing to subroutes
- [x] Shared layout/page state

### 7. Developer Experience
- [x] Hot reload on file changes
- [x] Browser auto-refresh (tower-livereload)
- [x] Template reloading without restart
- [x] File watching for pages/components/src
- [x] Basic error pages (404/500)

### 8. Build & Deployment
- [x] Single binary compilation
- [x] No runtime dependencies

**Location References:**
- `src/router.rs:321` - Route matching engine
- `src/template_loader.rs:330` - Template loading
- `src/renderer.rs:744` - Rendering pipeline
- `src/hot_reload.rs:104` - Hot reload system
- `src/parser/directive.rs` - Directive parsing
- `src/parser/css.rs` - CSS scoping

---

## 📝 Notes

### Current Blockers
1. **No data fetching** - Pages can't load their own data
2. **No query params** - Can't read `?filter=active`
3. **No form handling** - Only GET requests supported
4. **No request context** - Can't access cookies/headers
5. **Hardcoded demo data** - See `main.rs:247-298`

### Known Issues
- Route matching is case-sensitive (should be configurable)
- `rhtml.toml` exists but is empty
- No middleware system
- Error pages are hardcoded HTML
- Components can't pass structured props (only string attributes)

### Design Decisions
- SSR-only (use HTMX/Alpine for client interactivity)
- Runtime processing (no build-time codegen yet)
- Axum-only (no multi-framework support yet)
- File-based routing (no manual route registration)

---

## 🎯 Success Criteria

### MVP (Minimum Viable Product)
✅ File-based routing
✅ Layouts and components
✅ Basic directives (if/for/match)
✅ Hot reload
❌ Data fetching in pages
❌ Query parameters
❌ Form handling
❌ Request context

### v0.1.0 (Production Ready)
- [ ] All MVP features
- [ ] Content negotiation (HTML/JSON)
- [ ] Essential directives (r-attr, r-class, r-props)
- [ ] Configuration system
- [ ] Documentation

### v0.2.0 (Feature Complete)
- [ ] All v0.1.0 features
- [ ] Catch-all routes
- [ ] Theme system
- [ ] Middleware
- [ ] CLI tool

### v1.0.0 (Stable)
- [ ] All v0.2.0 features
- [ ] Theme ecosystem (5+ themes)
- [ ] Production deployments
- [ ] Comprehensive docs
- [ ] Community adoption

---

## 📚 References

- **Feature Audit:** See `FEATURE_AUDIT.md`
- **Roadmap:** See `ROADMAP.md`
- **Documentation:** See `README.md`
- **PDF Spec:** See `Vision.pdf`
