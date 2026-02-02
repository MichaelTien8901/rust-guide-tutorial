---
layout: default
title: Part 2 - Fundamentals
nav_order: 3
has_children: true
permalink: /part2/
---

# Part 2: Rust Fundamentals

This part covers the core concepts that make Rust unique: ownership, borrowing, and lifetimes. Mastering these concepts is essential for writing idiomatic Rust code.

## What You'll Learn

- Variables, mutability, and data types
- Functions and control flow
- Rust's ownership system
- References and borrowing
- Lifetimes
- Structs and enums
- Pattern matching
- Error handling
- Traits and generics

## Chapters

1. [Variables and Types]({% link part2/01-variables-types.md %}) - let, mut, scalars, and compounds
2. [Functions]({% link part2/02-functions.md %}) - Syntax, parameters, and returns
3. [Ownership]({% link part2/03-ownership.md %}) - Rust's unique memory model
4. [Borrowing]({% link part2/04-borrowing.md %}) - References and the borrowing rules
5. [Lifetimes]({% link part2/05-lifetimes.md %}) - Annotations and elision
6. [Structs]({% link part2/06-structs.md %}) - Custom data types
7. [Enums]({% link part2/07-enums.md %}) - Pattern matching with Option and Result
8. [Error Handling]({% link part2/08-error-handling.md %}) - Panic, Result, and the ? operator
9. [Traits]({% link part2/09-traits.md %}) - Shared behavior
10. [Generics]({% link part2/10-generics.md %}) - Type parameters and bounds

## The Big Picture

```mermaid
graph TD
    A[Variables] --> B[Ownership]
    B --> C[Borrowing]
    C --> D[Lifetimes]

    E[Structs] --> F[Methods]
    G[Enums] --> H[Pattern Matching]

    F --> I[Traits]
    H --> I
    I --> J[Generics]

    D --> K[Error Handling]
    H --> K
```

## Prerequisites

- Completed [Part 1: Getting Started]({% link part1/index.md %})
- Rust installed and working
- Basic programming knowledge

## Key Concepts Preview

### Ownership

Every value has exactly one owner. When the owner goes out of scope, the value is dropped:

```rust
{
    let s = String::from("hello");  // s owns the String
}  // s goes out of scope, String is dropped
```

### Borrowing

You can borrow references to values without taking ownership:

```rust
fn calculate_length(s: &String) -> usize {
    s.len()  // Use s, but don't own it
}
```

### Lifetimes

Lifetimes ensure references are valid:

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

{: .tip }
Don't worry if these look confusing now. Each chapter explains these concepts step by step with plenty of examples.

## Time Estimate

Plan for 4-6 hours to work through this part thoroughly. Take breaks between chapters to let the concepts sink in.

## Next Steps

Start with [Variables and Types]({% link part2/01-variables-types.md %}) to begin your Rust journey.
