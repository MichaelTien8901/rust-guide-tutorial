//! # Debugging Embedded Applications Example
//!
//! Demonstrates debugging tools and techniques for embedded Rust,
//! targeting the STM32F769I-DISCO evaluation board.
//!
//! This example compiles as standard Rust for CI validation.
//! Embedded-specific APIs (defmt, RTT, probe-rs) are simulated
//! with host equivalents so the patterns remain instructive.

use std::collections::VecDeque;
use std::fmt;

fn main() {
    println!("=== Debugging Embedded Applications ===\n");

    demonstrate_defmt_log_levels();
    demonstrate_rtt_channel();
    demonstrate_panic_handler_strategies();
    demonstrate_variable_inspection();
    demonstrate_debug_vs_release();
    demonstrate_debugging_pitfalls();
}

// ---------------------------------------------------------------------------
// 1. defmt-style Logging (simulated with println!)
// ---------------------------------------------------------------------------

/// Log levels matching defmt's severity hierarchy.
///
/// In a real embedded project you would use the `defmt` crate:
/// ```ignore
/// // Cargo.toml
/// defmt = "0.3"
/// defmt-rtt = "0.4"       // transport over RTT
/// panic-probe = "0.3"     // panic handler that works with probe-rs
///
/// // source code
/// defmt::info!("Booted, clock = {} MHz", sysclk / 1_000_000);
/// ```
///
/// defmt encodes log messages as compact integer tokens on the target.
/// The host-side tooling (probe-rs, defmt-print) decodes them using
/// the ELF's string table, keeping firmware size and runtime overhead
/// extremely small compared to formatted strings.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum LogLevel {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info  => write!(f, "INFO "),
            LogLevel::Warn  => write!(f, "WARN "),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// A simulated defmt logger that filters by minimum level.
///
/// On real hardware, log level filtering is set at compile time
/// via Cargo features (e.g., `defmt = { version = "0.3", features = ["defmt-trace"] }`).
/// Messages below the compiled level are eliminated entirely — zero cost.
struct DefmtLogger {
    min_level: LogLevel,
    message_count: usize,
}

impl DefmtLogger {
    fn new(min_level: LogLevel) -> Self {
        Self { min_level, message_count: 0 }
    }

    fn log(&mut self, level: LogLevel, message: &str) {
        if level >= self.min_level {
            // On embedded: defmt encodes this as a compact token, not a string
            println!("  [{}] {}", level, message);
            self.message_count += 1;
        }
    }
}

fn demonstrate_defmt_log_levels() {
    println!("--- defmt-style Logging ---");
    println!("  (simulated with println!; real defmt uses compact token encoding)\n");

    let mut logger = DefmtLogger::new(LogLevel::Debug);

    // These mirror typical embedded log calls
    logger.log(LogLevel::Trace, "Register GPIOJ->MODER = 0x0400_0000");
    logger.log(LogLevel::Debug, "LED pin PJ13 configured as output");
    logger.log(LogLevel::Info,  "System clock: 216 MHz (HSE + PLL)");
    logger.log(LogLevel::Warn,  "UART TX buffer 80% full");
    logger.log(LogLevel::Error, "I2C timeout on address 0x48");

    println!();
    println!("  Messages emitted: {} (trace filtered at Debug level)", logger.message_count);
    println!("  defmt tip: use DEFMT_LOG=info to filter at runtime with probe-rs");
    println!();
}

// ---------------------------------------------------------------------------
// 2. RTT (Real-Time Transfer) Channel Simulation
// ---------------------------------------------------------------------------

/// Simulates an RTT up-channel (target -> host).
///
/// Real RTT uses a control block in RAM with ring buffers.
/// The debug probe reads these buffers over SWD without halting the CPU,
/// making RTT much faster than semihosting or UART logging.
///
/// Memory layout of a real RTT control block:
/// ```text
///   +---------------------+
///   | "SEGGER RTT" magic  |   <- probe scans RAM to find this
///   | num_up_channels      |
///   | num_down_channels    |
///   +---------------------+
///   | Up Channel 0        |
///   |   name_ptr           |
///   |   buffer_ptr         |   <- ring buffer in target RAM
///   |   size               |
///   |   write_offset       |   <- target writes here
///   |   read_offset        |   <- host reads here
///   |   flags              |
///   +---------------------+
/// ```
struct RttChannel {
    name: &'static str,
    buffer: VecDeque<u8>,
    capacity: usize,
    total_bytes_written: usize,
}

