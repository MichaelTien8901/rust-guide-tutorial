---
layout: default
title: Web Frameworks
parent: Libraries
grand_parent: Appendices
nav_order: 3
---

# Web Frameworks

Libraries for building web applications and APIs.

## axum

Modern, ergonomic web framework built on tokio.

```toml
[dependencies]
axum = "0.7"
tokio = { version = "1", features = ["full"] }
```

### Basic Server

```rust
use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Handlers and Extractors

```rust
use axum::{
    extract::{Path, Query, State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Params {
    page: Option<u32>,
}

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
}

// Path parameters
async fn get_user(Path(id): Path<u64>) -> impl IntoResponse {
    Json(User { id, name: "Alice".into() })
}

// Query parameters
async fn list_users(Query(params): Query<Params>) -> impl IntoResponse {
    let page = params.page.unwrap_or(1);
    format!("Page: {}", page)
}

// JSON body
async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    (StatusCode::CREATED, Json(user))
}
```

### State and Middleware

```rust
use axum::{middleware, Extension};
use std::sync::Arc;

struct AppState {
    db: Pool,
}

async fn handler(State(state): State<Arc<AppState>>) -> String {
    // Use state.db
    "OK".into()
}

let app = Router::new()
    .route("/", get(handler))
    .with_state(Arc::new(AppState { db: pool }))
    .layer(middleware::from_fn(logging_middleware));
```

## actix-web

High-performance, actor-based web framework.

```toml
[dependencies]
actix-web = "4"
```

### Basic Server

```rust
use actix_web::{web, App, HttpServer, HttpResponse};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Handlers and Extractors

```rust
use actix_web::{web, HttpResponse};

#[derive(Deserialize)]
struct Info {
    name: String,
}

async fn greet(path: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Hello {}!", path))
}

async fn create(info: web::Json<Info>) -> HttpResponse {
    HttpResponse::Created().json(&*info)
}

async fn query(query: web::Query<Info>) -> HttpResponse {
    HttpResponse::Ok().body(format!("Name: {}", query.name))
}
```

### App State

```rust
struct AppState {
    counter: std::sync::Mutex<i32>,
}

async fn increment(data: web::Data<AppState>) -> HttpResponse {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    HttpResponse::Ok().body(format!("Count: {}", counter))
}

let app = App::new()
    .app_data(web::Data::new(AppState {
        counter: std::sync::Mutex::new(0),
    }))
    .route("/", web::get().to(increment));
```

## Rocket

Web framework focusing on ease of use.

```toml
[dependencies]
rocket = "0.5"
```

### Basic Server

```rust
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
```

### Request Guards and Forms

```rust
use rocket::form::Form;
use rocket::serde::json::Json;

#[derive(FromForm)]
struct Login {
    username: String,
    password: String,
}

#[post("/login", data = "<form>")]
fn login(form: Form<Login>) -> String {
    format!("Welcome, {}!", form.username)
}

#[post("/api/user", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    user
}
```

## warp

Composable web framework using filters.

```toml
[dependencies]
warp = "0.3"
tokio = { version = "1", features = ["full"] }
```

### Basic Server

```rust
use warp::Filter;

#[tokio::main]
async fn main() {
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    let routes = warp::get().and(hello);

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}
```

### Filters and Combinators

```rust
use warp::Filter;

let api = warp::path("api");
let v1 = api.and(warp::path("v1"));

let users = v1
    .and(warp::path("users"))
    .and(warp::get())
    .and_then(list_users);

let create = v1
    .and(warp::path("users"))
    .and(warp::post())
    .and(warp::body::json())
    .and_then(create_user);

let routes = users.or(create);
```

## Comparison

| Framework | Performance | Ease of Use | Flexibility |
|-----------|-------------|-------------|-------------|
| axum | Excellent | Good | High |
| actix-web | Excellent | Good | High |
| Rocket | Good | Excellent | Medium |
| warp | Excellent | Medium | High |

## Middleware Ecosystem

### tower (for axum)

```rust
use tower_http::{cors::CorsLayer, trace::TraceLayer};

let app = Router::new()
    .route("/", get(handler))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::permissive());
```

### Common Middleware

| Crate | Purpose |
|-------|---------|
| tower-http | HTTP middleware |
| tower-cookies | Cookie handling |
| axum-extra | Additional extractors |
| actix-cors | CORS for actix |
| actix-session | Sessions for actix |

## Choosing a Framework

| Use Case | Recommendation |
|----------|----------------|
| New projects | axum |
| Maximum performance | actix-web |
| Rapid development | Rocket |
| Composable APIs | warp |

## Summary

| Framework | Runtime | Style |
|-----------|---------|-------|
| axum | tokio | Extractors, Router |
| actix-web | actix | Actors, Handlers |
| Rocket | tokio | Attributes, Guards |
| warp | tokio | Filters, Combinators |
