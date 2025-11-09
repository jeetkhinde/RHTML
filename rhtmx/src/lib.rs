// RHTMX - Rust + HTMX Framework
// Compile-time HTML generation with type safety and zero runtime overhead

pub mod html;

// Re-export the html! macro from rhtmx-macro
pub use rhtmx_macro::{html, css, get, post, put, patch, delete};

// Re-export core types and response builders
pub use html::{
    Html,
    OkResponse, ErrorResponse, RedirectResponse,
    Ok, Error, Redirect,
    ok, error, redirect,
};

// Re-export commonly used types from dependencies
pub use axum;
pub use axum::http::StatusCode;
