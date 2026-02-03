# Embedded HAL Example

This example demonstrates the embedded-hal (Hardware Abstraction Layer) trait patterns.

## Topics Covered

- GPIO traits (InputPin, OutputPin)
- SPI traits (SpiBus, SpiDevice)
- I2C traits (I2c)
- UART/Serial traits
- Delay traits
- PWM traits
- ADC traits
- Driver patterns using HAL traits

## Note

This is a conceptual example that runs in a standard Rust environment.
Real embedded-hal usage requires:
- The `embedded-hal` crate
- A HAL implementation for your target hardware
- `#![no_std]` environment

## Running

```bash
cargo run
```

## Related Documentation

See [Embedded HAL](../../part6/03-embedded-hal.md) for detailed explanations.
