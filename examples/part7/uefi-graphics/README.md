# UEFI Graphics Example

This example demonstrates UEFI Graphics Output Protocol (GOP) patterns.

## Topics Covered

- Graphics Output Protocol (GOP)
- Pixel formats (RGB, BGR, Bitmask)
- Mode enumeration and setting
- Framebuffer access and manipulation
- Basic drawing primitives
- Double buffering concepts

## Note

This is a conceptual example that runs in a standard Rust environment.
Real UEFI graphics programming requires:
- The `uefi` crate
- `#![no_std]` environment
- Access to UEFI Boot Services and GOP

## Running

```bash
cargo run
```

## Related Documentation

See [UEFI Graphics](../../part7/07-graphics.md) for detailed explanations.
