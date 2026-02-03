//! Device Drivers Example
//!
//! Demonstrates device driver patterns for embedded systems.
//!
//! # Driver Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────────┐
//!     │                   Driver Stack                              │
//!     ├─────────────────────────────────────────────────────────────┤
//!     │                                                             │
//!     │  ┌─────────────────────────────────────────────────────┐    │
//!     │  │           Application Layer                          │    │
//!     │  │     (High-level API, safe Rust)                      │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │           Driver Layer                               │    │
//!     │  │   (State machine, buffer management)                 │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │         Register Access Layer                        │    │
//!     │  │   (Volatile reads/writes, bitfields)                 │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │            Hardware (MMIO)                           │    │
//!     │  └─────────────────────────────────────────────────────┘    │
//!     │                                                             │
//!     └─────────────────────────────────────────────────────────────┘
//! ```

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicU32, Ordering};

fn main() {
    println!("=== Device Driver Concepts ===\n");

    println!("--- Volatile Access ---");
    volatile_access();

    println!("\n--- Register Definitions ---");
    register_definitions();

    println!("\n--- MMIO Patterns ---");
    mmio_patterns();

    println!("\n--- Interrupt Handling ---");
    interrupt_handling();

    println!("\n--- DMA Concepts ---");
    dma_concepts();

    println!("\n--- Complete UART Driver ---");
    uart_driver_example();
}

// ============================================
// Volatile Access
// ============================================

/// Volatile cell for hardware register access
/// Prevents compiler optimizations that could skip reads/writes
#[repr(transparent)]
struct VolatileCell<T: Copy> {
    value: UnsafeCell<T>,
}

impl<T: Copy> VolatileCell<T> {
    const fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    #[inline]
    fn read(&self) -> T {
        // SAFETY: Volatile read prevents optimization
        unsafe { std::ptr::read_volatile(self.value.get()) }
    }

    #[inline]
    fn write(&self, value: T) {
        // SAFETY: Volatile write ensures value is written
        unsafe { std::ptr::write_volatile(self.value.get(), value) }
    }
}

// VolatileCell is safe to share because hardware registers
// need to be accessed from multiple contexts
unsafe impl<T: Copy> Sync for VolatileCell<T> {}

fn volatile_access() {
    let reg = VolatileCell::new(0u32);

    println!("  Volatile register operations:");
    println!("    Initial value: 0x{:08X}", reg.read());

    reg.write(0xDEADBEEF);
    println!("    After write: 0x{:08X}", reg.read());

    // Multiple reads always hit the "hardware"
    let val1 = reg.read();
    let val2 = reg.read();
    println!("    Two reads: 0x{:08X}, 0x{:08X}", val1, val2);

    println!("\n  Why volatile matters:");
    println!("    - Compiler can't optimize away reads");
    println!("    - Compiler can't reorder accesses");
    println!("    - Every read/write actually happens");
}

// ============================================
// Register Definitions
// ============================================

/// Read-only register
#[repr(transparent)]
struct ReadOnly<T: Copy>(VolatileCell<T>);

impl<T: Copy> ReadOnly<T> {
    fn read(&self) -> T {
        self.0.read()
    }
}

/// Write-only register
#[repr(transparent)]
struct WriteOnly<T: Copy>(VolatileCell<T>);

impl<T: Copy> WriteOnly<T> {
    fn write(&self, value: T) {
        self.0.write(value);
    }
}

/// Read-write register
#[repr(transparent)]
struct ReadWrite<T: Copy>(VolatileCell<T>);

impl<T: Copy> ReadWrite<T> {
    fn read(&self) -> T {
        self.0.read()
    }

    fn write(&self, value: T) {
        self.0.write(value);
    }

    fn modify<F>(&self, f: F)
    where
        F: FnOnce(T) -> T,
    {
        let value = self.read();
        self.write(f(value));
    }
}

/// Bitfield helper for registers
struct Bitfield {
    offset: u32,
    width: u32,
}

impl Bitfield {
    const fn new(offset: u32, width: u32) -> Self {
        Bitfield { offset, width }
    }

    fn mask(&self) -> u32 {
        ((1u32 << self.width) - 1) << self.offset
    }

    fn read(&self, value: u32) -> u32 {
        (value & self.mask()) >> self.offset
    }

