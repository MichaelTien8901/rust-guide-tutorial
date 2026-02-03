---
layout: default
title: Iterators
parent: Part 3 - Intermediate
nav_order: 2
---

# Iterators

Iterators provide a way to process sequences of elements. They're lazy, composable, and zero-cost.

## The Iterator Trait

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

## Creating Iterators

```rust
fn main() {
    let v = vec![1, 2, 3];

    // iter() - borrows
    for x in v.iter() {
        println!("{}", x);  // x is &i32
    }

    // iter_mut() - mutably borrows
    let mut v = vec![1, 2, 3];
    for x in v.iter_mut() {
        *x += 10;  // x is &mut i32
    }

    // into_iter() - consumes
    let v = vec![1, 2, 3];
    for x in v.into_iter() {
        println!("{}", x);  // x is i32
    }
    // v is no longer valid
}
```

## Iterator Adapters

Adapters transform iterators lazily:

### map()

```rust
fn main() {
    let v = vec![1, 2, 3];
    let doubled: Vec<_> = v.iter().map(|x| x * 2).collect();
    // [2, 4, 6]
}
```

### filter()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let evens: Vec<_> = v.iter().filter(|x| *x % 2 == 0).collect();
    // [2, 4]
}
```

### take() and skip()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let first_three: Vec<_> = v.iter().take(3).collect();
    // [1, 2, 3]

    let skip_two: Vec<_> = v.iter().skip(2).collect();
    // [3, 4, 5]
}
```

### enumerate()

```rust
fn main() {
    let v = vec!['a', 'b', 'c'];
    for (i, c) in v.iter().enumerate() {
        println!("{}: {}", i, c);
    }
    // 0: a, 1: b, 2: c
}
```

### zip()

```rust
fn main() {
    let a = [1, 2, 3];
    let b = ["one", "two", "three"];

    let pairs: Vec<_> = a.iter().zip(b.iter()).collect();
    // [(1, "one"), (2, "two"), (3, "three")]
}
```

### chain()

```rust
fn main() {
    let a = vec![1, 2];
    let b = vec![3, 4];

    let combined: Vec<_> = a.iter().chain(b.iter()).collect();
    // [1, 2, 3, 4]
}
```

### flatten()

```rust
fn main() {
    let nested = vec![vec![1, 2], vec![3, 4]];
    let flat: Vec<_> = nested.into_iter().flatten().collect();
    // [1, 2, 3, 4]
}
```

### flat_map()

```rust
fn main() {
    let words = vec!["hello", "world"];
    let chars: Vec<_> = words.iter().flat_map(|s| s.chars()).collect();
    // ['h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd']
}
```

## Consuming Adapters

These consume the iterator:

### collect()

```rust
fn main() {
    let v = vec![1, 2, 3];
    let set: std::collections::HashSet<_> = v.into_iter().collect();
}
```

### sum() and product()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let total: i32 = v.iter().sum();     // 15
    let product: i32 = v.iter().product();  // 120
}
```

### fold()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let sum = v.iter().fold(0, |acc, x| acc + x);  // 15

    // With more complex accumulator
    let result = v.iter().fold(String::new(), |acc, x| {
        format!("{}{}", acc, x)
    });  // "12345"
}
```

### find() and position()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let first_even = v.iter().find(|x| *x % 2 == 0);  // Some(&2)
    let position = v.iter().position(|x| *x == 3);   // Some(2)
}
```

### any() and all()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let has_even = v.iter().any(|x| x % 2 == 0);   // true
    let all_positive = v.iter().all(|x| *x > 0);   // true
}
```

### count()

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let count = v.iter().filter(|x| *x % 2 == 0).count();  // 2
}
```

### min() and max()

```rust
fn main() {
    let v = vec![3, 1, 4, 1, 5, 9];

    let min = v.iter().min();  // Some(&1)
    let max = v.iter().max();  // Some(&9)

    // With custom comparison
    let min_by = v.iter().min_by(|a, b| a.cmp(b));
}
```

## Creating Custom Iterators

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Counter {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let sum: u32 = Counter::new(5).sum();  // 15
}
```

## Iterator Performance

Iterators compile to the same code as manual loops:

```rust
// These produce identical assembly
let sum: i32 = (0..1000).filter(|x| x % 2 == 0).sum();

let mut sum = 0;
for i in 0..1000 {
    if i % 2 == 0 {
        sum += i;
    }
}
```

## Common Patterns

### Process and Collect

```rust
let processed: Vec<_> = data.iter()
    .filter(|x| x.is_valid())
    .map(|x| x.transform())
    .collect();
```

### Find and Extract

```rust
let result = items.iter()
    .find(|x| x.matches_criteria())
    .map(|x| x.extract_value());
```

### Group and Aggregate

```rust
use std::collections::HashMap;

let groups: HashMap<_, Vec<_>> = items.iter()
    .fold(HashMap::new(), |mut acc, item| {
        acc.entry(item.category()).or_default().push(item);
        acc
    });
```

## Summary

| Adapter | Purpose |
|---------|---------|
| `map()` | Transform elements |
| `filter()` | Keep matching elements |
| `take(n)` | First n elements |
| `skip(n)` | Skip first n |
| `enumerate()` | Add index |
| `zip()` | Pair with another iterator |
| `chain()` | Concatenate iterators |
| `flatten()` | Flatten nested iterators |
| `collect()` | Gather into collection |
| `fold()` | Reduce to single value |
| `find()` | First matching element |

## See Also

- [Utilities Libraries]({% link appendices/libraries/utilities.md %}) - itertools and more iterator utilities

## Next Steps

Learn about [Closures]({% link part3/03-closures.md %}) to understand the anonymous functions used in iterators.
