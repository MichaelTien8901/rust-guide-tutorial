//! Logging and Tracing Example
//!
//! Demonstrates structured logging with the tracing crate.
//!
//! # Logging Levels
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                    Log Levels                           │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  ERROR  - Something went wrong, action needed           │
//!     │  WARN   - Potential problem, may need attention         │
//!     │  INFO   - General operational information               │
//!     │  DEBUG  - Detailed information for debugging            │
//!     │  TRACE  - Very detailed, per-operation logging          │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//!
//!     Production: INFO and above
//!     Development: DEBUG and above
//!     Troubleshooting: TRACE
//! ```

use tracing::{debug, error, info, info_span, instrument, trace, warn, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn main() {
    // Initialize the tracing subscriber
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env().add_directive(Level::TRACE.into()))
        .init();

    println!("=== Logging and Tracing ===\n");

    info!("Application started");

    basic_logging();
    structured_logging();
    spans_example();
    instrumented_functions();
    error_context();

    info!("Application finished");
}

// ============================================
// Basic Logging
// ============================================

fn basic_logging() {
    info!("--- Basic Logging ---");

    // Different log levels
    error!("This is an error message");
    warn!("This is a warning message");
    info!("This is an info message");
    debug!("This is a debug message");
    trace!("This is a trace message");

    // Logging with format arguments
    let user = "Alice";
    let count = 42;
    info!("User {} has {} items", user, count);

    // Printf-style formatting
    info!(user = %user, count = count, "User activity");
}

// ============================================
// Structured Logging
// ============================================

fn structured_logging() {
    info!("--- Structured Logging ---");

    // Structured fields
    info!(
        user_id = 123,
        action = "login",
        ip = "192.168.1.1",
        "User logged in"
    );

    // Display vs Debug formatting
    let data = vec![1, 2, 3];
    info!(data = ?data, "Debug format");        // Uses {:?}
    info!(count = %data.len(), "Display format"); // Uses {}

    // Nested structured data
    struct Request {
        method: String,
        path: String,
        status: u16,
        duration_ms: u64,
    }

    let req = Request {
        method: "GET".to_string(),
        path: "/api/users".to_string(),
        status: 200,
        duration_ms: 45,
    };

    info!(
        method = %req.method,
        path = %req.path,
        status = req.status,
        duration_ms = req.duration_ms,
        "Request completed"
    );

    // Optional fields
    let error: Option<&str> = None;
    if let Some(e) = error {
        error!(error = %e, "Request failed");
    } else {
        info!("Request succeeded");
    }
}

// ============================================
// Spans (Execution Context)
// ============================================

fn spans_example() {
    info!("--- Spans ---");

    // Create a span for a unit of work
    let span = info_span!("process_request", request_id = 12345);
    let _guard = span.enter();

    // All logs within this scope include the span context
    info!("Processing started");
    debug!("Step 1 complete");
    debug!("Step 2 complete");
    info!("Processing finished");

    // Nested spans
    let outer = info_span!("outer_operation");
    let _outer_guard = outer.enter();

    info!("In outer span");

    {
        let inner = info_span!("inner_operation", item_count = 5);
        let _inner_guard = inner.enter();

        info!("In inner span");
        debug!("Processing items");
    }

    info!("Back in outer span");
}

// ============================================
// Instrumented Functions
// ============================================

#[instrument]
fn instrumented_functions() {
    info!("--- Instrumented Functions ---");

    let result = calculate_value(10, 20);
    info!(result = result, "Calculation complete");

    let _ = process_user(123, "Alice");

    // With skip
    let _ = with_sensitive_data("user@example.com", "secret123");
}

#[instrument(level = "debug")]
fn calculate_value(a: i32, b: i32) -> i32 {
    debug!("Starting calculation");
    let result = a + b;
    trace!(result = result, "Calculation done");
    result
}

