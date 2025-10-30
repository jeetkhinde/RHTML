use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rhtml_app::{Renderer, TemplateLoader};
use std::sync::Arc;

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    template_loader: Arc<TemplateLoader>,
    renderer: Arc<Renderer>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 RHTML App Starting...");

    // Load all templates
    let mut loader = TemplateLoader::new("pages");
    match loader.load_all() {
        Ok(_) => {
            println!("✅ Loaded {} templates", loader.count());
            println!("📋 Routes:");
            for route in loader.list_routes() {
                println!("   {} -> template", route);
            }
        }
        Err(e) => {
            eprintln!("❌ Failed to load templates: {}", e);
            std::process::exit(1);
        }
    }

    // Create renderer
    let renderer = Renderer::new();

    // Setup application state
    let state = AppState {
        template_loader: Arc::new(loader),
        renderer: Arc::new(renderer),
    };

    // Build router
    let app = Router::new()
        .route("/", get(index_handler))
        .route("/*path", get(template_handler))
        .with_state(state);

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server running at http://localhost:3000");
    println!("🎯 Try visiting: http://localhost:3000/\n");

    axum::serve(listener, app).await.unwrap();
}

/// Handler for home page "/"
async fn index_handler(State(state): State<AppState>) -> Response {
    render_route(&state, "/").await
}

/// Handler for all other routes
async fn template_handler(
    State(state): State<AppState>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> Response {
    let route = format!("/{}", path);
    render_route(&state, &route).await
}

/// Render a route with layout
async fn render_route(state: &AppState, route: &str) -> Response {
    // Get the page template
    let page_template = match state.template_loader.get(route) {
        Some(t) => t,
        None => {
            return error_response(
                404,
                "Page Not Found",
                &format!("Route '{}' not found", route),
            );
        }
    };

    // Get the layout template
    let layout_template = match state.template_loader.get_layout() {
        Some(t) => t,
        None => {
            return error_response(
                500,
                "Layout Not Found",
                "Missing _layout.rhtml in pages directory",
            );
        }
    };

    // Render the page with layout
    match state
        .renderer
        .render_with_layout(&layout_template.content, &page_template.content)
    {
        Ok(html) => Html(html).into_response(),
        Err(e) => error_response(500, "Render Error", &format!("{}", e)),
    }
}

/// Create an error response
fn error_response(status: u16, title: &str, message: &str) -> Response {
    let html = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>{title}</title>
            <script src="https://cdn.tailwindcss.com"></script>
        </head>
        <body class="bg-gray-100">
            <div class="min-h-screen flex items-center justify-center">
                <div class="bg-white rounded-lg shadow-lg p-8 max-w-md">
                    <h1 class="text-4xl font-bold text-red-600 mb-4">{status}</h1>
                    <h2 class="text-2xl font-semibold text-gray-800 mb-2">{title}</h2>
                    <p class="text-gray-600">{message}</p>
                    <a href="/" class="mt-4 inline-block bg-blue-600 text-white px-4 py-2 rounded hover:bg-blue-700">
                        Go Home
                    </a>
                </div>
            </div>
        </body>
        </html>
        "#,
        status = status,
        title = title,
        message = message
    );

    (
        axum::http::StatusCode::from_u16(status).unwrap(),
        Html(html),
    )
        .into_response()
}
