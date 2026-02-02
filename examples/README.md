# Code Examples

This folder contains runnable Rust code examples that accompany the tutorial chapters.

## Structure

```
examples/
├── part1/                    # Getting Started
│   ├── hello-world/          # First Rust program
│   └── cargo-basics/         # Cargo project structure
├── part2/                    # Fundamentals
│   ├── ownership/            # Ownership examples
│   ├── borrowing/            # Borrowing and references
│   ├── lifetimes/            # Lifetime annotations
│   └── error-handling/       # Result and Option
├── part3/                    # Intermediate
│   ├── collections/          # Vec, HashMap, etc.
│   ├── iterators/            # Iterator patterns
│   ├── concurrency/          # Threads, channels
│   └── async-await/          # Async programming
├── part4/                    # Advanced
│   ├── unsafe-rust/          # Unsafe operations
│   ├── ffi/                  # Foreign function interface
│   └── macros/               # Macro examples
├── part5/                    # Patterns & Use Cases
│   ├── cli-app/              # Command-line application
│   ├── web-service/          # Web API example
│   └── embedded/             # Embedded/no_std
├── part6/                    # Systems Programming
│   ├── drivers/              # Driver examples
│   └── bare-metal/           # No_std bare metal
└── part7/                    # UEFI
    ├── uefi-hello/           # UEFI hello world
    └── uefi-boot/            # Boot services example
```

## Running Examples

Each example is a complete Cargo project. To run:

```bash
cd examples/part1/hello-world
cargo run
```

## Docker Option

You can also run examples using Docker:

```bash
docker run --rm -v $(pwd):/workspace -w /workspace rust:latest cargo run
```

## Conventions

- Each example includes a `README.md` explaining the concept
- Code is thoroughly commented
- Examples build on previous concepts progressively
- All examples are tested with the latest stable Rust
