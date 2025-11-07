// File: rhtml-macro/src/lib.rs
// Purpose: Procedural macro for #[webpage] attribute

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

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
