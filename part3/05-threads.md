---
layout: default
title: Threads
parent: Part 3 - Intermediate
nav_order: 5
---

# Threads

Rust provides safe concurrent programming through its ownership system.

## Spawning Threads

```rust
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();  // Wait for thread to finish
}
```

## Moving Data into Threads

```rust
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("{:?}", v);
    });

    handle.join().unwrap();
}
```

## Thread Builder

```rust
use std::thread;

fn main() {
    let builder = thread::Builder::new()
        .name("worker".into())
        .stack_size(32 * 1024);

    let handle = builder.spawn(|| {
        println!("Named thread");
    }).unwrap();

    handle.join().unwrap();
}
```

## Returning Values

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        let sum: i32 = (1..=100).sum();
        sum
    });

    let result = handle.join().unwrap();
    println!("Sum: {}", result);
}
```

## Scoped Threads

Borrow data without moving:

```rust
use std::thread;

fn main() {
    let data = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("{:?}", data);  // Borrows data
        });
    });

    println!("{:?}", data);  // data still valid
}
```

## See Also

- [Real-Time Constraints]({% link part6/06-real-time.md %}) - Deterministic threading for embedded systems
- [RTOS Integration]({% link part6/07-rtos.md %}) - Real-time operating system patterns

## Next Steps

Learn about [Channels]({% link part3/06-channels.md %}) for message passing between threads.
