//! Builder Pattern Example
//!
//! Demonstrates the builder pattern for fluent API construction.
//!
//! # Builder Pattern Flow
//! ```text
//!     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
//!     │   Builder    │────►│  Configure   │────►│    Build     │
//!     │   ::new()    │     │   Methods    │     │   Result     │
//!     └──────────────┘     └──────────────┘     └──────────────┘
//!            │                    │                    │
//!            ▼                    ▼                    ▼
//!     ┌──────────────────────────────────────────────────────┐
//!     │  ServerBuilder::new()                                │
//!     │      .host("localhost")    // Returns Self           │
//!     │      .port(8080)           // Returns Self           │
//!     │      .max_connections(100) // Returns Self           │
//!     │      .build()              // Returns Server/Result  │
//!     └──────────────────────────────────────────────────────┘
//! ```

fn main() {
    println!("=== Builder Pattern ===\n");

    println!("--- Basic Builder ---");
    basic_builder();

    println!("\n--- Builder with Validation ---");
    builder_with_validation();

    println!("\n--- Typestate Builder ---");
    typestate_builder();

    println!("\n--- Derive-style Builder ---");
    derive_style_builder();

    println!("\n--- Builder with Defaults ---");
    builder_with_defaults();
}

// ============================================
// Basic Builder Pattern
// ============================================

#[derive(Debug)]
struct Server {
    host: String,
    port: u16,
    max_connections: usize,
    timeout_seconds: u64,
}

struct ServerBuilder {
    host: Option<String>,
    port: Option<u16>,
    max_connections: Option<usize>,
    timeout_seconds: Option<u64>,
}

impl ServerBuilder {
    fn new() -> Self {
        ServerBuilder {
            host: None,
            port: None,
            max_connections: None,
            timeout_seconds: None,
        }
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn max_connections(mut self, max: usize) -> Self {
        self.max_connections = Some(max);
        self
    }

    fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = Some(seconds);
        self
    }

    fn build(self) -> Server {
        Server {
            host: self.host.unwrap_or_else(|| "localhost".to_string()),
            port: self.port.unwrap_or(8080),
            max_connections: self.max_connections.unwrap_or(100),
            timeout_seconds: self.timeout_seconds.unwrap_or(30),
        }
    }
}

impl Server {
    fn builder() -> ServerBuilder {
        ServerBuilder::new()
    }
}

fn basic_builder() {
    // Full configuration
    let server = Server::builder()
        .host("0.0.0.0")
        .port(3000)
        .max_connections(500)
        .timeout(60)
        .build();

    println!("  Full config: {:?}", server);

    // Partial configuration (uses defaults)
    let default_server = Server::builder()
        .host("127.0.0.1")
        .build();

    println!("  With defaults: {:?}", default_server);
}

// ============================================
// Builder with Validation
// ============================================

#[derive(Debug)]
struct DatabaseConfig {
    host: String,
    port: u16,
    database: String,
    username: String,
    password: String,
    max_pool_size: u32,
}

#[derive(Debug)]
enum ConfigError {
    MissingField(&'static str),
    InvalidPort,
    InvalidPoolSize,
}

struct DatabaseConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    database: Option<String>,
    username: Option<String>,
    password: Option<String>,
    max_pool_size: Option<u32>,
}

impl DatabaseConfigBuilder {
    fn new() -> Self {
        DatabaseConfigBuilder {
            host: None,
            port: None,
            database: None,
            username: None,
            password: None,
            max_pool_size: None,
        }
    }

    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn database(mut self, db: impl Into<String>) -> Self {
        self.database = Some(db.into());
        self
    }

    fn credentials(mut self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.username = Some(username.into());
        self.password = Some(password.into());
        self
    }

    fn max_pool_size(mut self, size: u32) -> Self {
        self.max_pool_size = Some(size);
        self
    }

