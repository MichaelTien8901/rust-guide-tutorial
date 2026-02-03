//! Error Patterns Example
//!
//! Demonstrates error handling with thiserror and anyhow.
//!
//! # Error Handling Decision Tree
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │            When to use which error type?                │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!            ┌───────────────────────────────┐
//!            │  Are you writing a library?   │
//!            └───────────────────────────────┘
//!                     │           │
//!                    Yes          No
//!                     │           │
//!                     ▼           ▼
//!     ┌───────────────────┐   ┌───────────────────┐
//!     │    thiserror      │   │  Is context       │
//!     │  (typed errors)   │   │  important?       │
//!     └───────────────────┘   └───────────────────┘
//!                                  │         │
//!                                 Yes        No
//!                                  │         │
//!                                  ▼         ▼
//!                     ┌─────────────┐  ┌─────────────┐
//!                     │   anyhow    │  │  Simple     │
//!                     │ (contextual)│  │  Result<T,E>│
//!                     └─────────────┘  └─────────────┘
//! ```

use anyhow::{anyhow, bail, Context, Result as AnyhowResult};
use std::fs;
use std::io;
use thiserror::Error;

fn main() {
    println!("=== Error Patterns ===\n");

    println!("--- Custom Errors with thiserror ---");
    thiserror_example();

    println!("\n--- Error Context with anyhow ---");
    anyhow_example();

    println!("\n--- Error Conversion ---");
    error_conversion();

    println!("\n--- Error Handling Patterns ---");
    error_patterns();

    println!("\n--- Recoverable vs Fatal ---");
    recoverable_vs_fatal();
}

// ============================================
// thiserror - Typed Error Definitions
// ============================================

/// Domain-specific errors for a user service
#[derive(Error, Debug)]
enum UserError {
    #[error("user not found: {0}")]
    NotFound(String),

    #[error("invalid email format: {email}")]
    InvalidEmail { email: String },

    #[error("user already exists with id {id}")]
    AlreadyExists { id: u64 },

    #[error("authentication failed for user {username}")]
    AuthFailed { username: String },

    #[error("database error")]
    Database(#[from] DatabaseError),

    #[error("validation failed: {0}")]
    Validation(String),
}

#[derive(Error, Debug)]
enum DatabaseError {
    #[error("connection failed: {0}")]
    Connection(String),

    #[error("query failed: {0}")]
    Query(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}

fn thiserror_example() {
    // Creating errors
    let not_found = UserError::NotFound("user123".to_string());
    println!("  Error: {}", not_found);

    let invalid_email = UserError::InvalidEmail {
        email: "not-an-email".to_string(),
    };
    println!("  Error: {}", invalid_email);

    // Error chain with #[from]
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file missing");
    let db_error: DatabaseError = io_error.into();
    let user_error: UserError = db_error.into();
    println!("  Chained error: {}", user_error);

    // Using in Result
    fn find_user(id: &str) -> Result<String, UserError> {
        if id == "admin" {
            Ok("Admin User".to_string())
        } else {
            Err(UserError::NotFound(id.to_string()))
        }
    }

    match find_user("guest") {
        Ok(name) => println!("  Found: {}", name),
        Err(e) => println!("  Error: {}", e),
    }
}

// ============================================
// anyhow - Contextual Error Handling
// ============================================

fn anyhow_example() {
    // Adding context to errors
    fn read_config(path: &str) -> AnyhowResult<String> {
        fs::read_to_string(path).with_context(|| format!("Failed to read config file: {}", path))
    }

    match read_config("/nonexistent/config.toml") {
        Ok(content) => println!("  Config: {}", content),
        Err(e) => println!("  Error with context: {}", e),
    }

    // Creating ad-hoc errors
    fn validate_port(port: u16) -> AnyhowResult<()> {
        if port < 1024 {
            bail!("Port {} is a privileged port (< 1024)", port);
        }
        Ok(())
    }

    match validate_port(80) {
        Ok(()) => println!("  Port is valid"),
        Err(e) => println!("  Validation error: {}", e),
    }

    // Chaining context
    fn load_app_config() -> AnyhowResult<String> {
        let config_path = "/app/config.json";
        let content = fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read {}", config_path))?;

        // Simulate parsing
        if content.is_empty() {
            bail!("Config file is empty");
        }

        Ok(content)
    }

    match load_app_config() {
        Ok(c) => println!("  Loaded: {}", c),
        Err(e) => {
            println!("  Error chain:");
            for (i, cause) in e.chain().enumerate() {
                println!("    {}: {}", i, cause);
            }
        }
    }

    // anyhow! macro for custom errors
    fn check_positive(n: i32) -> AnyhowResult<i32> {
        if n <= 0 {
            Err(anyhow!("Expected positive number, got {}", n))
        } else {
            Ok(n)
        }
    }

    match check_positive(-5) {
        Ok(n) => println!("  Positive: {}", n),
        Err(e) => println!("  Error: {}", e),
    }
}

// ============================================
// Error Conversion
// ============================================

/// Application error that can hold different error types
#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),

