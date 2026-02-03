# Cross-Compilation Example

This example demonstrates cross-compilation concepts and target configuration patterns.

## Topics Covered

- Target triple format and components
- Conditional compilation with cfg
- Target-specific code organization
- Build scripts for cross-compilation
- Linker configuration
- Size optimization techniques
- Binary inspection and analysis

## Note

This is a conceptual example that runs in a standard Rust environment.
Real cross-compilation requires:
- Target toolchain installation (rustup target add)
- Cross-linker (arm-none-eabi-gcc, etc.)
- Target-specific runtime or bare-metal setup
- Proper .cargo/config.toml configuration

## Running

```bash
cargo run
```

## Cross-Compilation Example Commands

```bash
# List available targets
rustup target list

# Add a target
rustup target add thumbv7em-none-eabihf

# Build for target
cargo build --target thumbv7em-none-eabihf --release

# Using cross (Docker-based)
cargo install cross
cross build --target aarch64-unknown-linux-gnu
```

## Related Documentation

See [Cross-Compilation](../../part6/08-cross-compilation.md) for detailed explanations.
