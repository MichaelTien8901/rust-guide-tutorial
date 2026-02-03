---
layout: default
title: Traits
parent: Part 2 - Fundamentals
nav_order: 9
---

# Traits

Traits define shared behavior. They're similar to interfaces in other languages.

## Defining a Trait

```rust
trait Summary {
    fn summarize(&self) -> String;
}
```

## Implementing a Trait

```rust
struct Article {
    title: String,
    author: String,
    content: String,
}

impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## Using Traits

```rust
fn main() {
    let article = Article {
        title: String::from("Rust is Great"),
        author: String::from("Jane Doe"),
        content: String::from("..."),
    };

    println!("New article: {}", article.summarize());
}
```

## Default Implementations

```rust
trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
    // Uses default summarize()
}
```

## Traits as Parameters

### `impl Trait` Syntax

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

### Trait Bound Syntax

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

### Multiple Trait Bounds

```rust
// impl Trait syntax
fn notify(item: &(impl Summary + Display)) {
    // ...
}

// Trait bound syntax
fn notify<T: Summary + Display>(item: &T) {
    // ...
}
```

### `where` Clauses

For complex bounds:

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

## Returning Types that Implement Traits

```rust
fn create_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course"),
    }
}
```

{: .warning }
You can only return one concrete type with `impl Trait`. This won't work:
```rust
fn create_summarizable(switch: bool) -> impl Summary {
    if switch {
        Article { /* ... */ }  // One type
    } else {
        Tweet { /* ... */ }    // Different type - Error!
    }
}
```

## Trait Objects for Dynamic Dispatch

Use `dyn Trait` for runtime polymorphism:

```rust
fn print_all(items: &[&dyn Summary]) {
    for item in items {
        println!("{}", item.summarize());
    }
}

fn main() {
    let article = Article { /* ... */ };
    let tweet = Tweet { /* ... */ };

    print_all(&[&article, &tweet]);
}
```

### Box<dyn Trait>

For owned trait objects:

```rust
fn create_random() -> Box<dyn Summary> {
    if rand::random() {
        Box::new(Article { /* ... */ })
    } else {
        Box::new(Tweet { /* ... */ })
    }
}
```

## Common Standard Library Traits

### Display and Debug

{% raw %}
```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}
```
{% endraw %}

### Clone and Copy

```rust
#[derive(Clone)]
struct DeepClone {
    data: Vec<i32>,
}

#[derive(Clone, Copy)]
struct ShallowCopy {
    x: i32,
    y: i32,
}
```

### PartialEq and Eq

```rust
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    assert!(p1 == p2);
}
```

### PartialOrd and Ord

```rust
#[derive(PartialEq, PartialOrd)]
struct Version {
    major: u32,
    minor: u32,
}
```

### Default

```rust
#[derive(Default)]
struct Config {
    debug: bool,
    timeout: u32,
}

fn main() {
    let config = Config::default();
    let config = Config {
        debug: true,
        ..Default::default()
    };
}
```

### From and Into

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl From<Celsius> for Fahrenheit {
    fn from(c: Celsius) -> Self {
        Fahrenheit(c.0 * 9.0 / 5.0 + 32.0)
    }
}

fn main() {
    let c = Celsius(100.0);
    let f: Fahrenheit = c.into();  // Into is auto-implemented
}
```

## Operator Overloading

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("{:?}", p3);  // Point { x: 3, y: 3 }
}
```

## Supertraits

Require another trait:

```rust
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}
```

## Associated Types

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    count: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
```

## Blanket Implementations

Implement trait for all types that satisfy bounds:

```rust
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        // ...
    }
}
```

## Summary Table

| Trait | Purpose | Derive |
|-------|---------|--------|
| `Debug` | `{:?}` formatting | Yes |
| `Display` | `{}` formatting | No |
| `Clone` | `.clone()` | Yes |
| `Copy` | Implicit copy | Yes |
| `PartialEq` | `==` comparison | Yes |
| `Eq` | Strict equality | Yes |
| `PartialOrd` | `<`, `>` comparison | Yes |
| `Ord` | Total ordering | Yes |
| `Hash` | Hashing | Yes |
| `Default` | Default value | Yes |
| `From`/`Into` | Type conversion | No |

## Exercises

1. Create a `Drawable` trait with a `draw()` method and implement it for `Circle` and `Rectangle`
2. Implement `Display` for a custom `Temperature` type
3. Create a `Validator` trait and implement it for different validation rules

## See Also

- [Advanced Traits]({% link part4/05-advanced-traits.md %}) - Associated types, supertraits, and more

## Next Steps

Learn about [Generics]({% link part2/10-generics.md %}) to write flexible, reusable code.