    fn build(self) -> Result<DatabaseConfig, ConfigError> {
        let port = self.port.unwrap_or(5432);
        if port == 0 {
            return Err(ConfigError::InvalidPort);
        }

        let pool_size = self.max_pool_size.unwrap_or(10);
        if pool_size == 0 || pool_size > 1000 {
            return Err(ConfigError::InvalidPoolSize);
        }

        Ok(DatabaseConfig {
            host: self.host.ok_or(ConfigError::MissingField("host"))?,
            port,
            database: self.database.ok_or(ConfigError::MissingField("database"))?,
            username: self.username.ok_or(ConfigError::MissingField("username"))?,
            password: self.password.ok_or(ConfigError::MissingField("password"))?,
            max_pool_size: pool_size,
        })
    }
}

fn builder_with_validation() {
    // Valid configuration
    let config = DatabaseConfigBuilder::new()
        .host("localhost")
        .port(5432)
        .database("myapp")
        .credentials("user", "secret")
        .max_pool_size(20)
        .build();

    match config {
        Ok(c) => println!("  Valid config: {:?}", c),
        Err(e) => println!("  Error: {:?}", e),
    }

    // Invalid - missing required field
    let invalid = DatabaseConfigBuilder::new()
        .host("localhost")
        .build();

    match invalid {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  Expected error: {:?}", e),
    }
}

// ============================================
// Typestate Builder (Compile-time Validation)
// ============================================

/// Marker types for builder states
mod typestate {
    pub struct NoHost;
    pub struct HasHost;
    pub struct NoPort;
    pub struct HasPort;
}

use typestate::*;

struct HttpClientBuilder<H, P> {
    host: Option<String>,
    port: Option<u16>,
    timeout: Option<u64>,
    _host_state: std::marker::PhantomData<H>,
    _port_state: std::marker::PhantomData<P>,
}

#[derive(Debug)]
struct HttpClient {
    host: String,
    port: u16,
    timeout: u64,
}

impl HttpClientBuilder<NoHost, NoPort> {
    fn new() -> Self {
        HttpClientBuilder {
            host: None,
            port: None,
            timeout: None,
            _host_state: std::marker::PhantomData,
            _port_state: std::marker::PhantomData,
        }
    }
}

impl<P> HttpClientBuilder<NoHost, P> {
    fn host(self, host: impl Into<String>) -> HttpClientBuilder<HasHost, P> {
        HttpClientBuilder {
            host: Some(host.into()),
            port: self.port,
            timeout: self.timeout,
            _host_state: std::marker::PhantomData,
            _port_state: std::marker::PhantomData,
        }
    }
}

impl<H> HttpClientBuilder<H, NoPort> {
    fn port(self, port: u16) -> HttpClientBuilder<H, HasPort> {
        HttpClientBuilder {
            host: self.host,
            port: Some(port),
            timeout: self.timeout,
            _host_state: std::marker::PhantomData,
            _port_state: std::marker::PhantomData,
        }
    }
}

impl<H, P> HttpClientBuilder<H, P> {
    fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = Some(seconds);
        self
    }
}

// build() only available when both host AND port are set
impl HttpClientBuilder<HasHost, HasPort> {
    fn build(self) -> HttpClient {
        HttpClient {
            host: self.host.unwrap(),
            port: self.port.unwrap(),
            timeout: self.timeout.unwrap_or(30),
        }
    }
}

fn typestate_builder() {
    // This compiles - all required fields set
    let client = HttpClientBuilder::new()
        .host("api.example.com")
        .port(443)
        .timeout(60)
        .build();

    println!("  Typestate builder: {:?}", client);

    // This would NOT compile - missing port:
    // let invalid = HttpClientBuilder::new()
    //     .host("api.example.com")
    //     .build();  // Error: build() not available

    println!("  (Missing required fields = compile error)");
}

// ============================================
// Derive-style Builder (Simulated)
// ============================================

/// In practice, use the `derive_builder` crate
/// This simulates what it generates

#[derive(Debug, Clone)]
struct Email {
    from: String,
    to: Vec<String>,
    subject: String,
    body: String,
    cc: Vec<String>,
    attachments: Vec<String>,
}

#[derive(Default)]
struct EmailBuilder {
    from: Option<String>,
    to: Vec<String>,
    subject: Option<String>,
    body: Option<String>,
    cc: Vec<String>,
    attachments: Vec<String>,
}

