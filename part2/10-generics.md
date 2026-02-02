---
layout: default
title: Generics
parent: Part 2 - Fundamentals
nav_order: 10
---

# Generics

Generics let you write code that works with multiple types while maintaining type safety.

## The Problem Generics Solve

Without generics, you'd need separate functions:

```rust
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

## Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let numbers = vec![34, 50, 25, 100, 65];
    println!("Largest: {}", largest(&numbers));

    let chars = vec!['y', 'm', 'a', 'q'];
    println!("Largest: {}", largest(&chars));
}
```

## Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
}
```

### Multiple Type Parameters

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let mixed = Point { x: 5, y: 4.0 };
}
```

## Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

## Generic Methods

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}

// Methods only for specific types
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### Different Generic Parameters

```rust
impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

## Trait Bounds

Constrain generic types:

```rust
use std::fmt::Display;

fn print_pair<T: Display, U: Display>(a: T, b: U) {
    println!("({}, {})", a, b);
}
```

### Multiple Bounds

```rust
fn compare_and_display<T: PartialOrd + Display>(a: T, b: T) {
    if a > b {
        println!("{} > {}", a, b);
    } else {
        println!("{} <= {}", a, b);
    }
}
```

### `where` Clause

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Conditional Implementation

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

// Always available
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Only when T implements Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("Largest: {}", self.x);
        } else {
            println!("Largest: {}", self.y);
        }
    }
}
```

## Blanket Implementations

```rust
// Implement ToString for any type that implements Display
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}
```

## Const Generics

Generic over constant values:

```rust
struct Array<T, const N: usize> {
    data: [T; N],
}

impl<T: Default + Copy, const N: usize> Array<T, N> {
    fn new() -> Self {
        Array {
            data: [T::default(); N],
        }
    }
}

fn main() {
    let arr: Array<i32, 5> = Array::new();
}
```

## Generic Lifetimes

```rust
struct Excerpt<'a> {
    part: &'a str,
}

impl<'a> Excerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce(&self, announcement: &str) -> &str {
        println!("Attention: {}", announcement);
        self.part
    }
}
```

### Combining Lifetimes and Type Generics

```rust
use std::fmt::Display;

fn longest_with_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() { x } else { y }
}
```

## Turbofish Syntax

Explicitly specify type parameters:

```rust
fn main() {
    // Type inference
    let v: Vec<i32> = Vec::new();

    // Turbofish syntax
    let v = Vec::<i32>::new();

    // With parse
    let n: i32 = "42".parse().unwrap();
    let n = "42".parse::<i32>().unwrap();
}
```

## Phantom Types

Type parameter not used in the struct but affects type:

```rust
use std::marker::PhantomData;

struct Tagged<T, Tag> {
    value: T,
    _marker: PhantomData<Tag>,
}

struct Meters;
struct Feet;

type Distance<U> = Tagged<f64, U>;

fn main() {
    let d1: Distance<Meters> = Tagged { value: 100.0, _marker: PhantomData };
    let d2: Distance<Feet> = Tagged { value: 328.0, _marker: PhantomData };

    // d1 and d2 are different types, can't mix them accidentally
}
```

## Performance

Generics use **monomorphization** - the compiler generates specific code for each concrete type used:

```rust
// Your code
fn largest<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

let x = largest(5, 10);
let y = largest(1.0, 2.0);

// Compiler generates
fn largest_i32(a: i32, b: i32) -> i32 { ... }
fn largest_f64(a: f64, b: f64) -> f64 { ... }
```

{: .tip }
Generics have **zero runtime cost** - they're as fast as writing specific functions.

## Common Patterns

### Generic Constructor

```rust
impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
}
```

### Generic Conversion

```rust
impl<T, U> From<Container<T>> for Container<U>
where
    U: From<T>,
{
    fn from(container: Container<T>) -> Self {
        Container {
            value: U::from(container.value),
        }
    }
}
```

### Builder with Generics

```rust
struct Builder<T> {
    value: Option<T>,
}

impl<T> Builder<T> {
    fn new() -> Self {
        Builder { value: None }
    }

    fn value(mut self, v: T) -> Self {
        self.value = Some(v);
        self
    }

    fn build(self) -> Option<T> {
        self.value
    }
}
```

## Summary

| Feature | Syntax |
|---------|--------|
| Generic function | `fn foo<T>(x: T)` |
| Generic struct | `struct Foo<T> { x: T }` |
| Generic enum | `enum Foo<T> { A(T) }` |
| Trait bound | `fn foo<T: Trait>(x: T)` |
| Multiple bounds | `T: Trait1 + Trait2` |
| Where clause | `where T: Trait` |
| Const generic | `struct Foo<const N: usize>` |
| Lifetime | `fn foo<'a>(x: &'a str)` |

## Exercises

1. Create a generic `Stack<T>` with push, pop, and peek methods
2. Implement a generic `min` function for any type that implements `PartialOrd`
3. Create a `Matrix<T, const ROWS: usize, const COLS: usize>` type

## Next Steps

Congratulations! You've completed Part 2: Fundamentals. Continue to [Part 3: Intermediate]({% link part3/index.md %}) to learn about collections, iterators, and concurrency.
