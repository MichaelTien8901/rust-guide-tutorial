//! # Memory Management Example
//!
//! Demonstrates embedded memory management concepts in standard Rust for CI:
//! - Fixed-capacity collections (simulated heapless Vec and String)
//! - Memory pool (fixed-size block allocator)
//! - Stack vs heap comparison
//! - STM32F769 memory region simulation

fn main() {
    println!("=== Memory Management in no_std ===\n");

    demonstrate_fixed_vec();
    demonstrate_fixed_string();
    demonstrate_memory_pool();
    demonstrate_stack_vs_heap();
    demonstrate_memory_regions();
}

// ---------------------------------------------------------------------------
// Fixed-Capacity Vec (simulated heapless::Vec)
// ---------------------------------------------------------------------------

/// A fixed-capacity vector backed by an array — no heap allocation.
/// Mirrors the API of `heapless::Vec<T, N>`.
struct FixedVec<T: Copy + Default, const N: usize> {
    buf: [T; N],
    len: usize,
}

impl<T: Copy + Default, const N: usize> FixedVec<T, N> {
    fn new() -> Self {
        Self {
            buf: [T::default(); N],
            len: 0,
        }
    }

    fn push(&mut self, value: T) -> Result<(), T> {
        if self.len < N {
            self.buf[self.len] = value;
            self.len += 1;
            Ok(())
        } else {
            Err(value) // Buffer full — return the value back
        }
    }

    fn len(&self) -> usize {
        self.len
    }

    fn capacity(&self) -> usize {
        N
    }

    fn as_slice(&self) -> &[T] {
        &self.buf[..self.len]
    }
}

fn demonstrate_fixed_vec() {
    println!("--- Fixed-Capacity Vec (heapless::Vec simulation) ---");

    let mut readings: FixedVec<u16, 8> = FixedVec::new();
    for val in [1023, 512, 768, 255] {
        readings.push(val).unwrap();
    }

    println!("  Capacity: {}, Used: {}", readings.capacity(), readings.len());
    println!("  Values: {:?}", readings.as_slice());

    // Fill to capacity
    for i in 0..4 {
        readings.push(100 + i).unwrap();
    }
    println!("  After filling: {}/{}", readings.len(), readings.capacity());

    // Next push should fail
    match readings.push(999) {
        Ok(()) => println!("  Push succeeded (unexpected)"),
        Err(v) => println!("  Push rejected (buffer full): value {} returned", v),
    }
    println!();
}

// ---------------------------------------------------------------------------
// Fixed-Capacity String (simulated heapless::String)
// ---------------------------------------------------------------------------

/// A fixed-capacity string backed by a byte array.
/// Mirrors `heapless::String<N>`.
struct FixedString<const N: usize> {
    buf: [u8; N],
    len: usize,
}

impl<const N: usize> FixedString<N> {
    fn new() -> Self {
        Self {
            buf: [0u8; N],
            len: 0,
        }
    }

    fn push_str(&mut self, s: &str) -> Result<(), ()> {
        let bytes = s.as_bytes();
        if self.len + bytes.len() > N {
            return Err(()); // Would overflow
        }
        self.buf[self.len..self.len + bytes.len()].copy_from_slice(bytes);
        self.len += bytes.len();
        Ok(())
    }

    fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.len]).unwrap()
    }

    fn remaining(&self) -> usize {
        N - self.len
    }
}

fn demonstrate_fixed_string() {
    println!("--- Fixed-Capacity String (heapless::String simulation) ---");

    let mut msg: FixedString<64> = FixedString::new();
    msg.push_str("ADC reading: ").unwrap();
    msg.push_str("1023").unwrap();
    msg.push_str(" mV").unwrap();

    println!("  Message: \"{}\"", msg.as_str());
    println!("  Used: {} / {} bytes", msg.len, 64);
    println!("  Remaining: {} bytes", msg.remaining());
    println!();
}

// ---------------------------------------------------------------------------
// Memory Pool (fixed-size block allocator)
// ---------------------------------------------------------------------------

/// A pool of N fixed-size blocks. O(1) alloc/dealloc, zero fragmentation.
struct MemoryPool<const N: usize, const BLOCK_SIZE: usize> {
    storage: [[u8; BLOCK_SIZE]; N],
    free: [bool; N],
}

