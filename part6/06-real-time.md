---
layout: default
title: Real-Time
parent: Part 6 - Systems
nav_order: 6
---

# Real-Time Programming

Deterministic execution and heapless programming for real-time systems.

## Real-Time Requirements

Real-time systems must respond within guaranteed time bounds:

| Type | Requirement | Example |
|------|-------------|---------|
| Hard real-time | Miss = failure | Airbag deployment |
| Firm real-time | Miss = degraded | Video streaming |
| Soft real-time | Miss = lower quality | User interface |

## Challenges for Rust

| Challenge | Solution |
|-----------|----------|
| Heap allocation | Use heapless collections |
| Garbage collection | Rust has none! |
| Unbounded operations | Use fixed-size buffers |
| Priority inversion | Proper mutex design |

## Heapless Collections

Stack-allocated, fixed-capacity data structures:

```rust
use heapless::{Vec, String, FnvIndexMap};

fn example() {
    // Vector with capacity 16
    let mut vec: Vec<u32, 16> = Vec::new();
    vec.push(1).unwrap();
    vec.push(2).unwrap();

    // String with capacity 64 bytes
    let mut s: String<64> = String::new();
    s.push_str("Hello").unwrap();

    // HashMap with capacity 8
    let mut map: FnvIndexMap<&str, u32, 8> = FnvIndexMap::new();
    map.insert("key", 42).unwrap();
}
```

Add to Cargo.toml:
```toml
[dependencies]
heapless = "0.8"
```

## Heapless Queues

For inter-task communication:

```rust
use heapless::spsc::Queue;

// Single-producer, single-consumer queue
static mut QUEUE: Queue<u32, 16> = Queue::new();

fn producer() {
    unsafe {
        let mut producer = QUEUE.split().0;
        producer.enqueue(42).ok();
    }
}

fn consumer() {
    unsafe {
        let mut consumer = QUEUE.split().1;
        if let Some(value) = consumer.dequeue() {
            // Process value
        }
    }
}
```

## Memory Pools

Pre-allocate fixed-size blocks:

```rust
use heapless::pool::Pool;
use heapless::pool::singleton::Pool as SingletonPool;

// Define a memory pool
static POOL: SingletonPool<[u8; 128]> = SingletonPool::new();

fn init_pool() {
    // Grow pool with static memory
    static mut MEMORY: [u8; 128 * 10] = [0; 128 * 10];
    unsafe {
        POOL.grow(&mut MEMORY);
    }
}

fn allocate() -> Option<Box<[u8; 128]>> {
    POOL.alloc()
}
```

## Bounded Execution Time

Avoid unbounded operations:

```rust
// BAD: Unbounded iteration
fn bad_search(data: &[u32], target: u32) -> Option<usize> {
    data.iter().position(|&x| x == target)
}

// GOOD: Bounded iteration with maximum
fn bounded_search(data: &[u32], target: u32, max_iters: usize) -> Option<usize> {
    for (i, &val) in data.iter().take(max_iters).enumerate() {
        if val == target {
            return Some(i);
        }
    }
    None
}

// GOOD: Fixed-size lookup table
const LOOKUP: [u32; 256] = [/* ... */];
fn constant_time_lookup(index: u8) -> u32 {
    LOOKUP[index as usize]
}
```

## Atomic Operations

Lock-free synchronization for real-time:

```rust
use core::sync::atomic::{AtomicU32, AtomicBool, Ordering};

static COUNTER: AtomicU32 = AtomicU32::new(0);
static FLAG: AtomicBool = AtomicBool::new(false);

fn increment() {
    COUNTER.fetch_add(1, Ordering::SeqCst);
}

fn signal() {
    FLAG.store(true, Ordering::Release);
}

fn wait_for_signal() {
    while !FLAG.load(Ordering::Acquire) {
        core::hint::spin_loop();
    }
}
```

## Critical Sections

Disable interrupts for atomic access:

```rust
use cortex_m::interrupt;

static mut SHARED: u32 = 0;

fn critical_update(value: u32) {
    interrupt::free(|_| {
        unsafe {
            SHARED = value;
        }
    });
}

// With RTIC (Real-Time Interrupt-driven Concurrency)
#[rtic::app(device = stm32f4xx_hal::pac)]
mod app {
    #[shared]
    struct Shared {
        counter: u32,
    }

    #[task(shared = [counter])]
    fn task1(mut ctx: task1::Context) {
        ctx.shared.counter.lock(|counter| {
            *counter += 1;
        });
    }
}
```

## Timing and Deadlines

```rust
use core::time::Duration;

struct Deadline {
    start: u64,
    timeout_us: u64,
}

impl Deadline {
    fn new(timeout: Duration) -> Self {
        Deadline {
            start: get_time_us(),
            timeout_us: timeout.as_micros() as u64,
        }
    }

    fn is_expired(&self) -> bool {
        get_time_us() - self.start >= self.timeout_us
    }

    fn remaining(&self) -> Option<Duration> {
        let elapsed = get_time_us() - self.start;
        if elapsed >= self.timeout_us {
            None
        } else {
            Some(Duration::from_micros(self.timeout_us - elapsed))
        }
    }
}

fn task_with_deadline() -> Result<(), TimeoutError> {
    let deadline = Deadline::new(Duration::from_millis(10));

    while !work_complete() {
        if deadline.is_expired() {
            return Err(TimeoutError);
        }
        do_work_chunk();
    }
    Ok(())
}
```

## Watchdog Timer

Detect and recover from hangs:

```rust
struct Watchdog {
    // Hardware watchdog registers
}

impl Watchdog {
    fn feed(&mut self) {
        // Reset watchdog counter
        unsafe {
            write_volatile(WATCHDOG_RELOAD, WATCHDOG_KEY);
        }
    }

    fn enable(&mut self, timeout_ms: u32) {
        // Configure and enable watchdog
    }
}

fn main_loop(mut watchdog: Watchdog) -> ! {
    loop {
        // Do work
        process_inputs();
        update_state();
        generate_outputs();

        // Feed watchdog to prevent reset
        watchdog.feed();
    }
}
```

## Stack Usage Analysis

Monitor stack usage to prevent overflow:

```rust
const STACK_CANARY: u32 = 0xDEAD_BEEF;

#[link_section = ".stack_guard"]
static STACK_GUARD: u32 = STACK_CANARY;

fn check_stack_overflow() -> bool {
    unsafe {
        core::ptr::read_volatile(&STACK_GUARD) != STACK_CANARY
    }
}

// Paint stack for high-water mark analysis
fn paint_stack(stack: &mut [u32]) {
    for word in stack.iter_mut() {
        *word = STACK_CANARY;
    }
}

fn measure_stack_usage(stack: &[u32]) -> usize {
    let mut used = 0;
    for word in stack.iter().rev() {
        if *word != STACK_CANARY {
            used += 4;
        } else {
            break;
        }
    }
    used
}
```

## Best Practices

1. **No heap allocation** in real-time paths
2. **Bound all loops** with maximum iterations
3. **Use atomic operations** for lock-free code
4. **Minimize critical sections**
5. **Use watchdogs** for fault recovery
6. **Analyze stack usage**
7. **Test worst-case timing**

## Summary

| Technique | Purpose |
|-----------|---------|
| Heapless collections | Fixed-size data structures |
| Atomics | Lock-free synchronization |
| Critical sections | Mutual exclusion |
| Deadlines | Time-bounded operations |
| Watchdog | Fault recovery |

## See Also

- [Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/master/examples/part6/real-time)

## Next Steps

Learn about [RTOS]({% link part6/07-rtos.md %}) integration with FreeRTOS and Embassy.
