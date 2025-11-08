pub mod action_executor;
pub mod action_handlers;
pub mod actions;
pub mod component;
pub mod config;
pub mod example_actions;
pub mod hot_reload;
pub mod renderer;
pub mod request_context;
pub mod template_loader;
pub mod validation;

// Re-export router from rhtml-router crate
pub use rhtml_router::{Route, RouteMatch, Router};

pub use action_executor::{deserialize_form, validate_request, ActionResult, form_to_json};
pub use action_handlers::{ActionHandler, ActionHandlerRegistry, register_built_in_handlers};
pub use actions::{ActionInfo, ActionMethod, ActionRegistry, ActionResponse, Empty, ResultExt};
pub use component::{Component, ComponentRegistry, get_component, register_component};
pub use config::Config;
pub use renderer::{LayoutDirective, Renderer};
pub use request_context::{FormData, QueryParams, RequestContext};
pub use rhtml_parser::{DirectiveParser, ExpressionEvaluator};
pub use template_loader::{Template, TemplateLoader};
pub use validation::{Validate, ValidationResult};
