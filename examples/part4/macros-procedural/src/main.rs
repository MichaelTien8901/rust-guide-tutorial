//! Procedural Macros Example
//!
//! Demonstrates procedural macro concepts and usage patterns.
//!
//! # Procedural Macro Types
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │              Procedural Macro Types                     │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  1. Derive Macros:   #[derive(MyTrait)]                 │
//!     │     - Auto-implement traits                             │
//!     │     - Input: struct/enum definition                     │
//!     │     - Output: impl block                                │
//!     │                                                         │
//!     │  2. Attribute Macros: #[my_attribute]                   │
//!     │     - Transform items                                   │
//!     │     - Input: any item (fn, struct, etc.)                │
//!     │     - Output: modified/replaced item                    │
//!     │                                                         │
//!     │  3. Function-like:    my_macro!(...)                    │
//!     │     - Called like functions                             │
//!     │     - Input: arbitrary tokens                           │
//!     │     - Output: arbitrary tokens                          │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: Procedural macros must be defined in a separate crate
//! with `proc-macro = true` in Cargo.toml. This example shows
//! usage patterns with commonly used proc macro crates.

use serde::{Deserialize, Serialize};
use thiserror::Error;

fn main() {
    println!("=== Procedural Macros ===\n");

    println!("--- Derive Macros ---");
    derive_macros_example();

    println!("\n--- Serde Derive ---");
    serde_derive_example();

    println!("\n--- Custom Error Types ---");
    thiserror_example();

    println!("\n--- Derive Macro Internals ---");
    derive_internals_explanation();

    println!("\n--- Common Patterns ---");
    common_patterns();
}

// ============================================
// Basic Derive Macros
// ============================================

/// Standard library derive macros
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Person {
    name: String,
    age: u32,
}

fn derive_macros_example() {
    // Debug
    let p = Point { x: 10, y: 20 };
    println!("  Debug: {:?}", p);

    // Clone
    let p2 = p.clone();
    println!("  Clone: {:?}", p2);

    // PartialEq
    println!("  p == p2: {}", p == p2);

    // Default
    let default_p = Point::default();
    println!("  Default: {:?}", default_p);

    // Hash (can be used in HashSet/HashMap)
    use std::collections::HashSet;
    let mut set = HashSet::new();
    set.insert(p);
    println!("  In HashSet: {}", set.contains(&p));

    // Ord for sorting
    let mut people = vec![
        Person { name: "Bob".into(), age: 30 },
        Person { name: "Alice".into(), age: 25 },
        Person { name: "Alice".into(), age: 20 },
    ];
    people.sort();
    println!("  Sorted people: {:?}", people);
}

// ============================================
// Serde Derive Macros
// ============================================

#[derive(Debug, Serialize, Deserialize)]
struct User {
    #[serde(rename = "user_id")]
    id: u64,

    #[serde(default)]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    #[serde(rename = "text")]
    Text { content: String },

    #[serde(rename = "image")]
    Image { url: String, width: u32, height: u32 },
}

fn serde_derive_example() {
    let user = User {
        id: 1,
        name: "Alice".to_string(),
        email: Some("alice@example.com".to_string()),
    };

    // Serialize to JSON
    let json = serde_json::to_string_pretty(&user).unwrap();
    println!("  Serialized User:\n{}", json);

    // Deserialize from JSON
    let json_input = r#"{"user_id": 2, "name": "Bob"}"#;
    let user2: User = serde_json::from_str(json_input).unwrap();
    println!("  Deserialized User: {:?}", user2);

    // Tagged enum
    let msg = Message::Text { content: "Hello!".into() };
    let json = serde_json::to_string(&msg).unwrap();
    println!("  Tagged enum: {}", json);
}

// ============================================
// thiserror Derive Macro
// ============================================

#[derive(Error, Debug)]
enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {msg}")]
    Parse { msg: String },

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Multiple errors occurred")]
    Multiple(Vec<String>),
}

fn thiserror_example() {
    let err1 = AppError::NotFound("user_123".into());
    println!("  Error display: {}", err1);
    println!("  Error debug: {:?}", err1);

    let err2 = AppError::Parse { msg: "invalid JSON".into() };
    println!("  Error display: {}", err2);

    // From conversion
    fn read_file() -> Result<String, AppError> {
        // std::io::Error automatically converts to AppError::Io
        let content = std::fs::read_to_string("/nonexistent")?;
        Ok(content)
    }

    match read_file() {
        Ok(_) => println!("  File read successfully"),
        Err(e) => println!("  Expected error: {}", e),
    }
}

// ============================================
// How Derive Macros Work (Conceptually)
// ============================================

