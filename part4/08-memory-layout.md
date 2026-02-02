---
layout: default
title: Memory Layout
parent: Part 4 - Advanced
nav_order: 8
---

# Memory Layout

Struct layout, padding, and repr attributes.

## Checking Size and Alignment

```rust
use std::mem;

struct Example {
    a: u8,
    b: u32,
    c: u8,
}

fn main() {
    println!("Size: {}", mem::size_of::<Example>());
    println!("Align: {}", mem::align_of::<Example>());
}
```

## Repr Attributes

```rust
#[repr(C)]
struct CCompatible {
    a: u8,
    b: u32,
}

#[repr(packed)]
struct Packed {
    a: u8,
    b: u32,
}
```

## Next Steps

Continue to [Part 5: Patterns]({% link part5/index.md %}).