impl RttChannel {
    fn new(name: &'static str, capacity: usize) -> Self {
        Self {
            name,
            buffer: VecDeque::with_capacity(capacity),
            capacity,
            total_bytes_written: 0,
        }
    }

    /// Target writes data into the ring buffer.
    /// Returns the number of bytes actually written (may be less if buffer is full).
    fn write(&mut self, data: &[u8]) -> usize {
        let available = self.capacity - self.buffer.len();
        let to_write = data.len().min(available);
        for &byte in &data[..to_write] {
            self.buffer.push_back(byte);
        }
        self.total_bytes_written += to_write;
        to_write
    }

    /// Host (probe) reads data from the ring buffer without halting CPU.
    fn read(&mut self, buf: &mut [u8]) -> usize {
        let to_read = buf.len().min(self.buffer.len());
        for byte in buf.iter_mut().take(to_read) {
            *byte = self.buffer.pop_front().unwrap();
        }
        to_read
    }

    fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn name(&self) -> &str {
        self.name
    }
}

fn demonstrate_rtt_channel() {
    println!("--- RTT (Real-Time Transfer) Channel ---");
    println!("  Shared-memory ring buffer: target writes, probe reads via SWD\n");

    let mut channel = RttChannel::new("Terminal", 64);
    println!("  Channel: {:?} (capacity: {} bytes)", channel.name(), 64);

    // Target writes a log message
    let msg = b"INFO: Boot complete\n";
    let written = channel.write(msg);
    println!("  Target wrote {} bytes -> buffer has {} bytes", written, channel.len());

    // Host reads the message
    let mut host_buf = [0u8; 64];
    let read = channel.read(&mut host_buf);
    let decoded = std::str::from_utf8(&host_buf[..read]).unwrap_or("<invalid utf8>");
    println!("  Host  read {} bytes: {:?}", read, decoded.trim());
    println!("  Buffer now empty: {}", channel.is_empty());

    // Demonstrate buffer-full behavior (blocking vs non-blocking)
    println!();
    println!("  RTT modes:");
    println!("    NoBlockSkip   - Drop data if buffer full (default, no CPU stall)");
    println!("    NoBlockTrim   - Write partial data, discard the rest");
    println!("    BlockIfFull   - Stall CPU until host reads (useful for debugging)");

    // Fill the buffer to demonstrate overflow
    let large_msg = vec![b'X'; 100];
    let written = channel.write(&large_msg);
    println!();
    println!("  Wrote 100 bytes into 64-byte channel: {} bytes accepted", written);
    println!("  {} bytes lost (NoBlockTrim behavior)", 100 - written);
    println!();
}

// ---------------------------------------------------------------------------
// 3. Panic Handler Strategies
// ---------------------------------------------------------------------------

/// The three common panic handler strategies for embedded Rust.
///
/// In `#![no_std]` programs, you must provide a `#[panic_handler]` function.
/// The ecosystem offers several crate-based implementations:
///
/// | Crate             | Behavior                                     |
/// |--------------------|----------------------------------------------|
/// | `panic-halt`      | Enters infinite loop — safe, simple           |
/// | `panic-abort`     | Calls `core::intrinsics::abort()`             |
/// | `panic-probe`     | Logs via defmt, then triggers breakpoint       |
/// | `panic-semihosting`| Prints to host via semihosting (slow)        |
///
/// Only one panic handler can exist in the final binary.
#[derive(Debug, Clone, Copy, PartialEq)]
enum PanicStrategy {
    Halt,
    Abort,
    ProbeBreakpoint,
}

impl fmt::Display for PanicStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PanicStrategy::Halt => write!(f, "panic-halt (infinite loop)"),
            PanicStrategy::Abort => write!(f, "panic-abort (immediate termination)"),
            PanicStrategy::ProbeBreakpoint => write!(f, "panic-probe (defmt log + BKPT)"),
        }
    }
}

