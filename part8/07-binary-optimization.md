---
layout: default
title: Binary Optimization
parent: Part 8 - Embedded
nav_order: 7
---

# Binary Optimization

Minimizing binary size to fit within flash memory constraints.

{: .note }
> **Prerequisites:** This chapter builds on [Performance]({% link part4/07-performance.md %}) from Part 4. Review that chapter for general Rust optimization techniques before diving into embedded-specific strategies.

Embedded targets have strict flash memory budgets. The STM32F769 provides 2 MB of flash, which sounds generous until you add a display driver, networking stack, and cryptography library. Every kilobyte matters. This chapter covers systematic techniques for reducing binary size from Cargo profiles down to individual lines of code.

## Cargo Profile Tuning

Cargo profiles control how `rustc` compiles your code. The right settings can reduce binary size by 50% or more with no code changes.

### Profile Options Reference

| Option | Values | Effect | Size Impact |
|:-------|:-------|:-------|:------------|
| `opt-level` | `0` | No optimization (fast compile) | Baseline (largest) |
| | `1` | Basic optimizations | ~30% smaller than `0` |
| | `2` | Most optimizations | ~40% smaller than `0` |
| | `3` | All optimizations (speed priority) | Similar to `2`, may be larger |
| | `s` | Optimize for size | ~45% smaller than `0` |
| | `z` | Aggressively optimize for size | ~50% smaller than `0` |
| `lto` | `false` / `"off"` | No link-time optimization | Baseline |
| | `"thin"` | Fast LTO across crates | 10-20% smaller |
| | `true` / `"fat"` | Full LTO (slow compile) | 15-25% smaller |
| `codegen-units` | `16` (default) | Parallel compilation | Baseline |
| | `1` | Single codegen unit | 5-10% smaller (enables more inlining) |
| `debug` | `true` / `2` | Full debug info | No flash impact (debug info in ELF only) |
| | `false` / `0` | No debug info | Smaller ELF file on disk |
| `strip` | `false` | Keep symbols | Baseline |
| | `true` | Strip all symbols | Smaller ELF, no symbol names in debugger |

### Recommended Profiles

```toml
# Cargo.toml

# Development: fast compile, debuggable
[profile.dev]
opt-level = 1          # Some optimization (default 0 is too slow on target)
debug = true           # Full debug info for probe-rs / GDB
lto = false            # Fast compilation
codegen-units = 16     # Maximum parallelism

# Release: smallest binary
[profile.release]
opt-level = "z"        # Aggressively optimize for size
lto = "fat"            # Full link-time optimization
codegen-units = 1      # Single unit for maximum optimization
debug = true           # Keep debug info (does NOT increase flash usage)
strip = false          # Keep symbols for cargo-size analysis
```

{: .tip }
> Always set `debug = true` even in release builds. Debug information is stored in ELF sections that are **not** flashed to the target. You get full debugging capability with zero flash cost.

### Real-World Size Comparison

Typical sizes for a minimal `#[entry]` blinky program on STM32F769:

| Profile | opt-level | LTO | codegen-units | Flash Usage |
|:--------|:----------|:----|:--------------|:------------|
| dev (default) | 0 | off | 16 | ~48 KB |
| dev (tuned) | 1 | off | 16 | ~22 KB |
| release (default) | 3 | off | 16 | ~18 KB |
| release (size) | z | fat | 1 | ~8 KB |
| release (size + strip) | z | fat | 1 | ~8 KB (same flash, smaller ELF) |

## Panic Handler Strategies

The panic handler is a mandatory component in `no_std` programs. Your choice directly affects binary size and debuggability.

### Panic Handler Comparison

| Crate | Behavior | Typical Size | Debuggability | Use Case |
|:------|:---------|:-------------|:--------------|:---------|
| `panic-halt` | Infinite loop (`loop {}`) | ~0 bytes | None (silent hang) | Production, size-critical |
| `panic-abort` | Calls `core::intrinsics::abort` | ~0 bytes | Triggers fault (catchable by debugger) | Production |
| `panic-probe` | Hits breakpoint + `defmt` log | ~200-500 bytes | Excellent (message + location) | Development with probe-rs |
| `panic-semihosting` | Prints to debug host | ~2-4 KB | Good (full message on host) | Development with OpenOCD |

### Configuration

```rust
// Choose ONE panic handler per binary

// Option 1: Minimal size — silent halt
use panic_halt as _;

// Option 2: Development — breakpoint with defmt message
use panic_probe as _;

// Option 3: Semihosting — full panic message to host console
use panic_semihosting as _;
```

{: .important }
> Never use `panic-semihosting` in production. If no debugger is attached, the semihosting call triggers a HardFault. Use `panic-halt` or `panic-abort` for deployed firmware.

## Binary Size Analysis Tools

Before optimizing, you need to measure. These `cargo-binutils` tools show exactly where your bytes are going.

### cargo size — Section Breakdown

```bash
$ cargo size --release -- -A
binary-optimization  :
section               size      addr
.vector_table          1024  0x8000000
.text                  6472  0x8000400
.rodata                 832  0x8001d38
.data                    16  0x20000000
.bss                    268  0x20000010
.uninit                1024  0x2000011c
Total                  9636
```

