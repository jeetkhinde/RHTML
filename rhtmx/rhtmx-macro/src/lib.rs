// RHTMX Procedural Macros
// Provides compile-time HTML generation and HTTP routing macros

use proc_macro::TokenStream;
use quote::quote;


mod html;
mod http;

/// The html! macro for compile-time HTML generation
///
/// Parses JSX-like syntax and generates efficient Rust code with r-directives support.
///
/// # Example
///
/// ```ignore
/// fn user_card(user: &User) -> Html {
///     html! {
///         <div class="card">
///             <h3>{user.name}</h3>
///             <p>{user.email}</p>
///         </div>
///     }
/// }
/// ```
///
/// # R-Directives
///
/// - `r-for="item in items"` - Loop over collections
/// - `r-for="(i, item) in items"` - Loop with index
/// - `r-if="condition"` - Conditional rendering
///
#[proc_macro]
pub fn html(input: TokenStream) -> TokenStream {
    let input_str = input.to_string();

    let mut parser = html::HtmlParser::new(input_str);
    let nodes = match parser.parse() {
        Ok(nodes) => nodes,
        Err(e) => return e.to_compile_error().into(),
    };

    let output = html::CodeGenerator::generate(nodes);
    output.into()
}

/// The css! macro for scoped CSS generation
///
/// Generates scoped CSS with automatic class prefixing.
///
/// # Example
///
/// ```ignore
/// css! {
///     .card {
///         border: 1px solid #ccc;
///         padding: 1rem;
///     }
///     .title {
///         font-size: 1.5rem;
///     }
/// }
/// ```
#[proc_macro]
pub fn css(input: TokenStream) -> TokenStream {
    // For now, just return the input as a string
    // TODO: Implement proper scoping and hash generation
    let input_str = input.to_string();

    quote! {
        {
            // TODO: Add scoping logic
            #input_str
        }
    }.into()
}

/// HTTP GET handler macro
///
/// Marks a function as a GET request handler. When used with file-based routing,
/// the route is determined by the file location.
///
/// # Example
///
/// ```ignore
/// // File: pages/users.rs
/// #[get]  // Handles GET /users
/// fn index() -> OkResponse {
///     let users = db::get_users()?;
///     Ok().render(users_list, users)
/// }
///
/// #[get("partial=stats")]  // Handles GET /users?partial=stats
/// fn stats() -> OkResponse {
///     Ok().render(stats_component, get_stats())
/// }
/// ```
#[proc_macro_attribute]
pub fn get(args: TokenStream, input: TokenStream) -> TokenStream {
    http::http_handler("GET", args, input)
}

/// HTTP POST handler macro
///
/// # Example
///
/// ```ignore
/// #[post]
/// fn create(req: CreateUserRequest) -> OkResponse {
///     let user = db::create_user(req)?;
///     Ok().render(user_card, user)
///         .toast("User created!")
/// }
/// ```
#[proc_macro_attribute]
pub fn post(args: TokenStream, input: TokenStream) -> TokenStream {
    http::http_handler("POST", args, input)
}

/// HTTP PUT handler macro
///
/// # Example
///
/// ```ignore
/// #[put(":id")]
/// fn update(id: i32, req: UpdateUserRequest) -> OkResponse {
///     let user = db::update_user(id, req)?;
///     Ok().render(user_card, user)
///         .toast("User updated!")
/// }
/// ```
#[proc_macro_attribute]
pub fn put(args: TokenStream, input: TokenStream) -> TokenStream {
    http::http_handler("PUT", args, input)
}

/// HTTP PATCH handler macro
///
/// # Example
///
/// ```ignore
/// #[patch(":id")]
/// fn partial_update(id: i32, req: PatchUserRequest) -> OkResponse {
///     let user = db::patch_user(id, req)?;
///     Ok().render(user_card, user)
/// }
/// ```
#[proc_macro_attribute]
pub fn patch(args: TokenStream, input: TokenStream) -> TokenStream {
    http::http_handler("PATCH", args, input)
}

/// HTTP DELETE handler macro
///
/// # Example
///
/// ```ignore
/// #[delete(":id")]
/// fn delete(id: i32) -> OkResponse {
///     db::delete_user(id)?;
///     Ok().toast("User deleted!")
/// }
/// ```
#[proc_macro_attribute]
pub fn delete(args: TokenStream, input: TokenStream) -> TokenStream {
    http::http_handler("DELETE", args, input)
}
