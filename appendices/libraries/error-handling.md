---
layout: default
title: Error Handling
parent: Libraries
grand_parent: Appendices
nav_order: 6
---

# Error Handling Libraries

Libraries for creating and managing errors in Rust.

## thiserror

Derive macro for custom error types.

```toml
[dependencies]
thiserror = "1.0"
```

### Basic Usage

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(#[from] std::num::ParseIntError),
}
```

### Attributes

| Attribute | Purpose |
|-----------|---------|
| `#[error("...")]` | Error message |
| `#[from]` | Implement From trait |
| `#[source]` | Mark error source |
| `#[transparent]` | Delegate Display/source |

### Advanced Usage

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Connection failed to {host}:{port}")]
    ConnectionFailed {
        host: String,
        port: u16,
        #[source]
        source: std::io::Error,
    },

    #[error("Query failed: {query}")]
    QueryFailed {
        query: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },

    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
```

## anyhow

Flexible error handling for applications.

```toml
[dependencies]
anyhow = "1.0"
```

### Basic Usage

```rust
use anyhow::{Result, Context, anyhow, bail};

fn read_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.toml")
        .context("Failed to read config file")?;

    let config: Config = toml::from_str(&content)
        .context("Failed to parse config")?;

    Ok(config)
}

fn validate(value: i32) -> Result<()> {
    if value < 0 {
        bail!("Value must be non-negative, got {}", value);
    }
    Ok(())
}

fn create_error() -> Result<()> {
    Err(anyhow!("Something went wrong"))
}
```

### Context and Chaining

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<()> {
    let data = std::fs::read(path)
        .with_context(|| format!("Failed to read {}", path))?;

    parse_data(&data)
        .context("Failed to parse data")?;

    Ok(())
}
```

### Downcasting

```rust
use anyhow::Result;

fn handle_error(result: Result<()>) {
    if let Err(err) = result {
        // Try to downcast to specific error type
        if let Some(io_err) = err.downcast_ref::<std::io::Error>() {
            eprintln!("IO Error: {}", io_err);
        } else {
            eprintln!("Error: {}", err);
        }

        // Print full error chain
        for cause in err.chain() {
            eprintln!("Caused by: {}", cause);
        }
    }
}
```

## eyre

Fork of anyhow with better error reporting.

```toml
[dependencies]
eyre = "0.6"
color-eyre = "0.6"
```

```rust
use eyre::{Result, WrapErr, eyre};
use color_eyre::eyre::Report;

fn main() -> Result<()> {
    color_eyre::install()?;

    do_something().wrap_err("Failed to do something")?;

    Ok(())
}
```

## miette

Diagnostic error reporting with source code.

```toml
[dependencies]
miette = { version = "7", features = ["fancy"] }
```

```rust
use miette::{Diagnostic, SourceSpan, Report};
use thiserror::Error;

#[derive(Error, Diagnostic, Debug)]
#[error("Parse error")]
#[diagnostic(code(parser::syntax_error))]
struct ParseError {
    #[source_code]
    src: String,

    #[label("This bit here")]
    bad_bit: SourceSpan,

    #[help]
    advice: Option<String>,
}

fn main() -> miette::Result<()> {
    miette::set_hook(Box::new(|_| {
        Box::new(miette::MietteHandlerOpts::new().build())
    }))?;

    Err(ParseError {
        src: "let x = ;".into(),
        bad_bit: (8, 1).into(),
        advice: Some("Add an expression after '='".into()),
    })?;

    Ok(())
}
```

## error-stack

Rich error context with stack traces.

```toml
[dependencies]
error-stack = "0.5"
```

```rust
use error_stack::{Report, ResultExt, report};

#[derive(Debug)]
struct ParseError;

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse")
    }
}

impl std::error::Error for ParseError {}

fn parse_config(path: &str) -> Result<Config, Report<ParseError>> {
    let content = std::fs::read_to_string(path)
        .change_context(ParseError)?;

    toml::from_str(&content)
        .change_context(ParseError)
}
```

## Comparison

| Crate | Purpose | Best For |
|-------|---------|----------|
| thiserror | Define errors | Libraries |
| anyhow | Handle errors | Applications |
| eyre | Better reporting | CLI apps |
| miette | Diagnostics | Compilers, linters |
| error-stack | Error context | Complex apps |

## Library vs Application Pattern

### For Libraries

Use `thiserror` to define specific error types:

```rust
// lib.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyLibError {
    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Network error")]
    Network(#[from] std::io::Error),
}

pub fn library_function() -> Result<(), MyLibError> {
    // ...
    Ok(())
}
```

### For Applications

Use `anyhow` for convenient error handling:

```rust
// main.rs
use anyhow::{Result, Context};

fn main() -> Result<()> {
    let config = load_config()
        .context("Failed to load configuration")?;

    run_app(config)?;

    Ok(())
}
```

## Error Conversion

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] DatabaseError),

    #[error("Network error")]
    Network(#[from] NetworkError),
}

// Automatic conversion with ? operator
fn do_work() -> Result<(), AppError> {
    database_operation()?;  // DatabaseError -> AppError
    network_operation()?;   // NetworkError -> AppError
    Ok(())
}
```

## Summary

| Crate | Use Case |
|-------|----------|
| thiserror | Custom error types |
| anyhow | Application errors |
| eyre | Colorful error reports |
| miette | Source code diagnostics |
| error-stack | Error context chains |

## Choosing a Strategy

| Scenario | Recommendation |
|----------|----------------|
| Writing a library | thiserror |
| Writing an application | anyhow |
| CLI with nice errors | eyre + color-eyre |
| Compiler/linter | miette |
| Need full context | error-stack |
