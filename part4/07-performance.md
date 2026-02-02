---
layout: default
title: Performance
parent: Part 4 - Advanced
nav_order: 7
---

# Performance

Profiling, benchmarking, and optimization.

## Benchmarking with Criterion

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn benchmark(c: &mut Criterion) {
    c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
```

## Release Build Optimizations

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

## Next Steps

Learn about [Memory Layout]({% link part4/08-memory-layout.md %}).