/// Simulates what each panic handler does when a panic occurs.
fn simulate_panic_handler(strategy: PanicStrategy, info: &str) -> &'static str {
    match strategy {
        PanicStrategy::Halt => {
            // Real implementation:
            //   #[panic_handler]
            //   fn panic(_info: &PanicInfo) -> ! {
            //       loop { cortex_m::asm::nop(); }
            //   }
            println!("    [HALT] Panic: {}  ->  entering infinite loop", info);
            "device halted in loop"
        }
        PanicStrategy::Abort => {
            // Real implementation:
            //   #[panic_handler]
            //   fn panic(_info: &PanicInfo) -> ! {
            //       cortex_m::asm::udf();  // undefined instruction -> HardFault
            //   }
            println!("    [ABORT] Panic: {}  ->  triggering HardFault (UDF)", info);
            "device reset via HardFault"
        }
        PanicStrategy::ProbeBreakpoint => {
            // Real implementation (panic-probe):
            //   #[panic_handler]
            //   fn panic(info: &PanicInfo) -> ! {
            //       defmt::error!("{}", defmt::Display2Format(info));
            //       cortex_m::asm::bkpt();  // halts if debugger attached
            //       loop {}
            //   }
            println!("    [PROBE] Panic: {}  ->  defmt::error! + BKPT", info);
            "debugger halted at breakpoint"
        }
    }
}

fn demonstrate_panic_handler_strategies() {
    println!("--- Panic Handler Strategies ---");
    println!("  Only ONE panic handler can exist in a #![no_std] binary\n");

    for strategy in [PanicStrategy::Halt, PanicStrategy::Abort, PanicStrategy::ProbeBreakpoint] {
        let result = simulate_panic_handler(strategy, "index out of bounds: len is 4 but index is 7");
        println!("    Result: {}\n", result);
    }

    println!("  Recommendation: use panic-probe during development (best debug info),");
    println!("  switch to panic-halt or panic-abort for production firmware.");
    println!();
}

// ---------------------------------------------------------------------------
// 4. Variable Inspection / Struct Debugging
// ---------------------------------------------------------------------------

