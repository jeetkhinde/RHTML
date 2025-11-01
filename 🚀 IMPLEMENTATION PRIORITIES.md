🚀 IMPLEMENTATION PRIORITIES
To reach the PDF vision, you need:
Phase 1: Data Layer (Most Critical)
✅ Add data fn parsing and execution
✅ Add query parameter extraction
✅ Add form handling (POST/PUT/DELETE)
✅ Type PageProps with actual data: PageProps<T>
Phase 2: Directives (Essential)
✅ Implement r-attr (dynamic attributes)
✅ Implement r-class (conditional classes)
✅ Implement r-html (unescaped HTML)
✅ Implement r-props="{...}" syntax
Phase 3: Routing (Nice to Have)
✅ Catch-all routes [...slug]
✅ Route aliases/redirects
✅ Content negotiation (HTML vs JSON)
Phase 4: Config & Themes
✅ Parse rhtml.toml configuration
✅ Theme system with CSS variable injection

This is way more powerful than Next.js because you have:
Full Rust type system
Direct database access
No JSON serialization overhead
Compile-time safety
Would you like me to:
Implement the data fn system first?
Add query parameter support?
Show you how to handle the badge/HTMX case with current implementation?
Create a theming system?
Let me know which priority makes sense for your project!


Scan what complete feature list is pending, i believe in our previous chats we have implemented 8 features. two are pending number #9, and #10.
In this chat, we find out we need to implement or make chanages, Todos like Data layer, Directives, r-attr, r-class, r-html, r-props="{....}" syntax, Routing (nice to have) Catch-all routes [...slug], Route aliases/redirects, content negotiations (html vs json) We should use Check request and return different content - because we are fetching data at one place vs pages/api/users.rhtml we have to fetch data here as well. but we are not limiting the dev, we can use this way if he wants. I prefer data fetch and returning either json or html at single file for a route. less files to explore and understand, and even authenticate, authorise. 
I would prefer that we must implement that routes are case insensitive. and the hugo stuff we just discussed.

⏭️ Feature #9: Form Handling & Validation
⏭️ Feature #10: Production Build
- Case-insenstive urls
- url parms
- 

Start implementing Sprint 1 (data layer)?
Parse data fn from .rhtml files
Extract query parameters
Handle POST/PUT/DELETE requests
Create a ROADMAP.md with detailed sprint plans?
Design the data layer architecture before coding?
Review and prioritize the feature list differently?
Create example implementations for any specific feature?




Todos:
- read them these files 1) todo.md, 2) feature_audit.md, 3) Readme.md, 4) Vision.pdf files.
- Complete Critical priorities mentioned in todo.md file one by one.


instead of partials, how about if router handles localhost:3000/users?cmp=CountOfUsers
CountofUsers is a component defined in users.rthml or pages/users/index.rhmlt file.
when request comes on localhost:3000/users it means load full page with layout, if like this localhost:3000/users?cmp=CountOfUsers load just component? 