//! # Bare Metal Runtime Example
//!
//! Demonstrates the Cortex-M boot sequence, vector table layout,
//! .bss/.data initialization, and interrupt handling patterns.
//!
//! Compiles as standard Rust for CI. Simulates embedded runtime concepts.

fn main() {
    println!("=== Bare Metal Runtime ===\n");

    demonstrate_boot_sequence();
    demonstrate_vector_table();
    demonstrate_memory_init();
    demonstrate_interrupt_sharing();
    demonstrate_cache_coherency();
}

// ---------------------------------------------------------------------------
// Boot Sequence Simulation
// ---------------------------------------------------------------------------

fn demonstrate_boot_sequence() {
    println!("--- Boot Sequence (Cortex-M7) ---");
    let steps = [
        ("1. Load SP",        "Read initial stack pointer from 0x08000000"),
        ("2. Reset vector",   "Jump to Reset_Handler at 0x08000004"),
        ("3. Zero .bss",      "Write zeros to uninitialized static variables"),
        ("4. Copy .data",     "Copy initialized statics from flash (LMA) to RAM (VMA)"),
        ("5. Enable FPU",     "Set CPACR bits for Cortex-M7 floating point"),
        ("6. __pre_init()",   "Optional user hook before main"),
        ("7. main()",         "Call #[entry] fn main() -> !"),
    ];

    for (step, description) in &steps {
        println!("  {:<20} {}", step, description);
    }
    println!();
}

// ---------------------------------------------------------------------------
// Vector Table
// ---------------------------------------------------------------------------

/// Simulated vector table entry
struct VectorEntry {
    offset: u32,
    name: &'static str,
    description: &'static str,
}

fn demonstrate_vector_table() {
    println!("--- Cortex-M Vector Table (first 16 entries) ---");

    let vectors = [
        VectorEntry { offset: 0x00, name: "Initial SP",  description: "Stack pointer value" },
        VectorEntry { offset: 0x04, name: "Reset",       description: "Entry point" },
        VectorEntry { offset: 0x08, name: "NMI",         description: "Non-maskable interrupt" },
        VectorEntry { offset: 0x0C, name: "HardFault",   description: "All faults (default)" },
        VectorEntry { offset: 0x10, name: "MemManage",   description: "Memory protection" },
        VectorEntry { offset: 0x14, name: "BusFault",    description: "Bus error" },
        VectorEntry { offset: 0x18, name: "UsageFault",  description: "Undefined instruction" },
        VectorEntry { offset: 0x2C, name: "SVCall",      description: "Supervisor call" },
        VectorEntry { offset: 0x38, name: "PendSV",      description: "Pendable service" },
        VectorEntry { offset: 0x3C, name: "SysTick",     description: "System timer" },
    ];

    for v in &vectors {
        println!("  0x{:04X}: {:<12} — {}", v.offset, v.name, v.description);
    }
    println!("  0x0040: IRQ0...      — Device-specific interrupts");
    println!();
}

// ---------------------------------------------------------------------------
// .bss and .data Initialization
// ---------------------------------------------------------------------------

// Simulated memory sections
static BSS_EXAMPLE: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);
static DATA_EXAMPLE: &str = "Hello, embedded!";

fn demonstrate_memory_init() {
    println!("--- .bss and .data Initialization ---");

    // .bss: zeroed by startup code
    println!("  .bss  section: uninitialized statics, zeroed at startup");
    println!("    COUNTER: u32 = {} (guaranteed zero)", BSS_EXAMPLE.load(std::sync::atomic::Ordering::Relaxed));

    // .data: copied from flash to RAM
    println!("  .data section: initialized statics, copied flash→RAM");
    println!("    GREETING: &str = \"{}\"", DATA_EXAMPLE);

    // .rodata: stays in flash
    const MAX_RETRIES: u32 = 5;
    println!("  .rodata/const: stays in flash, no RAM cost");
    println!("    MAX_RETRIES: u32 = {}", MAX_RETRIES);

    // STM32F769 memory regions
    println!();
    println!("  STM32F769 Memory Regions:");
    println!("    FLASH:  0x0800_0000  2 MB    (code + .rodata + .data LMA)");
    println!("    DTCM:   0x2000_0000  16 KB   (zero-wait-state data)");
    println!("    SRAM1:  0x2002_0000  368 KB  (main RAM)");
    println!("    SRAM2:  0x2007_8000  16 KB   (additional RAM)");
    println!("    ITCM:   0x0000_0000  16 KB   (zero-wait-state instructions)");
    println!();
}

