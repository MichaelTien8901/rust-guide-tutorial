//! Performance Example
//!
//! Demonstrates profiling, benchmarking, and optimization techniques.
//!
//! # Optimization Decision Flow
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │              Performance Optimization Flow              │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  1. Measure First (Don't guess!)                        │
//!     │     - cargo bench                                       │
//!     │     - profiling tools (perf, flamegraph)                │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  2. Identify Bottlenecks                                │
//!     │     - Hot loops                                         │
//!     │     - Memory allocations                                │
//!     │     - Cache misses                                      │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  3. Apply Optimizations                                 │
//!     │     - Better algorithms (O(n²) → O(n log n))            │
//!     │     - Reduce allocations (reuse buffers)                │
//!     │     - Improve cache locality                            │
//!     │     - Use SIMD/parallel processing                      │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │  4. Verify Improvement                                  │
//!     │     - Re-measure with benchmarks                        │
//!     │     - Ensure correctness preserved                      │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use std::collections::HashMap;
use std::time::Instant;

fn main() {
    println!("=== Performance ===\n");

    println!("--- Manual Timing ---");
    manual_timing();

    println!("\n--- Algorithm Complexity ---");
    algorithm_complexity();

    println!("\n--- Memory Optimization ---");
    memory_optimization();

    println!("\n--- Cache-Friendly Code ---");
    cache_friendly();

    println!("\n--- Avoiding Allocations ---");
    avoiding_allocations();

    println!("\n--- Iterator Optimization ---");
    iterator_optimization();

    println!("\n--- Compiler Hints ---");
    compiler_hints();
}

// ============================================
// Manual Timing
// ============================================

fn manual_timing() {
    // Simple timing with Instant
    let start = Instant::now();

    let sum: u64 = (0..1_000_000).sum();

    let elapsed = start.elapsed();
    println!("  Sum of 0..1M = {}", sum);
    println!("  Time: {:?}", elapsed);

    // Timing helper function
    fn time_it<F, T>(name: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        let start = Instant::now();
        let result = f();
        println!("  {} took {:?}", name, start.elapsed());
        result
    }

    time_it("Vec creation", || {
        let v: Vec<i32> = (0..100_000).collect();
        v.len()
    });

    time_it("HashMap creation", || {
        let m: HashMap<i32, i32> = (0..10_000).map(|i| (i, i * 2)).collect();
        m.len()
    });
}

// ============================================
// Algorithm Complexity
// ============================================

fn algorithm_complexity() {
    let data: Vec<i32> = (0..10_000).collect();

    // O(n) linear search
    fn linear_search(data: &[i32], target: i32) -> Option<usize> {
        for (i, &val) in data.iter().enumerate() {
            if val == target {
                return Some(i);
            }
        }
        None
    }

    // O(log n) binary search (requires sorted data)
    fn binary_search(data: &[i32], target: i32) -> Option<usize> {
        data.binary_search(&target).ok()
    }

    let target = 9_999;

    let start = Instant::now();
    let _ = linear_search(&data, target);
    let linear_time = start.elapsed();

    let start = Instant::now();
    let _ = binary_search(&data, target);
    let binary_time = start.elapsed();

    println!("  Searching for {} in 10,000 elements:", target);
    println!("    Linear search: {:?}", linear_time);
    println!("    Binary search: {:?}", binary_time);

    // O(n²) vs O(n log n) sorting
    fn bubble_sort(arr: &mut [i32]) {
        let n = arr.len();
        for i in 0..n {
            for j in 0..n - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }

    let mut small_data: Vec<i32> = (0..1000).rev().collect();
    let mut small_data2 = small_data.clone();

    let start = Instant::now();
    bubble_sort(&mut small_data);
    let bubble_time = start.elapsed();

    let start = Instant::now();
    small_data2.sort(); // Uses introsort - O(n log n)
    let std_time = start.elapsed();

    println!("  Sorting 1,000 elements:");
    println!("    Bubble sort O(n²): {:?}", bubble_time);
    println!("    std::sort O(n log n): {:?}", std_time);
}

// ============================================
// Memory Optimization
// ============================================

fn memory_optimization() {
    // Field ordering affects struct size
    #[repr(C)]
    struct Unoptimized {
        a: u8,      // 1 byte
        // 7 bytes padding
        b: u64,     // 8 bytes
        c: u8,      // 1 byte
        // 7 bytes padding
    }

    #[repr(C)]
    struct Optimized {
        b: u64,     // 8 bytes
        a: u8,      // 1 byte
        c: u8,      // 1 byte
        // 6 bytes padding
    }

    // Rust optimizes this automatically without repr(C)
    struct RustOptimized {
        a: u8,
        b: u64,
        c: u8,
    }

    println!("  Struct sizes:");
    println!("    Unoptimized (repr(C)): {} bytes", std::mem::size_of::<Unoptimized>());
    println!("    Optimized (repr(C)): {} bytes", std::mem::size_of::<Optimized>());
    println!("    RustOptimized: {} bytes", std::mem::size_of::<RustOptimized>());

    // Using smaller types
    struct LargeIds {
        user_id: u64,
        product_id: u64,
        order_id: u64,
    }

    struct SmallIds {
        user_id: u32,
        product_id: u32,
        order_id: u32,
    }

    println!("  ID struct sizes:");
    println!("    LargeIds (u64): {} bytes", std::mem::size_of::<LargeIds>());
    println!("    SmallIds (u32): {} bytes", std::mem::size_of::<SmallIds>());

    // Box for large data on heap
    struct LargeData {
        data: [u8; 10000],
    }

    println!("  LargeData on stack: {} bytes", std::mem::size_of::<LargeData>());
    println!("  Box<LargeData>: {} bytes (pointer)", std::mem::size_of::<Box<LargeData>>());
}

// ============================================
// Cache-Friendly Code
// ============================================

fn cache_friendly() {
    const SIZE: usize = 1000;

    // Row-major access (cache-friendly)
    let matrix: Vec<Vec<i32>> = vec![vec![1; SIZE]; SIZE];

    let start = Instant::now();
    let mut sum = 0i64;
    for row in &matrix {
        for &val in row {
            sum += val as i64;
        }
    }
    let row_major_time = start.elapsed();

    // Column-major access (cache-unfriendly)
    let start = Instant::now();
    let mut sum = 0i64;
    for col in 0..SIZE {
        for row in 0..SIZE {
            sum += matrix[row][col] as i64;
        }
    }
    let col_major_time = start.elapsed();

    println!("  Matrix sum ({}x{}):", SIZE, SIZE);
    println!("    Row-major (cache-friendly): {:?}", row_major_time);
    println!("    Column-major (cache-unfriendly): {:?}", col_major_time);

    // Flat array is even better
    let flat: Vec<i32> = vec![1; SIZE * SIZE];

    let start = Instant::now();
    let sum: i64 = flat.iter().map(|&x| x as i64).sum();
    let flat_time = start.elapsed();

    println!("    Flat array: {:?}", flat_time);
    println!("  Sum: {}", sum);
}

// ============================================
// Avoiding Allocations
// ============================================

fn avoiding_allocations() {
    // Preallocate capacity
    let start = Instant::now();
    let mut v1 = Vec::new();
    for i in 0..100_000 {
        v1.push(i);
    }
    let no_capacity_time = start.elapsed();

    let start = Instant::now();
    let mut v2 = Vec::with_capacity(100_000);
    for i in 0..100_000 {
        v2.push(i);
    }
    let with_capacity_time = start.elapsed();

    println!("  Vec push 100,000 elements:");
    println!("    Without capacity: {:?}", no_capacity_time);
    println!("    With capacity: {:?}", with_capacity_time);

    // Reuse buffers
    fn process_with_allocation(data: &[i32]) -> i32 {
        let doubled: Vec<i32> = data.iter().map(|x| x * 2).collect();
        doubled.iter().sum()
    }

    fn process_without_allocation(data: &[i32], buffer: &mut Vec<i32>) -> i32 {
        buffer.clear();
        buffer.extend(data.iter().map(|x| x * 2));
        buffer.iter().sum()
    }

    let data: Vec<i32> = (0..10_000).collect();

    let start = Instant::now();
    for _ in 0..1000 {
        let _ = process_with_allocation(&data);
    }
    let alloc_time = start.elapsed();

    let mut buffer = Vec::with_capacity(data.len());
    let start = Instant::now();
    for _ in 0..1000 {
        let _ = process_without_allocation(&data, &mut buffer);
    }
    let reuse_time = start.elapsed();

    println!("  Process 10,000 elements 1,000 times:");
    println!("    New allocation each time: {:?}", alloc_time);
    println!("    Reusing buffer: {:?}", reuse_time);

    // Use slices instead of owned collections
    fn sum_slice(data: &[i32]) -> i32 {
        data.iter().sum()
    }

    fn sum_vec(data: Vec<i32>) -> i32 {
        data.iter().sum()
    }

    println!("  Prefer &[T] over Vec<T> for read-only access");
}

// ============================================
// Iterator Optimization
// ============================================

fn iterator_optimization() {
    let data: Vec<i32> = (0..100_000).collect();

    // Iterators are lazy and often zero-cost
    let start = Instant::now();
    let sum1: i32 = data.iter()
        .filter(|&&x| x % 2 == 0)
        .map(|x| x * 2)
        .sum();
    let iterator_time = start.elapsed();

    // Imperative approach
    let start = Instant::now();
    let mut sum2 = 0;
    for &x in &data {
        if x % 2 == 0 {
            sum2 += x * 2;
        }
    }
    let imperative_time = start.elapsed();

    println!("  Filter + map + sum on 100,000 elements:");
    println!("    Iterator chain: {:?}", iterator_time);
    println!("    Imperative loop: {:?}", imperative_time);
    println!("    Results match: {}", sum1 == sum2);

    // Avoid collect() when not needed
    let start = Instant::now();
    let sum_with_collect: i32 = data.iter()
        .map(|x| x * 2)
        .collect::<Vec<_>>()
        .iter()
        .sum();
    let collect_time = start.elapsed();

    let start = Instant::now();
    let sum_without_collect: i32 = data.iter()
        .map(|x| x * 2)
        .sum();
    let no_collect_time = start.elapsed();

    println!("  Map + sum:");
    println!("    With unnecessary collect: {:?}", collect_time);
    println!("    Without collect: {:?}", no_collect_time);
}

// ============================================
// Compiler Hints
// ============================================

fn compiler_hints() {
    // #[inline] hint
    #[inline]
    fn add_inline(a: i32, b: i32) -> i32 {
        a + b
    }

    // #[inline(always)] - force inlining
    #[inline(always)]
    fn add_always_inline(a: i32, b: i32) -> i32 {
        a + b
    }

    // #[cold] - hint that this path is unlikely
    #[cold]
    fn handle_error() {
        eprintln!("    Error occurred!");
    }

    // Likely/unlikely branch hints
    fn process_value(x: i32) -> i32 {
        if x > 0 {
            // This branch is likely
            x * 2
        } else {
            // This branch is cold/unlikely
            handle_error();
            0
        }
    }

    println!("  Compiler hints:");
    println!("    #[inline] - suggest inlining");
    println!("    #[inline(always)] - force inlining");
    println!("    #[inline(never)] - prevent inlining");
    println!("    #[cold] - mark unlikely code paths");

    // Release vs Debug builds
    #[cfg(debug_assertions)]
    println!("  Running in DEBUG mode (slower, more checks)");

    #[cfg(not(debug_assertions))]
    println!("  Running in RELEASE mode (optimized)");

    // Bounds checking
    let arr = [1, 2, 3, 4, 5];

    // Safe but has bounds check
    let _val = arr[2];

    // Unsafe: no bounds check
    let _val = unsafe { *arr.get_unchecked(2) };

    // Better: use get() and handle None
    if let Some(&val) = arr.get(2) {
        println!("  arr[2] = {}", val);
    }

    println!("\n  Optimization tips:");
    println!("    - Use release builds: cargo build --release");
    println!("    - Enable LTO: lto = true in Cargo.toml");
    println!("    - Profile before optimizing");
    println!("    - Measure, don't guess!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_search() {
        let data = vec![1, 2, 3, 4, 5];
        assert_eq!(linear_search(&data, 3), Some(2));
        assert_eq!(linear_search(&data, 10), None);
    }

    fn linear_search(data: &[i32], target: i32) -> Option<usize> {
        data.iter().position(|&x| x == target)
    }

    #[test]
    fn test_binary_search() {
        let data = vec![1, 2, 3, 4, 5];
        assert!(data.binary_search(&3).is_ok());
        assert!(data.binary_search(&10).is_err());
    }

    #[test]
    fn test_preallocated_vec() {
        let mut v = Vec::with_capacity(100);
        for i in 0..100 {
            v.push(i);
        }
        assert_eq!(v.len(), 100);
        assert_eq!(v.capacity(), 100); // No reallocation
    }
}
