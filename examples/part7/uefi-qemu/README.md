# UEFI QEMU Testing Example

This example demonstrates how to test UEFI applications with QEMU and OVMF.

## Topics Covered

- QEMU setup for UEFI
- OVMF firmware configuration
- Creating bootable disk images
- Debug output and logging
- GDB debugging setup
- Common QEMU flags for UEFI

## Directory Structure

```
uefi-qemu/
├── Cargo.toml
├── README.md
├── src/
│   └── main.rs          # Conceptual example
├── scripts/
│   ├── run-qemu.sh      # QEMU launch script
│   ├── create-disk.sh   # Disk image creation
│   └── debug-qemu.sh    # GDB debugging script
└── esp/                  # Example ESP structure
    └── EFI/
        └── BOOT/
            └── .gitkeep
```

## Prerequisites for Real UEFI Testing

1. **QEMU**: `sudo apt install qemu-system-x86`
2. **OVMF**: `sudo apt install ovmf`
3. **Rust target**: `rustup target add x86_64-unknown-uefi`

## Running the Conceptual Example

```bash
cargo run
```

## Testing Real UEFI Applications

1. Build your UEFI application:
   ```bash
   cargo build --target x86_64-unknown-uefi --release
   ```

2. Copy to ESP:
   ```bash
   cp target/x86_64-unknown-uefi/release/your_app.efi esp/EFI/BOOT/BOOTX64.EFI
   ```

3. Run with QEMU:
   ```bash
   ./scripts/run-qemu.sh
   ```

## Related Documentation

See [QEMU Testing](../../part7/09-qemu-testing.md) for detailed explanations.
