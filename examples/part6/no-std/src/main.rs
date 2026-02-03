//! No-std Programming Concepts
//!
//! Demonstrates concepts used in no_std Rust programming.
//!
//! # no_std Environment
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                Standard Library Layers                  │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  std   ─── Full standard library                        │
//!     │         │  - OS abstractions (files, threads, etc.)     │
//!     │         │  - Heap allocation by default                 │
//!     │         │  - Panics can unwind                          │
//!     │         ▼                                               │
//!     │  alloc ─── Heap allocation types                        │
//!     │         │  - Vec, String, Box, Rc, Arc                  │
//!     │         │  - Requires global allocator                  │
//!     │         ▼                                               │
//!     │  core  ─── Fundamental types (always available)         │
//!     │            - Option, Result, iterators                  │
//!     │            - Primitive types, traits                    │
//!     │            - No heap, no OS dependencies                │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example demonstrates no_std concepts while still
//! being compilable in a std environment for learning purposes.

// In a real no_std crate, you would have:
// #![no_std]
// extern crate alloc; // If using alloc

use std::collections::BTreeMap; // In no_std: use alloc::collections::BTreeMap

fn main() {
    println!("=== No-std Programming Concepts ===\n");

    println!("--- Core Library Types ---");
    core_types();

    println!("\n--- Fixed-Size Collections ---");
    fixed_collections();

    println!("\n--- Static Allocation ---");
    static_allocation();

    println!("\n--- Heapless Patterns ---");
    heapless_patterns();

    println!("\n--- Error Handling without Panic ---");
    error_handling();

    println!("\n--- Bit Manipulation ---");
    bit_manipulation();
}

// ============================================
// Core Library Types
// ============================================

/// Types available in core (no heap required)
fn core_types() {
    // Option and Result - always available
    let some_value: Option<i32> = Some(42);
    let none_value: Option<i32> = None;

    println!("  Option: {:?}, {:?}", some_value, none_value);

    let ok_result: Result<i32, &str> = Ok(100);
    let err_result: Result<i32, &str> = Err("error");

    println!("  Result: {:?}, {:?}", ok_result, err_result);

    // Core iterators work without heap
    let arr = [1, 2, 3, 4, 5];
    let sum: i32 = arr.iter().sum();
    println!("  Array sum (iterator): {}", sum);

    // Slices - reference to contiguous memory
    let slice: &[i32] = &arr[1..4];
    println!("  Slice: {:?}", slice);

    // Core traits: Copy, Clone, Default, etc.
    #[derive(Debug, Clone, Copy, Default)]
    struct Point {
        x: i32,
        y: i32,
    }

    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // Copy, not move
    println!("  Point (Copy): {:?}, {:?}", p1, p2);
}

// ============================================
// Fixed-Size Collections
// ============================================

/// Collections that don't require heap allocation
fn fixed_collections() {
    // Fixed-size arrays
    let mut buffer: [u8; 256] = [0; 256];
    buffer[0] = 1;
    buffer[1] = 2;
    println!("  Fixed buffer: [{}, {}, ...]", buffer[0], buffer[1]);

    // Array-based stack
    struct FixedStack<T, const N: usize> {
        data: [Option<T>; N],
        len: usize,
    }

    impl<T: Copy + Default, const N: usize> FixedStack<T, N> {
        fn new() -> Self {
            FixedStack {
                data: [None; N],
                len: 0,
            }
        }

        fn push(&mut self, value: T) -> Result<(), &'static str> {
            if self.len >= N {
                return Err("Stack full");
            }
            self.data[self.len] = Some(value);
            self.len += 1;
            Ok(())
        }

        fn pop(&mut self) -> Option<T> {
            if self.len == 0 {
                return None;
            }
            self.len -= 1;
            self.data[self.len].take()
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    let mut stack: FixedStack<i32, 10> = FixedStack::new();
    stack.push(1).unwrap();
    stack.push(2).unwrap();
    stack.push(3).unwrap();

    println!("  Fixed stack len: {}", stack.len());
    println!("  Pop: {:?}", stack.pop());
    println!("  Pop: {:?}", stack.pop());
}

// ============================================
// Static Allocation
// ============================================

/// Using static memory instead of heap
fn static_allocation() {
    // Static mutable data (requires unsafe or sync primitives)
    static COUNTER: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

    COUNTER.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    println!(
        "  Static counter: {}",
        COUNTER.load(std::sync::atomic::Ordering::SeqCst)
    );

    // Static buffers
    static mut BUFFER: [u8; 1024] = [0; 1024];

    // Safe wrapper for static buffer access
    fn get_buffer_slice(start: usize, len: usize) -> &'static [u8] {
        unsafe { &BUFFER[start..start + len] }
    }

    let slice = get_buffer_slice(0, 10);
    println!("  Static buffer slice len: {}", slice.len());

    // Compile-time computed values
    const LOOKUP_TABLE: [u32; 16] = {
        let mut table = [0u32; 16];
        let mut i = 0;
        while i < 16 {
            table[i] = i as u32 * i as u32;
            i += 1;
        }
        table
    };

    println!("  Const lookup table[5]: {}", LOOKUP_TABLE[5]);
}

