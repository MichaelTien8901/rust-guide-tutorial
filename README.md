# Rust Programming Guide

A comprehensive guide for Rust programming — from beginner to professional.

**Live site**: [https://michaeltien8901.github.io/rust-guide-tutorial/](https://michaeltien8901.github.io/rust-guide-tutorial/)

## Structure

```
docs/
├── index.md               # Home page
├── part1/                 # Getting Started — installation, IDE setup, Hello World
├── part2/                 # Fundamentals — ownership, borrowing, structs, enums, traits
├── part3/                 # Intermediate — collections, iterators, concurrency, async
├── part4/                 # Advanced — unsafe, FFI, macros, performance
├── part5/                 # Patterns — design patterns, CLI, web, testing
├── part6/                 # Systems — no_std, bare metal, embedded HAL, drivers, RTOS
├── part7/                 # UEFI — firmware development with uefi-rs
├── part8/                 # Embedded — STM32F769-DISCO development workflow
├── appendices/            # Libraries, ecosystem, glossary
└── examples/              # Runnable Rust code for every chapter
    ├── part1/ ... part8/  # One Cargo project per topic
    ├── build-all.sh       # Build all examples (Docker or local)
    └── README.md          # Example conventions and usage
```

## Running Locally

```bash
bundle install
bundle exec jekyll serve
```

Open [http://localhost:4000/rust-guide-tutorial/](http://localhost:4000/rust-guide-tutorial/).

## Examples

Each chapter has a companion example in `examples/`. Every example is a standalone Cargo project:

```bash
# Run a single example
cd examples/part1/hello-world
cargo run

# Build all examples via Docker
./examples/build-all.sh
```

See [examples/README.md](examples/README.md) for details.

## Built With

- [Jekyll](https://jekyllrb.com/) with [just-the-docs](https://just-the-docs.com/) theme
- [Mermaid](https://mermaid.js.org/) for diagrams
- Dark color scheme

## Contributing

[Open an issue](https://github.com/MichaelTien8901/rust-guide-tutorial/issues) on GitHub.
