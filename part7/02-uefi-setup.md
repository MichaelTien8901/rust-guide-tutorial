---
layout: default
title: UEFI Setup
parent: Part 7 - UEFI
nav_order: 2
---

# UEFI Setup

Setting up a Rust UEFI development environment.

## Prerequisites

Install Rust nightly and the UEFI target:

```bash
# Install nightly toolchain
rustup install nightly

# Add UEFI target
rustup target add x86_64-unknown-uefi --toolchain nightly

# Or for ARM64
rustup target add aarch64-unknown-uefi --toolchain nightly
```

## Project Setup

### Create New Project

```bash
cargo new uefi-app --edition 2021
cd uefi-app
```

### Cargo.toml

```toml
[package]
name = "uefi-app"
version = "0.1.0"
edition = "2021"

[dependencies]
uefi = "0.32"
log = "0.4"

[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
opt-level = "z"     # Optimize for size
lto = true
codegen-units = 1
strip = true
```

### rust-toolchain.toml

Pin to nightly:

```toml
[toolchain]
channel = "nightly"
components = ["rust-src"]
targets = ["x86_64-unknown-uefi"]
```

### .cargo/config.toml

Configure build settings:

```toml
[build]
target = "x86_64-unknown-uefi"

[unstable]
build-std = ["core", "alloc"]
build-std-features = ["compiler-builtins-mem"]
```

## Project Structure

```
uefi-app/
├── .cargo/
│   └── config.toml
├── src/
│   └── main.rs
├── Cargo.toml
├── rust-toolchain.toml
└── scripts/
    └── run-qemu.sh
```

## Basic Application Template

```rust
// src/main.rs
#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();
    log::info!("UEFI application started!");

    Status::SUCCESS
}
```

## Building

```bash
# Build the application
cargo build --release

# Output location
ls target/x86_64-unknown-uefi/release/uefi-app.efi
```

## Installing QEMU and OVMF

### Ubuntu/Debian

```bash
# Install QEMU
sudo apt install qemu-system-x86 ovmf

# OVMF firmware location
ls /usr/share/OVMF/OVMF_CODE.fd
ls /usr/share/OVMF/OVMF_VARS.fd
```

### Fedora

```bash
sudo dnf install qemu-system-x86 edk2-ovmf
# Files in /usr/share/edk2/ovmf/
```

### macOS

```bash
brew install qemu
# Download OVMF from https://retrage.github.io/edk2-nightly/
```

### Windows

Download from:
- QEMU: https://www.qemu.org/download/#windows
- OVMF: https://retrage.github.io/edk2-nightly/

## Running with QEMU

### Basic Run Script

Create `scripts/run-qemu.sh`:

```bash
#!/bin/bash
set -e

# Build
cargo build --release

# Create ESP directory structure
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/uefi-app.efi esp/EFI/BOOT/BOOTX64.EFI

# Run QEMU
qemu-system-x86_64 \
    -nodefaults \
    -machine q35 \
    -bios /usr/share/OVMF/OVMF_CODE.fd \
    -drive format=raw,file=fat:rw:esp \
    -serial stdio \
    -display none
```

### With Graphics

```bash
qemu-system-x86_64 \
    -nodefaults \
    -machine q35 \
    -bios /usr/share/OVMF/OVMF_CODE.fd \
    -drive format=raw,file=fat:rw:esp \
    -device virtio-gpu-pci \
    -display gtk
```

### With Debugging

```bash
qemu-system-x86_64 \
    -nodefaults \
    -machine q35 \
    -bios /usr/share/OVMF/OVMF_CODE.fd \
    -drive format=raw,file=fat:rw:esp \
    -serial stdio \
    -debugcon file:debug.log \
    -global isa-debugcon.iobase=0x402 \
    -s -S  # GDB server on port 1234, pause at start
```

## Using cargo-uefi

The `cargo-uefi` tool simplifies building and running:

```bash
# Install
cargo install cargo-uefi

# Build and run
cargo uefi run
```

## Alternative: uefi-run

```bash
# Install
cargo install uefi-run

# Run
uefi-run target/x86_64-unknown-uefi/release/uefi-app.efi
```

## Makefile Setup

Create a `Makefile` for convenience:

```makefile
.PHONY: build run debug clean

TARGET = x86_64-unknown-uefi
OVMF = /usr/share/OVMF/OVMF_CODE.fd
EFI = target/$(TARGET)/release/uefi-app.efi

build:
	cargo build --release

esp/EFI/BOOT/BOOTX64.EFI: $(EFI)
	mkdir -p esp/EFI/BOOT
	cp $(EFI) esp/EFI/BOOT/BOOTX64.EFI

run: build esp/EFI/BOOT/BOOTX64.EFI
	qemu-system-x86_64 \
		-nodefaults \
		-machine q35 \
		-bios $(OVMF) \
		-drive format=raw,file=fat:rw:esp \
		-serial stdio

debug: build esp/EFI/BOOT/BOOTX64.EFI
	qemu-system-x86_64 \
		-nodefaults \
		-machine q35 \
		-bios $(OVMF) \
		-drive format=raw,file=fat:rw:esp \
		-serial stdio \
		-s -S

clean:
	cargo clean
	rm -rf esp
```

## Docker Development Environment

Create `Dockerfile`:

```dockerfile
FROM rust:latest

# Install QEMU and OVMF
RUN apt-get update && apt-get install -y \
    qemu-system-x86 \
    ovmf \
    && rm -rf /var/lib/apt/lists/*

# Install nightly and UEFI target
RUN rustup install nightly \
    && rustup target add x86_64-unknown-uefi --toolchain nightly

WORKDIR /app
```

And `docker-compose.yml`:

```yaml
version: '3.8'
services:
  uefi-dev:
    build: .
    volumes:
      - .:/app
    working_dir: /app
```

## IDE Configuration

### VS Code settings.json

```json
{
    "rust-analyzer.cargo.target": "x86_64-unknown-uefi",
    "rust-analyzer.cargo.features": [],
    "rust-analyzer.checkOnSave.allTargets": false
}
```

### VS Code launch.json (for debugging)

```json
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "cppdbg",
            "request": "launch",
            "name": "Debug UEFI",
            "program": "${workspaceFolder}/target/x86_64-unknown-uefi/release/uefi-app.efi",
            "miDebuggerServerAddress": "localhost:1234",
            "miDebuggerPath": "gdb",
            "cwd": "${workspaceFolder}",
            "preLaunchTask": "qemu-debug"
        }
    ]
}
```

## Common Issues

| Issue | Solution |
|-------|----------|
| Missing target | `rustup target add x86_64-unknown-uefi` |
| Build errors | Use nightly: `rustup override set nightly` |
| OVMF not found | Install `ovmf` package or download |
| QEMU hangs | Check OVMF path, use `-serial stdio` |

## Summary

| Component | Purpose |
|-----------|---------|
| `uefi` crate | Safe UEFI bindings |
| Nightly Rust | Required for UEFI target |
| QEMU | VM for testing |
| OVMF | Open-source UEFI firmware |

## Next Steps

Create your first UEFI application in [UEFI Hello World]({% link part7/03-uefi-hello.md %}).