// ============================================
// Heapless Patterns
// ============================================

/// Patterns used in no_std code
fn heapless_patterns() {
    // Ring buffer (circular buffer)
    struct RingBuffer<T, const N: usize> {
        data: [Option<T>; N],
        read_idx: usize,
        write_idx: usize,
        count: usize,
    }

    impl<T: Copy + Default, const N: usize> RingBuffer<T, N> {
        fn new() -> Self {
            RingBuffer {
                data: [None; N],
                read_idx: 0,
                write_idx: 0,
                count: 0,
            }
        }

        fn push(&mut self, value: T) -> Result<(), T> {
            if self.count >= N {
                return Err(value);
            }
            self.data[self.write_idx] = Some(value);
            self.write_idx = (self.write_idx + 1) % N;
            self.count += 1;
            Ok(())
        }

        fn pop(&mut self) -> Option<T> {
            if self.count == 0 {
                return None;
            }
            let value = self.data[self.read_idx].take();
            self.read_idx = (self.read_idx + 1) % N;
            self.count -= 1;
            value
        }
    }

    let mut ring: RingBuffer<u8, 4> = RingBuffer::new();
    ring.push(1).unwrap();
    ring.push(2).unwrap();
    ring.push(3).unwrap();
    println!("  Ring buffer: {:?}, {:?}", ring.pop(), ring.pop());

    // Fixed-capacity string
    struct FixedString<const N: usize> {
        data: [u8; N],
        len: usize,
    }

    impl<const N: usize> FixedString<N> {
        fn new() -> Self {
            FixedString {
                data: [0; N],
                len: 0,
            }
        }

        fn push_str(&mut self, s: &str) -> Result<(), ()> {
            let bytes = s.as_bytes();
            if self.len + bytes.len() > N {
                return Err(());
            }
            self.data[self.len..self.len + bytes.len()].copy_from_slice(bytes);
            self.len += bytes.len();
            Ok(())
        }

        fn as_str(&self) -> &str {
            std::str::from_utf8(&self.data[..self.len]).unwrap_or("")
        }
    }

    let mut s: FixedString<32> = FixedString::new();
    s.push_str("Hello, ").unwrap();
    s.push_str("World!").unwrap();
    println!("  Fixed string: \"{}\"", s.as_str());
}

// ============================================
// Error Handling without Panic
// ============================================

