---
layout: default
title: Cross-Compilation
parent: Part 6 - Systems
nav_order: 8
---

# Cross-Compilation

Building Rust code for different target architectures.

## Understanding Targets

A Rust target triple has the format: `<arch>-<vendor>-<os>-<env>`

| Component | Examples |
|-----------|----------|
| arch | x86_64, aarch64, thumbv7em, riscv32 |
| vendor | unknown, apple, pc |
| os | linux, windows, none |
| env | gnu, musl, eabi, eabihf |

## Common Targets

| Target | Description |
|--------|-------------|
| `x86_64-unknown-linux-gnu` | Linux 64-bit |
| `aarch64-unknown-linux-gnu` | ARM64 Linux |
| `thumbv7em-none-eabihf` | ARM Cortex-M4F |
| `riscv32imac-unknown-none-elf` | RISC-V 32-bit |
| `wasm32-unknown-unknown` | WebAssembly |
| `x86_64-pc-windows-msvc` | Windows 64-bit |
| `aarch64-apple-darwin` | macOS ARM64 |

## Installing Targets

```bash
# List installed targets
rustup target list --installed

# Add a new target
rustup target add thumbv7em-none-eabihf
rustup target add aarch64-unknown-linux-gnu

# List all available targets
rustup target list
```

## Basic Cross-Compilation

```bash
# Build for a specific target
cargo build --target thumbv7em-none-eabihf

# Release build
cargo build --release --target aarch64-unknown-linux-gnu
```

## Cargo Configuration

Create `.cargo/config.toml` in your project:

```toml
[build]
target = "thumbv7em-none-eabihf"

[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F411CEUx"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"
```

## Linker Configuration

For cross-compilation, you often need a cross-linker:

### Linux to Linux (different arch)

```bash
# Install cross toolchain (Ubuntu/Debian)
sudo apt install gcc-aarch64-linux-gnu

# Or for ARM 32-bit
sudo apt install gcc-arm-linux-gnueabihf
```

### Using Zig as Linker

Zig can cross-compile to many targets without additional toolchains:

```toml
# .cargo/config.toml
[target.aarch64-unknown-linux-gnu]
linker = "zig"
rustflags = ["-C", "linker-arg=cc", "-C", "linker-arg=-target", "-C", "linker-arg=aarch64-linux-gnu"]
```

## Embedded Targets

### ARM Cortex-M Setup

```bash
# Install target
rustup target add thumbv7em-none-eabihf

# Install tools
cargo install probe-rs --features cli
cargo install cargo-binutils
rustup component add llvm-tools-preview
```

Project `.cargo/config.toml`:

```toml
[target.thumbv7em-none-eabihf]
runner = "probe-rs run --chip STM32F411CEUx"
rustflags = [
    "-C", "link-arg=-Tlink.x",
]

[build]
target = "thumbv7em-none-eabihf"
```

### Memory Layout (memory.x)

```ld
MEMORY
{
  FLASH : ORIGIN = 0x08000000, LENGTH = 512K
  RAM : ORIGIN = 0x20000000, LENGTH = 128K
}
```

## RISC-V Cross-Compilation

```bash
# Install target
rustup target add riscv32imac-unknown-none-elf

# Install toolchain
# Ubuntu/Debian
sudo apt install gcc-riscv64-unknown-elf
```

```toml
# .cargo/config.toml
[target.riscv32imac-unknown-none-elf]
rustflags = [
    "-C", "link-arg=-Tlink.x",
]

[build]
target = "riscv32imac-unknown-none-elf"
```

## Build Scripts for Multiple Targets

Create a build script for CI:

```bash
#!/bin/bash
# build-all.sh

TARGETS=(
    "x86_64-unknown-linux-gnu"
    "aarch64-unknown-linux-gnu"
    "x86_64-pc-windows-gnu"
    "x86_64-apple-darwin"
)

for target in "${TARGETS[@]}"; do
    echo "Building for $target"
    cargo build --release --target "$target"
done
```

## Using cross

The `cross` tool simplifies cross-compilation using Docker:

```bash
# Install cross
cargo install cross

# Build for target (Docker handles toolchain)
cross build --target aarch64-unknown-linux-gnu
cross build --target armv7-unknown-linux-gnueabihf
```

## Conditional Compilation

Handle platform differences in code:

```rust
#[cfg(target_arch = "x86_64")]
fn arch_specific() {
    // x86_64 code
}

#[cfg(target_arch = "aarch64")]
fn arch_specific() {
    // ARM64 code
}

#[cfg(target_os = "linux")]
fn os_specific() {
    // Linux code
}

#[cfg(all(target_arch = "arm", target_os = "none"))]
fn embedded_specific() {
    // Embedded ARM code
}
```

## Inspecting Binaries

```bash
# Show binary info
cargo size --release --target thumbv7em-none-eabihf

# Detailed size breakdown
cargo size --release --target thumbv7em-none-eabihf -- -A

# Disassemble
cargo objdump --release --target thumbv7em-none-eabihf -- -d

# Generate binary file
cargo objcopy --release --target thumbv7em-none-eabihf -- -O binary app.bin

# Generate hex file
cargo objcopy --release --target thumbv7em-none-eabihf -- -O ihex app.hex
```

## Debugging Cross-Compiled Code

### Using GDB

```bash
# Start GDB server (probe-rs)
probe-rs gdb --chip STM32F411CEUx

# Connect with GDB
arm-none-eabi-gdb target/thumbv7em-none-eabihf/release/myapp
(gdb) target remote :1337
(gdb) load
(gdb) break main
(gdb) continue
```

### Using VS Code

`.vscode/launch.json`:
```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "probe-rs-debug",
            "request": "launch",
            "name": "Debug",
            "cwd": "${workspaceFolder}",
            "chip": "STM32F411CEUx",
            "flashingConfig": {
                "flashingEnabled": true
            },
            "coreConfigs": [
                {
                    "programBinary": "target/thumbv7em-none-eabihf/release/myapp"
                }
            ]
        }
    ]
}
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Missing linker | Install cross toolchain |
| Missing target | `rustup target add <target>` |
| Link errors | Check linker script |
| ABI mismatch | Verify target triple |
| Large binary | Enable LTO, optimize for size |

## Optimization for Size

```toml
# Cargo.toml
[profile.release]
opt-level = "s"      # Optimize for size
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = "abort"      # Smaller panic handling
strip = true         # Strip symbols
```

## Summary

| Tool | Purpose |
|------|---------|
| `rustup target` | Install targets |
| `.cargo/config.toml` | Configure linker, flags |
| `cross` | Docker-based cross-compilation |
| `cargo-binutils` | Inspect binaries |
| `probe-rs` | Embedded debugging |

## Next Steps

Continue to [Part 7: UEFI Development]({% link part7/index.md %}) for firmware programming.