#[instrument(skip(name), fields(user_id = id))]
fn process_user(id: u64, name: &str) -> Result<(), &'static str> {
    info!("Processing user");
    debug!(name_len = name.len(), "Name details");

    if name.is_empty() {
        warn!("Empty user name");
        return Err("Name cannot be empty");
    }

    info!("User processed successfully");
    Ok(())
}

// Skip sensitive data
#[instrument(skip(password), fields(email = %email))]
fn with_sensitive_data(email: &str, password: &str) -> bool {
    info!("Authenticating user");
    // Password is not logged
    let valid = !password.is_empty();
    if valid {
        info!("Authentication successful");
    } else {
        warn!("Authentication failed");
    }
    valid
}

// ============================================
// Error Context
// ============================================

fn error_context() {
    info!("--- Error Context ---");

    // Logging errors with context
    match risky_operation() {
        Ok(value) => info!(value = value, "Operation succeeded"),
        Err(e) => error!(error = %e, "Operation failed"),
    }

    // Error chains
    if let Err(e) = complex_operation() {
        error!(
            error = %e,
            source = ?e.source(),
            "Complex operation failed"
        );
    }

    // Recording error in span
    let span = info_span!("failing_operation");
    let _guard = span.enter();

    match always_fails() {
        Ok(_) => info!("Unexpected success"),
        Err(e) => {
            span.record("error", &e.to_string().as_str());
            error!(error = %e, "Expected failure");
        }
    }
}

fn risky_operation() -> Result<i32, &'static str> {
    debug!("Attempting risky operation");
    // Simulate failure
    Err("Something went wrong")
}

#[derive(Debug)]
struct ComplexError {
    message: String,
    code: u32,
}

impl std::fmt::Display for ComplexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for ComplexError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

fn complex_operation() -> Result<(), ComplexError> {
    Err(ComplexError {
        message: "Database connection failed".to_string(),
        code: 500,
    })
}

fn always_fails() -> Result<(), &'static str> {
    Err("This always fails")
}

// ============================================
// Subscriber Configuration Examples
// ============================================

/// Example of different subscriber configurations
#[allow(dead_code)]
mod subscriber_examples {
    use tracing_subscriber::{fmt, prelude::*, EnvFilter};

    /// Basic console logging
    pub fn basic_setup() {
        tracing_subscriber::fmt::init();
    }

    /// With environment filter (RUST_LOG=info,myapp=debug)
    pub fn with_env_filter() {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();
    }

    /// JSON output for structured log aggregation
    pub fn json_output() {
        tracing_subscriber::fmt()
            .json()
            .init();
    }

    /// Compact format for development
    pub fn compact_format() {
        tracing_subscriber::fmt()
            .compact()
            .with_target(false)
            .init();
    }

    /// Pretty format for development
    pub fn pretty_format() {
        tracing_subscriber::fmt()
            .pretty()
            .with_thread_names(true)
            .init();
    }

    /// Multiple layers
    pub fn multi_layer() {
        use tracing::Level;

        tracing_subscriber::registry()
            .with(fmt::layer())
            .with(EnvFilter::from_default_env()
                .add_directive(Level::INFO.into()))
            .init();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt::MakeWriter;
    use std::sync::{Arc, Mutex};

    // Test helper to capture log output
    struct TestWriter {
        buf: Arc<Mutex<Vec<u8>>>,
    }

    impl std::io::Write for TestWriter {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.buf.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }

        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    #[test]
    fn test_calculate_value() {
        let result = calculate_value(5, 3);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_process_user_success() {
        let result = process_user(1, "Test");
        assert!(result.is_ok());
    }

    #[test]
    fn test_process_user_empty_name() {
        let result = process_user(1, "");
        assert!(result.is_err());
    }

    #[test]
    fn test_with_sensitive_data() {
        assert!(with_sensitive_data("test@test.com", "password"));
        assert!(!with_sensitive_data("test@test.com", ""));
    }
}
