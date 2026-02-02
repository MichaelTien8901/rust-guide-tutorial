//! Serialization Patterns Example
//!
//! Demonstrates serde for JSON, TOML, and custom serialization.
//!
//! # Serde Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                    Serde Flow                           │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!          ┌─────────────────┼─────────────────┐
//!          ▼                                   ▼
//!     ┌─────────────┐                    ┌─────────────┐
//!     │ Serialize   │                    │ Deserialize │
//!     │  Rust → Data│                    │ Data → Rust │
//!     └─────────────┘                    └─────────────┘
//!          │                                   │
//!          ▼                                   ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  Formats: JSON, TOML, YAML, MessagePack, etc.           │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use serde::{Deserialize, Serialize};

fn main() {
    println!("=== Serialization Patterns ===\n");

    println!("--- Basic Serde ---");
    basic_serde();

    println!("\n--- Serde Attributes ---");
    serde_attributes();

    println!("\n--- Enum Serialization ---");
    enum_serialization();

    println!("\n--- Custom Serialization ---");
    custom_serialization();

    println!("\n--- TOML Config ---");
    toml_config();

    println!("\n--- Optional and Default ---");
    optional_and_default();

    println!("\n--- Flatten and Skip ---");
    flatten_and_skip();
}

// ============================================
// Basic Serde
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

fn basic_serde() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
        email: "alice@example.com".to_string(),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&person).unwrap();
    println!("  Serialized:\n{}", json);

    // Deserialize from JSON
    let json_input = r#"{"name": "Bob", "age": 25, "email": "bob@example.com"}"#;
    let deserialized: Person = serde_json::from_str(json_input).unwrap();
    println!("  Deserialized: {:?}", deserialized);

    // Serialize to Value (dynamic JSON)
    let value = serde_json::to_value(&person).unwrap();
    println!("  As Value: {}", value["name"]);
}

// ============================================
// Serde Attributes
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_id")]
    id: u64,

    #[serde(rename = "user_name")]
    name: String,

    #[serde(rename = "email_address")]
    email: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    phone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CamelCaseStruct {
    user_name: String,
    email_address: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
struct ScreamingStruct {
    user_name: String,
    email_address: String,
}

fn serde_attributes() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
        phone: None,
    };

    let json = serde_json::to_string(&user).unwrap();
    println!("  Renamed fields: {}", json);

    let camel = CamelCaseStruct {
        user_name: "Alice".to_string(),
        email_address: "alice@example.com".to_string(),
        created_at: "2024-01-15".to_string(),
    };

    let json = serde_json::to_string(&camel).unwrap();
    println!("  camelCase: {}", json);

    let screaming = ScreamingStruct {
        user_name: "Alice".to_string(),
        email_address: "alice@example.com".to_string(),
    };

    let json = serde_json::to_string(&screaming).unwrap();
    println!("  SCREAMING_SNAKE: {}", json);
}

// ============================================
// Enum Serialization
// ============================================

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Active,
    Completed,
    #[serde(rename = "cancelled")]
    Canceled,
}

// Externally tagged (default)
#[derive(Debug, Serialize, Deserialize)]
enum Message {
    Text { content: String },
    Image { url: String, size: u64 },
    File { name: String, data: Vec<u8> },
}

// Internally tagged
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum InternallyTagged {
    #[serde(rename = "text")]
    Text { content: String },
    #[serde(rename = "image")]
    Image { url: String },
}

// Adjacently tagged
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
enum AdjacentlyTagged {
    Text { content: String },
    Image { url: String },
}

// Untagged
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum Untagged {
    Integer(i64),
    Float(f64),
    String(String),
}

fn enum_serialization() {
    // Simple enum
    let status = Status::Active;
    let json = serde_json::to_string(&status).unwrap();
    println!("  Simple enum: {}", json);

    // Externally tagged (default)
    let msg = Message::Text { content: "Hello".to_string() };
    let json = serde_json::to_string(&msg).unwrap();
    println!("  Externally tagged: {}", json);

    // Internally tagged
    let msg = InternallyTagged::Text { content: "Hello".to_string() };
    let json = serde_json::to_string(&msg).unwrap();
    println!("  Internally tagged: {}", json);

    // Adjacently tagged
    let msg = AdjacentlyTagged::Image { url: "http://example.com/img.png".to_string() };
    let json = serde_json::to_string(&msg).unwrap();
    println!("  Adjacently tagged: {}", json);

    // Untagged (tries each variant in order)
    let values = vec![
        Untagged::Integer(42),
        Untagged::Float(3.14),
        Untagged::String("hello".to_string()),
    ];
    for v in values {
        println!("  Untagged: {}", serde_json::to_string(&v).unwrap());
    }
}

// ============================================
// Custom Serialization
// ============================================

