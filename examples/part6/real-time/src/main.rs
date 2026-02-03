//! Real-Time Programming Patterns
//!
//! This example demonstrates patterns for real-time systems including
//! heapless data structures, static allocation, and deterministic execution.
//!
//! Note: This is a conceptual example. Real real-time systems use
//! the `heapless` crate and run in `#![no_std]` environments.

use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

// ============================================================================
// Heapless Vec - Fixed-capacity vector without heap allocation
// ============================================================================

/// A fixed-capacity vector that stores elements inline.
/// This is similar to `heapless::Vec<T, N>`.
pub struct HeaplessVec<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    len: usize,
}

impl<T, const N: usize> HeaplessVec<T, N> {
    /// Create a new empty vector.
    pub const fn new() -> Self {
        Self {
            // SAFETY: MaybeUninit doesn't require initialization
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the vector.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the vector is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Push an element to the vector.
    /// Returns `Err(value)` if the vector is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.len >= N {
            return Err(value);
        }
        self.buffer[self.len].write(value);
        self.len += 1;
        Ok(())
    }

    /// Pop an element from the vector.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        // SAFETY: Element at len was initialized
        Some(unsafe { self.buffer[self.len].assume_init_read() })
    }

    /// Get a reference to an element.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        // SAFETY: Element at index was initialized
        Some(unsafe { self.buffer[index].assume_init_ref() })
    }

    /// Get a mutable reference to an element.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        // SAFETY: Element at index was initialized
        Some(unsafe { self.buffer[index].assume_init_mut() })
    }

    /// Clear all elements.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// Iterate over elements.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.len).map(|i| unsafe { self.buffer[i].assume_init_ref() })
    }
}

impl<T, const N: usize> Default for HeaplessVec<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for HeaplessVec<T, N> {
    fn drop(&mut self) {
        self.clear();
    }
}

// ============================================================================
// Heapless String - Fixed-capacity string without heap allocation
// ============================================================================

/// A fixed-capacity string that stores characters inline.
/// This is similar to `heapless::String<N>`.
pub struct HeaplessString<const N: usize> {
    buffer: [u8; N],
    len: usize,
}

impl<const N: usize> HeaplessString<N> {
    /// Create a new empty string.
    pub const fn new() -> Self {
        Self {
            buffer: [0; N],
            len: 0,
        }
    }

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Push a character to the string.
    pub fn push(&mut self, ch: char) -> Result<(), ()> {
        let mut buf = [0u8; 4];
        let encoded = ch.encode_utf8(&mut buf);
        if self.len + encoded.len() > N {
            return Err(());
        }
        self.buffer[self.len..self.len + encoded.len()].copy_from_slice(encoded.as_bytes());
        self.len += encoded.len();
        Ok(())
    }

    /// Push a string slice.
    pub fn push_str(&mut self, s: &str) -> Result<(), ()> {
        if self.len + s.len() > N {
            return Err(());
        }
        self.buffer[self.len..self.len + s.len()].copy_from_slice(s.as_bytes());
        self.len += s.len();
        Ok(())
    }

    /// Get the string as a slice.
    pub fn as_str(&self) -> &str {
        // SAFETY: We only write valid UTF-8
        unsafe { std::str::from_utf8_unchecked(&self.buffer[..self.len]) }
    }

    /// Clear the string.
    pub fn clear(&mut self) {
        self.len = 0;
    }
}

impl<const N: usize> Default for HeaplessString<N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize> std::fmt::Display for HeaplessString<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

// ============================================================================
// Ring Buffer / Queue - FIFO queue with fixed capacity
// ============================================================================

/// A fixed-capacity FIFO queue (ring buffer).
/// This is similar to `heapless::spsc::Queue<T, N>`.
pub struct RingBuffer<T, const N: usize> {
    buffer: [MaybeUninit<T>; N],
    head: usize, // Read position
    tail: usize, // Write position
    len: usize,
}