    fn write(&self, reg_value: u32, field_value: u32) -> u32 {
        (reg_value & !self.mask()) | ((field_value << self.offset) & self.mask())
    }
}

fn register_definitions() {
    // Simulated control register
    let ctrl_reg = ReadWrite(VolatileCell::new(0u32));

    // Define bitfields
    let enable = Bitfield::new(0, 1);
    let mode = Bitfield::new(1, 3);
    let prescaler = Bitfield::new(4, 4);
    let interrupt_enable = Bitfield::new(8, 1);

    println!("  Register bitfields:");
    println!("    ENABLE: bit 0");
    println!("    MODE: bits 1-3");
    println!("    PRESCALER: bits 4-7");
    println!("    INT_EN: bit 8");

    // Read-modify-write pattern
    println!("\n  Configuring register:");
    let mut value = ctrl_reg.read();
    value = enable.write(value, 1);
    value = mode.write(value, 0b101);
    value = prescaler.write(value, 8);
    value = interrupt_enable.write(value, 1);
    ctrl_reg.write(value);

    println!("    Written: 0x{:08X}", ctrl_reg.read());
    println!(
        "    ENABLE={}, MODE={}, PRESCALER={}, INT_EN={}",
        enable.read(ctrl_reg.read()),
        mode.read(ctrl_reg.read()),
        prescaler.read(ctrl_reg.read()),
        interrupt_enable.read(ctrl_reg.read())
    );

    // Using modify helper
    println!("\n  Using modify helper:");
    ctrl_reg.modify(|v| enable.write(v, 0));
    println!(
        "    After disabling: ENABLE={}",
        enable.read(ctrl_reg.read())
    );
}

// ============================================
// MMIO Patterns
// ============================================

/// Simulated peripheral register block
#[repr(C)]
struct TimerRegisters {
    control: ReadWrite<u32>,
    status: ReadOnly<u32>,
    counter: ReadOnly<u32>,
    compare: ReadWrite<u32>,
    prescaler: ReadWrite<u32>,
}

impl TimerRegisters {
    // Control register bits
    const CTRL_ENABLE: u32 = 1 << 0;
    const CTRL_INT_ENABLE: u32 = 1 << 1;
    const CTRL_ONE_SHOT: u32 = 1 << 2;

    // Status register bits
    const STATUS_RUNNING: u32 = 1 << 0;
    const STATUS_OVERFLOW: u32 = 1 << 1;
    const STATUS_COMPARE_MATCH: u32 = 1 << 2;
}

/// Timer driver using MMIO
struct Timer {
    regs: &'static TimerRegisters,
}

impl Timer {
    /// Create timer from base address
    /// SAFETY: Address must point to valid timer registers
    unsafe fn from_base_addr(base: usize) -> Self {
        Timer {
            regs: &*(base as *const TimerRegisters),
        }
    }

    fn enable(&self) {
        self.regs
            .control
            .modify(|v| v | TimerRegisters::CTRL_ENABLE);
    }

    fn disable(&self) {
        self.regs
            .control
            .modify(|v| v & !TimerRegisters::CTRL_ENABLE);
    }

    fn set_compare(&self, value: u32) {
        self.regs.compare.write(value);
    }

    fn set_prescaler(&self, value: u32) {
        self.regs.prescaler.write(value);
    }

    fn is_running(&self) -> bool {
        (self.regs.status.read() & TimerRegisters::STATUS_RUNNING) != 0
    }

    fn counter(&self) -> u32 {
        self.regs.counter.read()
    }

    fn has_overflowed(&self) -> bool {
        (self.regs.status.read() & TimerRegisters::STATUS_OVERFLOW) != 0
    }
}

