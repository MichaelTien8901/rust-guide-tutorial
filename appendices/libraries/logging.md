---
layout: default
title: Logging
parent: Libraries
grand_parent: Appendices
nav_order: 5
---

# Logging Libraries

Libraries for application logging and diagnostics.

## log

The standard logging facade for Rust.

```toml
[dependencies]
log = "0.4"
```

### Basic Usage

```rust
use log::{trace, debug, info, warn, error};

fn process_data() {
    trace!("Entering process_data");
    debug!("Processing {} items", 42);
    info!("Processing complete");
    warn!("Disk space low");
    error!("Failed to write file: {}", err);
}
```

### Log Levels

| Level | Purpose |
|-------|---------|
| `error!` | Errors that prevent operation |
| `warn!` | Warnings, recoverable issues |
| `info!` | General information |
| `debug!` | Debug information |
| `trace!` | Fine-grained tracing |

## env_logger

Simple environment-based logger.

```toml
[dependencies]
env_logger = "0.11"
log = "0.4"
```

```rust
fn main() {
    env_logger::init();

    log::info!("Application started");
}
```

```bash
# Set log level via environment
RUST_LOG=info ./myapp
RUST_LOG=debug ./myapp
RUST_LOG=mymodule=trace ./myapp
RUST_LOG=myapp=debug,other_crate=warn ./myapp
```

### Custom Format

```rust
use env_logger::Builder;
use std::io::Write;

Builder::new()
    .format(|buf, record| {
        writeln!(
            buf,
            "{} [{}] {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter_level(log::LevelFilter::Info)
    .init();
```

## tracing

Modern, async-aware diagnostics framework.

```toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
```

### Basic Usage

```rust
use tracing::{info, debug, warn, error, span, Level};

fn main() {
    tracing_subscriber::fmt::init();

    info!("Application started");

    let span = span!(Level::INFO, "processing", items = 42);
    let _guard = span.enter();

    debug!("Processing items");
}
```

### Spans

```rust
use tracing::{info_span, instrument};

#[instrument]
fn process_request(request_id: u64) {
    // Automatically creates span with function name and args
    info!("Processing request");
    do_work();
}

fn manual_span() {
    let span = info_span!("my_operation", key = "value");
    let _guard = span.enter();

    // All logs here include span context
    info!("Inside span");
}
```

### Async Support

```rust
use tracing::instrument;

#[instrument]
async fn async_operation() {
    // Span persists across await points
    do_async_work().await;
    info!("Work complete");
}
```

### Subscribers

```rust
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

fn setup_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}
```

## tracing-subscriber

Configurable tracing subscriber.

```toml
[dependencies]
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### JSON Output

```rust
use tracing_subscriber::fmt;

fn main() {
    tracing_subscriber::fmt()
        .json()
        .init();

    tracing::info!(user = "alice", action = "login", "User logged in");
}
```

### Multiple Outputs

```rust
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};

tracing_subscriber::registry()
    .with(fmt::layer().with_writer(std::io::stdout))
    .with(fmt::layer().json().with_writer(file))
    .init();
```

## tracing-appender

Non-blocking file logging.

```toml
[dependencies]
tracing-appender = "0.2"
```

```rust
use tracing_appender::rolling::{RollingFileAppender, Rotation};

let file_appender = RollingFileAppender::new(
    Rotation::DAILY,
    "/var/log/myapp",
    "app.log",
);

let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

tracing_subscriber::fmt()
    .with_writer(non_blocking)
    .init();

// Keep _guard alive for the duration of the program
```

## fern

Flexible logging configuration.

```toml
[dependencies]
fern = "0.6"
log = "0.4"
chrono = "0.4"
```

```rust
use fern::Dispatch;

fn setup_logger() -> Result<(), fern::InitError> {
    Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "[{} {} {}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.target(),
                message
            ))
        })
        .level(log::LevelFilter::Info)
        .level_for("noisy_crate", log::LevelFilter::Warn)
        .chain(std::io::stdout())
        .chain(fern::log_file("output.log")?)
        .apply()?;
    Ok(())
}
```

## slog

Structured logging.

```toml
[dependencies]
slog = "2"
slog-term = "2"
slog-async = "2"
```

```rust
use slog::{o, info, Logger, Drain};

fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = Logger::root(drain, o!("version" => "1.0"));

    info!(log, "Application started"; "port" => 8080);
}
```

## Comparison

| Crate | Style | Async | Structured |
|-------|-------|-------|------------|
| log + env_logger | Simple | No | No |
| tracing | Modern | Yes | Yes |
| fern | Flexible | No | No |
| slog | Structured | Yes | Yes |

## Best Practices

### Library Code

```rust
// In library code, use log or tracing macros
// Let the application choose the implementation

use log::debug;

pub fn library_function() {
    debug!("Library debug message");
}
```

### Application Code

```rust
// In application code, set up the subscriber/logger

fn main() {
    // Choose one logging implementation
    tracing_subscriber::fmt::init();

    // Or for simpler needs
    env_logger::init();
}
```

### Structured Fields

```rust
use tracing::{info, instrument};

#[instrument(skip(password))]
fn login(username: &str, password: &str) -> Result<(), Error> {
    info!(username, "Login attempt");
    // password is not logged due to skip
    Ok(())
}
```

## Summary

| Crate | Use Case |
|-------|----------|
| log | Logging facade |
| env_logger | Simple logging |
| tracing | Modern diagnostics |
| tracing-subscriber | Tracing configuration |
| tracing-appender | File logging |
| fern | Flexible log routing |
| slog | Structured logging |

## Choosing a Solution

| Need | Recommendation |
|------|----------------|
| Simple logging | log + env_logger |
| Async applications | tracing |
| Structured logs | tracing or slog |
| File rotation | tracing-appender |
| Complex routing | fern |
