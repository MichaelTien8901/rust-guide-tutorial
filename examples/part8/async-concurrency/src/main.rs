//! # Async Concurrency Example
//!
//! Demonstrates Embassy-style async patterns using standard Rust for CI.
//! Covers: manual future polling, simulated task spawning, channel
//! communication, timer/ticker simulation, and cooperative multitasking.

use std::future::Future;
use std::pin::Pin;
use std::sync::mpsc;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
use std::time::{Duration, Instant};

fn main() {
    println!("=== Async Concurrency ===\n");

    demonstrate_manual_executor();
    demonstrate_task_spawning();
    demonstrate_channel_communication();
    demonstrate_ticker();
    demonstrate_cooperative_multitasking();
}

// ---------------------------------------------------------------------------
// Manual Executor — polling futures by hand
// ---------------------------------------------------------------------------

/// A future that counts down from `n` to zero, yielding Pending each poll.
struct CountdownFuture {
    remaining: u32,
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.remaining == 0 {
            Poll::Ready("countdown complete")
        } else {
            self.remaining -= 1;
            Poll::Pending
        }
    }
}

/// Minimal no-op waker (mirrors what an embedded executor does before
/// hardware wakers are set up).
fn noop_waker() -> Waker {
    fn noop(_: *const ()) {}
    fn clone(p: *const ()) -> RawWaker {
        RawWaker::new(p, &VTABLE)
    }
    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

fn demonstrate_manual_executor() {
    println!("--- Manual Executor (polling a future) ---");

    let mut future = CountdownFuture { remaining: 3 };
    let waker = noop_waker();
    let mut cx = Context::from_waker(&waker);

    let mut polls = 0u32;
    loop {
        polls += 1;
        // SAFETY: future is on the stack and never moved after pinning.
        let pinned = unsafe { Pin::new_unchecked(&mut future) };
        match pinned.poll(&mut cx) {
            Poll::Pending => println!("  poll #{}: Pending", polls),
            Poll::Ready(msg) => {
                println!("  poll #{}: Ready — {}", polls, msg);
                break;
            }
        }
    }
    println!("  Total polls: {}\n", polls);
}

// ---------------------------------------------------------------------------
// Simulated Task Spawning — LED blink + button monitor
// ---------------------------------------------------------------------------

/// Simulates an LED blink task (toggles state each "tick").
struct LedState {
    on: bool,
    toggles: u32,
}

/// Simulates a button monitor task.
struct ButtonState {
    presses: u32,
    schedule: Vec<u32>, // tick numbers when the button is pressed
}

fn demonstrate_task_spawning() {
    println!("--- Task Spawning (LED blink + button monitor) ---");

    let mut led = LedState { on: false, toggles: 0 };
    let mut button = ButtonState {
        presses: 0,
        schedule: vec![2, 5, 7],
    };

    // Simulate 10 executor ticks with two concurrent tasks
    for tick in 0..10 {
        // Task 1: blink LED every 2 ticks
        if tick % 2 == 0 {
            led.on = !led.on;
            led.toggles += 1;
            println!(
                "  tick {}: LED {}",
                tick,
                if led.on { "ON" } else { "OFF" }
            );
        }

        // Task 2: button press on scheduled ticks
        if button.schedule.contains(&tick) {
            button.presses += 1;
            println!("  tick {}: Button pressed (count={})", tick, button.presses);
        }
    }

    println!("  Summary: {} LED toggles, {} button presses\n", led.toggles, button.presses);
}

// ---------------------------------------------------------------------------
// Channel-Based Communication (producer → consumer)
// ---------------------------------------------------------------------------

fn demonstrate_channel_communication() {
    println!("--- Channel Communication (bounded, backpressure) ---");

    // std::sync::mpsc stands in for embassy_sync::channel::Channel
    let (tx, rx) = mpsc::sync_channel::<u16>(4); // capacity 4

    // Producer: send sensor readings
    let producer = std::thread::spawn(move || {
        let readings: Vec<u16> = vec![100, 250, 400, 800, 150, 950];
        for value in readings {
            tx.send(value).unwrap();
        }
    });

    // Consumer: process readings, flag threshold breaches
    let threshold = 500;
    let mut alerts = 0u32;
    let mut total = 0u32;

    producer.join().unwrap();

    while let Ok(value) = rx.try_recv() {
        total += 1;
        if value > threshold {
            alerts += 1;
            println!("  ALERT: sensor value {} exceeds threshold {}", value, threshold);
        } else {
            println!("  OK:    sensor value {}", value);
        }
    }

    println!("  Processed {} readings, {} alerts\n", total, alerts);
}

// ---------------------------------------------------------------------------
// Timer / Ticker Simulation
// ---------------------------------------------------------------------------

fn demonstrate_ticker() {
    println!("--- Ticker Simulation (periodic execution) ---");

    let interval = Duration::from_millis(50);
    let start = Instant::now();
    let mut tick_count = 0u32;

    // Simulate a ticker that fires 5 times at 50ms intervals
    while tick_count < 5 {
        let expected = interval * tick_count;
        let actual = start.elapsed();

        // Busy-wait until the next tick (simulates Timer::after().await)
        while start.elapsed() < interval * (tick_count + 1) {
            std::hint::spin_loop();
        }

        tick_count += 1;
        let elapsed = start.elapsed();
        println!(
            "  tick {}: elapsed {:>4}ms (target {}ms)",
            tick_count,
            elapsed.as_millis(),
            (interval * tick_count).as_millis()
        );
    }

    let drift = start.elapsed().as_millis() as i64 - (interval * tick_count).as_millis() as i64;
    println!("  Total drift: {}ms\n", drift);
}

// ---------------------------------------------------------------------------
// Cooperative Multitasking — demonstrating yield points
// ---------------------------------------------------------------------------

fn demonstrate_cooperative_multitasking() {
    println!("--- Cooperative Multitasking (yield points) ---");

    // Simulate processing data in chunks with yields between them
    let data: Vec<u8> = (0..200).collect();
    let chunk_size = 64;
    let mut checksum: u32 = 0;
    let mut yields = 0u32;

    for (i, chunk) in data.chunks(chunk_size).enumerate() {
        // Process chunk
        for byte in chunk {
            checksum = checksum.wrapping_add(*byte as u32);
        }

        // Yield point — in Embassy this would be yield_now().await
        yields += 1;
        println!(
            "  chunk {}: processed {} bytes, yielded (checksum so far: {})",
            i,
            chunk.len(),
            checksum
        );
    }

    println!("  Final checksum: {}, yield points: {}", checksum, yields);
    println!("  Without yields: other tasks would be starved for the entire computation");
    println!();
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_future_completes() {
        let mut future = CountdownFuture { remaining: 5 };
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);

        for i in (0..5).rev() {
            let pinned = unsafe { Pin::new_unchecked(&mut future) };
            assert!(matches!(pinned.poll(&mut cx), Poll::Pending));
            assert_eq!(future.remaining, i);
        }

        let pinned = unsafe { Pin::new_unchecked(&mut future) };
        assert!(matches!(pinned.poll(&mut cx), Poll::Ready("countdown complete")));
    }

    #[test]
    fn test_channel_backpressure() {
        let (tx, rx) = mpsc::sync_channel::<u16>(2);
        tx.send(10).unwrap();
        tx.send(20).unwrap();
        // Channel is full — try_send should fail
        assert!(tx.try_send(30).is_err());

        assert_eq!(rx.recv().unwrap(), 10);
        assert_eq!(rx.recv().unwrap(), 20);
    }

    #[test]
    fn test_cooperative_chunking() {
        let data: Vec<u8> = (0..128).collect();
        let chunk_size = 64;
        let mut checksum: u32 = 0;
        let mut chunk_count = 0u32;

        for chunk in data.chunks(chunk_size) {
            for byte in chunk {
                checksum = checksum.wrapping_add(*byte as u32);
            }
            chunk_count += 1;
        }

        // Sum of 0..128 = 127 * 128 / 2 = 8128
        assert_eq!(checksum, 8128);
        assert_eq!(chunk_count, 2);
    }

    #[test]
    fn test_led_toggle_logic() {
        let mut led = LedState { on: false, toggles: 0 };
        for _ in 0..6 {
            led.on = !led.on;
            led.toggles += 1;
        }
        // 6 toggles from false: false->true->false->true->false->true->false
        assert_eq!(led.on, false);
        assert_eq!(led.toggles, 6);
    }

    #[test]
    fn test_noop_waker_does_not_panic() {
        let waker = noop_waker();
        waker.wake_by_ref(); // should not panic
        waker.clone().wake(); // should not panic
    }
}
