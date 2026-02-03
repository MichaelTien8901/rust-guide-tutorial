---
layout: default
title: RTOS
parent: Part 6 - Systems
nav_order: 7
---

# RTOS Integration

Using Rust with FreeRTOS and async embedded with Embassy.

## FreeRTOS with Rust

FreeRTOS is a popular RTOS that can be used from Rust via FFI bindings.

### FreeRTOS Bindings

```rust
// Using freertos-rust crate
use freertos_rust::*;

fn main() {
    let task1 = Task::new()
        .name("task1")
        .stack_size(256)
        .priority(TaskPriority(1))
        .start(|| {
            loop {
                // Task work
                CurrentTask::delay(Duration::ms(100));
            }
        })
        .unwrap();

    FreeRtosUtils::start_scheduler();
}
```

Add to Cargo.toml:
```toml
[dependencies]
freertos-rust = "0.1"
```

### Tasks and Synchronization

```rust
use freertos_rust::*;

static QUEUE: Queue<u32> = Queue::new(10).unwrap();
static MUTEX: Mutex<u32> = Mutex::new(0).unwrap();
static SEMAPHORE: Semaphore = Semaphore::new_binary().unwrap();

fn producer_task() {
    loop {
        QUEUE.send(42, Duration::infinite()).unwrap();
        CurrentTask::delay(Duration::ms(100));
    }
}

fn consumer_task() {
    loop {
        if let Ok(value) = QUEUE.receive(Duration::ms(500)) {
            // Process value
        }
    }
}

fn mutex_example() {
    let mut guard = MUTEX.lock(Duration::infinite()).unwrap();
    *guard += 1;
}
```

## Embassy - Async Embedded Rust

Embassy is a modern async runtime for embedded systems, providing:
- Async/await for embedded
- Zero-cost abstractions
- No heap required
- HAL implementations

### Embassy Project Setup

```toml
# Cargo.toml
[dependencies]
embassy-executor = { version = "0.5", features = ["arch-cortex-m", "executor-thread"] }
embassy-time = { version = "0.3", features = ["tick-hz-32_768"] }
embassy-stm32 = { version = "0.1", features = ["stm32f411ce", "time-driver-any"] }
cortex-m = "0.7"
cortex-m-rt = "0.7"
defmt = "0.3"
defmt-rtt = "0.4"
panic-probe = { version = "0.3", features = ["print-defmt"] }
```

### Basic Embassy Application

```rust
#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_stm32::gpio::{Level, Output, Speed};
use embassy_time::Timer;
use defmt::info;
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut led = Output::new(p.PC13, Level::High, Speed::Low);

    loop {
        info!("LED on");
        led.set_low();
        Timer::after_millis(500).await;

        info!("LED off");
        led.set_high();
        Timer::after_millis(500).await;
    }
}
```

### Multiple Tasks

```rust
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

static CHANNEL: Channel<CriticalSectionRawMutex, u32, 4> = Channel::new();

#[embassy_executor::task]
async fn producer() {
    let mut counter = 0u32;
    loop {
        CHANNEL.send(counter).await;
        counter += 1;
        Timer::after(Duration::from_millis(100)).await;
    }
}

#[embassy_executor::task]
async fn consumer() {
    loop {
        let value = CHANNEL.receive().await;
        info!("Received: {}", value);
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let _p = embassy_stm32::init(Default::default());

    spawner.spawn(producer()).unwrap();
    spawner.spawn(consumer()).unwrap();

    // Main task can do other work or just idle
    loop {
        Timer::after(Duration::from_secs(1)).await;
    }
}
```

### Embassy Synchronization Primitives

```rust
use embassy_sync::mutex::Mutex;
use embassy_sync::signal::Signal;
use embassy_sync::channel::Channel;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;

// Async mutex
static SHARED_DATA: Mutex<CriticalSectionRawMutex, u32> = Mutex::new(0);

// Signal for notifications
static SIGNAL: Signal<CriticalSectionRawMutex, ()> = Signal::new();

async fn update_shared() {
    let mut data = SHARED_DATA.lock().await;
    *data += 1;
}

async fn wait_for_signal() {
    SIGNAL.wait().await;
}

fn trigger_signal() {
    SIGNAL.signal(());
}
```

### Embassy UART Example

```rust
use embassy_stm32::usart::{Config, Uart};
use embassy_stm32::bind_interrupts;
use embassy_stm32::peripherals;

bind_interrupts!(struct Irqs {
    USART2 => embassy_stm32::usart::InterruptHandler<peripherals::USART2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let config = Config::default();
    let mut usart = Uart::new(
        p.USART2,
        p.PA3,  // RX
        p.PA2,  // TX
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH5,
        config,
    ).unwrap();

    // Echo received data
    let mut buf = [0u8; 64];
    loop {
        let n = usart.read_until_idle(&mut buf).await.unwrap();
        usart.write(&buf[..n]).await.unwrap();
    }
}
```

### Embassy vs FreeRTOS

| Aspect | Embassy | FreeRTOS |
|--------|---------|----------|
| Language | Pure Rust | C with Rust bindings |
| Concurrency | async/await | Tasks with preemption |
| Memory | No heap required | Heap for tasks |
| Size | Very small | Larger footprint |
| Ecosystem | Growing | Mature |
| Debugging | Rust tooling | Traditional RTOS tools |

## RTIC - Real-Time Interrupt-driven Concurrency

RTIC is a concurrency framework for building real-time systems:

```rust
#![no_std]
#![no_main]

use panic_halt as _;
use rtic::app;
use stm32f4xx_hal::{pac, prelude::*};

#[app(device = stm32f4xx_hal::pac, peripherals = true)]
mod app {
    use super::*;

    #[shared]
    struct Shared {
        counter: u32,
    }

    #[local]
    struct Local {
        led: gpio::PC13<gpio::Output<gpio::PushPull>>,
    }

    #[init]
    fn init(ctx: init::Context) -> (Shared, Local) {
        let gpioc = ctx.device.GPIOC.split();
        let led = gpioc.pc13.into_push_pull_output();

        (
            Shared { counter: 0 },
            Local { led },
        )
    }

    #[task(shared = [counter])]
    async fn task1(mut ctx: task1::Context) {
        ctx.shared.counter.lock(|counter| {
            *counter += 1;
        });
    }

    #[task(binds = EXTI0, local = [led], shared = [counter])]
    fn button_pressed(mut ctx: button_pressed::Context) {
        ctx.local.led.toggle();
        ctx.shared.counter.lock(|counter| {
            *counter += 1;
        });
    }
}
```

## Choosing an RTOS Approach

| Use Case | Recommendation |
|----------|----------------|
| New project, Rust-first | Embassy |
| Existing FreeRTOS codebase | freertos-rust bindings |
| Hard real-time, simple | RTIC |
| Complex async workflows | Embassy |
| Legacy hardware support | FreeRTOS |

## Summary

| Framework | Model | Best For |
|-----------|-------|----------|
| Embassy | Async/await | Modern embedded Rust |
| RTIC | Interrupt-driven | Hard real-time |
| FreeRTOS | Traditional RTOS | Legacy/mixed codebases |

## See Also

- [Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/master/examples/part6/rtos-patterns)

## Next Steps

Learn about [Cross-Compilation]({% link part6/08-cross-compilation.md %}) for different targets.
