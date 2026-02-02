---
layout: default
title: Async Runtimes
parent: Libraries
grand_parent: Appendices
nav_order: 2
---

# Async Runtimes

Libraries for executing asynchronous Rust code.

## tokio

The most popular async runtime for Rust.

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### Basic Usage

```rust
#[tokio::main]
async fn main() {
    println!("Hello, async world!");

    // Spawn a task
    let handle = tokio::spawn(async {
        // Async work
        42
    });

    let result = handle.await.unwrap();
}
```

### Features

| Feature | Purpose |
|---------|---------|
| `rt` | Basic runtime |
| `rt-multi-thread` | Multi-threaded runtime |
| `macros` | `#[tokio::main]`, `#[tokio::test]` |
| `io-util` | I/O utilities |
| `net` | TCP/UDP networking |
| `time` | Timers and delays |
| `sync` | Channels, mutexes |
| `fs` | Async filesystem |
| `full` | All features |

### Spawning Tasks

```rust
use tokio::task;

// Spawn async task
let handle = tokio::spawn(async {
    expensive_async_work().await
});

// Spawn blocking task (for CPU-bound work)
let result = task::spawn_blocking(|| {
    expensive_cpu_work()
}).await?;

// Run on current thread
let local = task::LocalSet::new();
local.run_until(async {
    task::spawn_local(async { /* ... */ }).await
}).await;
```

### Channels

```rust
use tokio::sync::{mpsc, oneshot, broadcast, watch};

// Multi-producer, single-consumer
let (tx, mut rx) = mpsc::channel(32);
tx.send("message").await?;
let msg = rx.recv().await;

// One-shot (single value)
let (tx, rx) = oneshot::channel();
tx.send("done")?;
let result = rx.await?;

// Broadcast (multi-consumer)
let (tx, mut rx1) = broadcast::channel(16);
let mut rx2 = tx.subscribe();

// Watch (latest value)
let (tx, rx) = watch::channel("initial");
tx.send("updated")?;
```

### Time

```rust
use tokio::time::{sleep, timeout, interval, Duration};

// Sleep
sleep(Duration::from_secs(1)).await;

// Timeout
match timeout(Duration::from_secs(5), long_operation()).await {
    Ok(result) => println!("Completed: {:?}", result),
    Err(_) => println!("Timed out"),
}

// Interval
let mut interval = interval(Duration::from_millis(100));
loop {
    interval.tick().await;
    // Runs every 100ms
}
```

### Synchronization

```rust
use tokio::sync::{Mutex, RwLock, Semaphore};

// Async mutex
let data = Mutex::new(0);
{
    let mut lock = data.lock().await;
    *lock += 1;
}

// Read-write lock
let data = RwLock::new(vec![]);
let read = data.read().await;
let mut write = data.write().await;

// Semaphore
let semaphore = Semaphore::new(3);
let permit = semaphore.acquire().await?;
```

## async-std

Alternative async runtime with std-like API.

```toml
[dependencies]
async-std = { version = "1", features = ["attributes"] }
```

```rust
use async_std::prelude::*;
use async_std::task;

#[async_std::main]
async fn main() {
    let handle = task::spawn(async {
        42
    });

    let result = handle.await;
}
```

### async-std Features

```rust
use async_std::{fs, net, channel, sync};

// File I/O
let content = fs::read_to_string("file.txt").await?;

// Networking
let stream = net::TcpStream::connect("127.0.0.1:8080").await?;

// Channels
let (tx, rx) = channel::bounded(10);

// Timeout
use async_std::future::timeout;
timeout(Duration::from_secs(5), async_operation()).await?;
```

## smol

Minimal async runtime.

```toml
[dependencies]
smol = "2"
```

```rust
use smol::{block_on, spawn};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    block_on(async {
        let task = spawn(async { 1 + 2 });
        let result = task.await;
        println!("Result: {}", result);
        Ok(())
    })
}
```

## futures

Core async utilities (runtime-agnostic).

```toml
[dependencies]
futures = "0.3"
```

```rust
use futures::{future, stream, StreamExt, FutureExt};

// Join multiple futures
let (a, b, c) = future::join3(fut1, fut2, fut3).await;

// Select first to complete
let result = future::select(fut1, fut2).await;

// Try join (fail fast)
let (a, b) = future::try_join(fut1, fut2).await?;

// Stream processing
let results: Vec<_> = stream::iter(items)
    .map(|x| async move { process(x).await })
    .buffer_unordered(10)
    .collect()
    .await;
```

## Comparison

| Runtime | Size | Features | Best For |
|---------|------|----------|----------|
| tokio | Large | Full-featured | Production apps |
| async-std | Medium | Std-like API | Familiar API |
| smol | Small | Minimal | Embedded, small apps |

## Choosing a Runtime

| Use Case | Recommendation |
|----------|----------------|
| Web servers | tokio |
| General async | tokio or async-std |
| Minimal dependencies | smol |
| Library development | futures (runtime-agnostic) |

## Runtime-Agnostic Code

```rust
// Use futures traits for compatibility
use futures::{AsyncRead, AsyncWrite};

async fn copy<R, W>(reader: R, writer: W) -> std::io::Result<u64>
where
    R: AsyncRead + Unpin,
    W: AsyncWrite + Unpin,
{
    futures::io::copy(reader, writer).await
}
```

## Summary

| Crate | Purpose |
|-------|---------|
| tokio | Full-featured async runtime |
| async-std | Std-like async runtime |
| smol | Minimal async runtime |
| futures | Async utilities and traits |