    #[error("User error: {0}")]
    User(#[from] UserError),

    #[error("Internal error: {0}")]
    Internal(String),
}

fn error_conversion() {
    // Automatic conversion with ?
    fn process_file(path: &str) -> Result<i32, AppError> {
        let content = fs::read_to_string(path)?; // io::Error -> AppError
        let number: i32 = content.trim().parse()?; // ParseIntError -> AppError
        Ok(number * 2)
    }

    match process_file("/nonexistent") {
        Ok(n) => println!("  Result: {}", n),
        Err(e) => println!("  Converted error: {}", e),
    }

    // Manual conversion
    fn manual_convert() -> Result<(), AppError> {
        Err(AppError::Internal("Something went wrong".to_string()))
    }

    if let Err(e) = manual_convert() {
        println!("  Manual error: {}", e);
    }

    // Map error types
    fn map_error_example() -> Result<String, String> {
        fs::read_to_string("/nonexistent").map_err(|e| format!("Failed to read file: {}", e))
    }

    match map_error_example() {
        Ok(s) => println!("  Content: {}", s),
        Err(e) => println!("  Mapped error: {}", e),
    }
}

// ============================================
// Common Error Handling Patterns
// ============================================

fn error_patterns() {
    // Pattern 1: Default on error
    let value =
        fs::read_to_string("/nonexistent").unwrap_or_else(|_| "default content".to_string());
    println!("  unwrap_or_else: {}", value);

    // Pattern 2: Transform Option to Result
    fn get_first_char(s: &str) -> Result<char, &'static str> {
        s.chars().next().ok_or("String is empty")
    }

    match get_first_char("") {
        Ok(c) => println!("  First char: {}", c),
        Err(e) => println!("  Option->Result error: {}", e),
    }

    // Pattern 3: Collect results
    let numbers = vec!["1", "2", "three", "4"];
    let parsed: Result<Vec<i32>, _> = numbers.iter().map(|s| s.parse::<i32>()).collect();

    match parsed {
        Ok(v) => println!("  All parsed: {:?}", v),
        Err(e) => println!("  Parse failed: {}", e),
    }

    // Pattern 4: Filter and handle errors separately
    let (successes, failures): (Vec<_>, Vec<_>) = numbers
        .iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let successes: Vec<i32> = successes.into_iter().map(Result::unwrap).collect();
    let failures: Vec<_> = failures.into_iter().map(Result::unwrap_err).collect();

    println!("  Successes: {:?}", successes);
    println!("  Failures: {} errors", failures.len());

    // Pattern 5: Early return with ?
    fn multi_step() -> Result<String, &'static str> {
        let step1 = Some("data").ok_or("Step 1 failed")?;
        let step2 = Some(step1.to_uppercase()).ok_or("Step 2 failed")?;
        let step3 = Some(format!("Result: {}", step2)).ok_or("Step 3 failed")?;
        Ok(step3)
    }

    match multi_step() {
        Ok(result) => println!("  Multi-step result: {}", result),
        Err(e) => println!("  Multi-step error: {}", e),
    }
}

// ============================================
// Recoverable vs Fatal Errors
// ============================================

fn recoverable_vs_fatal() {
    // Recoverable: return Result, let caller decide
    fn parse_config_value(s: &str) -> Result<i32, std::num::ParseIntError> {
        s.parse()
    }

    // Recoverable with retry
    fn with_retry<T, E, F>(mut f: F, max_attempts: u32) -> Result<T, E>
    where
        F: FnMut() -> Result<T, E>,
    {
        let mut last_error = None;
        for _ in 0..max_attempts {
            match f() {
                Ok(v) => return Ok(v),
                Err(e) => last_error = Some(e),
            }
        }
        Err(last_error.unwrap())
    }

    let mut attempts = 0;
    let result = with_retry(
        || {
            attempts += 1;
            if attempts < 3 {
                Err("not ready yet")
            } else {
                Ok("success!")
            }
        },
        5,
    );
    println!("  Retry result after {} attempts: {:?}", attempts, result);

    // Fatal: use panic! or expect() for unrecoverable situations
    fn get_required_env(key: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| {
            // In real code, this might panic for required config
            format!("[{} not set]", key)
        })
    }

    let home = get_required_env("HOME");
    println!("  HOME: {}", home);

    // When to panic vs return error:
    println!("\n  When to panic:");
    println!("    - Programming errors (bugs)");
    println!("    - Invalid state that shouldn't occur");
    println!("    - During development/prototyping");

    println!("\n  When to return Result:");
    println!("    - Expected failure cases");
    println!("    - User input validation");
    println!("    - External resource failures");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_error_display() {
        let err = UserError::NotFound("test".to_string());
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn test_error_conversion() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "missing");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Io(_)));
    }

    #[test]
    fn test_anyhow_context() {
        let result: AnyhowResult<()> = Err(anyhow!("base error")).context("additional context");

        let err = result.unwrap_err();
        assert!(err.to_string().contains("additional context"));
    }

    #[test]
    fn test_error_chain() {
        let db_err = DatabaseError::Connection("timeout".to_string());
        let user_err: UserError = db_err.into();
        assert!(matches!(user_err, UserError::Database(_)));
    }
}
