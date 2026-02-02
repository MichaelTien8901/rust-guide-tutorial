---
layout: default
title: Cargo Basics
parent: Part 1 - Getting Started
nav_order: 7
---

# Cargo Basics

Cargo is Rust's build system and package manager. It handles:
- Building your code
- Downloading dependencies
- Running tests
- Generating documentation

## Creating Projects

### New Binary (Executable)

```bash
cargo new my-app
```

Creates:
```
my-app/
├── Cargo.toml
└── src/
    └── main.rs
```

### New Library

```bash
cargo new my-lib --lib
```

Creates:
```
my-lib/
├── Cargo.toml
└── src/
    └── lib.rs
```

### Initialize in Existing Directory

```bash
mkdir my-project
cd my-project
cargo init
```

## Essential Commands

| Command | Description |
|---------|-------------|
| `cargo new <name>` | Create new project |
| `cargo build` | Compile the project |
| `cargo run` | Build and run |
| `cargo test` | Run tests |
| `cargo check` | Check for errors (fast) |
| `cargo doc` | Generate documentation |
| `cargo clean` | Remove build artifacts |

## Build Profiles

### Debug (Default)

```bash
cargo build
# or
cargo build --debug
```

- Fast compilation
- No optimizations
- Debug symbols included
- Output: `target/debug/`

### Release

```bash
cargo build --release
```

- Slower compilation
- Full optimizations
- Smaller binary
- Output: `target/release/`

## Cargo.toml

The project manifest:

```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <you@example.com>"]
description = "A sample project"
license = "MIT"
repository = "https://github.com/you/my-project"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[build-dependencies]
cc = "1.0"
```

### Sections Explained

| Section | Purpose |
|---------|---------|
| `[package]` | Project metadata |
| `[dependencies]` | Runtime dependencies |
| `[dev-dependencies]` | Test/build dependencies |
| `[build-dependencies]` | Build script dependencies |
| `[features]` | Optional features |

## Adding Dependencies

### From crates.io

Add to Cargo.toml:

```toml
[dependencies]
serde = "1.0"
```

Or use the CLI:

```bash
cargo add serde
```

### Version Requirements

| Syntax | Meaning |
|--------|---------|
| `"1.0"` | >= 1.0.0, < 2.0.0 |
| `"1.2.3"` | >= 1.2.3, < 2.0.0 |
| `"=1.2.3"` | Exactly 1.2.3 |
| `">=1.0"` | >= 1.0.0 |
| `"^1.2.3"` | Same as "1.2.3" |
| `"~1.2.3"` | >= 1.2.3, < 1.3.0 |
| `"*"` | Any version |

### With Features

```toml
[dependencies]
tokio = { version = "1.0", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
```

### From Git

```toml
[dependencies]
my-crate = { git = "https://github.com/user/repo" }
my-crate = { git = "https://github.com/user/repo", branch = "main" }
my-crate = { git = "https://github.com/user/repo", tag = "v1.0" }
```

### From Local Path

```toml
[dependencies]
my-local-crate = { path = "../my-local-crate" }
```

## Cargo.lock

Cargo.lock pins exact dependency versions:

```
# Cargo.lock
[[package]]
name = "serde"
version = "1.0.193"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "..."
```

{: .important }
**Commit Cargo.lock** for binaries. For libraries, it's often gitignored.

## Running Programs

### Basic Run

```bash
cargo run
```

### With Arguments

```bash
cargo run -- --flag value
```

### Run Specific Binary

```bash
cargo run --bin my-binary
```

### Run Example

```bash
cargo run --example my-example
```

## Testing

### Run All Tests

```bash
cargo test
```

### Run Specific Test

```bash
cargo test test_name
```

### Run Tests with Output

```bash
cargo test -- --nocapture
```

## Useful Cargo Plugins

Install with `cargo install`:

| Plugin | Purpose |
|--------|---------|
| `cargo-watch` | Auto-rebuild on changes |
| `cargo-edit` | Add/remove dependencies |
| `cargo-expand` | Show macro expansions |
| `cargo-audit` | Security vulnerability check |
| `cargo-outdated` | Check for outdated deps |
| `cargo-tree` | Show dependency tree |

### Example: cargo-watch

```bash
cargo install cargo-watch

# Auto-run on file changes
cargo watch -x run

# Auto-test on changes
cargo watch -x test
```

## Workspaces

For multi-crate projects:

```toml
# Cargo.toml (workspace root)
[workspace]
members = [
    "crate-a",
    "crate-b",
]
```

```
my-workspace/
├── Cargo.toml        # Workspace manifest
├── crate-a/
│   ├── Cargo.toml
│   └── src/
└── crate-b/
    ├── Cargo.toml
    └── src/
```

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `CARGO_HOME` | Cargo's home directory |
| `CARGO_TARGET_DIR` | Override target directory |
| `RUST_BACKTRACE=1` | Enable backtraces |
| `RUST_LOG` | Configure logging |

## Configuration

Create `.cargo/config.toml` for project-specific settings:

```toml
[build]
target-dir = "custom-target"

[alias]
b = "build"
r = "run"
t = "test"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
```

## Quick Reference

```bash
# Project management
cargo new my-app       # New binary
cargo new my-lib --lib # New library
cargo init             # Initialize in current dir

# Building
cargo build            # Debug build
cargo build --release  # Release build
cargo check            # Check without building

# Running
cargo run              # Build and run
cargo run -- args      # Run with arguments

# Testing
cargo test             # Run all tests
cargo test name        # Run specific test

# Dependencies
cargo add serde        # Add dependency
cargo update           # Update dependencies
cargo tree             # Show dependency tree

# Documentation
cargo doc              # Generate docs
cargo doc --open       # Generate and open

# Cleaning
cargo clean            # Remove target/
```

## Next Steps

You now have all the basics to start learning Rust! Continue to [Part 2: Fundamentals]({% link part2/index.md %}) to learn about ownership, borrowing, and other core concepts.
