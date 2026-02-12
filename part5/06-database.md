---
layout: default
title: Database
parent: Part 5 - Patterns
nav_order: 6
---

# Database Access

Compile-time checked SQL queries with sqlx.

## Overview

Database access in Rust emphasizes type safety and compile-time verification. sqlx is the leading async database library, offering compile-time SQL checking without requiring an ORM.

```mermaid
flowchart TB
    subgraph "sqlx Features"
        CT[Compile-Time Checking]
        AS[Async Support]
        TS[Type Safety]
        MB[Multi-Backend]
    end

    CT --> CT1["Verifies SQL at build<br/>Catches typos early"]
    AS --> AS1["tokio/async-std<br/>Non-blocking I/O"]
    TS --> TS1["FromRow derive<br/>Query macros"]
    MB --> MB1["PostgreSQL<br/>MySQL<br/>SQLite"]

    style CT fill:#c8e6c9
    style TS fill:#e3f2fd
```

## When to Use sqlx

```mermaid
flowchart TD
    A[Database Access] --> B{ORM or Raw SQL?}

    B -->|"Raw SQL control"| C[sqlx]
    B -->|"Full ORM"| D[Diesel or SeaORM]

    C --> C1{Compile-time<br/>checking needed?}
    C1 -->|Yes| E["query! macros"]
    C1 -->|No| F["QueryBuilder"]

    style C fill:#c8e6c9
    style E fill:#e3f2fd
```

{: .best-practice }
> **Database Access Strategy:**
> - Use **sqlx** when you want SQL control with compile-time safety
> - Use **Diesel** when you want a full ORM with migrations
> - Use **SeaORM** when you want async ORM capabilities
> - Always use connection pools in production

## sqlx Overview

sqlx provides:
- Compile-time query verification
- Async support
- Type-safe results
- Multiple database backends (PostgreSQL, MySQL, SQLite)

Add to Cargo.toml:
```toml
[dependencies]
sqlx = { version = "0.7", features = ["runtime-tokio", "sqlite", "postgres"] }
tokio = { version = "1", features = ["full"] }
```

## Connection Pool

```rust
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://database.db").await?;

    // Use pool for queries
    Ok(())
}
```

For PostgreSQL:
```rust
use sqlx::postgres::PgPoolOptions;

let pool = PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://user:pass@localhost/db").await?;
```

## Basic Queries

### Fetch One

```rust
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
}

async fn get_user(pool: &SqlitePool, id: i64) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE id = ?",
        id
    )
    .fetch_one(pool)
    .await
}
```

### Fetch All

```rust
async fn list_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await
}
```

### Fetch Optional

```rust
async fn find_user_by_email(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as!(
        User,
        "SELECT id, name, email FROM users WHERE email = ?",
        email
    )
    .fetch_optional(pool)
    .await
}
```

## Insert, Update, Delete

### Insert

```rust
async fn create_user(
    pool: &SqlitePool,
    name: &str,
    email: &str,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        name,
        email
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}
```

### Update

```rust
async fn update_user(
    pool: &SqlitePool,
    id: i64,
    name: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!(
        "UPDATE users SET name = ? WHERE id = ?",
        name,
        id
    )
    .execute(pool)
    .await?;

    Ok(result.rows_affected() > 0)
}
```

### Delete

```rust
async fn delete_user(pool: &SqlitePool, id: i64) -> Result<bool, sqlx::Error> {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() > 0)
}
```

## Transactions

```rust
async fn transfer_funds(
    pool: &SqlitePool,
    from_id: i64,
    to_id: i64,
    amount: f64,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;

    sqlx::query!(
        "UPDATE accounts SET balance = balance - ? WHERE id = ?",
        amount,
        from_id
    )
    .execute(&mut *tx)
    .await?;

    sqlx::query!(
        "UPDATE accounts SET balance = balance + ? WHERE id = ?",
        amount,
        to_id
    )
    .execute(&mut *tx)
    .await?;

    tx.commit().await?;
    Ok(())
}
```

## Migrations

Create migrations:
```bash
sqlx migrate add create_users_table
```

Migration file (`migrations/20231001_create_users_table.sql`):
```sql
CREATE TABLE users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
```

Run migrations:
```rust
sqlx::migrate!("./migrations")
    .run(&pool)
    .await?;
```

## Compile-Time Verification

Set the database URL for compile-time checking:
```bash
export DATABASE_URL="sqlite://database.db"
```

Or create `.env` file:
```
DATABASE_URL=sqlite://database.db
```

## Dynamic Queries

When you need dynamic queries (compile-time checking not possible):

```rust
use sqlx::{QueryBuilder, Sqlite};

async fn search_users(
    pool: &SqlitePool,
    name: Option<&str>,
    email: Option<&str>,
) -> Result<Vec<User>, sqlx::Error> {
    let mut builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT id, name, email FROM users WHERE 1=1"
    );

    if let Some(name) = name {
        builder.push(" AND name LIKE ");
        builder.push_bind(format!("%{}%", name));
    }

    if let Some(email) = email {
        builder.push(" AND email = ");
        builder.push_bind(email);
    }

    builder
        .build_query_as::<User>()
        .fetch_all(pool)
        .await
}
```

## Connection with axum

```rust
use axum::{extract::State, routing::get, Router, Json};
use sqlx::SqlitePool;
use std::sync::Arc;

struct AppState {
    pool: SqlitePool,
}

async fn list_users(
    State(state): State<Arc<AppState>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite://database.db").await.unwrap();

    let state = Arc::new(AppState { pool });

    let app = Router::new()
        .route("/users", get(list_users))
        .with_state(state);

    // Start server...
}
```

## Query Execution Flow

```mermaid
flowchart LR
    subgraph "Query Lifecycle"
        Q[Query] --> P[Pool]
        P --> C[Connection]
        C --> E[Execute]
        E --> R[Results]
    end

    Q --> Q1["query! or query_as!"]
    P --> P1["Connection pool<br/>Manages lifecycle"]
    R --> R1["fetch_one/all/optional"]

    style Q fill:#c8e6c9
    style P fill:#e3f2fd
```

## Best Practices

{: .best-practice }
> **Database Access Guidelines:**
> 1. **Use connection pools** - never create connections per request
> 2. **Use query macros** for compile-time verification
> 3. **Handle migrations** at startup
> 4. **Use transactions** for multi-step operations
> 5. **Set appropriate pool sizes** based on workload
> 6. **Use `fetch_optional`** when rows might not exist
> 7. **Leverage `FromRow`** derive for clean mapping

## Common Mistakes

{: .warning }
> **Avoid these database anti-patterns:**
> - Creating connections per request (use pools!)
> - N+1 query problems (batch or join queries)
> - Not using transactions for related operations
> - Ignoring connection pool limits
> - Storing `DATABASE_URL` in code (use environment variables)

## Summary

| Feature | Method |
|---------|--------|
| Single row | `fetch_one()` |
| Optional row | `fetch_optional()` |
| Multiple rows | `fetch_all()` |
| Execute | `execute()` |
| Transaction | `pool.begin()` |

## See Also

- [Web Services]({% link part5/05-web-services.md %}) - Integrating with web frameworks
- [Database Libraries]({% link appendices/libraries/database.md %}) - Comprehensive database library reference
- [Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/main/examples/part5/database)

## Next Steps

Learn about [Serialization]({% link part5/07-serialization.md %}) with serde.