impl<T, const N: usize> RingBuffer<T, N> {
    /// Create a new empty queue.
    pub const fn new() -> Self {
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            head: 0,
            tail: 0,
            len: 0,
        }
    }

    /// Returns the number of elements in the queue.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the capacity of the queue.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns true if the queue is full.
    pub fn is_full(&self) -> bool {
        self.len == N
    }

    /// Enqueue an element (add to back).
    pub fn enqueue(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.buffer[self.tail].write(value);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }

    /// Dequeue an element (remove from front).
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let value = unsafe { self.buffer[self.head].assume_init_read() };
        self.head = (self.head + 1) % N;
        self.len -= 1;
        Some(value)
    }

    /// Peek at the front element without removing it.
    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        Some(unsafe { self.buffer[self.head].assume_init_ref() })
    }
}

impl<T, const N: usize> Default for RingBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for RingBuffer<T, N> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
    }
}

// ============================================================================
// Memory Pool - Fixed-size object allocator
// ============================================================================

/// A memory pool for fixed-size allocations.
/// This is similar to `heapless::pool::Pool`.
pub struct MemoryPool<T, const N: usize> {
    storage: UnsafeCell<[MaybeUninit<T>; N]>,
    free_list: UnsafeCell<[usize; N]>,
    free_count: AtomicUsize,
}

/// A handle to an allocated object from the pool.
pub struct PoolBox<'a, T, const N: usize> {
    pool: &'a MemoryPool<T, N>,
    index: usize,
}

impl<T, const N: usize> MemoryPool<T, N> {
    /// Create a new memory pool.
    pub fn new() -> Self {
        let mut free_list: [usize; N] = [0; N];
        for i in 0..N {
            free_list[i] = i;
        }
        Self {
            storage: UnsafeCell::new(unsafe { MaybeUninit::uninit().assume_init() }),
            free_list: UnsafeCell::new(free_list),
            free_count: AtomicUsize::new(N),
        }
    }

    /// Allocate an object from the pool.
    pub fn alloc(&self, value: T) -> Option<PoolBox<'_, T, N>> {
        let count = self.free_count.load(Ordering::Acquire);
        if count == 0 {
            return None;
        }

        // Note: This is a simplified implementation
        // Real implementation would use atomic CAS for thread safety
        let new_count = count - 1;
        self.free_count.store(new_count, Ordering::Release);

        let free_list = unsafe { &mut *self.free_list.get() };
        let index = free_list[new_count];

        let storage = unsafe { &mut *self.storage.get() };
        storage[index].write(value);

        Some(PoolBox { pool: self, index })
    }

    /// Return the number of available slots.
    pub fn available(&self) -> usize {
        self.free_count.load(Ordering::Relaxed)
    }

    fn free(&self, index: usize) {
        let storage = unsafe { &mut *self.storage.get() };
        unsafe { storage[index].assume_init_drop() };

        let free_list = unsafe { &mut *self.free_list.get() };
        let count = self.free_count.load(Ordering::Acquire);
        free_list[count] = index;
        self.free_count.store(count + 1, Ordering::Release);
    }
}

impl<T, const N: usize> Default for MemoryPool<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T, const N: usize> std::ops::Deref for PoolBox<'a, T, N> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        let storage = unsafe { &*self.pool.storage.get() };
        unsafe { storage[self.index].assume_init_ref() }
    }
}

impl<'a, T, const N: usize> std::ops::DerefMut for PoolBox<'a, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let storage = unsafe { &mut *self.pool.storage.get() };
        unsafe { storage[self.index].assume_init_mut() }
    }
}

impl<'a, T, const N: usize> Drop for PoolBox<'a, T, N> {
    fn drop(&mut self) {
        self.pool.free(self.index);
    }
}

// ============================================================================
// Lock-Free SPSC Queue - Single Producer Single Consumer
// ============================================================================

/// A lock-free single-producer single-consumer queue.
/// This is similar to `heapless::spsc::Queue` with split producers/consumers.
pub struct SpscQueue<T, const N: usize> {
    buffer: UnsafeCell<[MaybeUninit<T>; N]>,
    head: AtomicUsize, // Read index (consumer)
    tail: AtomicUsize, // Write index (producer)
}