mod custom_date {
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    pub fn serialize<S>(date: &str, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(date)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        // In real code, you'd validate the format
        Ok(s)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    name: String,
    #[serde(with = "custom_date")]
    date: String,
}

// Custom serializer for sensitive data
mod masked {
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(_: &String, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str("********")
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<String, D::Error>
    where
        D: Deserializer<'de>,
    {
        String::deserialize(deserializer)
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    username: String,
    #[serde(with = "masked")]
    password: String,
}

fn custom_serialization() {
    let event = Event {
        name: "Conference".to_string(),
        date: "2024-06-15".to_string(),
    };
    println!("  Custom date: {}", serde_json::to_string(&event).unwrap());

    let creds = Credentials {
        username: "admin".to_string(),
        password: "secret123".to_string(),
    };
    println!("  Masked password: {}", serde_json::to_string(&creds).unwrap());
}

// ============================================
// TOML Config
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    server: ServerConfig,
    database: DatabaseConfig,
    features: Features,
}

#[derive(Debug, Serialize, Deserialize)]
struct ServerConfig {
    host: String,
    port: u16,
    workers: usize,
}

#[derive(Debug, Serialize, Deserialize)]
struct DatabaseConfig {
    url: String,
    max_connections: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Features {
    enable_cache: bool,
    enable_logging: bool,
}

fn toml_config() {
    let config = Config {
        server: ServerConfig {
            host: "0.0.0.0".to_string(),
            port: 8080,
            workers: 4,
        },
        database: DatabaseConfig {
            url: "postgres://localhost/myapp".to_string(),
            max_connections: 20,
        },
        features: Features {
            enable_cache: true,
            enable_logging: true,
        },
    };

    let toml_str = toml::to_string_pretty(&config).unwrap();
    println!("  TOML config:\n{}", toml_str);

    // Parse TOML
    let toml_input = r#"
[server]
host = "127.0.0.1"
port = 3000
workers = 2

[database]
url = "sqlite::memory:"
max_connections = 5

[features]
enable_cache = false
enable_logging = true
"#;

    let parsed: Config = toml::from_str(toml_input).unwrap();
    println!("  Parsed TOML: {:?}", parsed);
}

// ============================================
// Optional and Default
// ============================================

fn default_port() -> u16 { 8080 }
fn default_workers() -> usize { 4 }

#[derive(Debug, Serialize, Deserialize)]
struct ServerOpts {
    host: String,

    #[serde(default = "default_port")]
    port: u16,

    #[serde(default = "default_workers")]
    workers: usize,

    #[serde(default)]
    debug: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    tls_cert: Option<String>,
}

fn optional_and_default() {
    // Minimal JSON
    let json = r#"{"host": "localhost"}"#;
    let opts: ServerOpts = serde_json::from_str(json).unwrap();
    println!("  With defaults: {:?}", opts);

    // Full JSON
    let json = r#"{
        "host": "0.0.0.0",
        "port": 3000,
        "workers": 8,
        "debug": true,
        "tls_cert": "/path/to/cert.pem"
    }"#;
    let opts: ServerOpts = serde_json::from_str(json).unwrap();
    println!("  Full config: {:?}", opts);

    // Serializing skips None
    let opts = ServerOpts {
        host: "localhost".to_string(),
        port: 8080,
        workers: 4,
        debug: false,
        tls_cert: None,
    };
    let json = serde_json::to_string(&opts).unwrap();
    println!("  Serialized (skip None): {}", json);
}

// ============================================
// Flatten and Skip
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct Metadata {
    created_by: String,
    created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Document {
    title: String,
    content: String,

    #[serde(flatten)]
    metadata: Metadata,

    #[serde(skip)]
    internal_id: u64,
}

fn flatten_and_skip() {
    let doc = Document {
        title: "My Document".to_string(),
        content: "Document content here".to_string(),
        metadata: Metadata {
            created_by: "Alice".to_string(),
            created_at: "2024-01-15".to_string(),
        },
        internal_id: 12345, // This will be skipped
    };

    let json = serde_json::to_string_pretty(&doc).unwrap();
    println!("  Flattened (no nesting):\n{}", json);

    // Note: internal_id is not in output
    // Note: metadata fields are at top level
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_roundtrip() {
        let person = Person {
            name: "Test".to_string(),
            age: 25,
            email: "test@example.com".to_string(),
        };

        let json = serde_json::to_string(&person).unwrap();
        let back: Person = serde_json::from_str(&json).unwrap();

        assert_eq!(person.name, back.name);
        assert_eq!(person.age, back.age);
    }

    #[test]
    fn test_renamed_fields() {
        let user = User {
            id: 1,
            name: "Test".to_string(),
            email: "test@test.com".to_string(),
            phone: None,
        };

        let json = serde_json::to_string(&user).unwrap();
        assert!(json.contains("user_id"));
        assert!(json.contains("user_name"));
        assert!(!json.contains("phone")); // None is skipped
    }

    #[test]
    fn test_enum_variants() {
        let status = Status::Canceled;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"cancelled\""); // Renamed
    }

    #[test]
    fn test_default_values() {
        let json = r#"{"host": "localhost"}"#;
        let opts: ServerOpts = serde_json::from_str(json).unwrap();

        assert_eq!(opts.port, 8080); // default
        assert_eq!(opts.workers, 4); // default
        assert!(!opts.debug); // default bool
    }

    #[test]
    fn test_toml_roundtrip() {
        let config = Config {
            server: ServerConfig {
                host: "test".to_string(),
                port: 3000,
                workers: 2,
            },
            database: DatabaseConfig {
                url: "test://db".to_string(),
                max_connections: 5,
            },
            features: Features {
                enable_cache: true,
                enable_logging: false,
            },
        };

        let toml_str = toml::to_string(&config).unwrap();
        let back: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.server.port, back.server.port);
    }
}
