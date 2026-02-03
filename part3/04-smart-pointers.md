---
layout: default
title: Smart Pointers
parent: Part 3 - Intermediate
nav_order: 4
---

# Smart Pointers

Smart pointers are data structures that act like pointers but have additional metadata and capabilities.

## Box<T> - Heap Allocation

```rust
fn main() {
    // Allocate on heap
    let b = Box::new(5);
    println!("b = {}", b);  // Auto-derefs

    // Useful for recursive types
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
```

## Rc<T> - Reference Counting

For shared ownership (single-threaded):

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(5);
    let b = Rc::clone(&a);  // Increment ref count
    let c = Rc::clone(&a);

    println!("count: {}", Rc::strong_count(&a));  // 3
}
```

## Arc<T> - Atomic Reference Counting

Thread-safe version of `Rc`:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let data = Arc::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3).map(|_| {
        let data = Arc::clone(&data);
        thread::spawn(move || {
            println!("{:?}", data);
        })
    }).collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
```

## RefCell<T> - Interior Mutability

Borrow checking at runtime:

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);

    // Borrow mutably at runtime
    *data.borrow_mut() += 1;

    println!("{}", data.borrow());  // 6
}
```

## Combining Rc and RefCell

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared = Rc::new(RefCell::new(vec![1, 2, 3]));

    let a = Rc::clone(&shared);
    let b = Rc::clone(&shared);

    a.borrow_mut().push(4);
    b.borrow_mut().push(5);

    println!("{:?}", shared.borrow());  // [1, 2, 3, 4, 5]
}
```

## Weak<T> - Weak References

Prevent reference cycles:

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}
```

## Summary

| Type | Ownership | Thread-Safe | Mutability |
|------|-----------|-------------|------------|
| `Box<T>` | Single | No | If `mut` |
| `Rc<T>` | Shared | No | Read-only |
| `Arc<T>` | Shared | Yes | Read-only |
| `RefCell<T>` | Single | No | Runtime checked |
| `Mutex<T>` | Shared | Yes | Locked access |

## See Also

- [Memory Layout]({% link part4/08-memory-layout.md %}) - How pointers are represented in memory
- [Ownership]({% link part2/03-ownership.md %}) - Foundational ownership concepts

## Next Steps

Learn about [Threads]({% link part3/05-threads.md %}) for concurrent programming.