**Interpreting the output:**

| Section | Location | What to Check |
|:--------|:---------|:--------------|
| `.text` | Flash | Your code + library code. Largest section to optimize. |
| `.rodata` | Flash | String literals, constants, vtables. Watch for format strings. |
| `.vector_table` | Flash | Fixed size (varies by interrupt count). Cannot reduce. |
| `.data` | Flash + RAM | Initialized statics. Small is normal. |
| `.bss` | RAM only | Uninitialized statics. Does not consume flash. |

### cargo bloat — Per-Function Analysis

```bash
$ cargo bloat --release -n 10
 File  .text    Size      Crate Name
 5.8%  28.3%  1,832B      core  core::fmt::write
 3.1%  15.1%    976B      core  core::fmt::Formatter::pad
 2.4%  11.7%    756B  my_crate  my_crate::main
 1.2%   5.9%    380B      core  core::fmt::num::<impl core::fmt::Display for u32>::fmt
 0.8%   3.9%    252B       std  core::str::slice_error_fail
 ...
```

{: .important }
> If `core::fmt` functions dominate your bloat report, you have format string bloat. See [Code-Level Optimizations](#code-level-optimizations) below.

### cargo nm — Symbol Listing

```bash
# List all symbols sorted by size (largest first)
$ cargo nm --release -- --size-sort --reverse | head -20
00000728 T core::fmt::write
000003d0 T core::fmt::Formatter::pad
000002f4 T my_crate::main
```

Use `cargo nm` to find specific symbols when `cargo bloat` does not provide enough detail.

## Linker Script Optimization

The default `cortex-m-rt` linker script covers most use cases, but custom sections and careful placement can improve both size and performance.

### ITCM Placement for Critical Code

On the STM32F769, ITCM (Instruction Tightly Coupled Memory) provides zero-wait-state instruction fetch. Place interrupt handlers and hot loops there:

```
/* Additions to memory.x for STM32F769 */
MEMORY
{
    FLASH  : ORIGIN = 0x08000000, LENGTH = 2M
    RAM    : ORIGIN = 0x20020000, LENGTH = 368K
    ITCM   : ORIGIN = 0x00000000, LENGTH = 16K
    DTCM   : ORIGIN = 0x20000000, LENGTH = 128K
}

SECTIONS
{
    /* Place performance-critical code in ITCM */
    .itcm : AT(__eitcm_load)
    {
        __sitcm = .;
        *(.itcm .itcm.*);
        . = ALIGN(4);
        __eitcm = .;
    } > ITCM

    /* Load address in flash for ITCM initialization */
    __eitcm_load = LOADADDR(.itcm) + SIZEOF(.itcm);
}
```

### KEEP for Interrupt Vectors

The `KEEP` directive prevents the linker from discarding sections that appear unused:

```
SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        KEEP(*(.vector_table));
        KEEP(*(.vector_table.exceptions));
        KEEP(*(.vector_table.interrupts));
    } > FLASH
}
```

Without `KEEP`, link-time optimization might remove interrupt vectors that are only referenced by hardware, not by code.

### Removing Unused Sections

Enable garbage collection of unused sections in `.cargo/config.toml`:

```toml
[target.thumbv7em-none-eabihf]
rustflags = [
    "-C", "link-arg=-Wl,--gc-sections",
]
```

This works with LTO to remove dead code that survives Rust-level optimization but can be identified at link time.

## Code-Level Optimizations

Profile settings only go so far. The biggest wins often come from avoiding patterns that pull in large chunks of `core`.

### core::fmt Bloat

The `core::fmt` formatting machinery is the single largest source of unexpected binary size in embedded Rust. A single `panic!("{}", x)` can add **20 KB or more** to your binary.

**Why it happens:**

```rust
// This pulls in core::fmt::write, Display for u32, padding, alignment...
panic!("Temperature {} exceeds max {}", temp, MAX_TEMP);
// Adds: ~20 KB of formatting code

// This uses no formatting machinery
panic!("Temperature exceeds maximum");
// Adds: ~0 bytes (just a string literal in .rodata)
```

| Pattern | Approximate Cost | Alternative |
|:--------|:-----------------|:------------|
| `panic!("{}", x)` | ~20 KB | `panic!("static message")` |
| `write!(buf, "{}", x)` | ~20 KB | `defmt::write!` or manual conversion |
| `format!("{}", x)` | N/A (requires alloc) | Avoid entirely in `no_std` |
| `defmt::info!("{}", x)` | ~200 bytes | Use `defmt` for all logging |

### Using defmt Instead of core::fmt

`defmt` (deferred formatting) moves formatting work to the host PC. The target only sends compact tokens:

```rust
// Instead of this (20+ KB):
use core::fmt::Write;
writeln!(uart, "Sensor reading: {}", value).ok();

// Use this (~200 bytes):
defmt::info!("Sensor reading: {}", value);
```

`defmt` achieves this by:
1. Storing format strings in a separate ELF section (not flashed)
2. Sending only a string index + raw argument bytes over the probe
3. Reconstructing the full message on the host with `defmt-print` or `probe-rs`

### Minimizing Generic Monomorphization

Every distinct type parameter creates a new copy of a generic function:

```rust
// This generic function is compiled THREE times:
fn process<T: Sensor>(sensor: &T) { /* ... */ }

process(&temperature_sensor);  // process::<TemperatureSensor>
process(&humidity_sensor);     // process::<HumiditySensor>
process(&pressure_sensor);     // process::<PressureSensor>
```

**Strategies to reduce monomorphization:**

```rust
// Strategy 1: Use trait objects for non-hot-path code
fn process(sensor: &dyn Sensor) { /* ... */ }
// One copy, small vtable overhead, slight runtime cost

// Strategy 2: Extract non-generic inner function
fn process<T: Sensor>(sensor: &T) {
    let reading = sensor.read();     // Generic (small, inlined)
    process_reading(reading);        // Non-generic (one copy)
}

fn process_reading(value: f32) {
    // All the heavy logic lives here — compiled once
}
```

### Inline Control

```rust
// Rarely-called error handlers: prevent inlining to keep callers small
#[inline(never)]
fn handle_error(code: u32) {
    // Error handling logic — one copy in .text
}

// Hot inner loops: hint the compiler to inline
#[inline(always)]
fn fast_checksum(byte: u8, acc: u32) -> u32 {
    acc.wrapping_add(byte as u32)
}
```

### Patterns to Avoid

| Pattern | Problem | Alternative |
|:--------|:--------|:------------|
| `String` / `format!()` | Requires allocator, pulls in alloc | Fixed buffers, `heapless::String` |
| `panic!("{}", val)` | Pulls in `core::fmt` (~20 KB) | `panic!("static message")` or `defmt::panic!` |
| Deep generic nesting | Exponential monomorphization | Trait objects, extract inner functions |
| Large match arms in generics | Each variant monomorphized per type | Factor out common logic |
| `Debug` derive on large enums | Format machinery for every variant | Manual `defmt::Format` impl |

## Release Build Checklist

Follow this checklist before flashing a release build to verify your binary fits in flash and is production-ready.

### Step 1: Profile Settings

```toml
# Verify Cargo.toml [profile.release]
[profile.release]
opt-level = "z"
lto = "fat"
codegen-units = 1
strip = false          # Keep for analysis; enable for final production
```

### Step 2: Panic Handler

```rust
// Development → switch to production handler
// use panic_probe as _;      // Development
use panic_halt as _;          // Production
```

### Step 3: Dependency Audit

```bash
# Check which crates contribute the most to binary size
$ cargo bloat --release --crates
 File  .text    Size Crate
10.2%  49.8%  3,224B core
 4.1%  20.0%  1,296B stm32f7xx_hal
 3.3%  16.1%  1,044B my_firmware
 1.1%   5.4%    348B cortex_m
```

Review each dependency: is it pulling in more than expected? Check for feature flags that can disable unused functionality.

### Step 4: Feature Flags

```toml
[dependencies]
# Disable default features and enable only what you need
stm32f7xx-hal = { version = "0.7", default-features = false, features = ["stm32f769", "rt"] }
```

### Step 5: Size Verification

```bash
# Check total flash usage
$ cargo size --release -- -A | grep Total
Total                  9636

# Compare against flash capacity
# STM32F769: 2,097,152 bytes (2 MB)
# Usage: 9,636 bytes (0.46%)
```

### Quick Reference Table

| Step | Command / Action | Target |
|:-----|:-----------------|:-------|
| Profile | Set `opt-level = "z"`, `lto = "fat"`, `codegen-units = 1` | Cargo.toml |
| Panic handler | Switch to `panic-halt` or `panic-abort` | `src/main.rs` |
| Dependency audit | `cargo bloat --release --crates` | Terminal |
| Feature flags | Disable default features, enable selectively | Cargo.toml |
| Size check | `cargo size --release -- -A` | Terminal |
| Flash capacity | Verify total < target flash size | Datasheet |

## Best Practices

- **Measure before optimizing** — use `cargo size` and `cargo bloat` to identify the actual largest contributors before making code changes
- **Set `debug = true` in release** — debug info stays in the ELF file and is not written to flash, so you get free debuggability
- **Use `defmt` for all logging** — it replaces `core::fmt` with a minimal on-target footprint and full formatting on the host
- **Prefer `opt-level = "z"` over `"s"`** — `"z"` is almost always smaller for embedded targets; `"s"` occasionally produces faster code at slightly larger size
- **Enable LTO for release** — `"fat"` LTO enables cross-crate dead code elimination that is impossible without it
- **Audit dependencies regularly** — a single dependency pulling in `core::fmt` or an allocator can undo all your optimization work
- **Use `#[inline(never)]` on error paths** — error handling code is rarely executed; preventing inlining keeps hot paths compact
- **Avoid `String` and `format!` entirely** — use `heapless::String` or fixed-size buffers for any string manipulation in `no_std`

## Next Steps

With your binary optimized for size and performance, learn how to handle concurrency without an OS in [Async and Concurrency]({% link part8/08-async-concurrency.md %}).

[Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/master/examples/part8/binary-optimization)