fn mmio_patterns() {
    // Create simulated registers in memory
    static TIMER_REGS: TimerRegisters = TimerRegisters {
        control: ReadWrite(VolatileCell::new(0)),
        status: ReadOnly(VolatileCell::new(TimerRegisters::STATUS_RUNNING)),
        counter: ReadOnly(VolatileCell::new(12345)),
        compare: ReadWrite(VolatileCell::new(0)),
        prescaler: ReadWrite(VolatileCell::new(1)),
    };

    println!("  Timer register block layout:");
    println!("    Offset 0x00: CONTROL");
    println!("    Offset 0x04: STATUS (RO)");
    println!("    Offset 0x08: COUNTER (RO)");
    println!("    Offset 0x0C: COMPARE");
    println!("    Offset 0x10: PRESCALER");

    // Get timer instance
    let timer = unsafe { Timer::from_base_addr(&TIMER_REGS as *const _ as usize) };

    println!("\n  Timer operations:");
    println!("    Counter: {}", timer.counter());
    println!("    Running: {}", timer.is_running());

    timer.set_prescaler(8);
    timer.set_compare(1000);
    timer.enable();

    println!("    After config:");
    println!("      Control: 0x{:08X}", TIMER_REGS.control.read());
    println!("      Compare: {}", TIMER_REGS.compare.read());
    println!("      Prescaler: {}", TIMER_REGS.prescaler.read());
}

// ============================================
// Interrupt Handling
// ============================================

/// Interrupt handler type
type InterruptHandler = fn();

/// Simulated interrupt controller
struct InterruptController {
    enabled: AtomicU32,
    pending: AtomicU32,
    handlers: [Option<InterruptHandler>; 32],
}

impl InterruptController {
    const fn new() -> Self {
        InterruptController {
            enabled: AtomicU32::new(0),
            pending: AtomicU32::new(0),
            handlers: [None; 32],
        }
    }

    fn enable_irq(&self, irq: u32) {
        self.enabled.fetch_or(1 << irq, Ordering::SeqCst);
    }

    fn disable_irq(&self, irq: u32) {
        self.enabled.fetch_and(!(1 << irq), Ordering::SeqCst);
    }

    fn is_enabled(&self, irq: u32) -> bool {
        (self.enabled.load(Ordering::SeqCst) & (1 << irq)) != 0
    }

    fn set_pending(&self, irq: u32) {
        self.pending.fetch_or(1 << irq, Ordering::SeqCst);
    }

    fn clear_pending(&self, irq: u32) {
        self.pending.fetch_and(!(1 << irq), Ordering::SeqCst);
    }

    fn is_pending(&self, irq: u32) -> bool {
        (self.pending.load(Ordering::SeqCst) & (1 << irq)) != 0
    }

    fn register_handler(&mut self, irq: u32, handler: InterruptHandler) {
        if irq < 32 {
            self.handlers[irq as usize] = Some(handler);
        }
    }

    fn handle_irq(&self, irq: u32) {
        if irq < 32 {
            if let Some(handler) = self.handlers[irq as usize] {
                handler();
            }
            self.clear_pending(irq);
        }
    }
}

/// Critical section guard
struct CriticalSection;

impl CriticalSection {
    fn enter() -> Self {
        println!("      [IRQ disabled]");
        CriticalSection
    }
}

impl Drop for CriticalSection {
    fn drop(&mut self) {
        println!("      [IRQ enabled]");
    }
}

/// Execute closure in critical section
fn critical_section<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    let _cs = CriticalSection::enter();
    f()
}

fn interrupt_handling() {
    static mut INTC: InterruptController = InterruptController::new();

    // IRQ numbers
    const TIMER_IRQ: u32 = 5;
    const UART_IRQ: u32 = 12;

    fn timer_handler() {
        println!("      Timer interrupt handled!");
    }

    fn uart_handler() {
        println!("      UART interrupt handled!");
    }

    unsafe {
        INTC.register_handler(TIMER_IRQ, timer_handler);
        INTC.register_handler(UART_IRQ, uart_handler);
    }

    println!("  Interrupt controller:");

    unsafe {
        INTC.enable_irq(TIMER_IRQ);
        INTC.enable_irq(UART_IRQ);
    }

    println!("    Timer IRQ {}: enabled", TIMER_IRQ);
    println!("    UART IRQ {}: enabled", UART_IRQ);

    // Simulate interrupt
    println!("\n  Simulating timer interrupt:");
    unsafe {
        INTC.set_pending(TIMER_IRQ);
        if INTC.is_pending(TIMER_IRQ) && INTC.is_enabled(TIMER_IRQ) {
            INTC.handle_irq(TIMER_IRQ);
        }
    }

    // Critical section example
    println!("\n  Critical section:");
    critical_section(|| {
        println!("      Accessing shared resource safely");
    });
}