// SAFETY: Safe to send between threads if T is Send
unsafe impl<T: Send, const N: usize> Send for SpscQueue<T, N> {}
unsafe impl<T: Send, const N: usize> Sync for SpscQueue<T, N> {}

impl<T, const N: usize> SpscQueue<T, N> {
    /// Create a new empty queue.
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new(unsafe { MaybeUninit::uninit().assume_init() }),
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    /// Check if the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire) == self.tail.load(Ordering::Acquire)
    }

    /// Check if the queue is full.
    pub fn is_full(&self) -> bool {
        let head = self.head.load(Ordering::Acquire);
        let tail = self.tail.load(Ordering::Acquire);
        (tail + 1) % N == head
    }

    /// Enqueue (producer side).
    /// Returns Err if the queue is full.
    pub fn enqueue(&self, value: T) -> Result<(), T> {
        let tail = self.tail.load(Ordering::Relaxed);
        let next_tail = (tail + 1) % N;

        if next_tail == self.head.load(Ordering::Acquire) {
            return Err(value); // Queue is full
        }

        let buffer = unsafe { &mut *self.buffer.get() };
        buffer[tail].write(value);

        self.tail.store(next_tail, Ordering::Release);
        Ok(())
    }

    /// Dequeue (consumer side).
    /// Returns None if the queue is empty.
    pub fn dequeue(&self) -> Option<T> {
        let head = self.head.load(Ordering::Relaxed);

        if head == self.tail.load(Ordering::Acquire) {
            return None; // Queue is empty
        }

        let buffer = unsafe { &mut *self.buffer.get() };
        let value = unsafe { buffer[head].assume_init_read() };

        self.head.store((head + 1) % N, Ordering::Release);
        Some(value)
    }
}

impl<T, const N: usize> Default for SpscQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Priority Queue - For task scheduling
// ============================================================================

/// A task with a priority level.
#[derive(Debug, Clone)]
pub struct PriorityTask<T> {
    pub priority: u8, // Lower number = higher priority
    pub data: T,
}

/// A fixed-capacity priority queue for task scheduling.
/// Uses a simple sorted array approach (not a heap for clarity).
pub struct PriorityQueue<T, const N: usize> {
    // Store as (priority, data) tuples, sorted by priority
    buffer: [MaybeUninit<(u8, T)>; N],
    len: usize,
}

impl<T, const N: usize> PriorityQueue<T, N> {
    /// Create a new empty priority queue.
    pub fn new() -> Self {
        Self {
            buffer: unsafe { MaybeUninit::uninit().assume_init() },
            len: 0,
        }
    }

    /// Returns the number of tasks.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns true if full.
    pub fn is_full(&self) -> bool {
        self.len >= N
    }

    /// Insert a task with given priority.
    /// Lower priority numbers are higher priority.
    pub fn insert(&mut self, priority: u8, data: T) -> Result<(), T> {
        if self.is_full() {
            return Err(data);
        }

        // Find insertion point (maintain sorted order, lower priority number = higher priority)
        let mut insert_idx = self.len;
        for i in 0..self.len {
            let existing_priority = unsafe { (*self.buffer[i].as_ptr()).0 };
            if priority < existing_priority {
                insert_idx = i;
                break;
            }
        }

        // Shift elements to make room
        for i in (insert_idx..self.len).rev() {
            unsafe {
                let src = self.buffer[i].as_ptr();
                let dst = self.buffer[i + 1].as_mut_ptr();
                std::ptr::copy_nonoverlapping(src, dst, 1);
            }
        }

        // Insert new element
        self.buffer[insert_idx].write((priority, data));
        self.len += 1;

        Ok(())
    }

    /// Get the highest priority task (lowest priority number).
    pub fn pop(&mut self) -> Option<PriorityTask<T>> {
        if self.is_empty() {
            return None;
        }

        // Read the first element
        let (priority, data) = unsafe { self.buffer[0].assume_init_read() };

        // Shift all elements down
        for i in 1..self.len {
            unsafe {
                let src = self.buffer[i].as_ptr();
                let dst = self.buffer[i - 1].as_mut_ptr();
                std::ptr::copy_nonoverlapping(src, dst, 1);
            }
        }

        self.len -= 1;

        Some(PriorityTask { priority, data })
    }

