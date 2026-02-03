---
layout: default
title: Unsafe Rust
parent: Part 4 - Advanced
nav_order: 1
---

# Unsafe Rust

Unsafe Rust gives you superpowers that the borrow checker can't verify.

## The Five Unsafe Superpowers

1. Dereference raw pointers
2. Call unsafe functions
3. Access mutable static variables
4. Implement unsafe traits
5. Access union fields

## Raw Pointers

```rust
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1: {}", *r1);
        *r2 = 10;
        println!("r2: {}", *r2);
    }
}
```

## Unsafe Functions

```rust
unsafe fn dangerous() {
    // Unsafe operations here
}

fn main() {
    unsafe {
        dangerous();
    }
}
```

## Safe Abstractions over Unsafe

```rust
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            std::slice::from_raw_parts_mut(ptr, mid),
            std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```

{: .warning }
Only use unsafe when necessary. Document safety invariants.

## See Also

- [Memory Layout]({% link part4/08-memory-layout.md %}) - Understanding memory representation
- [no_std Basics]({% link part6/01-no-std.md %}) - Systems programming without the standard library

## Next Steps

Learn about [FFI]({% link part4/02-ffi.md %}) for calling C code.
