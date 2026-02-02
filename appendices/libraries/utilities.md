---
layout: default
title: Utilities
parent: Libraries
grand_parent: Appendices
nav_order: 8
---

# Utility Libraries

Essential utility crates for Rust development.

## itertools

Extended iterator methods.

```toml
[dependencies]
itertools = "0.13"
```

```rust
use itertools::Itertools;

// Unique elements
let unique: Vec<_> = vec![1, 2, 2, 3, 3, 3].into_iter().unique().collect();

// Chunks
for chunk in &vec![1, 2, 3, 4, 5].into_iter().chunks(2) {
    let chunk: Vec<_> = chunk.collect();
    println!("{:?}", chunk);
}

// Combinations
for combo in (0..4).combinations(2) {
    println!("{:?}", combo);  // [0,1], [0,2], [0,3], [1,2], ...
}

// Permutations
for perm in (0..3).permutations(2) {
    println!("{:?}", perm);
}

// Join
let joined = vec!["a", "b", "c"].into_iter().join(", ");

// Group by
let groups = vec![1, 1, 2, 2, 2, 3].into_iter()
    .group_by(|&x| x);

// Interleave
let interleaved: Vec<_> = [1, 2, 3].into_iter()
    .interleave([10, 20, 30])
    .collect();  // [1, 10, 2, 20, 3, 30]
```

## once_cell / LazyLock

Lazy initialization.

```toml
[dependencies]
once_cell = "1.19"
```

```rust
use once_cell::sync::Lazy;
use std::collections::HashMap;

// Global lazy initialization
static CONFIG: Lazy<HashMap<String, String>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("key".into(), "value".into());
    m
});

// Use it
fn get_config() -> &'static HashMap<String, String> {
    &CONFIG
}
```

### std::sync::LazyLock (Rust 1.80+)

```rust
use std::sync::LazyLock;

static DATA: LazyLock<Vec<i32>> = LazyLock::new(|| {
    vec![1, 2, 3, 4, 5]
});
```

## regex

Regular expressions.

```toml
[dependencies]
regex = "1"
```

```rust
use regex::Regex;

let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();

// Check if matches
if re.is_match("2024-01-15") {
    println!("Valid date format");
}

// Extract captures
if let Some(caps) = re.captures("2024-01-15") {
    let year = &caps[1];
    let month = &caps[2];
    let day = &caps[3];
}

// Find all matches
for cap in re.captures_iter(text) {
    println!("Found: {}", &cap[0]);
}

// Replace
let result = re.replace_all("2024-01-15", "$2/$3/$1");  // 01/15/2024
```

## chrono

Date and time handling.

```toml
[dependencies]
chrono = "0.4"
```

```rust
use chrono::{DateTime, Utc, Local, NaiveDate, Duration};

// Current time
let now = Utc::now();
let local = Local::now();

// Parse date
let date = NaiveDate::parse_from_str("2024-01-15", "%Y-%m-%d")?;

// Format
let formatted = now.format("%Y-%m-%d %H:%M:%S").to_string();

// Duration
let tomorrow = now + Duration::days(1);
let week_ago = now - Duration::weeks(1);

// Comparison
if date1 > date2 {
    println!("date1 is later");
}
```

## uuid

UUID generation.

```toml
[dependencies]
uuid = { version = "1", features = ["v4", "serde"] }
```

```rust
use uuid::Uuid;

// Generate random UUID (v4)
let id = Uuid::new_v4();
println!("{}", id);  // e.g., 67e55044-10b1-426f-9247-bb680e5fe0c8

// Parse UUID
let parsed = Uuid::parse_str("67e55044-10b1-426f-9247-bb680e5fe0c8")?;

// Check if nil
if id.is_nil() {
    println!("Nil UUID");
}
```

## rand

Random number generation.

```toml
[dependencies]
rand = "0.8"
```