    /// Peek at the highest priority task.
    pub fn peek(&self) -> Option<PriorityTask<&T>> {
        if self.is_empty() {
            return None;
        }
        let (priority, ref data) = unsafe { &*self.buffer[0].as_ptr() };
        Some(PriorityTask {
            priority: *priority,
            data,
        })
    }
}

impl<T, const N: usize> Default for PriorityQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Drop for PriorityQueue<T, N> {
    fn drop(&mut self) {
        // Drop all remaining elements
        for i in 0..self.len {
            unsafe { self.buffer[i].assume_init_drop() };
        }
    }
}

// ============================================================================
// Static Allocation Patterns
// ============================================================================

/// Statically allocated buffer for sensor readings.
static SENSOR_BUFFER: StaticBuffer<f32, 64> = StaticBuffer::new();

/// A statically allocated buffer with interior mutability.
pub struct StaticBuffer<T: Copy, const N: usize> {
    buffer: UnsafeCell<[T; N]>,
    len: AtomicUsize,
    initialized: AtomicBool,
}

// Note: Not truly thread-safe, demonstration only
unsafe impl<T: Copy + Send, const N: usize> Sync for StaticBuffer<T, N> {}

impl<T: Copy + Default, const N: usize> StaticBuffer<T, N> {
    /// Create a new buffer (const constructor for static allocation).
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([unsafe { std::mem::zeroed() }; N]),
            len: AtomicUsize::new(0),
            initialized: AtomicBool::new(false),
        }
    }

    /// Initialize the buffer (call once at startup).
    pub fn init(&self) {
        if self.initialized.swap(true, Ordering::SeqCst) {
            return; // Already initialized
        }
        let buffer = unsafe { &mut *self.buffer.get() };
        for elem in buffer.iter_mut() {
            *elem = T::default();
        }
    }

    /// Push a value to the buffer.
    pub fn push(&self, value: T) -> Result<(), T> {
        let len = self.len.load(Ordering::Acquire);
        if len >= N {
            return Err(value);
        }

        let buffer = unsafe { &mut *self.buffer.get() };
        buffer[len] = value;
        self.len.store(len + 1, Ordering::Release);
        Ok(())
    }

    /// Get all values as a slice.
    pub fn as_slice(&self) -> &[T] {
        let len = self.len.load(Ordering::Acquire);
        let buffer = unsafe { &*self.buffer.get() };
        &buffer[..len]
    }

    /// Clear the buffer.
    pub fn clear(&self) {
        self.len.store(0, Ordering::Release);
    }
}

// ============================================================================
// Deadline Scheduling Simulation
// ============================================================================

/// Represents a real-time task with deadline.
#[derive(Debug, Clone)]
pub struct RealTimeTask {
    pub name: &'static str,
    pub period_ms: u32,    // Task period
    pub deadline_ms: u32,  // Relative deadline
    pub wcet_ms: u32,      // Worst-case execution time
    pub last_release: u64, // Last release time
}

impl RealTimeTask {
    /// Check if a task set is schedulable using Rate Monotonic Analysis.
    /// This is a utilization-based test.
    pub fn is_schedulable_rm(tasks: &[RealTimeTask]) -> bool {
        let n = tasks.len() as f64;
        // Utilization bound for RM: n * (2^(1/n) - 1)
        let bound = n * (2.0_f64.powf(1.0 / n) - 1.0);

        let total_utilization: f64 = tasks
            .iter()
            .map(|t| t.wcet_ms as f64 / t.period_ms as f64)
            .sum();

        println!("  Total utilization: {:.2}%", total_utilization * 100.0);
        println!(
            "  RM bound for {} tasks: {:.2}%",
            tasks.len(),
            bound * 100.0
        );

        total_utilization <= bound
    }