impl<const N: usize, const BLOCK_SIZE: usize> MemoryPool<N, BLOCK_SIZE> {
    fn new() -> Self {
        Self {
            storage: [[0u8; BLOCK_SIZE]; N],
            free: [true; N],
        }
    }

    /// Allocate a block. Returns (index, mutable reference) or None.
    fn alloc(&mut self) -> Option<(usize, &mut [u8; BLOCK_SIZE])> {
        for i in 0..N {
            if self.free[i] {
                self.free[i] = false;
                return Some((i, &mut self.storage[i]));
            }
        }
        None
    }

    /// Return a block to the pool.
    fn dealloc(&mut self, index: usize) {
        assert!(index < N, "Invalid pool index");
        self.storage[index] = [0u8; BLOCK_SIZE]; // Clear on free
        self.free[index] = true;
    }

    /// Count of available blocks.
    fn available(&self) -> usize {
        self.free.iter().filter(|&&f| f).count()
    }
}

fn demonstrate_memory_pool() {
    println!("--- Memory Pool (fixed-size block allocator) ---");

    // 4 packet buffers, each 128 bytes (small for demo)
    let mut pool: MemoryPool<4, 128> = MemoryPool::new();
    println!("  Pool: {} blocks x {} bytes", 4, 128);
    println!("  Available: {}", pool.available());

    // Allocate two blocks
    let (idx_a, buf_a) = pool.alloc().expect("pool not empty");
    buf_a[..5].copy_from_slice(b"PKT_A");
    println!("  Allocated block {} -> {:?}", idx_a, &buf_a[..5]);

    let (idx_b, buf_b) = pool.alloc().expect("pool not empty");
    buf_b[..5].copy_from_slice(b"PKT_B");
    println!("  Allocated block {} -> {:?}", idx_b, &buf_b[..5]);

    println!("  Available after 2 allocs: {}", pool.available());

    // Free the first block
    pool.dealloc(idx_a);
    println!("  Freed block {}", idx_a);
    println!("  Available after dealloc: {}", pool.available());

    // Allocate until exhausted
    let _ = pool.alloc(); // block 0 (reused)
    let _ = pool.alloc(); // block 2
    let _ = pool.alloc(); // block 3
    assert!(pool.alloc().is_none(), "pool should be exhausted");
    println!("  Pool exhausted: alloc returns None (no panic, no UB)");
    println!();
}

// ---------------------------------------------------------------------------
// Stack vs Heap Comparison
// ---------------------------------------------------------------------------

fn demonstrate_stack_vs_heap() {
    println!("--- Stack vs Heap ---");

    // Stack allocation — fixed size, instant, deterministic
    let stack_buf: [u8; 256] = [0xAA; 256];
    println!("  Stack buffer: {} bytes at {:p}", stack_buf.len(), &stack_buf);

    // Heap allocation — dynamic size, involves allocator
    let heap_buf: Vec<u8> = vec![0xBB; 256];
    println!("  Heap  buffer: {} bytes at {:p}", heap_buf.len(), heap_buf.as_ptr());

    println!();
    println!("  Comparison:");
    println!("    Stack: O(1) alloc, no fragmentation, size fixed at compile time");
    println!("    Heap:  variable alloc time, fragmentation risk, dynamic sizing");
    println!("    Embedded default: prefer stack + heapless collections");
    println!();
}

// ---------------------------------------------------------------------------
// STM32F769 Memory Region Simulation
// ---------------------------------------------------------------------------

/// Simulated memory region descriptor for STM32F769.
struct MemRegion {
    name: &'static str,
    base: u32,
    size_kb: u32,
    wait_states: u8,
    dma_accessible: bool,
    best_for: &'static str,
}

