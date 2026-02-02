---
layout: default
title: Installation
parent: Part 1 - Getting Started
nav_order: 2
---

# Installing Rust

The recommended way to install Rust is through **rustup**, the official Rust toolchain manager.

## Linux and macOS

Open a terminal and run:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen instructions. The default installation is recommended for most users.

After installation, restart your terminal or run:

```bash
source $HOME/.cargo/env
```

### Verify Installation

```bash
rustc --version
cargo --version
```

You should see output like:
```
rustc 1.75.0 (82e1608df 2023-12-21)
cargo 1.75.0 (1d8b05cdd 2023-11-20)
```

## Windows

### Option 1: Using rustup-init.exe (Recommended)

1. Download [rustup-init.exe](https://win.rustup.rs/)
2. Run the installer
3. Follow the on-screen instructions

{: .important }
You'll need the Visual Studio C++ Build Tools. The installer will guide you through this.

### Option 2: Visual Studio Build Tools

If you don't have Visual Studio:

1. Download [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
2. Install "Desktop development with C++"
3. Then run rustup-init.exe

### Verify on Windows

Open Command Prompt or PowerShell:

```powershell
rustc --version
cargo --version
```

## Understanding the Toolchain

After installation, you have:

| Tool | Purpose |
|------|---------|
| `rustc` | The Rust compiler |
| `cargo` | Package manager and build tool |
| `rustup` | Toolchain manager |
| `rustfmt` | Code formatter |
| `clippy` | Linter (install separately) |

## Toolchain Channels

Rust has three release channels:

```mermaid
graph LR
    Nightly[Nightly] -->|6 weeks| Beta[Beta]
    Beta -->|6 weeks| Stable[Stable]
```

- **Stable**: Recommended for most use cases (default)
- **Beta**: Next stable release, for testing
- **Nightly**: Latest features, may break

### Switching Channels

```bash
# Install nightly
rustup install nightly

# Use nightly for current project
rustup override set nightly

# Use nightly globally
rustup default nightly

# Return to stable
rustup default stable
```

## Adding Components

Install additional tools:

```bash
# Code formatter
rustup component add rustfmt

# Linter
rustup component add clippy

# Language server (for IDEs)
rustup component add rust-analyzer
```

## Cross-Compilation Targets

Add compilation targets for other platforms:

```bash
# List available targets
rustup target list

# Add WebAssembly target
rustup target add wasm32-unknown-unknown

# Add ARM target
rustup target add aarch64-unknown-linux-gnu
```

## Updating Rust

Keep your toolchain current:

```bash
rustup update
```

## Uninstalling Rust

If you ever need to uninstall:

```bash
rustup self uninstall
```

## Troubleshooting

### PATH Issues

If `rustc` isn't found after installation:

**Linux/macOS:**
```bash
export PATH="$HOME/.cargo/bin:$PATH"
# Add to ~/.bashrc or ~/.zshrc for permanent fix
```

**Windows:**
Restart your terminal or computer.

### Permission Denied (Linux/macOS)

Don't use `sudo` with rustup. If you have permission issues:

```bash
# Fix ownership
sudo chown -R $(whoami) ~/.cargo ~/.rustup
```

### Linker Errors (Linux)

Install build essentials:

```bash
# Ubuntu/Debian
sudo apt install build-essential

# Fedora
sudo dnf install gcc

# Arch
sudo pacman -S base-devel
```

## Next Steps

With Rust installed, let's [set up your IDE]({% link part1/03-ide-setup.md %}) for the best development experience.