impl EmailBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    fn to(mut self, to: impl Into<String>) -> Self {
        self.to.push(to.into());
        self
    }

    fn subject(mut self, subject: impl Into<String>) -> Self {
        self.subject = Some(subject.into());
        self
    }

    fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    fn cc(mut self, cc: impl Into<String>) -> Self {
        self.cc.push(cc.into());
        self
    }

    fn attachment(mut self, path: impl Into<String>) -> Self {
        self.attachments.push(path.into());
        self
    }

    fn build(self) -> Result<Email, &'static str> {
        Ok(Email {
            from: self.from.ok_or("from is required")?,
            to: if self.to.is_empty() {
                return Err("at least one recipient required");
            } else {
                self.to
            },
            subject: self.subject.ok_or("subject is required")?,
            body: self.body.unwrap_or_default(),
            cc: self.cc,
            attachments: self.attachments,
        })
    }
}

fn derive_style_builder() {
    let email = EmailBuilder::new()
        .from("sender@example.com")
        .to("recipient1@example.com")
        .to("recipient2@example.com")
        .subject("Hello!")
        .body("This is the email body.")
        .cc("cc@example.com")
        .attachment("/path/to/file.pdf")
        .build()
        .unwrap();

    println!("  Email: {:?}", email);
}

// ============================================
// Builder with Defaults (Default trait)
// ============================================

#[derive(Debug)]
struct AppConfig {
    debug: bool,
    log_level: String,
    workers: usize,
    buffer_size: usize,
    retry_attempts: u32,
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig {
            debug: false,
            log_level: "info".to_string(),
            workers: num_cpus(),
            buffer_size: 4096,
            retry_attempts: 3,
        }
    }
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

impl AppConfig {
    fn builder() -> AppConfigBuilder {
        AppConfigBuilder::default()
    }
}

#[derive(Default)]
struct AppConfigBuilder {
    debug: Option<bool>,
    log_level: Option<String>,
    workers: Option<usize>,
    buffer_size: Option<usize>,
    retry_attempts: Option<u32>,
}

impl AppConfigBuilder {
    fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
    }

    fn log_level(mut self, level: impl Into<String>) -> Self {
        self.log_level = Some(level.into());
        self
    }

    fn workers(mut self, n: usize) -> Self {
        self.workers = Some(n);
        self
    }

    fn buffer_size(mut self, size: usize) -> Self {
        self.buffer_size = Some(size);
        self
    }

    fn retry_attempts(mut self, attempts: u32) -> Self {
        self.retry_attempts = Some(attempts);
        self
    }

    fn build(self) -> AppConfig {
        let defaults = AppConfig::default();
        AppConfig {
            debug: self.debug.unwrap_or(defaults.debug),
            log_level: self.log_level.unwrap_or(defaults.log_level),
            workers: self.workers.unwrap_or(defaults.workers),
            buffer_size: self.buffer_size.unwrap_or(defaults.buffer_size),
            retry_attempts: self.retry_attempts.unwrap_or(defaults.retry_attempts),
        }
    }
}

fn builder_with_defaults() {
    // Use all defaults
    let default_config = AppConfig::default();
    println!("  Default config: {:?}", default_config);

    // Override specific fields
    let custom_config = AppConfig::builder()
        .debug(true)
        .log_level("debug")
        .workers(4)
        .build();

    println!("  Custom config: {:?}", custom_config);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_builder() {
        let server = Server::builder()
            .host("test")
            .port(9000)
            .build();

        assert_eq!(server.host, "test");
        assert_eq!(server.port, 9000);
    }

    #[test]
    fn test_builder_defaults() {
        let server = Server::builder().build();

        assert_eq!(server.host, "localhost");
        assert_eq!(server.port, 8080);
    }

    #[test]
    fn test_validated_builder_success() {
        let config = DatabaseConfigBuilder::new()
            .host("localhost")
            .database("test")
            .credentials("user", "pass")
            .build();

        assert!(config.is_ok());
    }

    #[test]
    fn test_validated_builder_failure() {
        let config = DatabaseConfigBuilder::new()
            .host("localhost")
            .build();

        assert!(config.is_err());
    }

    #[test]
    fn test_email_builder() {
        let email = EmailBuilder::new()
            .from("a@b.com")
            .to("c@d.com")
            .subject("Test")
            .build();

        assert!(email.is_ok());
    }
}
