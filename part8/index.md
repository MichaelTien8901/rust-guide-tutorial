---
layout: default
title: Part 8 - Embedded
nav_order: 9
has_children: true
permalink: /part8/
---

# Part 8: Embedded Systems Development

A practical guide to building embedded applications with Rust, using the STM32F769I-DISCO evaluation board. Part 6 teaches the building blocks; Part 8 teaches the complete development workflow.

## Chapters

1. [Toolchain Setup]({% link part8/01-toolchain-setup.md %}) - Development environment and project configuration
2. [Embedded Software]({% link part8/02-embedded-software.md %}) - Writing embedded Rust with HALs
3. [Debugging]({% link part8/03-debugging.md %}) - Tools and techniques for embedded debugging
4. [Bare Metal Runtime]({% link part8/04-bare-metal-runtime.md %}) - From reset vector to main
5. [Memory Management]({% link part8/05-memory-management.md %}) - Allocation strategies for constrained systems
6. [C Interoperability]({% link part8/06-c-interop.md %}) - Mixing Rust with C firmware
7. [Binary Optimization]({% link part8/07-binary-optimization.md %}) - Size and performance tuning
8. [Async Concurrency]({% link part8/08-async-concurrency.md %}) - Embassy executor and async tasks

## Prerequisites

- Completed Parts 1-6
- Understanding of no_std, bare metal basics, and embedded HAL (Part 6)
- STM32F769I-DISCO evaluation board (recommended, but concepts apply to any Cortex-M board)

## Next Steps

Start with [Toolchain Setup]({% link part8/01-toolchain-setup.md %}).