    /// Calculate response time using response time analysis.
    pub fn response_time_analysis(task_idx: usize, tasks: &[RealTimeTask]) -> Option<u32> {
        let task = &tasks[task_idx];
        let mut r = task.wcet_ms;

        // Iterate until convergence
        for _ in 0..100 {
            let mut interference: u32 = 0;

            // Sum interference from higher priority tasks
            for (i, hp_task) in tasks.iter().enumerate() {
                if i >= task_idx {
                    break;
                }
                let activations = (r as f64 / hp_task.period_ms as f64).ceil() as u32;
                interference += activations * hp_task.wcet_ms;
            }

            let new_r = task.wcet_ms + interference;
            if new_r == r {
                return if r <= task.deadline_ms { Some(r) } else { None };
            }
            if new_r > task.deadline_ms {
                return None;
            }
            r = new_r;
        }

        None // Did not converge
    }
}

// ============================================================================
// Demonstration Functions
// ============================================================================

fn demo_heapless_vec() {
    println!("=== Heapless Vec Demo ===\n");

    let mut vec: HeaplessVec<i32, 8> = HeaplessVec::new();

    println!("Capacity: {}", vec.capacity());
    println!("Initial length: {}", vec.len());

    // Push elements
    for i in 0..5 {
        vec.push(i * 10).unwrap();
    }
    println!("After pushing 5 elements: len={}", vec.len());

    // Access elements
    println!("Element at index 2: {:?}", vec.get(2));

    // Iterate
    print!("All elements: ");
    for elem in vec.iter() {
        print!("{} ", elem);
    }
    println!();

    // Pop elements
    while let Some(val) = vec.pop() {
        print!("Popped: {} ", val);
    }
    println!("\nFinal length: {}", vec.len());
    println!();
}

fn demo_heapless_string() {
    println!("=== Heapless String Demo ===\n");

    let mut s: HeaplessString<32> = HeaplessString::new();

    s.push_str("Hello").unwrap();
    s.push(' ').unwrap();
    s.push_str("World!").unwrap();

    println!("String: \"{}\"", s);
    println!("Length: {} bytes", s.len());
    println!("Capacity: {} bytes", s.capacity());

    // Try to overflow
    let result = s.push_str(" This is a very long string that won't fit");
    println!("Overflow attempt: {:?}", result.err());
    println!();
}

fn demo_ring_buffer() {
    println!("=== Ring Buffer Demo ===\n");

    let mut queue: RingBuffer<char, 4> = RingBuffer::new();

    // Enqueue
    for ch in ['A', 'B', 'C'] {
        queue.enqueue(ch).unwrap();
        println!("Enqueued: {}", ch);
    }

    // Peek
    println!("Front element: {:?}", queue.peek());

    // Dequeue some
    println!("Dequeued: {:?}", queue.dequeue());
    println!("Dequeued: {:?}", queue.dequeue());

    // Enqueue more (wraps around)
    for ch in ['D', 'E'] {
        queue.enqueue(ch).unwrap();
        println!("Enqueued: {}", ch);
    }

    // Drain
    print!("Remaining: ");
    while let Some(ch) = queue.dequeue() {
        print!("{} ", ch);
    }
    println!("\n");
}

fn demo_memory_pool() {
    println!("=== Memory Pool Demo ===\n");

    let pool: MemoryPool<[u8; 64], 4> = MemoryPool::new();

    println!("Pool capacity: 4 blocks of 64 bytes each");
    println!("Available before allocation: {}", pool.available());

    // Allocate some blocks
    let mut block1 = pool.alloc([0u8; 64]).unwrap();
    let block2 = pool.alloc([0u8; 64]).unwrap();

    println!("Available after 2 allocations: {}", pool.available());

    // Use the block
    block1[0] = 42;
    block1[1] = 43;
    println!("Wrote to block1: [{}, {}, ...]", block1[0], block1[1]);

    // Drop block2
    drop(block2);
    println!("Available after dropping block2: {}", pool.available());

    // Allocate again
    let _block3 = pool.alloc([0u8; 64]).unwrap();
    println!("Available after new allocation: {}", pool.available());
    println!();
}