fn demonstrate_memory_regions() {
    println!("--- STM32F769 Memory Regions ---");

    let regions = [
        MemRegion {
            name: "DTCM",
            base: 0x2000_0000,
            size_kb: 16,
            wait_states: 0,
            dma_accessible: false,
            best_for: "Stack, ISR state, hot variables",
        },
        MemRegion {
            name: "SRAM1",
            base: 0x2002_0000,
            size_kb: 368,
            wait_states: 1,
            dma_accessible: true,
            best_for: "Bulk data, DMA buffers, heap",
        },
        MemRegion {
            name: "SRAM2",
            base: 0x2007_C000,
            size_kb: 16,
            wait_states: 1,
            dma_accessible: true,
            best_for: "Secondary DMA buffers",
        },
    ];

    for r in &regions {
        println!(
            "  {:<6} 0x{:08X}  {:>4} KB  {} wait  DMA: {:<3}  -> {}",
            r.name,
            r.base,
            r.size_kb,
            r.wait_states,
            if r.dma_accessible { "yes" } else { "no " },
            r.best_for,
        );
    }

    println!();
    println!("  Optimization tips:");
    println!("    - Place stack in DTCM (zero-wait-state, single-cycle access)");
    println!("    - Place ISR-shared variables in DTCM via #[link_section = \".dtcm\"]");
    println!("    - Place DMA buffers in SRAM1/SRAM2 (DTCM has no DMA access)");
    println!("    - Use const for read-only data (stays in flash, saves RAM)");
    println!("    - Use the smallest integer type that fits: u8 for channels, u16 for ADC");
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_vec_push_and_overflow() {
        let mut v: FixedVec<u32, 4> = FixedVec::new();
        assert_eq!(v.len(), 0);
        assert_eq!(v.capacity(), 4);

        v.push(10).unwrap();
        v.push(20).unwrap();
        v.push(30).unwrap();
        v.push(40).unwrap();
        assert_eq!(v.len(), 4);
        assert_eq!(v.as_slice(), &[10, 20, 30, 40]);

        // 5th push must fail
        assert!(v.push(50).is_err());
        assert_eq!(v.len(), 4); // unchanged
    }

    #[test]
    fn test_fixed_string_push_and_overflow() {
        let mut s: FixedString<16> = FixedString::new();
        s.push_str("Hello").unwrap();
        assert_eq!(s.as_str(), "Hello");

        // This should fail: "Hello" (5) + "World!!!!!!!!!" (14) > 16
        assert!(s.push_str("World!!!!!!!!!").is_err());
        assert_eq!(s.as_str(), "Hello"); // unchanged
    }

    #[test]
    fn test_memory_pool_alloc_dealloc() {
        let mut pool: MemoryPool<3, 64> = MemoryPool::new();
        assert_eq!(pool.available(), 3);

        let (i0, _) = pool.alloc().unwrap();
        let (i1, _) = pool.alloc().unwrap();
        assert_eq!(pool.available(), 1);

        pool.dealloc(i0);
        assert_eq!(pool.available(), 2);

        // Reuse freed block
        let (i_reused, _) = pool.alloc().unwrap();
        assert_eq!(i_reused, i0); // first-fit reuses index 0

        // Allocate remaining
        let _ = pool.alloc().unwrap();
        assert_eq!(pool.available(), 0);

        // Pool exhausted
        assert!(pool.alloc().is_none());

        // Free one and re-allocate
        pool.dealloc(i1);
        assert_eq!(pool.available(), 1);
        assert!(pool.alloc().is_some());
    }

    #[test]
    fn test_stm32f769_memory_map_layout() {
        // DTCM starts at beginning of RAM
        let dtcm_start: u32 = 0x2000_0000;
        let dtcm_size: u32 = 16 * 1024;

        // SRAM1 follows (with a gap — the AHB bus matrix mapping)
        let sram1_start: u32 = 0x2002_0000;
        let sram1_size: u32 = 368 * 1024;

        // SRAM2 follows SRAM1
        let sram2_start: u32 = 0x2007_C000;
        let sram2_size: u32 = 16 * 1024;

        // Verify non-overlapping
        assert!(dtcm_start + dtcm_size <= sram1_start);
        assert!(sram1_start + sram1_size <= sram2_start);

        // Verify total RAM size
        let total_kb = (dtcm_size + sram1_size + sram2_size) / 1024;
        assert_eq!(total_kb, 400); // 16 + 368 + 16 = 400 KB
        // Note: STM32F769 reports 512 KB because of additional
        // system RAM not mapped in the standard user regions
    }

    #[test]
    fn test_const_vs_static() {
        // const: inlined, lives in .rodata (flash), no RAM cost
        const TABLE: [u8; 4] = [1, 2, 3, 4];

        // static: fixed address in RAM
        static COUNTER: std::sync::atomic::AtomicU32 =
            std::sync::atomic::AtomicU32::new(0);

        // const can be used in any const context
        assert_eq!(TABLE[2], 3);

        // static has a stable address (useful for sharing between ISR and main)
        COUNTER.store(42, std::sync::atomic::Ordering::Relaxed);
        assert_eq!(COUNTER.load(std::sync::atomic::Ordering::Relaxed), 42);
    }
}