/// Error handling strategies for no_std
fn error_handling() {
    // Return codes instead of panics
    #[derive(Debug, Clone, Copy)]
    enum Error {
        InvalidInput,
        BufferFull,
        NotFound,
        Timeout,
    }

    fn safe_divide(a: i32, b: i32) -> Result<i32, Error> {
        if b == 0 {
            Err(Error::InvalidInput)
        } else {
            Ok(a / b)
        }
    }

    match safe_divide(10, 0) {
        Ok(result) => println!("  Result: {}", result),
        Err(e) => println!("  Error: {:?}", e),
    }

    // Option for nullable values
    fn find_in_array(arr: &[i32], target: i32) -> Option<usize> {
        arr.iter().position(|&x| x == target)
    }

    let arr = [1, 2, 3, 4, 5];
    println!("  Find 3: {:?}", find_in_array(&arr, 3));
    println!("  Find 10: {:?}", find_in_array(&arr, 10));

    // Never panic - use checked operations
    let x: u8 = 200;
    let y: u8 = 100;

    // This would panic: let sum = x + y;
    let sum = x.checked_add(y);
    println!("  Checked add 200 + 100 (u8): {:?}", sum);

    let sum_saturating = x.saturating_add(y);
    println!("  Saturating add: {}", sum_saturating);

    let sum_wrapping = x.wrapping_add(y);
    println!("  Wrapping add: {}", sum_wrapping);
}

// ============================================
// Bit Manipulation
// ============================================

/// Common in embedded/systems programming
fn bit_manipulation() {
    // Setting and clearing bits
    let mut flags: u8 = 0b0000_0000;

    // Set bit 0
    flags |= 1 << 0;
    println!("  After set bit 0: {:08b}", flags);

    // Set bit 3
    flags |= 1 << 3;
    println!("  After set bit 3: {:08b}", flags);

    // Clear bit 0
    flags &= !(1 << 0);
    println!("  After clear bit 0: {:08b}", flags);

    // Toggle bit 3
    flags ^= 1 << 3;
    println!("  After toggle bit 3: {:08b}", flags);

    // Check if bit is set
    let bit_3_set = (flags & (1 << 3)) != 0;
    println!("  Bit 3 set: {}", bit_3_set);

    // Bitfield struct pattern
    #[derive(Debug, Clone, Copy)]
    struct StatusRegister(u8);

    impl StatusRegister {
        const CARRY: u8 = 1 << 0;
        const ZERO: u8 = 1 << 1;
        const INTERRUPT: u8 = 1 << 2;
        const OVERFLOW: u8 = 1 << 6;
        const NEGATIVE: u8 = 1 << 7;

        fn new() -> Self {
            StatusRegister(0)
        }

        fn set(&mut self, flag: u8) {
            self.0 |= flag;
        }

        fn clear(&mut self, flag: u8) {
            self.0 &= !flag;
        }

        fn is_set(&self, flag: u8) -> bool {
            (self.0 & flag) != 0
        }
    }

    let mut status = StatusRegister::new();
    status.set(StatusRegister::ZERO);
    status.set(StatusRegister::CARRY);

    println!("  Status register: {:08b}", status.0);
    println!("  Zero flag: {}", status.is_set(StatusRegister::ZERO));
    println!(
        "  Overflow flag: {}",
        status.is_set(StatusRegister::OVERFLOW)
    );

    // Extract bit fields
    let packed: u32 = 0xABCD1234;
    let byte0 = (packed & 0xFF) as u8;
    let byte1 = ((packed >> 8) & 0xFF) as u8;
    let byte2 = ((packed >> 16) & 0xFF) as u8;
    let byte3 = ((packed >> 24) & 0xFF) as u8;

    println!(
        "  Packed 0x{:08X} -> bytes: {:02X} {:02X} {:02X} {:02X}",
        packed, byte3, byte2, byte1, byte0
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_result() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;

        assert_eq!(some.unwrap(), 42);
        assert!(none.is_none());

        let ok: Result<i32, ()> = Ok(100);
        assert!(ok.is_ok());
    }

    #[test]
    fn test_checked_arithmetic() {
        let x: u8 = 200;
        let y: u8 = 100;

        assert!(x.checked_add(y).is_none());
        assert_eq!(x.saturating_add(y), 255);
        assert_eq!(x.wrapping_add(y), 44);
    }

    #[test]
    fn test_bit_operations() {
        let mut flags: u8 = 0;

        flags |= 1 << 2;
        assert_eq!(flags, 0b0000_0100);

        flags |= 1 << 5;
        assert_eq!(flags, 0b0010_0100);

        flags &= !(1 << 2);
        assert_eq!(flags, 0b0010_0000);
    }
}
