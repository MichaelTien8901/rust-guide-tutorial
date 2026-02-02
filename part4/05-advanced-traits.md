---
layout: default
title: Advanced Traits
parent: Part 4 - Advanced
nav_order: 5
---

# Advanced Traits

Associated types, supertraits, and advanced patterns.

## Associated Types

```rust
trait Container {
    type Item;
    fn get(&self) -> Option<&Self::Item>;
}

impl Container for Vec<i32> {
    type Item = i32;
    fn get(&self) -> Option<&i32> {
        self.first()
    }
}
```

## Supertraits

```rust
trait OutlinePrint: std::fmt::Display {
    fn outline_print(&self) {
        println!("* {} *", self);
    }
}
```

## Fully Qualified Syntax

```rust
trait Animal {
    fn name() -> String;
}

struct Dog;
impl Animal for Dog {
    fn name() -> String { "Dog".to_string() }
}

fn main() {
    println!("{}", <Dog as Animal>::name());
}
```

## Next Steps

Learn about [Advanced Types]({% link part4/06-advanced-types.md %}).
