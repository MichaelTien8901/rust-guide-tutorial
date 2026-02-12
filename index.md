---
layout: default
title: Home
nav_order: 1
description: "Rust Programming Guide - Learn Rust from beginner to professional"
permalink: /
---

# Rust Programming Guide

Welcome to the comprehensive Rust programming guide. This tutorial takes you from complete beginner to professional-level systems programming, covering everything from basic syntax to UEFI firmware development.

## What You'll Learn

- **Getting Started**: Set up your development environment and write your first Rust program
- **Fundamentals**: Master ownership, borrowing, lifetimes, and Rust's unique memory model
- **Intermediate**: Collections, iterators, concurrency, and async programming
- **Advanced**: Unsafe Rust, FFI, macros, and performance optimization
- **Patterns**: Real-world design patterns for CLI, web, and embedded applications
- **Systems**: Bare metal programming, drivers, and kernel modules
- **UEFI**: Firmware development with Rust using uefi-rs

## Quick Start

1. [Install Rust]({% link part1/02-installation.md %})
2. [Set up your IDE]({% link part1/03-ide-setup.md %})
3. [Write Hello World]({% link part1/06-hello-world.md %})

## How This Guide is Organized

| Section | Description | Difficulty |
|---------|-------------|------------|
| [Part 1: Getting Started]({% link part1/index.md %}) | Environment setup, tooling, first program | Beginner |
| [Part 2: Fundamentals]({% link part2/index.md %}) | Ownership, borrowing, structs, enums, traits | Beginner |
| [Part 3: Intermediate]({% link part3/index.md %}) | Collections, concurrency, async/await | Intermediate |
| [Part 4: Advanced]({% link part4/index.md %}) | Unsafe, FFI, macros, performance | Advanced |
| [Part 5: Patterns]({% link part5/index.md %}) | Design patterns, CLI, web, testing | Advanced |
| [Part 6: Systems]({% link part6/index.md %}) | no_std, bare metal, drivers, RTOS | Professional |
| [Part 7: UEFI]({% link part7/index.md %}) | UEFI development with uefi-rs | Professional |
| [Appendices]({% link appendices/index.md %}) | Libraries, ecosystem, glossary | Reference |

## Prerequisites

- Basic programming knowledge (any language)
- Command line familiarity
- Willingness to learn something new!

## Sample Code

All examples in this guide are tested and available in the [examples/](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/main/examples) directory. Each example includes:

- Complete, runnable Cargo project
- Detailed comments explaining the code
- README with instructions

## Docker Quick Start

Don't want to install Rust locally? Use our Docker environment:

```bash
docker-compose up -d
docker-compose exec rust cargo run
```

See [Docker Setup]({% link part1/05-docker-setup.md %}) for details.

## Contributing

Found an error? Want to add content? [Open an issue](https://github.com/MichaelTien8901/rust-guide-tutorial/issues) on GitHub.

---

{: .note }
This guide is community-maintained and not officially affiliated with the Rust Foundation. For official documentation, visit [doc.rust-lang.org](https://doc.rust-lang.org/).