fn derive_internals_explanation() {
    println!("  Derive macros transform code at compile time:");
    println!();

    println!("  Input (what you write):");
    println!("  ┌────────────────────────────────────┐");
    println!("  │ #[derive(Debug)]                   │");
    println!("  │ struct Point {{ x: i32, y: i32 }}    │");
    println!("  └────────────────────────────────────┘");
    println!();

    println!("  Output (what the compiler sees):");
    println!("  ┌────────────────────────────────────────────────┐");
    println!("  │ struct Point {{ x: i32, y: i32 }}                │");
    println!("  │                                                │");
    println!("  │ impl std::fmt::Debug for Point {{               │");
    println!("  │     fn fmt(&self, f: &mut std::fmt::Formatter) │");
    println!("  │         -> std::fmt::Result {{                  │");
    println!("  │         f.debug_struct(\"Point\")               │");
    println!("  │             .field(\"x\", &self.x)              │");
    println!("  │             .field(\"y\", &self.y)              │");
    println!("  │             .finish()                          │");
    println!("  │     }}                                          │");
    println!("  │ }}                                               │");
    println!("  └────────────────────────────────────────────────┘");
}

// ============================================
// Common Procedural Macro Patterns
// ============================================

/// Pattern: Builder with derive macro
/// (Simulated - actual implementation would be in proc macro crate)
#[derive(Debug, Clone, Default)]
struct Config {
    host: String,
    port: u16,
    debug: bool,
}

impl Config {
    fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

#[derive(Default)]
struct ConfigBuilder {
    host: Option<String>,
    port: Option<u16>,
    debug: Option<bool>,
}

impl ConfigBuilder {
    fn host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    fn debug(mut self, debug: bool) -> Self {
        self.debug = Some(debug);
        self
    }

    fn build(self) -> Result<Config, &'static str> {
        Ok(Config {
            host: self.host.ok_or("host is required")?,
            port: self.port.unwrap_or(8080),
            debug: self.debug.unwrap_or(false),
        })
    }
}

/// Pattern: FromStr derive (simulated)
#[derive(Debug, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl std::str::FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "north" | "n" => Ok(Direction::North),
            "south" | "s" => Ok(Direction::South),
            "east" | "e" => Ok(Direction::East),
            "west" | "w" => Ok(Direction::West),
            _ => Err(format!("Unknown direction: {}", s)),
        }
    }
}

fn common_patterns() {
    // Builder pattern
    let config = Config::builder()
        .host("localhost")
        .port(3000)
        .debug(true)
        .build()
        .unwrap();
    println!("  Config: {:?}", config);

    // FromStr pattern
    let dir: Direction = "North".parse().unwrap();
    println!("  Parsed direction: {:?}", dir);

    let dir2: Direction = "w".parse().unwrap();
    println!("  Parsed 'w': {:?}", dir2);

    // Proc macro crate structure explanation
    println!();
    println!("  To create a proc macro crate:");
    println!("  ┌───────────────────────────────────────┐");
    println!("  │ # Cargo.toml                          │");
    println!("  │ [lib]                                 │");
    println!("  │ proc-macro = true                     │");
    println!("  │                                       │");
    println!("  │ [dependencies]                        │");
    println!("  │ syn = \"2\"                             │");
    println!("  │ quote = \"1\"                           │");
    println!("  │ proc-macro2 = \"1\"                     │");
    println!("  └───────────────────────────────────────┘");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_debug() {
        let p = Point { x: 1, y: 2 };
        let debug_str = format!("{:?}", p);
        assert!(debug_str.contains("Point"));
        assert!(debug_str.contains("x: 1"));
    }

    #[test]
    fn test_derive_clone() {
        let p1 = Point { x: 5, y: 10 };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }

    #[test]
    fn test_serde_roundtrip() {
        let user = User {
            id: 42,
            name: "Test".into(),
            email: None,
        };
        let json = serde_json::to_string(&user).unwrap();
        let user2: User = serde_json::from_str(&json).unwrap();
        assert_eq!(user.id, user2.id);
        assert_eq!(user.name, user2.name);
    }

    #[test]
    fn test_builder() {
        let config = Config::builder()
            .host("test")
            .port(9000)
            .build()
            .unwrap();
        assert_eq!(config.host, "test");
        assert_eq!(config.port, 9000);
        assert!(!config.debug);
    }

    #[test]
    fn test_direction_parse() {
        assert_eq!("North".parse::<Direction>().unwrap(), Direction::North);
        assert_eq!("s".parse::<Direction>().unwrap(), Direction::South);
        assert!("invalid".parse::<Direction>().is_err());
    }
}
