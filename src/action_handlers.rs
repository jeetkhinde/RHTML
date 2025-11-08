// File: src/action_handlers.rs
// Purpose: Manual registration of action handlers for different routes
// This will be replaced by a proc macro system in the future

use crate::action_executor::ActionResult;
use crate::request_context::RequestContext;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;

/// Type alias for an action handler function
pub type ActionHandler = fn(RequestContext) -> Pin<Box<dyn Future<Output = ActionResult> + Send>>;

/// Registry for action handlers
pub struct ActionHandlerRegistry {
    handlers: HashMap<String, HashMap<String, ActionHandler>>,
}

impl ActionHandlerRegistry {
    /// Create a new action handler registry
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    /// Register an action handler for a route and method
    pub fn register(&mut self, route: &str, method: &str, handler: ActionHandler) {
        self.handlers
            .entry(route.to_string())
            .or_insert_with(HashMap::new)
            .insert(method.to_uppercase(), handler);
    }

    /// Find an action handler
    pub fn find(&self, route: &str, method: &str) -> Option<ActionHandler> {
        self.handlers
            .get(route)
            .and_then(|methods| methods.get(&method.to_uppercase()).copied())
    }

    /// Check if a route has an action
    pub fn has_action(&self, route: &str, method: &str) -> bool {
        self.handlers
            .get(route)
            .map(|methods| methods.contains_key(&method.to_uppercase()))
            .unwrap_or(false)
    }
}

impl Default for ActionHandlerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Register all built-in action handlers
pub fn register_built_in_handlers(registry: &mut ActionHandlerRegistry) {
    use crate::example_actions;

    // Register example actions
    registry.register(
        "/examples/actions-validation",
        "GET",
        |ctx| Box::pin(example_actions::get_actions_validation(ctx)),
    );

    registry.register(
        "/examples/actions-validation",
        "POST",
        |ctx| Box::pin(example_actions::post_actions_validation(ctx)),
    );

    registry.register(
        "/examples/actions-validation",
        "PATCH",
        |ctx| Box::pin(example_actions::patch_actions_validation(ctx)),
    );

    registry.register(
        "/examples/actions-validation",
        "DELETE",
        |ctx| Box::pin(example_actions::delete_actions_validation(ctx)),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_handler_registry() {
        let mut registry = ActionHandlerRegistry::new();

        // Create a dummy handler
        let handler: ActionHandler = |_ctx| Box::pin(async {
            ActionResult::Html {
                content: "test".to_string(),
                headers: Default::default(),
            }
        });

        registry.register("/test", "GET", handler);

        assert!(registry.has_action("/test", "GET"));
        assert!(!registry.has_action("/test", "POST"));
        assert!(registry.find("/test", "get").is_some());
    }
}