/// A register snapshot that a developer might inspect in a debugger.
///
/// When debugging with probe-rs or GDB, you can:
/// - `print peripheral_state` to see all fields
/// - Set watchpoints on specific fields to break on changes
/// - Use `info locals` in GDB to see all local variables
#[derive(Debug, Clone)]
struct PeripheralState {
    gpio_mode: u32,
    gpio_output: u32,
    uart_baud: u32,
    uart_status: UartStatus,
    timer_count: u32,
    timer_prescaler: u16,
    timer_auto_reload: u32,
    dma_enabled: bool,
    irq_priority: u8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UartStatus {
    Idle,
    Transmitting,
    Receiving,
    Error(UartError),
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum UartError {
    Overrun,
    FramingError,
    Noise,
}

impl PeripheralState {
    /// Creates a typical post-init state for the STM32F769I-DISCO.
    fn typical_init() -> Self {
        Self {
            // GPIOJ pin 13 as output push-pull (MODER bits [27:26] = 01)
            gpio_mode: 0x0400_0000,
            gpio_output: 0x0000_2000, // ODR bit 13 set (LED on)
            // 115200 baud at 54 MHz APB1 clock
            // BRR = 54_000_000 / 115_200 ~= 468.75 -> mantissa=468, frac=12
            uart_baud: 115_200,
            uart_status: UartStatus::Idle,
            // TIM2 configured for 1 Hz interrupt (216 MHz / (21600 * 10000))
            timer_count: 0,
            timer_prescaler: 21_600 - 1,
            timer_auto_reload: 10_000 - 1,
            dma_enabled: false,
            irq_priority: 4,
        }
    }

    /// Demonstrate how different views help debug peripheral state.
    fn debug_views(&self) {
        println!("  Debugger views of PeripheralState:");
        println!();

        // Pretty-print (like GDB `print` or probe-rs variable view)
        println!("  1. Variable view (GDB: `print state`, VS Code: Variables panel):");
        println!("     gpio_mode:        {:#010X}", self.gpio_mode);
        println!("     gpio_output:      {:#010X}", self.gpio_output);
        println!("     uart_baud:        {}", self.uart_baud);
        println!("     uart_status:      {:?}", self.uart_status);
        println!("     timer_count:      {}", self.timer_count);
        println!("     timer_prescaler:  {} (raw: {})", self.timer_prescaler + 1, self.timer_prescaler);
        println!("     timer_arr:        {} (raw: {})", self.timer_auto_reload + 1, self.timer_auto_reload);
        println!("     dma_enabled:      {}", self.dma_enabled);
        println!("     irq_priority:     {}", self.irq_priority);

        // Binary view for bit-field analysis
        println!();
        println!("  2. Binary view (for bit-field registers):");
        println!("     GPIOJ MODER: {:032b}", self.gpio_mode);
        println!("                  bit 26 = {} (pin 13 mode[0])", (self.gpio_mode >> 26) & 1);
        println!("                  bit 27 = {} (pin 13 mode[1])", (self.gpio_mode >> 27) & 1);
        println!("                  mode = {:02b} -> General purpose output", (self.gpio_mode >> 26) & 0b11);
    }
}

fn demonstrate_variable_inspection() {
    println!("--- Variable Inspection ---");
    println!("  Demonstrates the kind of state you inspect in a debugger\n");

    let state = PeripheralState::typical_init();
    state.debug_views();

    println!();
    println!("  probe-rs tip: `probe-rs run --chip STM32F769NIHx` automatically");
    println!("  decodes defmt logs and shows panic locations with source lines.");
    println!();
}

// ---------------------------------------------------------------------------
// 5. Debug Build vs Release Build
// ---------------------------------------------------------------------------

/// Compares debug-friendly code to optimized code.
///
/// In embedded Rust, the debug vs release distinction is critical:
///
/// ```toml
/// # Cargo.toml — common embedded settings
/// [profile.dev]
/// opt-level = "s"          # Still optimize for size even in debug
/// debug = 2                # Full debug symbols (DWARF)
/// overflow-checks = true
///
/// [profile.release]
/// opt-level = "z"          # Optimize aggressively for size
/// debug = 2                # Keep debug symbols (strip later if needed)
/// lto = true               # Link-time optimization
/// codegen-units = 1        # Better optimization, slower compile
/// overflow-checks = false  # Disabled for performance
/// ```
fn demonstrate_debug_vs_release() {
    println!("--- Debug Build vs Release Build ---\n");

    // Show what happens to a simple function in each profile
    let debug_result = fibonacci_debug(10);
    let release_result = fibonacci_release(10);

    println!("  fibonacci(10):");
    println!("    Debug build:   {} (with overflow checks, all variables visible)", debug_result);
    println!("    Release build: {} (overflow checks off, many variables optimized out)", release_result);
    println!();
    println!("  Typical binary sizes for a blinky example (STM32F769):");
    println!("    Profile        Flash     Symbols   Stepping");
    println!("    dev            ~64 KB    full      line-by-line");
    println!("    dev (opt=s)    ~12 KB    full      mostly line-by-line");
    println!("    release (lto)  ~4  KB    optional  jumps around");
    println!();
    println!("  Recommendation: Use `opt-level = \"s\"` even in dev profile");
    println!("  to avoid running out of flash, while keeping debug symbols.");
    println!();
}

/// Debug-friendly version: every intermediate value is visible in debugger.
fn fibonacci_debug(n: u32) -> u64 {
    // In a debug build, the compiler preserves a, b, temp, i
    // and you can step through each iteration.
    let mut a: u64 = 0;
    let mut b: u64 = 1;
    for _i in 0..n {
        let temp = a + b; // Visible in locals panel
        a = b;
        b = temp;
    }
    a
}

/// Release-optimized version: same result, but optimizer may remove temporaries.
///
/// In a real release build, the compiler would likely:
/// - Unroll the loop entirely for small n
/// - Eliminate the temp variable
/// - Possibly compute the result at compile time (constant folding)
fn fibonacci_release(n: u32) -> u64 {
    // Annotated to show what the optimizer does
    let (mut a, mut b) = (0u64, 1u64);
    for _ in 0..n {
        let t = a.wrapping_add(b); // wrapping_add: no overflow check in release
        a = b;
        b = t;
    }
    a
}

// ---------------------------------------------------------------------------
// 6. Common Debugging Pitfalls
// ---------------------------------------------------------------------------

/// Demonstrates pitfalls that catch embedded developers.
fn demonstrate_debugging_pitfalls() {
    println!("--- Common Debugging Pitfalls ---\n");

    pitfall_optimized_out_variable();
    pitfall_volatile_read();
    pitfall_292_bug();
    pitfall_semihosting_vs_rtt();
}

/// Pitfall: Variables optimized away in release builds.
///
/// When debugging optimized code, local variables may show as
/// "<optimized out>" in GDB or "unavailable" in VS Code.
fn pitfall_optimized_out_variable() {
    println!("  Pitfall 1: Optimized-out variables");
    println!();

    let sensor_value: u32 = 1023;
    let calibrated = sensor_value * 330 / 4096; // 3.3V ADC with 12-bit resolution
    let threshold = 165; // ~1.65V midpoint

    // In release: sensor_value and calibrated may be optimized out
    // since only `above_threshold` is used later.
    let above_threshold = calibrated > threshold;

    println!("    sensor_value = {} (may show '<optimized out>' in release)", sensor_value);
    println!("    calibrated   = {} (may be folded into comparison)", calibrated);
    println!("    above = {}", above_threshold);
    println!();
    println!("    Fix: use `core::hint::black_box(&variable)` to prevent optimization");
    println!("    Fix: inspect disassembly with `cargo objdump -- -d -S`");

    // Demonstrate black_box (stabilized in Rust 1.66)
    let preserved = std::hint::black_box(calibrated);
    println!("    black_box(calibrated) = {} (compiler cannot optimize this away)", preserved);
    println!();
}

/// Pitfall: Reading hardware registers requires volatile access.
///
/// Normal Rust reads can be elided by the compiler if it thinks
/// the value hasn't changed. Hardware registers change asynchronously,
/// so reads must be volatile.
fn pitfall_volatile_read() {
    println!("  Pitfall 2: Non-volatile reads of hardware registers");
    println!();

    // Simulate a hardware status register at a fixed memory address
    // In real embedded code: let sr = 0x4001_1000 as *const u32;
    let simulated_register: u32 = 0b0000_0000_0000_0000_0000_0000_0100_0000; // bit 6 = TXE

    // WRONG: compiler may cache or eliminate this read
    println!("    WRONG: let status = *status_register;");
    println!("           Compiler may optimize away repeated reads\n");

    // RIGHT: use volatile read
    //   let status = unsafe { core::ptr::read_volatile(status_register) };
    let status = std::hint::black_box(simulated_register); // stand-in for read_volatile
    let txe = (status >> 6) & 1;
    println!("    RIGHT: let status = read_volatile(status_register);");
    println!("           Status register = {:#010X}, TXE bit = {}", status, txe);
    println!();
    println!("    In practice, PAC crates handle volatile access for you:");
    println!("    `periph.USART1.sr.read().txe().bit_is_set()`");
    println!();
}

/// Pitfall: The 292-second overflow bug with u32 millisecond counters.
///
/// A 32-bit millisecond counter wraps at 2^32 ms = ~49.7 days,
/// but a 32-bit microsecond counter wraps at ~71.6 minutes.
/// Naive subtraction gives wrong results after wraparound.
fn pitfall_292_bug() {
    println!("  Pitfall 3: Timer overflow (the \"292 bug\")");
    println!();

    // u32 millisecond counter wrapping behavior
    let counter_max: u64 = u32::MAX as u64;
    let days = counter_max / 1000 / 60 / 60 / 24;
    println!("    u32 millisecond counter wraps at {:.1} days", days as f64 + (counter_max % (1000 * 60 * 60 * 24)) as f64 / (1000.0 * 60.0 * 60.0 * 24.0));

    // Demonstrate correct wrapping subtraction
    let start: u32 = u32::MAX - 100; // Near wraparound
    let end: u32 = start.wrapping_add(200); // Wrapped past MAX

    let elapsed_wrong = end as i64 - start as i64; // Negative! Bug!
    let elapsed_correct = end.wrapping_sub(start); // Correct: 200

    println!("    start = {}, end = {} (wrapped)", start, end);
    println!("    WRONG:   end - start = {} (negative/bogus!)", elapsed_wrong);
    println!("    CORRECT: end.wrapping_sub(start) = {}", elapsed_correct);
    println!();
    println!("    Always use wrapping_sub() for elapsed time calculations.");
    println!();
}

/// Pitfall: Semihosting stalls the CPU; RTT does not.
fn pitfall_semihosting_vs_rtt() {
    println!("  Pitfall 4: Semihosting vs RTT performance");
    println!();
    println!("    Method          Speed        CPU Impact    Needs Debugger");
    println!("    semihosting     ~100 chars/s halts CPU     yes");
    println!("    UART            115200 baud  minimal       no (hardware)");
    println!("    RTT             ~1 MB/s      none          yes");
    println!("    defmt + RTT     ~1 MB/s      minimal       yes");
    println!();
    println!("    Semihosting is useful for early bootstrap, but switch to");
    println!("    defmt + RTT for anything timing-sensitive.");
    println!();
    println!("  probe-rs workflow:");
    println!("    $ cargo install probe-rs-tools");
    println!("    $ probe-rs run --chip STM32F769NIHx target/thumbv7em-none-eabihf/debug/app");
    println!("    (automatically decodes defmt frames + shows RTT output)");
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Log level tests ---

    #[test]
    fn test_log_levels_are_ordered() {
        assert!(LogLevel::Trace < LogLevel::Debug);
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Info < LogLevel::Warn);
        assert!(LogLevel::Warn < LogLevel::Error);
    }

    #[test]
    fn test_logger_filters_below_min_level() {
        let mut logger = DefmtLogger::new(LogLevel::Warn);
        logger.log(LogLevel::Trace, "should be filtered");
        logger.log(LogLevel::Debug, "should be filtered");
        logger.log(LogLevel::Info, "should be filtered");
        logger.log(LogLevel::Warn, "should appear");
        logger.log(LogLevel::Error, "should appear");
        assert_eq!(logger.message_count, 2);
    }

    #[test]
    fn test_logger_at_trace_level_passes_everything() {
        let mut logger = DefmtLogger::new(LogLevel::Trace);
        logger.log(LogLevel::Trace, "t");
        logger.log(LogLevel::Debug, "d");
        logger.log(LogLevel::Info, "i");
        logger.log(LogLevel::Warn, "w");
        logger.log(LogLevel::Error, "e");
        assert_eq!(logger.message_count, 5);
    }

    #[test]
    fn test_logger_at_error_level_only_passes_error() {
        let mut logger = DefmtLogger::new(LogLevel::Error);
        logger.log(LogLevel::Trace, "t");
        logger.log(LogLevel::Warn, "w");
        logger.log(LogLevel::Error, "e");
        assert_eq!(logger.message_count, 1);
    }

    // --- RTT channel tests ---

    #[test]
    fn test_rtt_write_and_read() {
        let mut ch = RttChannel::new("test", 32);
        assert!(ch.is_empty());

        let written = ch.write(b"Hello");
        assert_eq!(written, 5);
        assert_eq!(ch.len(), 5);

        let mut buf = [0u8; 32];
        let read = ch.read(&mut buf);
        assert_eq!(read, 5);
        assert_eq!(&buf[..read], b"Hello");
        assert!(ch.is_empty());
    }

    #[test]
    fn test_rtt_channel_overflow() {
        let mut ch = RttChannel::new("small", 8);

        // Write more than capacity
        let written = ch.write(b"0123456789ABCDEF");
        assert_eq!(written, 8, "Should only write up to capacity");
        assert_eq!(ch.len(), 8);

        // Read everything
        let mut buf = [0u8; 16];
        let read = ch.read(&mut buf);
        assert_eq!(read, 8);
        assert_eq!(&buf[..read], b"01234567");
    }

    #[test]
    fn test_rtt_partial_read() {
        let mut ch = RttChannel::new("partial", 32);
        ch.write(b"Hello, RTT!");

        // Read only 5 bytes
        let mut buf = [0u8; 5];
        let read = ch.read(&mut buf);
        assert_eq!(read, 5);
        assert_eq!(&buf[..read], b"Hello");

        // Remaining bytes still in buffer
        assert_eq!(ch.len(), 6); // ", RTT!"
    }

    #[test]
    fn test_rtt_total_bytes_tracked() {
        let mut ch = RttChannel::new("tracked", 16);
        ch.write(b"AAAA");
        ch.write(b"BBBB");
        assert_eq!(ch.total_bytes_written, 8);

        // Read some, then write more
        let mut buf = [0u8; 4];
        ch.read(&mut buf);
        ch.write(b"CC");
        assert_eq!(ch.total_bytes_written, 10);
    }

    // --- Panic strategy tests ---

    #[test]
    fn test_panic_strategies_return_descriptions() {
        let halt = simulate_panic_handler(PanicStrategy::Halt, "test");
        assert_eq!(halt, "device halted in loop");

        let abort = simulate_panic_handler(PanicStrategy::Abort, "test");
        assert_eq!(abort, "device reset via HardFault");

        let probe = simulate_panic_handler(PanicStrategy::ProbeBreakpoint, "test");
        assert_eq!(probe, "debugger halted at breakpoint");
    }

    #[test]
    fn test_panic_strategy_display() {
        assert_eq!(format!("{}", PanicStrategy::Halt), "panic-halt (infinite loop)");
        assert_eq!(format!("{}", PanicStrategy::ProbeBreakpoint), "panic-probe (defmt log + BKPT)");
    }

    // --- Peripheral state tests ---

    #[test]
    fn test_peripheral_state_init() {
        let state = PeripheralState::typical_init();

        // GPIOJ pin 13 as output: MODER bits [27:26] = 0b01
        let pin13_mode = (state.gpio_mode >> 26) & 0b11;
        assert_eq!(pin13_mode, 0b01, "Pin 13 should be general-purpose output mode");

        // ODR bit 13 set (LED on)
        let pin13_odr = (state.gpio_output >> 13) & 1;
        assert_eq!(pin13_odr, 1, "Pin 13 ODR should be set (LED on)");

        assert_eq!(state.uart_baud, 115_200);
        assert_eq!(state.uart_status, UartStatus::Idle);
        assert_eq!(state.irq_priority, 4);
    }

    #[test]
    fn test_uart_error_variants() {
        let error_state = UartStatus::Error(UartError::Overrun);
        assert_ne!(error_state, UartStatus::Idle);
        assert_eq!(error_state, UartStatus::Error(UartError::Overrun));
        assert_ne!(error_state, UartStatus::Error(UartError::FramingError));
    }

    // --- Debug vs release tests ---

    #[test]
    fn test_fibonacci_both_versions_agree() {
        for n in 0..20 {
            assert_eq!(
                fibonacci_debug(n),
                fibonacci_release(n),
                "fibonacci({}) should be the same in debug and release",
                n
            );
        }
    }

    #[test]
    fn test_fibonacci_known_values() {
        // F(0)=0, F(1)=1, F(2)=1, F(3)=2, F(4)=3, F(5)=5, F(10)=55
        assert_eq!(fibonacci_debug(0), 0);
        assert_eq!(fibonacci_debug(1), 1);
        assert_eq!(fibonacci_debug(2), 1);
        assert_eq!(fibonacci_debug(5), 5);
        assert_eq!(fibonacci_debug(10), 55);
    }

    // --- Wrapping arithmetic (292 bug) tests ---

    #[test]
    fn test_wrapping_sub_near_overflow() {
        let start: u32 = u32::MAX - 100;
        let end: u32 = start.wrapping_add(200);

        // end has wrapped past u32::MAX
        assert!(end < start, "end should have wrapped around");

        // wrapping_sub gives the correct elapsed time
        let elapsed = end.wrapping_sub(start);
        assert_eq!(elapsed, 200);
    }

    #[test]
    fn test_wrapping_sub_normal_case() {
        let start: u32 = 1000;
        let end: u32 = 1500;
        let elapsed = end.wrapping_sub(start);
        assert_eq!(elapsed, 500);
    }

    #[test]
    fn test_wrapping_sub_at_exact_boundary() {
        let start: u32 = u32::MAX;
        let end: u32 = start.wrapping_add(1); // wraps to 0
        assert_eq!(end, 0);
        assert_eq!(end.wrapping_sub(start), 1);
    }

    // --- Volatile read concept test ---

    #[test]
    fn test_black_box_preserves_value() {
        let value: u32 = 42;
        let preserved = std::hint::black_box(value);
        assert_eq!(preserved, 42, "black_box must not change the value");
    }
}
