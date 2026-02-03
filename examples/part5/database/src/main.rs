//! Database Patterns Example
//!
//! Demonstrates database access patterns with sqlx.
//!
//! # Database Operations Flow
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                   SQLx Features                         │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  Compile-time query checking (with sqlx::query!)        │
//!     │  ├── Catches SQL errors at compile time                 │
//!     │  ├── Type-safe column mapping                           │
//!     │  └── IDE autocompletion support                         │
//!     │                                                         │
//!     │  Connection pooling                                     │
//!     │  ├── Automatic connection management                    │
//!     │  ├── Configurable pool size                             │
//!     │  └── Health checking                                    │
//!     │                                                         │
//!     │  Transaction support                                    │
//!     │  ├── ACID guarantees                                    │
//!     │  ├── Automatic rollback on error                        │
//!     │  └── Nested transactions (savepoints)                   │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use sqlx::{sqlite::SqlitePoolOptions, FromRow, Pool, Row, Sqlite};

// ============================================
// Data Models
// ============================================

#[derive(Debug, Clone, FromRow)]
struct User {
    id: i64,
    name: String,
    email: String,
    created_at: String,
}

#[derive(Debug)]
struct CreateUser {
    name: String,
    email: String,
}

#[derive(Debug, Clone, FromRow)]
struct Post {
    id: i64,
    user_id: i64,
    title: String,
    content: String,
    created_at: String,
}

// ============================================
// Repository Pattern
// ============================================

struct UserRepository {
    pool: Pool<Sqlite>,
}

impl UserRepository {
    fn new(pool: Pool<Sqlite>) -> Self {
        UserRepository { pool }
    }

    async fn create(&self, user: CreateUser) -> Result<User, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email, created_at",
        )
        .bind(&user.name)
        .bind(&user.email)
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: result.get("id"),
            name: result.get("name"),
            email: result.get("email"),
            created_at: result.get("created_at"),
        })
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await
    }

    async fn list_all(&self) -> Result<Vec<User>, sqlx::Error> {
        sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY id")
            .fetch_all(&self.pool)
            .await
    }

    async fn update(&self, id: i64, name: &str, email: &str) -> Result<Option<User>, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE users SET name = ?, email = ? WHERE id = ? RETURNING id, name, email, created_at",
        )
        .bind(name)
        .bind(email)
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(result.map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            created_at: row.get("created_at"),
        }))
    }

    async fn delete(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected() > 0)
    }
}

// ============================================
// Database Setup
// ============================================

async fn create_schema(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL DEFAULT (datetime('now'))
        )
        "#,
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT (datetime('now')),
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}

// ============================================
// Transaction Example
// ============================================

async fn create_user_with_post(
    pool: &Pool<Sqlite>,
    name: &str,
    email: &str,
    post_title: &str,
    post_content: &str,
) -> Result<(User, Post), sqlx::Error> {
    let mut tx = pool.begin().await?;

    // Create user
    let user_row = sqlx::query(
        "INSERT INTO users (name, email) VALUES (?, ?) RETURNING id, name, email, created_at",
    )
    .bind(name)
    .bind(email)
    .fetch_one(&mut *tx)
    .await?;

    let user = User {
        id: user_row.get("id"),
        name: user_row.get("name"),
        email: user_row.get("email"),
        created_at: user_row.get("created_at"),
    };

    // Create post for the user
    let post_row = sqlx::query(
        "INSERT INTO posts (user_id, title, content) VALUES (?, ?, ?) RETURNING id, user_id, title, content, created_at",
    )
    .bind(user.id)
    .bind(post_title)
    .bind(post_content)
    .fetch_one(&mut *tx)
    .await?;

    let post = Post {
        id: post_row.get("id"),
        user_id: post_row.get("user_id"),
        title: post_row.get("title"),
        content: post_row.get("content"),
        created_at: post_row.get("created_at"),
    };

    // Commit transaction
    tx.commit().await?;

    Ok((user, post))
}

// ============================================
// Query Patterns
// ============================================