```rust
use rand::Rng;
use rand::seq::SliceRandom;

let mut rng = rand::thread_rng();

// Random numbers
let n: u32 = rng.gen();
let n: f64 = rng.gen_range(0.0..1.0);
let n: i32 = rng.gen_range(1..=100);

// Random bool with probability
let b: bool = rng.gen_bool(0.5);

// Shuffle
let mut vec = vec![1, 2, 3, 4, 5];
vec.shuffle(&mut rng);

// Random element
let choice = vec.choose(&mut rng);
```

## bytes

Efficient byte manipulation.

```toml
[dependencies]
bytes = "1"
```

```rust
use bytes::{Bytes, BytesMut, Buf, BufMut};

// Immutable bytes
let b = Bytes::from("hello");

// Mutable buffer
let mut buf = BytesMut::with_capacity(64);
buf.put_u32(42);
buf.put_slice(b"hello");

// Freeze to immutable
let frozen = buf.freeze();

// Reading
let mut data = &b"hello world"[..];
let first = data.get_u8();
```

## parking_lot

Fast synchronization primitives.

```toml
[dependencies]
parking_lot = "0.12"
```

```rust
use parking_lot::{Mutex, RwLock, Once};

// Faster mutex
let data = Mutex::new(0);
{
    let mut guard = data.lock();
    *guard += 1;
}  // No poison handling needed

// RwLock
let data = RwLock::new(vec![]);
{
    let read = data.read();
    // Read access
}
{
    let mut write = data.write();
    write.push(1);
}

// Once
static INIT: Once = Once::new();
INIT.call_once(|| {
    // Initialize
});
```

## rayon

Data parallelism.

```toml
[dependencies]
rayon = "1.10"
```

```rust
use rayon::prelude::*;

// Parallel iterator
let sum: i32 = (0..1000).into_par_iter().sum();

// Parallel map
let squares: Vec<_> = (0..1000)
    .into_par_iter()
    .map(|x| x * x)
    .collect();

// Parallel filter
let evens: Vec<_> = (0..1000)
    .into_par_iter()
    .filter(|x| x % 2 == 0)
    .collect();

// Parallel sort
let mut data = vec![5, 2, 8, 1, 9];
data.par_sort();
```

## strum

Enum utilities.

```toml
[dependencies]
strum = "0.26"
strum_macros = "0.26"
```

```rust
use strum::{EnumIter, EnumString, Display, IntoStaticStr};

#[derive(Debug, EnumIter, EnumString, Display, IntoStaticStr)]
enum Color {
    Red,
    Green,
    Blue,
}

// Iterate over variants
for color in Color::iter() {
    println!("{}", color);
}

// Parse from string
let color: Color = "Red".parse()?;

// Convert to string
let s: &'static str = Color::Red.into();
```

## derive_more

Additional derive macros.

```toml
[dependencies]
derive_more = { version = "1", features = ["full"] }
```

```rust
use derive_more::{From, Into, Display, Add, Sub};

#[derive(From, Into, Display, Add, Sub, Clone, Copy)]
struct Meters(f64);

let m = Meters::from(5.0);
let total = m + Meters(3.0);
println!("{}", total);  // Prints "8"
```

## Summary

| Crate | Purpose |
|-------|---------|
| itertools | Extended iterators |
| once_cell | Lazy initialization |
| regex | Regular expressions |
| chrono | Date/time |
| uuid | UUID generation |
| rand | Random numbers |
| bytes | Byte buffers |
| parking_lot | Fast sync primitives |
| rayon | Parallelism |
| strum | Enum utilities |
| derive_more | Derive macros |

## Recommendations

| Need | Crate |
|------|-------|
| Iterator methods | itertools |
| Global state | once_cell |
| Text patterns | regex |
| Dates and times | chrono |
| Unique IDs | uuid |
| Randomness | rand |
| Network buffers | bytes |
| Better mutexes | parking_lot |
| CPU parallelism | rayon |
