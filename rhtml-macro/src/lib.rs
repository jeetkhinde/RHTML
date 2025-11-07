// File: rhtml-macro/src/lib.rs
// Purpose: Procedural macros for RHTML framework

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

mod layout;
mod layout_registry;
mod layout_resolver;
mod slot;

/// The #[webpage] attribute macro for defining pages
///
/// # Example
///
/// ```ignore
/// #[webpage]
/// pub fn users(props: UsersProps) {
///     <div class="container">
///         <h1>Users</h1>
///         <div r-for="user in props.data">
///             <user_card user={user} />
///         </div>
///     </div>
/// }
/// ```
///
/// This gets transformed into:
///
/// ```ignore
/// WebPage {
///     <div class="container">
///         <h1>Users</h1>
///         <div r-for="user in props.data">
///             <user_card user={user} />
///         </div>
///     </div>
/// }
/// ```
#[proc_macro_attribute]
pub fn webpage(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(item as ItemFn);

    // Extract the function body
    let body = &input_fn.block;

    // Transform to WebPage syntax
    // The body already contains the HTML/RHTML tokens
    let output = quote! {
        WebPage #body
    };

    output.into()
}

/// The #[layout] attribute macro for defining layouts
///
/// # Example
///
/// ```ignore
/// pub struct LayoutSlots {
///     pub content: impl Render,
///     pub title: &str,
///     pub description: Option<&str>,
/// }
///
/// #[layout]
/// pub fn layout(slots: LayoutSlots) {
///     <!DOCTYPE html>
///     <html>
///       <head><title>{slots.title}</title></head>
///       <body>
///         <main>{slots.content}</main>
///       </body>
///     </html>
/// }
/// ```
#[proc_macro_attribute]
pub fn layout(attr: TokenStream, item: TokenStream) -> TokenStream {
    layout::process_layout_macro(attr, item)
}

/// The slot! macro for providing slot values to layouts
///
/// # Example
///
/// ```ignore
/// slot! {
///     title: "Home Page",
///     description: "Welcome to our site"
/// }
///
/// #[webpage]
/// pub fn page(props: PageProps) {
///     <div>Page content</div>
/// }
/// ```
#[proc_macro]
pub fn slot(input: TokenStream) -> TokenStream {
    slot::process_slot_macro(input)
}

/// The #[component] attribute macro for defining reusable components
///
/// # Example
///
/// ```ignore
/// #[component]
/// pub fn button(props: ButtonProps) {
///     <button class="btn">{props.label}</button>
/// }
/// ```
#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // For now, components work the same as webpages
    // Just pass through - similar to #[webpage]
    let input_fn = parse_macro_input!(item as ItemFn);
    let body = &input_fn.block;

    let output = quote! {
        Component #body
    };

    output.into()
}
