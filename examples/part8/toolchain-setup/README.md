# Toolchain Setup Example

Demonstrates the project structure and configuration for a Rust embedded project targeting the STM32F769I-DISCO evaluation board.

## Running (Host)

```bash
cargo run
cargo test
```

## Cross-Compiling (STM32F769-DISCO)

To build for the actual hardware:

```bash
# Install the target
rustup target add thumbv7em-none-eabihf

# Build (requires .cargo/config.toml and memory.x from the chapter)
cargo build --target thumbv7em-none-eabihf

# Flash and run (requires probe-rs and connected board)
cargo embed
```

## Related

- [Chapter: Toolchain Setup](https://michaeltien8901.github.io/rust-guide-tutorial/part8/toolchain-setup/)
- [Part 6: Cross-Compilation](https://michaeltien8901.github.io/rust-guide-tutorial/part6/08-cross-compilation/)
