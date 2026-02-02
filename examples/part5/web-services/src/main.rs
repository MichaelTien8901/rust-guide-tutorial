//! Web Services Example
//!
//! Demonstrates web service patterns with axum.
//!
//! # Request Flow
//! ```text
//!     ┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
//!     │  Client  │────►│  Router  │────►│  Handler │────►│ Response │
//!     └──────────┘     └──────────┘     └──────────┘     └──────────┘
//!                           │
//!                           ▼
//!                    ┌──────────────┐
//!                    │  Middleware  │
//!                    │  (logging,   │
//!                    │   auth, etc) │
//!                    └──────────────┘
//! ```

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

// ============================================
// Data Models
// ============================================

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ListParams {
    #[serde(default)]
    limit: Option<usize>,
    #[serde(default)]
    offset: Option<usize>,
}

// ============================================
// Application State
// ============================================

#[derive(Clone)]
struct AppState {
    users: Arc<RwLock<HashMap<u64, User>>>,
    next_id: Arc<RwLock<u64>>,
}

impl AppState {
    fn new() -> Self {
        AppState {
            users: Arc::new(RwLock::new(HashMap::new())),
            next_id: Arc::new(RwLock::new(1)),
        }
    }
}

// ============================================
// Response Types
// ============================================

#[derive(Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
}

impl<T: Serialize> ApiResponse<T> {
    fn success(data: T) -> Json<Self> {
        Json(ApiResponse {
            success: true,
            data: Some(data),
            error: None,
        })
    }

    fn error(message: impl Into<String>) -> Json<Self> {
        Json(ApiResponse {
            success: false,
            data: None,
            error: Some(message.into()),
        })
    }
}

// ============================================
// Handlers
// ============================================

/// GET /health - Health check endpoint
async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({
        "status": "healthy",
        "version": "1.0.0"
    }))
}

/// GET /users - List all users
async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<ListParams>,
) -> impl IntoResponse {
    let users = state.users.read().unwrap();
    let mut user_list: Vec<User> = users.values().cloned().collect();

    // Apply pagination
    let offset = params.offset.unwrap_or(0);
    let limit = params.limit.unwrap_or(10);

    user_list.sort_by_key(|u| u.id);
    let paginated: Vec<User> = user_list
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    ApiResponse::success(paginated)
}

/// GET /users/:id - Get a specific user
async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let users = state.users.read().unwrap();

    match users.get(&id) {
        Some(user) => (StatusCode::OK, ApiResponse::success(user.clone())),
        None => (
            StatusCode::NOT_FOUND,
            ApiResponse::error(format!("User {} not found", id)),
        ),
    }
}

/// POST /users - Create a new user
async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> impl IntoResponse {
    // Validate input
    if input.name.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            ApiResponse::<User>::error("Name is required"),
        );
    }

    if !input.email.contains('@') {
        return (
            StatusCode::BAD_REQUEST,
            ApiResponse::<User>::error("Invalid email format"),
        );
    }

    // Generate ID and create user
    let id = {
        let mut next_id = state.next_id.write().unwrap();
        let id = *next_id;
        *next_id += 1;
        id
    };

    let user = User {
        id,
        name: input.name,
        email: input.email,
    };

    state.users.write().unwrap().insert(id, user.clone());

    (StatusCode::CREATED, ApiResponse::success(user))
}

/// PUT /users/:id - Update a user
async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateUser>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();

    match users.get_mut(&id) {
        Some(user) => {
            if let Some(name) = input.name {
                user.name = name;
            }
            if let Some(email) = input.email {
                user.email = email;
            }
            (StatusCode::OK, ApiResponse::success(user.clone()))
        }
        None => (
            StatusCode::NOT_FOUND,
            ApiResponse::error(format!("User {} not found", id)),
        ),
    }
}

/// DELETE /users/:id - Delete a user
async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    let mut users = state.users.write().unwrap();

    match users.remove(&id) {
        Some(user) => (StatusCode::OK, ApiResponse::success(user)),
        None => (
            StatusCode::NOT_FOUND,
            ApiResponse::<User>::error(format!("User {} not found", id)),
        ),
    }
}

// ============================================
// Router Setup
// ============================================

fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        // User CRUD routes
        .route("/users", get(list_users).post(create_user))
        .route(
            "/users/:id",
            get(get_user).put(update_user).delete(delete_user),
        )
        // Add shared state
        .with_state(state)
}

// ============================================
// Main Entry Point
// ============================================

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    println!("=== Web Services Example ===\n");

    // Create application state
    let state = AppState::new();

    // Seed some initial data
    {
        let mut users = state.users.write().unwrap();
        users.insert(1, User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        });
        users.insert(2, User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        });
        *state.next_id.write().unwrap() = 3;
    }

    // Create router
    let app = create_router(state);

    // Print API documentation
    println!("  API Endpoints:");
    println!("    GET    /health       - Health check");
    println!("    GET    /users        - List all users");
    println!("    POST   /users        - Create a user");
    println!("    GET    /users/:id    - Get a user");
    println!("    PUT    /users/:id    - Update a user");
    println!("    DELETE /users/:id    - Delete a user");
    println!();

    // In a real application, you would run the server:
    // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    // axum::serve(listener, app).await.unwrap();

    // For demonstration, we'll simulate some requests
    println!("  Simulating requests...\n");
    simulate_requests(app).await;
}

// ============================================
// Request Simulation (for demonstration)
// ============================================

async fn simulate_requests(app: Router) {
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    // Helper to make requests
    async fn make_request(app: &Router, method: &str, uri: &str, body: Option<&str>) {
        let body = body.map(|s| Body::from(s.to_string())).unwrap_or(Body::empty());

        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header("content-type", "application/json")
            .body(body)
            .unwrap();

        let response = app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let body_str = String::from_utf8_lossy(&body);

        println!("  {} {} -> {}", method, uri, status);
        println!("    Response: {}", body_str);
        println!();
    }

    // GET /health
    make_request(&app, "GET", "/health", None).await;

    // GET /users
    make_request(&app, "GET", "/users", None).await;

    // POST /users
    make_request(
        &app,
        "POST",
        "/users",
        Some(r#"{"name": "Charlie", "email": "charlie@example.com"}"#),
    ).await;

    // GET /users/3
    make_request(&app, "GET", "/users/3", None).await;

    // PUT /users/3
    make_request(
        &app,
        "PUT",
        "/users/3",
        Some(r#"{"name": "Charles"}"#),
    ).await;

    // DELETE /users/3
    make_request(&app, "DELETE", "/users/3", None).await;

    // GET /users/3 (after delete)
    make_request(&app, "GET", "/users/3", None).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    fn create_test_app() -> Router {
        let state = AppState::new();
        state.users.write().unwrap().insert(1, User {
            id: 1,
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
        });
        *state.next_id.write().unwrap() = 2;
        create_router(state)
    }

    #[tokio::test]
    async fn test_health_check() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_list_users() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: ApiResponse<Vec<User>> = serde_json::from_slice(&body).unwrap();
        assert!(json.success);
        assert_eq!(json.data.unwrap().len(), 1);
    }

    #[tokio::test]
    async fn test_create_user() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/users")
                    .header("content-type", "application/json")
                    .body(Body::from(r#"{"name": "New User", "email": "new@example.com"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);
    }

    #[tokio::test]
    async fn test_get_user_not_found() {
        let app = create_test_app();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/users/999")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