async fn query_examples(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
    println!("\n  --- Query Examples ---");

    // Simple query
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;
    println!("  Total users: {}", count.0);

    // Query with JOIN
    let user_posts: Vec<(String, String)> = sqlx::query_as(
        r#"
        SELECT users.name, posts.title
        FROM users
        JOIN posts ON users.id = posts.user_id
        ORDER BY posts.created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    println!("  User posts:");
    for (name, title) in user_posts {
        println!("    {} wrote: {}", name, title);
    }

    // Query with aggregation
    let stats: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT users.name, COUNT(posts.id) as post_count
        FROM users
        LEFT JOIN posts ON users.id = posts.user_id
        GROUP BY users.id
        ORDER BY post_count DESC
        "#,
    )
    .fetch_all(pool)
    .await?;

    println!("  Post counts:");
    for (name, count) in stats {
        println!("    {}: {} posts", name, count);
    }

    Ok(())
}

// ============================================
// Main Entry Point
// ============================================

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Database Patterns ===\n");

    // Create in-memory SQLite database
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    // Create schema
    create_schema(&pool).await?;
    println!("  Schema created");

    // Initialize repository
    let user_repo = UserRepository::new(pool.clone());

    // ============================================
    // CRUD Operations
    // ============================================
    println!("\n  --- CRUD Operations ---");

    // Create
    let user1 = user_repo
        .create(CreateUser {
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        })
        .await?;
    println!("  Created: {:?}", user1);

    let user2 = user_repo
        .create(CreateUser {
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        })
        .await?;
    println!("  Created: {:?}", user2);

    // Read
    let found = user_repo.find_by_id(1).await?;
    println!("  Found by ID: {:?}", found);

    let found_email = user_repo.find_by_email("bob@example.com").await?;
    println!("  Found by email: {:?}", found_email);

    // Update
    let updated = user_repo
        .update(1, "Alice Smith", "alice.smith@example.com")
        .await?;
    println!("  Updated: {:?}", updated);

    // List
    let all_users = user_repo.list_all().await?;
    println!("  All users: {:?}", all_users);

    // ============================================
    // Transaction Example
    // ============================================
    println!("\n  --- Transaction Example ---");

    let (charlie, post) = create_user_with_post(
        &pool,
        "Charlie",
        "charlie@example.com",
        "My First Post",
        "Hello, world!",
    )
    .await?;

    println!("  Created user and post in transaction:");
    println!("    User: {:?}", charlie);
    println!("    Post: {:?}", post);

    // ============================================
    // Query Examples
    // ============================================
    query_examples(&pool).await?;

    // ============================================
    // Delete
    // ============================================
    println!("\n  --- Delete ---");
    let deleted = user_repo.delete(2).await?;
    println!("  Deleted user 2: {}", deleted);

    let remaining = user_repo.list_all().await?;
    println!("  Remaining users: {:?}", remaining);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> Pool<Sqlite> {
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();

        create_schema(&pool).await.unwrap();
        pool
    }

    #[tokio::test]
    async fn test_create_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(pool);

        let user = repo
            .create(CreateUser {
                name: "Test".to_string(),
                email: "test@example.com".to_string(),
            })
            .await
            .unwrap();

        assert_eq!(user.name, "Test");
        assert_eq!(user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(pool);

        let created = repo
            .create(CreateUser {
                name: "Test".to_string(),
                email: "test@example.com".to_string(),
            })
            .await
            .unwrap();

        let found = repo.find_by_id(created.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().name, "Test");

        let not_found = repo.find_by_id(999).await.unwrap();
        assert!(not_found.is_none());
    }

    #[tokio::test]
    async fn test_update_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(pool);

        let created = repo
            .create(CreateUser {
                name: "Original".to_string(),
                email: "original@example.com".to_string(),
            })
            .await
            .unwrap();

        let updated = repo
            .update(created.id, "Updated", "updated@example.com")
            .await
            .unwrap();

        assert!(updated.is_some());
        let updated = updated.unwrap();
        assert_eq!(updated.name, "Updated");
        assert_eq!(updated.email, "updated@example.com");
    }

    #[tokio::test]
    async fn test_delete_user() {
        let pool = setup_test_db().await;
        let repo = UserRepository::new(pool);

        let created = repo
            .create(CreateUser {
                name: "ToDelete".to_string(),
                email: "delete@example.com".to_string(),
            })
            .await
            .unwrap();

        let deleted = repo.delete(created.id).await.unwrap();
        assert!(deleted);

        let found = repo.find_by_id(created.id).await.unwrap();
        assert!(found.is_none());
    }
}
