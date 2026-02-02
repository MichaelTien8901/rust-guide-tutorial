---
layout: default
title: Part 3 - Intermediate
nav_order: 4
has_children: true
permalink: /part3/
---

# Part 3: Intermediate Rust

This part covers collections, iterators, closures, smart pointers, and concurrency—the tools you need for real-world Rust programming.

## What You'll Learn

- Working with collections (Vec, HashMap, String)
- Iterator patterns and adaptors
- Closures and capturing
- Smart pointers (Box, Rc, Arc, RefCell)
- Multi-threaded programming
- Channels for message passing
- Mutexes and shared state
- Introduction to async/await

## Chapters

1. [Collections]({% link part3/01-collections.md %}) - Vec, HashMap, and String
2. [Iterators]({% link part3/02-iterators.md %}) - Patterns and adapters
3. [Closures]({% link part3/03-closures.md %}) - Anonymous functions and capturing
4. [Smart Pointers]({% link part3/04-smart-pointers.md %}) - Box, Rc, Arc, RefCell
5. [Threads]({% link part3/05-threads.md %}) - Spawning and joining
6. [Channels]({% link part3/06-channels.md %}) - Message passing
7. [Mutex]({% link part3/07-mutex.md %}) - Shared state concurrency
8. [Async Basics]({% link part3/08-async-basics.md %}) - Introduction to async/await

## The Big Picture

```mermaid
graph TD
    A[Collections] --> B[Iterators]
    B --> C[Closures]
    C --> D[Smart Pointers]

    D --> E[Concurrency]
    E --> F[Threads]
    E --> G[Channels]
    E --> H[Mutex]

    I[Async] --> J[Futures]
    I --> K[async/await]
```

## Prerequisites

- Completed [Part 2: Fundamentals]({% link part2/index.md %})
- Understanding of ownership and borrowing
- Familiarity with traits and generics

## Time Estimate

Plan for 4-6 hours to work through this part. Concurrency concepts may require extra practice.

## Next Steps

Start with [Collections]({% link part3/01-collections.md %}) to learn about Rust's data structures.