fn demo_spsc_queue() {
    println!("=== SPSC Queue Demo ===\n");

    let queue: SpscQueue<u32, 8> = SpscQueue::new();

    println!("Lock-free single-producer single-consumer queue");
    println!("Empty: {}", queue.is_empty());

    // Producer side
    for i in 1..=5 {
        queue.enqueue(i * 100).unwrap();
        println!("Produced: {}", i * 100);
    }

    // Consumer side
    print!("Consumed: ");
    while let Some(val) = queue.dequeue() {
        print!("{} ", val);
    }
    println!("\n");
}

fn demo_priority_queue() {
    println!("=== Priority Queue Demo ===\n");

    let mut pq: PriorityQueue<&str, 8> = PriorityQueue::new();

    // Insert tasks with different priorities
    pq.insert(3, "Low priority task").unwrap();
    pq.insert(1, "High priority task").unwrap();
    pq.insert(2, "Medium priority task").unwrap();
    pq.insert(0, "Critical task").unwrap();

    println!("Tasks in priority order:");
    while let Some(task) = pq.pop() {
        println!("  Priority {}: {}", task.priority, task.data);
    }
    println!();
}

fn demo_static_allocation() {
    println!("=== Static Allocation Demo ===\n");

    SENSOR_BUFFER.init();
    println!("Static sensor buffer initialized");

    // Simulate sensor readings
    for i in 0..5 {
        let reading = 20.0 + (i as f32) * 0.5;
        SENSOR_BUFFER.push(reading).unwrap();
        println!("Recorded sensor reading: {:.1}°C", reading);
    }

    let readings = SENSOR_BUFFER.as_slice();
    println!("All readings: {:?}", readings);

    let avg: f32 = readings.iter().sum::<f32>() / readings.len() as f32;
    println!("Average: {:.2}°C", avg);
    println!();
}

fn demo_schedulability_analysis() {
    println!("=== Real-Time Schedulability Analysis ===\n");

    let tasks = [
        RealTimeTask {
            name: "Sensor Read",
            period_ms: 10,
            deadline_ms: 10,
            wcet_ms: 2,
            last_release: 0,
        },
        RealTimeTask {
            name: "Control Loop",
            period_ms: 20,
            deadline_ms: 20,
            wcet_ms: 5,
            last_release: 0,
        },
        RealTimeTask {
            name: "Display Update",
            period_ms: 100,
            deadline_ms: 100,
            wcet_ms: 10,
            last_release: 0,
        },
    ];

    println!("Task Set:");
    for task in &tasks {
        println!(
            "  {} - Period: {}ms, Deadline: {}ms, WCET: {}ms",
            task.name, task.period_ms, task.deadline_ms, task.wcet_ms
        );
    }
    println!();

    println!("Rate Monotonic Analysis:");
    let schedulable = RealTimeTask::is_schedulable_rm(&tasks);
    println!("  Schedulable: {}", schedulable);
    println!();

    println!("Response Time Analysis:");
    for (i, task) in tasks.iter().enumerate() {
        match RealTimeTask::response_time_analysis(i, &tasks) {
            Some(rt) => println!(
                "  {} - Response time: {}ms (deadline: {}ms) ✓",
                task.name, rt, task.deadline_ms
            ),
            None => println!("  {} - Deadline miss! ✗", task.name),
        }
    }
    println!();
}

fn main() {
    println!("Real-Time Programming Patterns in Rust\n");
    println!("======================================\n");

    demo_heapless_vec();
    demo_heapless_string();
    demo_ring_buffer();
    demo_memory_pool();
    demo_spsc_queue();
    demo_priority_queue();
    demo_static_allocation();
    demo_schedulability_analysis();

    println!("=== Summary ===\n");
    println!("Key patterns demonstrated:");
    println!("  • HeaplessVec - Fixed-capacity vector without allocation");
    println!("  • HeaplessString - Fixed-capacity string without allocation");
    println!("  • RingBuffer - FIFO queue with constant-time operations");
    println!("  • MemoryPool - O(1) allocation from fixed pool");
    println!("  • SpscQueue - Lock-free single-producer single-consumer");
    println!("  • PriorityQueue - Task scheduling by priority");
    println!("  • StaticBuffer - Global static allocation");
    println!("  • Schedulability analysis - RM and response time analysis");
    println!();
    println!("For production use, see the `heapless` crate.");
}
