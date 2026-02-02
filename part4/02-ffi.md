---
layout: default
title: FFI
parent: Part 4 - Advanced
nav_order: 2
---

# Foreign Function Interface (FFI)

Call C code from Rust and expose Rust to C.

## Calling C Functions

```rust
extern "C" {
    fn abs(input: i32) -> i32;
    fn sqrt(x: f64) -> f64;
}

fn main() {
    unsafe {
        println!("abs(-3) = {}", abs(-3));
        println!("sqrt(9) = {}", sqrt(9.0));
    }
}
```

## Exposing Rust to C

```rust
#[no_mangle]
pub extern "C" fn rust_function(x: i32) -> i32 {
    x * 2
}
```

## C-Compatible Structs

```rust
#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}
```

## Next Steps

Learn about [Declarative Macros]({% link part4/03-macros-declarative.md %}).
