# Code Examples

This folder contains runnable Rust code examples that accompany the tutorial chapters.

## Structure

```
examples/
├── part1/                    # Getting Started
│   └── hello-world/          # First Rust program
├── part2/                    # Fundamentals
│   ├── variables-types/      # Variables and data types
│   ├── ownership/            # Ownership examples
│   ├── borrowing/            # Borrowing and references
│   ├── lifetimes/            # Lifetime annotations
│   ├── functions/            # Functions and methods
│   ├── structs/              # Structs and impl blocks
│   ├── enums/                # Enums and pattern matching
│   ├── traits/               # Traits and implementations
│   ├── generics/             # Generic types
│   └── error-handling/       # Result and Option
├── part3/                    # Intermediate
│   ├── collections/          # Vec, HashMap, etc.
│   ├── iterators/            # Iterator patterns
│   ├── closures/             # Closures and Fn traits
│   ├── smart-pointers/       # Box, Rc, RefCell
│   ├── threads/              # Basic threading
│   ├── channels/             # Message passing
│   ├── mutex/                # Shared state concurrency
│   └── async-basics/         # Async/await programming
├── part4/                    # Advanced
│   ├── unsafe-rust/          # Unsafe operations
│   ├── ffi/                  # Foreign function interface
│   ├── macros-declarative/   # macro_rules! macros
│   ├── macros-procedural/    # Proc macros
│   ├── advanced-traits/      # Associated types, supertraits
│   ├── advanced-types/       # Type system features
│   ├── memory-layout/        # Memory representation
│   └── performance/          # Optimization techniques
├── part5/                    # Patterns & Use Cases
│   ├── builder-pattern/      # Builder pattern
│   ├── state-machine/        # Type-state patterns
│   ├── error-patterns/       # Error handling patterns
│   ├── cli-apps/             # Command-line applications
│   ├── web-services/         # Web API example
│   ├── database/             # Database access
│   ├── serialization/        # Serde patterns
│   ├── logging/              # Logging and tracing
│   └── testing-patterns/     # Testing strategies
├── part6/                    # Systems Programming
│   ├── no-std/               # No_std basics
│   ├── embedded-basics/      # Embedded fundamentals
│   ├── embedded-hal/         # Hardware abstraction
│   ├── drivers/              # Driver examples
│   ├── real-time/            # Real-time patterns
│   ├── rtos-patterns/        # RTOS integration
│   └── cross-compile/        # Cross-compilation
└── part7/                    # UEFI
    ├── uefi-hello/           # UEFI hello world
    ├── uefi-filesystem/      # UEFI file operations
    ├── uefi-graphics/        # UEFI graphics
    └── uefi-qemu/            # QEMU testing setup
```

## Running Examples

Each example is a complete Cargo project. To run:

```bash
cd examples/part1/hello-world
cargo run
```

## Building All Examples

Use the build script to compile all examples at once:

```bash
# Build all examples using Docker (recommended)
./build-all.sh

# Build with local Rust installation
./build-all.sh --local

# Show list of examples with warnings
./build-all.sh --warnings
```

## Docker Option

You can also run individual examples using Docker:

```bash
docker run --rm -v $(pwd):/workspace -w /workspace/part1/hello-world rust:latest cargo run
```

## Conventions

- Each example is a standalone Cargo project
- Code is thoroughly commented to explain concepts
- Examples build on previous concepts progressively
- All examples are tested with the latest stable Rust
- Some examples have intentional unused code to demonstrate patterns
