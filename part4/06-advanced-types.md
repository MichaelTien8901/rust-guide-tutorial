---
layout: default
title: Advanced Types
parent: Part 4 - Advanced
nav_order: 6
---

# Advanced Types

Newtype, type aliases, and dynamically sized types.

## Newtype Pattern

```rust
struct Meters(f64);
struct Feet(f64);

// Type safety: can't accidentally mix units
fn add_meters(a: Meters, b: Meters) -> Meters {
    Meters(a.0 + b.0)
}
```

## Type Aliases

```rust
type Kilometers = i32;
type Result<T> = std::result::Result<T, std::io::Error>;
```

## Dynamically Sized Types

```rust
fn generic<T: ?Sized>(t: &T) {
    // T can be unsized (like str or [i32])
}
```

## Next Steps

Learn about [Performance]({% link part4/07-performance.md %}).