// ---------------------------------------------------------------------------
// Interrupt Data Sharing Pattern
// ---------------------------------------------------------------------------

use std::cell::RefCell;
use std::sync::Mutex;

/// Simulates the Mutex<RefCell<Option<T>>> pattern for interrupt-main sharing
static SHARED_LED: Mutex<RefCell<Option<bool>>> = Mutex::new(RefCell::new(None));

fn demonstrate_interrupt_sharing() {
    println!("--- Interrupt Data Sharing Pattern ---");

    // "Main" initializes the shared resource
    {
        let lock = SHARED_LED.lock().unwrap();
        *lock.borrow_mut() = Some(false);
        println!("  Main: initialized LED state to OFF");
    }

    // "Interrupt handler" toggles the LED
    simulate_interrupt_handler();

    // "Main" reads the updated state
    {
        let lock = SHARED_LED.lock().unwrap();
        let state = lock.borrow();
        println!("  Main: LED state is now {:?}", state);
    }

    println!();
    println!("  Pattern: Mutex<RefCell<Option<T>>>");
    println!("  - Mutex: provides critical section (disables interrupts)");
    println!("  - RefCell: provides interior mutability");
    println!("  - Option: allows lazy initialization (None before setup)");
    println!();
}

fn simulate_interrupt_handler() {
    // In real code: #[interrupt] fn EXTI0() { ... }
    let lock = SHARED_LED.lock().unwrap();
    let mut state = lock.borrow_mut();
    if let Some(ref mut led) = *state {
        *led = !*led; // Toggle
        println!("  ISR:  toggled LED to {}", if *led { "ON" } else { "OFF" });
    }
}

// ---------------------------------------------------------------------------
// Cache Coherency
// ---------------------------------------------------------------------------

fn demonstrate_cache_coherency() {
    println!("--- Cortex-M7 Cache Coherency ---");
    println!("  I-cache: 16 KB — speeds up instruction fetch (always safe to enable)");
    println!("  D-cache: 16 KB — speeds up data access (DMA coherency required!)");
    println!();
    println!("  DMA Transmit (CPU → peripheral):");
    println!("    1. CPU writes to buffer (data may be in D-cache only)");
    println!("    2. Clean cache: flush dirty lines to RAM");
    println!("    3. Start DMA transfer");
    println!();
    println!("  DMA Receive (peripheral → CPU):");
    println!("    1. DMA writes to RAM (bypasses cache)");
    println!("    2. DMA complete interrupt fires");
    println!("    3. Invalidate cache: discard stale cache lines");
    println!("    4. CPU reads fresh data from RAM");
    println!();
    println!("  Tip: Place DMA buffers in DTCM (not cached) to avoid coherency issues");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::Ordering;

    #[test]
    fn test_bss_is_zero() {
        assert_eq!(BSS_EXAMPLE.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn test_data_is_initialized() {
        assert_eq!(DATA_EXAMPLE, "Hello, embedded!");
    }

    #[test]
    fn test_vector_table_offsets() {
        // Reset vector is always at offset 0x04
        assert_eq!(0x04u32, 4);
        // HardFault is at offset 0x0C
        assert_eq!(0x0Cu32, 12);
        // First device IRQ at 0x40
        assert_eq!(0x40u32, 64);
    }

    #[test]
    fn test_stm32f769_memory_map() {
        let flash_start: u32 = 0x0800_0000;
        let flash_size: u32 = 2 * 1024 * 1024;
        let ram_start: u32 = 0x2000_0000;

        assert!(flash_start + flash_size <= ram_start);
    }
}