// ============================================
// DMA Concepts
// ============================================

/// DMA transfer direction
#[derive(Debug, Clone, Copy)]
enum DmaDirection {
    MemoryToPeripheral,
    PeripheralToMemory,
    MemoryToMemory,
}

/// DMA transfer status
#[derive(Debug, Clone, Copy, PartialEq)]
enum DmaStatus {
    Idle,
    Running,
    Complete,
    Error,
}

/// DMA channel configuration
struct DmaChannel {
    source: usize,
    dest: usize,
    count: usize,
    direction: DmaDirection,
    status: DmaStatus,
}

impl DmaChannel {
    fn new() -> Self {
        DmaChannel {
            source: 0,
            dest: 0,
            count: 0,
            direction: DmaDirection::MemoryToMemory,
            status: DmaStatus::Idle,
        }
    }

    fn configure(&mut self, source: usize, dest: usize, count: usize, direction: DmaDirection) {
        self.source = source;
        self.dest = dest;
        self.count = count;
        self.direction = direction;
        self.status = DmaStatus::Idle;
    }

    fn start(&mut self) {
        println!(
            "    DMA start: 0x{:08X} -> 0x{:08X}, {} bytes, {:?}",
            self.source, self.dest, self.count, self.direction
        );
        self.status = DmaStatus::Running;
    }

    fn is_complete(&self) -> bool {
        self.status == DmaStatus::Complete
    }

    // Simulate completion
    fn simulate_complete(&mut self) {
        self.status = DmaStatus::Complete;
    }
}

/// DMA-safe buffer
#[repr(C, align(4))]
struct DmaBuffer<const N: usize> {
    data: [u8; N],
}

impl<const N: usize> DmaBuffer<N> {
    const fn new() -> Self {
        DmaBuffer { data: [0; N] }
    }

    fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
}

fn dma_concepts() {
    let mut dma = DmaChannel::new();

    // Source and destination buffers
    static mut SRC_BUF: DmaBuffer<256> = DmaBuffer::new();
    static mut DST_BUF: DmaBuffer<256> = DmaBuffer::new();

    println!("  DMA transfer setup:");

    unsafe {
        // Fill source buffer
        for (i, byte) in SRC_BUF.data.iter_mut().enumerate() {
            *byte = i as u8;
        }

        dma.configure(
            SRC_BUF.as_ptr() as usize,
            DST_BUF.as_mut_ptr() as usize,
            256,
            DmaDirection::MemoryToMemory,
        );
    }

    dma.start();

    // Wait for completion (simulated)
    dma.simulate_complete();

    println!("    Transfer complete: {}", dma.is_complete());

    println!("\n  DMA buffer requirements:");
    println!("    - Aligned to DMA word size (often 4 bytes)");
    println!("    - Cache-coherent or manually managed");
    println!("    - Not moved while DMA active");
    println!("    - Proper lifetime management");
}

// ============================================
// Complete UART Driver
// ============================================

/// UART register block
#[repr(C)]
struct UartRegisters {
    data: ReadWrite<u32>,
    status: ReadOnly<u32>,
    control: ReadWrite<u32>,
    baud: ReadWrite<u32>,
}

impl UartRegisters {
    // Status bits
    const STATUS_TX_EMPTY: u32 = 1 << 0;
    const STATUS_RX_READY: u32 = 1 << 1;
    const STATUS_TX_FULL: u32 = 1 << 2;
    const STATUS_RX_OVERRUN: u32 = 1 << 3;

    // Control bits
    const CTRL_TX_ENABLE: u32 = 1 << 0;
    const CTRL_RX_ENABLE: u32 = 1 << 1;
    const CTRL_TX_INT: u32 = 1 << 2;
    const CTRL_RX_INT: u32 = 1 << 3;
}

/// UART driver state
#[derive(Debug, Clone, Copy, PartialEq)]
enum UartState {
    Uninitialized,
    Ready,
    Transmitting,
    Receiving,
    Error,
}

/// Complete UART driver
struct UartDriver {
    regs: &'static UartRegisters,
    state: UartState,
    tx_buffer: [u8; 64],
    tx_head: usize,
    tx_tail: usize,
    rx_buffer: [u8; 64],
    rx_head: usize,
    rx_tail: usize,
}

