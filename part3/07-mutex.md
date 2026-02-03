---
layout: default
title: Mutex
parent: Part 3 - Intermediate
nav_order: 7
---

# Mutex and Shared State

Mutex provides mutual exclusion for shared data between threads.

## Basic Mutex

```rust
use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // Lock released here

    println!("{:?}", m);
}
```

## Sharing Between Threads

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```

## RwLock

Multiple readers or single writer:

```rust
use std::sync::RwLock;

fn main() {
    let lock = RwLock::new(5);

    // Multiple readers
    {
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        println!("{} {}", *r1, *r2);
    }

    // Single writer
    {
        let mut w = lock.write().unwrap();
        *w += 1;
    }
}
```

## Deadlock Prevention

- Always lock in the same order
- Use `try_lock()` for non-blocking attempts
- Keep lock scopes short

## See Also

- [Real-Time Constraints]({% link part6/06-real-time.md %}) - Lock-free patterns for embedded systems

## Next Steps

Learn about [Async Basics]({% link part3/08-async-basics.md %}) for asynchronous programming.
