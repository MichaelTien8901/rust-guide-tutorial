---
layout: default
title: Error Patterns
parent: Part 5 - Patterns
nav_order: 2
---

# Error Patterns

Idiomatic error handling with thiserror and anyhow.

## Custom Error Types with thiserror

The `thiserror` crate derives `std::error::Error` implementations.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DataError {
    #[error("file not found: {0}")]
    NotFound(String),

    #[error("parse error at line {line}: {message}")]
    Parse { line: usize, message: String },

    #[error("io error")]
    Io(#[from] std::io::Error),

    #[error("invalid input: {0}")]
    Invalid(#[source] Box<dyn std::error::Error + Send + Sync>),
}

fn load_data(path: &str) -> Result<String, DataError> {
    if path.is_empty() {
        return Err(DataError::NotFound("empty path".into()));
    }

    let content = std::fs::read_to_string(path)?; // Auto-converts io::Error
    Ok(content)
}
```

Add to Cargo.toml:
```toml
[dependencies]
thiserror = "1"
```

## Quick Prototyping with anyhow

The `anyhow` crate provides a flexible error type for applications.

```rust
use anyhow::{Context, Result, bail, ensure};

fn read_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .context("failed to read config file")?;

    let config: Config = serde_json::from_str(&content)
        .context("failed to parse config")?;

    ensure!(config.port > 0, "port must be positive");

    if config.name.is_empty() {
        bail!("name cannot be empty");
    }

    Ok(config)
}

fn main() -> Result<()> {
    let config = read_config("config.json")?;
    println!("Loaded: {:?}", config);
    Ok(())
}
```

Add to Cargo.toml:
```toml
[dependencies]
anyhow = "1"
```

## thiserror vs anyhow

| Crate | Use For |
|-------|---------|
| `thiserror` | Libraries, specific error types |
| `anyhow` | Applications, quick prototyping |

```mermaid
graph LR
    A[Library Code] --> B[thiserror]
    C[Application Code] --> D[anyhow]
    B --> E[Specific errors for API]
    D --> F[Easy error propagation]
```

## Error Context Chains

```rust
use anyhow::{Context, Result};

fn process_file(path: &str) -> Result<()> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read {}", path))?;

    let data = parse(&content)
        .context("failed to parse content")?;

    save(&data)
        .context("failed to save processed data")?;

    Ok(())
}

fn main() {
    if let Err(e) = process_file("data.txt") {
        // Prints full error chain
        eprintln!("Error: {:#}", e);

        // Iterate through causes
        for cause in e.chain() {
            eprintln!("  Caused by: {}", cause);
        }
    }
}
```

## Downcasting Errors

```rust
use anyhow::Result;
use std::io;

fn handle_error(result: Result<()>) {
    if let Err(e) = result {
        // Try to downcast to specific error type
        if let Some(io_err) = e.downcast_ref::<io::Error>() {
            match io_err.kind() {
                io::ErrorKind::NotFound => println!("File not found"),
                io::ErrorKind::PermissionDenied => println!("Access denied"),
                _ => println!("IO error: {}", io_err),
            }
        } else {
            println!("Other error: {}", e);
        }
    }
}
```

## Error Wrapping Pattern

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("configuration error")]
    Config(#[source] ConfigError),

    #[error("database error")]
    Database(#[source] DatabaseError),

    #[error("network error")]
    Network(#[source] NetworkError),
}

// Specific error types for each subsystem
#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("missing field: {0}")]
    MissingField(String),
    #[error("invalid value: {0}")]
    InvalidValue(String),
}

impl From<ConfigError> for AppError {
    fn from(e: ConfigError) -> Self {
        AppError::Config(e)
    }
}
```

## Result Type Aliases

```rust
// In your library's error module
pub type Result<T> = std::result::Result<T, Error>;

// Usage in library code
pub fn connect() -> Result<Connection> {
    // ...
}
```

## Error Reporting

For CLI applications, use color-eyre for pretty error reports.

```rust
use color_eyre::eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;

    // Your application code
    process_data()?;

    Ok(())
}
```

## Best Practices

1. **Libraries**: Use `thiserror` with specific error enums
2. **Applications**: Use `anyhow` for flexible error handling
3. **Always add context** when propagating errors
4. **Don't lose information** - use `#[from]` and `#[source]`
5. **Use `bail!` and `ensure!`** for cleaner error creation

## Summary

| Pattern | Code |
|---------|------|
| Custom error | `#[derive(Error)]` |
| Wrap io::Error | `#[error("...")]` `Io(#[from] io::Error)` |
| Add context | `.context("message")?` |
| Early return | `bail!("error message")` |
| Assert condition | `ensure!(cond, "message")` |

## Next Steps

Learn about [State Machine]({% link part5/03-state-machine.md %}) patterns with enums.
