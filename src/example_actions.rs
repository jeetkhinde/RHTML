// File: src/example_actions.rs
// Purpose: Example action implementations for /examples/actions-validation
// This demonstrates how actions work with validation and form helpers

use crate::action_executor::{ActionResult, deserialize_form};
use crate::request_context::RequestContext;
use serde::{Deserialize, Serialize};

/// Example User struct (used for demonstration)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub age: i32,
    pub bio: Option<String>,
    pub username: String,
}

/// Create user request (with validation attributes processed by macro)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub age: i32,
    pub bio: Option<String>,
    pub username: String,
    pub website: Option<String>,
}

/// Update user request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub age: Option<i32>,
    pub bio: Option<String>,
}

/// Search request with query parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchUsersRequest {
    pub filter: Option<String>,
    pub page: Option<i32>,
}

/// Mock database functions
pub mod db {
    use super::*;

    pub fn get_users() -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@company.com".to_string(),
                age: 30,
                bio: Some("Software Engineer".to_string()),
                username: "alice".to_string(),
            },
            User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@company.com".to_string(),
                age: 25,
                bio: None,
                username: "bob".to_string(),
            },
        ]
    }

    pub fn create_user(req: CreateUserRequest) -> User {
        User {
            id: 999,
            name: req.name,
            email: req.email,
            age: req.age,
            bio: req.bio,
            username: req.username,
        }
    }

    pub fn count_users() -> i32 {
        get_users().len() as i32
    }
}

/// GET /examples/actions-validation
pub async fn get_actions_validation(_ctx: RequestContext) -> ActionResult {
    // For now, just return HTML indicating we're rendering the page
    // In a real implementation, this would use query params for filtering
    ActionResult::Html {
        content: "<p>GET /examples/actions-validation - Users page loaded</p>".to_string(),
        headers: Default::default(),
    }
}

/// POST /examples/actions-validation - Create a user
pub async fn post_actions_validation(ctx: RequestContext) -> ActionResult {
    // Deserialize form data into CreateUserRequest
    let req: CreateUserRequest = match deserialize_form(&ctx.form) {
        Ok(r) => r,
        Err(e) => {
            return ActionResult::Error {
                status: 400,
                message: format!("Failed to parse form data: {}", e),
            };
        }
    };

    // TODO: Validate request using Validate trait
    // For now, just create the user
    let user = db::create_user(req);
    let user_count = db::count_users();

    // Return HTML with toast and OOB update
    let response_html = format!(
        r#"<div class="user-card" id="user-{}">
            <h3>{} (@{})</h3>
            <p>Email: {}</p>
            <p>Age: {}</p>
        </div>"#,
        user.id, user.name, user.username, user.email, user.age
    );

    // Build response with HX-Trigger header for toast
    let mut headers = axum::http::HeaderMap::new();
    let trigger = serde_json::json!({
        "showToast": {
            "message": "User created!"
        }
    });
    if let Ok(value) = trigger.to_string().parse() {
        headers.insert("HX-Trigger", value);
    }

    // Add OOB update for user count
    let oob_html = format!(
        r#"<div id="user-count" hx-swap-oob="true">{}</div>"#,
        user_count
    );

    ActionResult::Html {
        content: format!("{}\n{}", response_html, oob_html),
        headers,
    }
}

/// PATCH /examples/actions-validation/:id - Update a user
pub async fn patch_actions_validation(_ctx: RequestContext) -> ActionResult {
    ActionResult::Html {
        content: "<p>PATCH /examples/actions-validation - User updated</p>".to_string(),
        headers: Default::default(),
    }
}

/// DELETE /examples/actions-validation/:id - Delete a user
pub async fn delete_actions_validation(_ctx: RequestContext) -> ActionResult {
    let count = db::count_users() - 1;

    // Return only OOB update
    let oob_html = format!(
        r#"<div id="user-count" hx-swap-oob="true">{}</div>"#,
        count
    );

    let mut headers = axum::http::HeaderMap::new();
    let trigger = serde_json::json!({
        "showToast": {
            "message": "User deleted!"
        }
    });
    if let Ok(value) = trigger.to_string().parse() {
        headers.insert("HX-Trigger", value);
    }

    ActionResult::Html {
        content: oob_html,
        headers,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_database() {
        let users = db::get_users();
        assert_eq!(users.len(), 2);
        assert_eq!(users[0].name, "Alice");
    }

    #[test]
    fn test_create_user() {
        let req = CreateUserRequest {
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
            password: "SecurePass123!".to_string(),
            age: 28,
            bio: Some("Developer".to_string()),
            username: "charlie".to_string(),
            website: None,
        };

        let user = db::create_user(req);
        assert_eq!(user.name, "Charlie");
        assert_eq!(user.email, "charlie@example.com");
    }
}
