use axum::{
    extract::State,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use rhtml_app::{Renderer, TemplateLoader};
use rhtml_app::hot_reload::{create_watcher, ChangeType};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_livereload::LiveReloadLayer;
use tracing::{info, error};

/// Application state shared across handlers
#[derive(Clone)]
struct AppState {
    template_loader: Arc<RwLock<TemplateLoader>>,
}

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("🚀 RHTML App Starting...");

    // Check if hot reload is enabled (default: true for development)
    let hot_reload_enabled = std::env::var("HOT_RELOAD")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

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

    // Wrap loader in Arc<RwLock> for thread-safe updates
    let template_loader = Arc::new(RwLock::new(loader));

    // Setup hot reload if enabled
    if hot_reload_enabled {
        println!("🔄 Hot Reload: ENABLED");

        // Create file watcher and spawn template reload task
        match create_watcher() {
            Ok(watcher) => {
                let loader_clone = template_loader.clone();
                let mut reload_rx = watcher.subscribe();

                tokio::spawn(async move {
                    let _watcher = watcher; // Keep watcher alive

                    while let Ok(file_change) = reload_rx.recv().await {
                        match file_change.change_type {
                            ChangeType::Template | ChangeType::Component => {
                                info!("🔄 Reloading template: {:?}", file_change.path);

                                let mut loader = loader_clone.write().await;
                                if let Err(e) = loader.reload_template(&file_change.path) {
                                    error!("❌ Failed to reload template: {}", e);
                                } else {
                                    info!("✅ Template reloaded successfully");
                                }
                            }
                            ChangeType::SourceCode => {
                                info!("⚠️  Source code changed - restart server for changes to take effect");
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("⚠️  Failed to create file watcher: {}", e);
                eprintln!("   Continuing without hot reload...");
            }
        }
    } else {
        println!("🔄 Hot Reload: DISABLED");
    }

    // Setup application state
    let state = AppState {
        template_loader: template_loader.clone(),
    };

    // Build router
    let mut app = Router::new()
        .route("/", get(index_handler))
        .route("/*path", get(template_handler))
        .with_state(state);

    // Add LiveReloadLayer if hot reload is enabled
    // tower-livereload has built-in file watching and will trigger browser reloads automatically
    if hot_reload_enabled {
        app = app.layer(LiveReloadLayer::new());
    }

    // Start server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("✅ Server running at http://localhost:3000");
    if hot_reload_enabled {
        println!("🔥 Hot reload enabled - edit templates and watch them update!");
    }
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
    let loader = state.template_loader.read().await;

    // Use the router to match the route
    let route_match = match loader.router().match_route(route) {
        Some(m) => m,
        None => {
            // Try direct template lookup as fallback
            if loader.get(route).is_some() {
                drop(loader);
                return render_route_direct(state, route).await;
            }
            return error_response(
                404,
                "Page Not Found",
                &format!("Route '{}' not found", route),
            );
        }
    };

    // Try to get template by pattern
    let page_template = loader.get(&route_match.route.pattern)
        .or_else(|| loader.get(route));

    let page_template = match page_template {
        Some(t) => t.clone(),
        None => {
            return error_response(
                404,
                "Template Not Found",
                &format!("Template for route '{}' not found", route),
            );
        }
    };

    // Get the appropriate layout (section-specific or root)
    let layout_template = match loader.get_layout_for_route(&route_match.route.pattern) {
        Some(t) => t.clone(),
        None => {
            return error_response(
                500,
                "Layout Not Found",
                "Missing _layout.rhtml in pages directory",
            );
        }
    };

    // Create Arc wrapper for the locked loader to pass to renderer
    let loader_arc = Arc::new((*loader).clone());
    drop(loader);

    // Create a new renderer for this request with component access
    let mut renderer = Renderer::with_loader(loader_arc);

    // Set route parameters as variables
    for (param_name, param_value) in &route_match.params {
        renderer.set_var(param_name, rhtml_app::parser::expression::Value::String(param_value.clone()));
    }

    // Set up demo data based on route
    setup_demo_data(&mut renderer, route, &route_match.params);

    // Render the page with layout
    match renderer.render_with_layout(&layout_template.content, &page_template.content) {
        Ok(html) => Html(html).into_response(),
        Err(e) => error_response(500, "Render Error", &format!("{}", e)),
    }
}

/// Render a route directly (fallback for old-style routes)
async fn render_route_direct(state: &AppState, route: &str) -> Response {
    let loader = state.template_loader.read().await;

    let page_template = match loader.get(route) {
        Some(t) => t.clone(),
        None => {
            return error_response(
                404,
                "Page Not Found",
                &format!("Route '{}' not found", route),
            );
        }
    };

    let layout_template = match loader.get_layout() {
        Some(t) => t.clone(),
        None => {
            return error_response(
                500,
                "Layout Not Found",
                "Missing _layout.rhtml in pages directory",
            );
        }
    };

    let loader_arc = Arc::new((*loader).clone());
    drop(loader);

    let mut renderer = Renderer::with_loader(loader_arc);
    setup_demo_data(&mut renderer, route, &std::collections::HashMap::new());

    match renderer.render_with_layout(&layout_template.content, &page_template.content) {
        Ok(html) => Html(html).into_response(),
        Err(e) => error_response(500, "Render Error", &format!("{}", e)),
    }
}

/// Setup demo data for specific routes
fn setup_demo_data(renderer: &mut Renderer, route: &str, _params: &std::collections::HashMap<String, String>) {
    use rhtml_app::parser::expression::Value;

    if route == "/loops" {
        // Example 1: Fruits array
        renderer.set_var("fruits", Value::Array(vec![
            Value::String("Apple".to_string()),
            Value::String("Banana".to_string()),
            Value::String("Cherry".to_string()),
            Value::String("Dragon Fruit".to_string()),
        ]));

        // Example 2: Colors array
        renderer.set_var("colors", Value::Array(vec![
            Value::String("Red".to_string()),
            Value::String("Green".to_string()),
            Value::String("Blue".to_string()),
            Value::String("Yellow".to_string()),
        ]));

        // Example 3: Tasks array
        renderer.set_var("tasks", Value::Array(vec![
            Value::String("Implement r-for directive".to_string()),
            Value::String("Create demo page".to_string()),
            Value::String("Test the feature".to_string()),
            Value::String("Write documentation".to_string()),
        ]));

        // Example 4: Numbers array
        renderer.set_var("numbers", Value::Array(vec![
            Value::Number(1.0),
            Value::Number(2.0),
            Value::Number(3.0),
            Value::Number(4.0),
            Value::Number(5.0),
            Value::Number(6.0),
            Value::Number(7.0),
            Value::Number(8.0),
        ]));
    } else if route == "/match" {
        // Example 1: User role
        renderer.set_var("user_role", Value::String("admin".to_string()));

        // Example 2: Order status
        renderer.set_var("order_status", Value::String("shipped".to_string()));

        // Example 3: Payment method
        renderer.set_var("payment_method", Value::String("card".to_string()));

        // Example 4: Theme
        renderer.set_var("theme", Value::String("dark".to_string()));
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
