---
layout: default
title: Channels
parent: Part 3 - Intermediate
nav_order: 6
---

# Channels

Channels enable message passing between threads.

## Basic Channel

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });

    let received = rx.recv().unwrap();
    println!("{}", received);
}
```

## Multiple Messages

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..5 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```

## Multiple Producers

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        tx.send("from tx").unwrap();
    });

    thread::spawn(move || {
        tx2.send("from tx2").unwrap();
    });

    for received in rx {
        println!("{}", received);
    }
}
```

## Non-blocking Receive

```rust
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    match rx.try_recv() {
        Ok(msg) => println!("{}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("No message"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Disconnected"),
    }
}
```

## Next Steps

Learn about [Mutex]({% link part3/07-mutex.md %}) for shared state concurrency.
