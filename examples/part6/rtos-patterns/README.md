# RTOS Patterns Example

This example demonstrates RTOS (Real-Time Operating System) patterns and async embedded programming concepts.

## Topics Covered

- Task management and scheduling
- Message passing between tasks
- Semaphores and mutexes
- Event flags and signals
- Timer services
- Embassy-style async patterns
- FreeRTOS-style patterns
- Resource management

## Note

This is a conceptual example that runs in a standard Rust environment.
Real RTOS development requires:
- An actual RTOS (FreeRTOS, Zephyr, RIOT, etc.) or
- Embassy framework for async embedded
- `#![no_std]` environment
- Target hardware

## Running

```bash
cargo run
```

## Related Documentation

See [RTOS Integration](../../part6/07-rtos.md) for detailed explanations.