impl UartDriver {
    unsafe fn new(base: usize) -> Self {
        UartDriver {
            regs: &*(base as *const UartRegisters),
            state: UartState::Uninitialized,
            tx_buffer: [0; 64],
            tx_head: 0,
            tx_tail: 0,
            rx_buffer: [0; 64],
            rx_head: 0,
            rx_tail: 0,
        }
    }

    fn init(&mut self, baud_rate: u32, clock_freq: u32) {
        // Calculate baud divisor
        let divisor = clock_freq / (16 * baud_rate);
        self.regs.baud.write(divisor);

        // Enable TX and RX
        self.regs.control.write(
            UartRegisters::CTRL_TX_ENABLE
                | UartRegisters::CTRL_RX_ENABLE
                | UartRegisters::CTRL_RX_INT,
        );

        self.state = UartState::Ready;
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), &'static str> {
        if self.state != UartState::Ready && self.state != UartState::Transmitting {
            return Err("UART not ready");
        }

        // Wait for TX empty
        while (self.regs.status.read() & UartRegisters::STATUS_TX_FULL) != 0 {
            core::hint::spin_loop();
        }

        self.regs.data.write(byte as u32);
        self.state = UartState::Transmitting;
        Ok(())
    }

    fn write_bytes(&mut self, data: &[u8]) -> Result<usize, &'static str> {
        for &byte in data {
            self.write_byte(byte)?;
        }
        self.state = UartState::Ready;
        Ok(data.len())
    }

    fn read_byte(&mut self) -> Option<u8> {
        if (self.regs.status.read() & UartRegisters::STATUS_RX_READY) != 0 {
            let byte = (self.regs.data.read() & 0xFF) as u8;
            Some(byte)
        } else {
            None
        }
    }

    fn is_tx_empty(&self) -> bool {
        (self.regs.status.read() & UartRegisters::STATUS_TX_EMPTY) != 0
    }

    fn has_rx_data(&self) -> bool {
        (self.regs.status.read() & UartRegisters::STATUS_RX_READY) != 0
    }
}

fn uart_driver_example() {
    // Simulated UART registers
    static UART_REGS: UartRegisters = UartRegisters {
        data: ReadWrite(VolatileCell::new(0)),
        status: ReadOnly(VolatileCell::new(UartRegisters::STATUS_TX_EMPTY)),
        control: ReadWrite(VolatileCell::new(0)),
        baud: ReadWrite(VolatileCell::new(0)),
    };

    let mut uart = unsafe { UartDriver::new(&UART_REGS as *const _ as usize) };

    println!("  UART driver initialization:");
    uart.init(115200, 48_000_000);
    println!("    Baud rate: 115200");
    println!("    Baud divisor: {}", UART_REGS.baud.read());
    println!("    Control: 0x{:08X}", UART_REGS.control.read());

    println!("\n  UART operations:");
    println!("    State: {:?}", uart.state);
    println!("    TX empty: {}", uart.is_tx_empty());

    match uart.write_bytes(b"Hello, UART!") {
        Ok(n) => println!("    Wrote {} bytes", n),
        Err(e) => println!("    Write error: {}", e),
    }

    println!("\n  Driver design principles:");
    println!("    - Encapsulate unsafe hardware access");
    println!("    - Expose safe high-level API");
    println!("    - Manage state transitions");
    println!("    - Handle errors gracefully");
    println!("    - Support interrupt-driven operation");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volatile_cell() {
        let cell = VolatileCell::new(42u32);
        assert_eq!(cell.read(), 42);
        cell.write(100);
        assert_eq!(cell.read(), 100);
    }

    #[test]
    fn test_bitfield() {
        let field = Bitfield::new(4, 4);
        let value = field.write(0, 0xF);
        assert_eq!(value, 0xF0);
        assert_eq!(field.read(value), 0xF);
    }

    #[test]
    fn test_dma_status() {
        let mut dma = DmaChannel::new();
        assert_eq!(dma.status, DmaStatus::Idle);
        dma.start();
        assert_eq!(dma.status, DmaStatus::Running);
        dma.simulate_complete();
        assert!(dma.is_complete());
    }
}
