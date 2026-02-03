---
layout: default
title: Async Basics
parent: Part 3 - Intermediate
nav_order: 8
---

# Async Basics

Rust's async/await enables efficient asynchronous programming.

## Async Functions

```rust
async fn hello() {
    println!("Hello, async world!");
}

async fn fetch_data() -> String {
    // Simulated async operation
    "data".to_string()
}
```

## Awaiting Futures

```rust
async fn main_async() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

## Running Async Code

Requires a runtime like tokio:

```rust
// Cargo.toml:
// [dependencies]
// tokio = { version = "1", features = ["full"] }

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("{}", result);
}
```

## Concurrent Execution

```rust
use tokio;

#[tokio::main]
async fn main() {
    let task1 = tokio::spawn(async {
        // Task 1
        1
    });

    let task2 = tokio::spawn(async {
        // Task 2
        2
    });

    let (r1, r2) = tokio::join!(task1, task2);
    println!("{:?} {:?}", r1, r2);
}
```

## Key Concepts

| Concept | Description |
|---------|-------------|
| `async fn` | Declares async function |
| `.await` | Suspends until ready |
| `Future` | Represents async computation |
| Runtime | Executes futures (tokio, async-std) |

## See Also

- [Web Services]({% link part5/05-web-services.md %}) - Build async web applications
- [Async Runtimes]({% link appendices/libraries/async-runtimes.md %}) - Runtime comparison and details

## Next Steps

Continue to [Part 4: Advanced]({% link part4/index.md %}) for unsafe, FFI, and macros.
