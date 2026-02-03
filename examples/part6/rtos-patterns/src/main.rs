//! RTOS Patterns and Async Embedded Programming
//!
//! This example demonstrates common RTOS patterns including task management,
//! synchronization primitives, and async embedded programming concepts.
//!
//! Note: This is a conceptual example. Real RTOS development uses
//! frameworks like Embassy or bindings to FreeRTOS, Zephyr, etc.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ============================================================================
// Task Priorities and States (FreeRTOS-style)
// ============================================================================

/// Task priority levels (lower number = higher priority in many RTOS).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TaskPriority {
    Idle = 0,
    Low = 1,
    BelowNormal = 2,
    Normal = 3,
    AboveNormal = 4,
    High = 5,
    RealTime = 6,
}

/// Task states in a typical RTOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskState {
    Ready,     // Ready to run
    Running,   // Currently executing
    Blocked,   // Waiting for resource/event
    Suspended, // Explicitly suspended
    Deleted,   // Marked for deletion
}

/// Task Control Block (TCB) - metadata for a task.
#[derive(Debug)]
pub struct TaskControlBlock {
    pub name: &'static str,
    pub priority: TaskPriority,
    pub state: TaskState,
    pub stack_size: usize,
    pub stack_high_water_mark: usize,
    pub run_count: u64,
}

impl TaskControlBlock {
    pub fn new(name: &'static str, priority: TaskPriority, stack_size: usize) -> Self {
        Self {
            name,
            priority,
            state: TaskState::Ready,
            stack_size,
            stack_high_water_mark: stack_size,
            run_count: 0,
        }
    }
}

// ============================================================================
// Message Queue (FreeRTOS xQueue style)
// ============================================================================

/// A fixed-size message queue for inter-task communication.
pub struct MessageQueue<T, const N: usize> {
    buffer: Mutex<VecDeque<T>>,
    capacity: usize,
}

impl<T, const N: usize> MessageQueue<T, N> {
    pub fn new() -> Self {
        Self {
            buffer: Mutex::new(VecDeque::with_capacity(N)),
            capacity: N,
        }
    }

    /// Send a message (blocks if full - simplified).
    pub fn send(&self, item: T, timeout_ms: u32) -> Result<(), T> {
        let start = Instant::now();
        loop {
            {
                let mut queue = self.buffer.lock().unwrap();
                if queue.len() < self.capacity {
                    queue.push_back(item);
                    return Ok(());
                }
            }
            if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
                // In real RTOS, would return the item
                return Err(item);
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// Receive a message (blocks if empty).
    pub fn receive(&self, timeout_ms: u32) -> Option<T> {
        let start = Instant::now();
        loop {
            {
                let mut queue = self.buffer.lock().unwrap();
                if let Some(item) = queue.pop_front() {
                    return Some(item);
                }
            }
            if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
                return None;
            }
            thread::sleep(Duration::from_millis(1));
        }
    }

    /// Check number of messages waiting.
    pub fn messages_waiting(&self) -> usize {
        self.buffer.lock().unwrap().len()
    }

    /// Check available space.
    pub fn spaces_available(&self) -> usize {
        self.capacity - self.buffer.lock().unwrap().len()
    }
}

impl<T, const N: usize> Default for MessageQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Semaphore (Counting and Binary)
// ============================================================================

/// A counting semaphore for resource management.
pub struct Semaphore {
    count: AtomicUsize,
    max_count: usize,
}

impl Semaphore {
    /// Create a counting semaphore.
    pub fn new(initial_count: usize, max_count: usize) -> Self {
        Self {
            count: AtomicUsize::new(initial_count),
            max_count,
        }
    }

    /// Create a binary semaphore (mutex-like).
    pub fn binary(initial: bool) -> Self {
        Self::new(if initial { 1 } else { 0 }, 1)
    }

    /// Take (acquire) the semaphore.
    pub fn take(&self, timeout_ms: u32) -> bool {
        let start = Instant::now();
        loop {
            let current = self.count.load(Ordering::Acquire);
            if current > 0 {
                if self
                    .count
                    .compare_exchange(current, current - 1, Ordering::AcqRel, Ordering::Relaxed)
                    .is_ok()
                {
                    return true;
                }
            }
            if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
                return false;
            }
            thread::yield_now();
        }
    }

    /// Give (release) the semaphore.
    pub fn give(&self) -> bool {
        loop {
            let current = self.count.load(Ordering::Acquire);
            if current >= self.max_count {
                return false; // Already at max
            }
            if self
                .count
                .compare_exchange(current, current + 1, Ordering::AcqRel, Ordering::Relaxed)
                .is_ok()
            {
                return true;
            }
        }
    }

    /// Get current count.
    pub fn count(&self) -> usize {
        self.count.load(Ordering::Relaxed)
    }
}

// ============================================================================
// Event Flags (Event Groups)
// ============================================================================

/// Event flags for task synchronization.
pub struct EventFlags {
    flags: AtomicU32,
}

impl EventFlags {
    pub fn new() -> Self {
        Self {
            flags: AtomicU32::new(0),
        }
    }

    /// Set specific flags.
    pub fn set(&self, bits: u32) {
        self.flags.fetch_or(bits, Ordering::Release);
    }

    /// Clear specific flags.
    pub fn clear(&self, bits: u32) {
        self.flags.fetch_and(!bits, Ordering::Release);
    }

    /// Wait for any of the specified flags.
    pub fn wait_any(&self, bits: u32, timeout_ms: u32, clear_on_exit: bool) -> Option<u32> {
        let start = Instant::now();
        loop {
            let current = self.flags.load(Ordering::Acquire);
            if (current & bits) != 0 {
                if clear_on_exit {
                    self.flags.fetch_and(!(current & bits), Ordering::Release);
                }
                return Some(current & bits);
            }
            if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
                return None;
            }
            thread::yield_now();
        }
    }

    /// Wait for all of the specified flags.
    pub fn wait_all(&self, bits: u32, timeout_ms: u32, clear_on_exit: bool) -> bool {
        let start = Instant::now();
        loop {
            let current = self.flags.load(Ordering::Acquire);
            if (current & bits) == bits {
                if clear_on_exit {
                    self.flags.fetch_and(!bits, Ordering::Release);
                }
                return true;
            }
            if start.elapsed() > Duration::from_millis(timeout_ms as u64) {
                return false;
            }
            thread::yield_now();
        }
    }

    /// Get current flags.
    pub fn get(&self) -> u32 {
        self.flags.load(Ordering::Relaxed)
    }
}

impl Default for EventFlags {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Software Timer Service
// ============================================================================

/// Timer callback type.
pub type TimerCallback = Box<dyn Fn() + Send + 'static>;

/// Timer mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimerMode {
    OneShot,
    Periodic,
}

/// Software timer (simplified).
pub struct SoftwareTimer {
    name: &'static str,
    period_ms: u32,
    mode: TimerMode,
    active: AtomicBool,
    callback: Mutex<Option<TimerCallback>>,
}

impl SoftwareTimer {
    pub fn new(name: &'static str, period_ms: u32, mode: TimerMode) -> Self {
        Self {
            name,
            period_ms,
            mode,
            active: AtomicBool::new(false),
            callback: Mutex::new(None),
        }
    }

    pub fn set_callback<F: Fn() + Send + 'static>(&self, callback: F) {
        *self.callback.lock().unwrap() = Some(Box::new(callback));
    }

    pub fn start(&self) {
        self.active.store(true, Ordering::Release);
        println!(
            "Timer '{}' started ({}ms, {:?})",
            self.name, self.period_ms, self.mode
        );
    }

    pub fn stop(&self) {
        self.active.store(false, Ordering::Release);
        println!("Timer '{}' stopped", self.name);
    }

    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Acquire)
    }

    /// Simulate timer tick (would be called by timer ISR).
    pub fn tick(&self) {
        if self.is_active() {
            if let Some(callback) = self.callback.lock().unwrap().as_ref() {
                callback();
            }
            if self.mode == TimerMode::OneShot {
                self.stop();
            }
        }
    }
}

// ============================================================================
// Embassy-style Async Patterns (Conceptual)
// ============================================================================

/// Simulated async task spawner (Embassy-style).
pub struct Executor {
    tasks: RefCell<Vec<Box<dyn FnMut() -> bool>>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            tasks: RefCell::new(Vec::new()),
        }
    }

    /// Spawn a task (simplified - real Embassy uses proper futures).
    pub fn spawn<F: FnMut() -> bool + 'static>(&self, task: F) {
        self.tasks.borrow_mut().push(Box::new(task));
    }

    /// Run the executor (simplified polling).
    pub fn run(&self, iterations: usize) {
        for i in 0..iterations {
            println!("  Executor tick {}", i);
            let mut tasks = self.tasks.borrow_mut();
            tasks.retain_mut(|task| task()); // Keep tasks that return true
            if tasks.is_empty() {
                println!("  All tasks complete");
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Channel (Embassy-style async channel)
// ============================================================================

/// A simple channel for async communication.
pub struct Channel<T, const N: usize> {
    queue: Mutex<VecDeque<T>>,
    sender_waker: AtomicBool,
    receiver_waker: AtomicBool,
}

impl<T, const N: usize> Channel<T, N> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::with_capacity(N)),
            sender_waker: AtomicBool::new(false),
            receiver_waker: AtomicBool::new(false),
        }
    }

    /// Try to send (non-blocking).
    pub fn try_send(&self, value: T) -> Result<(), T> {
        let mut queue = self.queue.lock().unwrap();
        if queue.len() >= N {
            return Err(value);
        }
        queue.push_back(value);
        self.receiver_waker.store(true, Ordering::Release);
        Ok(())
    }

    /// Try to receive (non-blocking).
    pub fn try_recv(&self) -> Option<T> {
        let mut queue = self.queue.lock().unwrap();
        let result = queue.pop_front();
        if result.is_some() {
            self.sender_waker.store(true, Ordering::Release);
        }
        result
    }

    /// Check if there are pending items.
    pub fn is_empty(&self) -> bool {
        self.queue.lock().unwrap().is_empty()
    }

    /// Check if channel is full.
    pub fn is_full(&self) -> bool {
        self.queue.lock().unwrap().len() >= N
    }
}

impl<T, const N: usize> Default for Channel<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Signal (Embassy-style)
// ============================================================================

/// A signal for task notification.
pub struct Signal<T: Copy> {
    value: Mutex<Option<T>>,
    signaled: AtomicBool,
}

impl<T: Copy> Signal<T> {
    pub fn new() -> Self {
        Self {
            value: Mutex::new(None),
            signaled: AtomicBool::new(false),
        }
    }

    /// Signal with a value.
    pub fn signal(&self, value: T) {
        *self.value.lock().unwrap() = Some(value);
        self.signaled.store(true, Ordering::Release);
    }

    /// Wait for signal (blocking, simplified).
    pub fn wait(&self) -> T {
        loop {
            if self.signaled.swap(false, Ordering::AcqRel) {
                return self.value.lock().unwrap().take().unwrap();
            }
            thread::yield_now();
        }
    }

    /// Try to get signaled value.
    pub fn try_take(&self) -> Option<T> {
        if self.signaled.swap(false, Ordering::AcqRel) {
            self.value.lock().unwrap().take()
        } else {
            None
        }
    }

    /// Check if signaled.
    pub fn is_signaled(&self) -> bool {
        self.signaled.load(Ordering::Acquire)
    }
}

impl<T: Copy> Default for Signal<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Mutex with Priority Inheritance (Conceptual)
// ============================================================================

/// A mutex that tracks owner priority for priority inheritance.
pub struct PriorityMutex<T> {
    data: Mutex<T>,
    owner_priority: AtomicUsize,
    locked: AtomicBool,
}

impl<T> PriorityMutex<T> {
    pub fn new(data: T) -> Self {
        Self {
            data: Mutex::new(data),
            owner_priority: AtomicUsize::new(0),
            locked: AtomicBool::new(false),
        }
    }

    /// Lock with priority (for priority inheritance).
    pub fn lock(&self, task_priority: TaskPriority) -> PriorityMutexGuard<'_, T> {
        // In a real RTOS, this would:
        // 1. If mutex is held by lower-priority task, boost that task's priority
        // 2. Block current task until mutex is available
        // 3. Record current task as owner

        while self
            .locked
            .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            // Check for priority inversion
            let owner_prio = self.owner_priority.load(Ordering::Acquire);
            if (task_priority as usize) > owner_prio {
                println!(
                    "    [Priority inheritance: boosting owner from {} to {}]",
                    owner_prio, task_priority as usize
                );
            }
            thread::yield_now();
        }

        self.owner_priority
            .store(task_priority as usize, Ordering::Release);

        PriorityMutexGuard {
            mutex: self,
            guard: self.data.lock().unwrap(),
        }
    }
}

pub struct PriorityMutexGuard<'a, T> {
    mutex: &'a PriorityMutex<T>,
    guard: std::sync::MutexGuard<'a, T>,
}

impl<T> std::ops::Deref for PriorityMutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> std::ops::DerefMut for PriorityMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<T> Drop for PriorityMutexGuard<'_, T> {
    fn drop(&mut self) {
        self.mutex.owner_priority.store(0, Ordering::Release);
        self.mutex.locked.store(false, Ordering::Release);
    }
}

// ============================================================================
// Resource Pool Pattern
// ============================================================================

/// A pool of reusable resources.
pub struct ResourcePool<T, const N: usize> {
    resources: Mutex<Vec<T>>,
    available: Semaphore,
}

impl<T, const N: usize> ResourcePool<T, N> {
    pub fn new<F: Fn(usize) -> T>(init: F) -> Self {
        let resources: Vec<T> = (0..N).map(init).collect();
        Self {
            resources: Mutex::new(resources),
            available: Semaphore::new(N, N),
        }
    }

    /// Acquire a resource.
    pub fn acquire(&self, timeout_ms: u32) -> Option<PooledResource<'_, T, N>> {
        if self.available.take(timeout_ms) {
            let resource = self.resources.lock().unwrap().pop();
            resource.map(|r| PooledResource {
                pool: self,
                resource: Some(r),
            })
        } else {
            None
        }
    }

    fn release(&self, resource: T) {
        self.resources.lock().unwrap().push(resource);
        self.available.give();
    }
}

pub struct PooledResource<'a, T, const N: usize> {
    pool: &'a ResourcePool<T, N>,
    resource: Option<T>,
}

impl<T, const N: usize> std::ops::Deref for PooledResource<'_, T, N> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.resource.as_ref().unwrap()
    }
}

impl<T, const N: usize> std::ops::DerefMut for PooledResource<'_, T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.resource.as_mut().unwrap()
    }
}

impl<T, const N: usize> Drop for PooledResource<'_, T, N> {
    fn drop(&mut self) {
        if let Some(resource) = self.resource.take() {
            self.pool.release(resource);
        }
    }
}

// ============================================================================
// Demonstration Functions
// ============================================================================

fn demo_task_management() {
    println!("=== Task Management Demo ===\n");

    let tasks = vec![
        TaskControlBlock::new("SensorTask", TaskPriority::High, 1024),
        TaskControlBlock::new("ControlTask", TaskPriority::RealTime, 2048),
        TaskControlBlock::new("DisplayTask", TaskPriority::Normal, 512),
        TaskControlBlock::new("LoggingTask", TaskPriority::Low, 256),
    ];

    println!("Task Control Blocks:");
    for task in &tasks {
        println!(
            "  {} - Priority: {:?}, Stack: {} bytes, State: {:?}",
            task.name, task.priority, task.stack_size, task.state
        );
    }
    println!();
}

fn demo_message_queue() {
    println!("=== Message Queue Demo ===\n");

    #[derive(Debug)]
    struct SensorReading {
        sensor_id: u8,
        value: f32,
    }

    let queue: Arc<MessageQueue<SensorReading, 8>> = Arc::new(MessageQueue::new());
    let queue_producer = Arc::clone(&queue);
    let queue_consumer = Arc::clone(&queue);

    // Producer task
    let producer = thread::spawn(move || {
        for i in 0..5 {
            let reading = SensorReading {
                sensor_id: 1,
                value: 20.0 + i as f32,
            };
            println!("  Producer: Sending {:?}", reading);
            queue_producer.send(reading, 100).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Consumer task
    let consumer = thread::spawn(move || {
        for _ in 0..5 {
            if let Some(reading) = queue_consumer.receive(200) {
                println!("  Consumer: Received {:?}", reading);
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    println!();
}

fn demo_semaphore() {
    println!("=== Semaphore Demo ===\n");

    // Counting semaphore for resource pool
    let sem = Arc::new(Semaphore::new(3, 3)); // 3 resources available

    println!("Resource pool with 3 slots:");

    let mut handles = vec![];
    for i in 0..5 {
        let sem = Arc::clone(&sem);
        handles.push(thread::spawn(move || {
            println!("  Task {} waiting for resource...", i);
            if sem.take(100) {
                println!(
                    "  Task {} acquired resource (available: {})",
                    i,
                    sem.count()
                );
                thread::sleep(Duration::from_millis(50));
                sem.give();
                println!("  Task {} released resource", i);
            } else {
                println!("  Task {} timed out!", i);
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}

fn demo_event_flags() {
    println!("=== Event Flags Demo ===\n");

    const FLAG_SENSOR_READY: u32 = 0x01;
    const FLAG_BUTTON_PRESSED: u32 = 0x02;
    const FLAG_TIMER_EXPIRED: u32 = 0x04;

    let flags = Arc::new(EventFlags::new());

    // Event producer
    let flags_producer = Arc::clone(&flags);
    let producer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        println!("  Setting FLAG_SENSOR_READY");
        flags_producer.set(FLAG_SENSOR_READY);

        thread::sleep(Duration::from_millis(20));
        println!("  Setting FLAG_BUTTON_PRESSED");
        flags_producer.set(FLAG_BUTTON_PRESSED);

        thread::sleep(Duration::from_millis(20));
        println!("  Setting FLAG_TIMER_EXPIRED");
        flags_producer.set(FLAG_TIMER_EXPIRED);
    });

    // Event consumer (waits for any)
    let flags_consumer = Arc::clone(&flags);
    let consumer = thread::spawn(move || {
        // Wait for sensor or button
        if let Some(triggered) =
            flags_consumer.wait_any(FLAG_SENSOR_READY | FLAG_BUTTON_PRESSED, 100, true)
        {
            println!("  Event triggered: 0x{:02x}", triggered);
        }

        // Wait for timer
        if let Some(triggered) = flags_consumer.wait_any(FLAG_TIMER_EXPIRED, 100, true) {
            println!("  Timer event triggered: 0x{:02x}", triggered);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
    println!();
}

fn demo_software_timer() {
    println!("=== Software Timer Demo ===\n");

    let counter = Arc::new(AtomicU32::new(0));
    let counter_clone = Arc::clone(&counter);

    let timer = SoftwareTimer::new("HeartbeatTimer", 100, TimerMode::Periodic);
    timer.set_callback(move || {
        let count = counter_clone.fetch_add(1, Ordering::Relaxed) + 1;
        println!("  Timer callback executed (count: {})", count);
    });

    timer.start();

    // Simulate timer service running
    for _ in 0..3 {
        thread::sleep(Duration::from_millis(100));
        timer.tick();
    }

    timer.stop();
    println!("  Final count: {}", counter.load(Ordering::Relaxed));
    println!();
}

fn demo_embassy_style() {
    println!("=== Embassy-style Async Demo ===\n");

    let executor = Executor::new();
    let counter = Arc::new(AtomicU32::new(0));

    // Spawn task 1: Blinky
    let counter1 = Arc::clone(&counter);
    let mut blink_state = false;
    executor.spawn(move || {
        blink_state = !blink_state;
        println!("  Blinky: LED {}", if blink_state { "ON" } else { "OFF" });
        counter1.fetch_add(1, Ordering::Relaxed) < 3
    });

    // Spawn task 2: Sensor polling
    let counter2 = Arc::clone(&counter);
    let mut reading = 0;
    executor.spawn(move || {
        reading += 1;
        println!("  Sensor: Reading {}", reading);
        counter2.fetch_add(1, Ordering::Relaxed) < 4
    });

    println!("Running executor:");
    executor.run(10);
    println!();
}

fn demo_channel() {
    println!("=== Channel Demo ===\n");

    let channel: Arc<Channel<u32, 4>> = Arc::new(Channel::new());

    let tx = Arc::clone(&channel);
    let rx = Arc::clone(&channel);

    // Sender
    let sender = thread::spawn(move || {
        for i in 1..=5 {
            match tx.try_send(i * 10) {
                Ok(()) => println!("  Sent: {}", i * 10),
                Err(v) => println!("  Failed to send: {} (channel full)", v),
            }
            thread::sleep(Duration::from_millis(10));
        }
    });

    // Receiver
    let receiver = thread::spawn(move || {
        thread::sleep(Duration::from_millis(25));
        while let Some(value) = rx.try_recv() {
            println!("  Received: {}", value);
        }
    });

    sender.join().unwrap();
    receiver.join().unwrap();
    println!();
}

fn demo_signal() {
    println!("=== Signal Demo ===\n");

    let signal: Arc<Signal<u32>> = Arc::new(Signal::new());

    let sig_sender = Arc::clone(&signal);
    let sig_receiver = Arc::clone(&signal);

    // Waiter
    let waiter = thread::spawn(move || {
        println!("  Waiter: Checking for signal...");
        for _ in 0..5 {
            if let Some(value) = sig_receiver.try_take() {
                println!("  Waiter: Received signal with value: {}", value);
                break;
            }
            println!("  Waiter: No signal yet, polling...");
            thread::sleep(Duration::from_millis(20));
        }
    });

    // Signaler
    thread::sleep(Duration::from_millis(50));
    println!("  Signaler: Sending signal with value 42");
    sig_sender.signal(42);

    waiter.join().unwrap();
    println!();
}

fn demo_priority_mutex() {
    println!("=== Priority Mutex Demo ===\n");

    let shared_data = Arc::new(PriorityMutex::new(0u32));

    let data1 = Arc::clone(&shared_data);
    let data2 = Arc::clone(&shared_data);

    println!("Low priority task acquires mutex first:");

    // Low priority task
    let low_prio = thread::spawn(move || {
        let mut guard = data1.lock(TaskPriority::Low);
        println!("  Low priority: Holding mutex, value = {}", *guard);
        thread::sleep(Duration::from_millis(50));
        *guard += 1;
        println!("  Low priority: Updated value to {}", *guard);
    });

    thread::sleep(Duration::from_millis(10));

    // High priority task
    let high_prio = thread::spawn(move || {
        println!("  High priority: Attempting to acquire mutex...");
        let mut guard = data2.lock(TaskPriority::High);
        println!("  High priority: Acquired mutex, value = {}", *guard);
        *guard += 10;
        println!("  High priority: Updated value to {}", *guard);
    });

    low_prio.join().unwrap();
    high_prio.join().unwrap();
    println!();
}

fn demo_resource_pool() {
    println!("=== Resource Pool Demo ===\n");

    #[derive(Debug)]
    struct Connection {
        id: usize,
    }

    let pool: Arc<ResourcePool<Connection, 3>> =
        Arc::new(ResourcePool::new(|id| Connection { id }));

    println!("Connection pool with 3 connections:");

    let mut handles = vec![];
    for task_id in 0..4 {
        let pool = Arc::clone(&pool);
        handles.push(thread::spawn(move || {
            println!("  Task {}: Requesting connection...", task_id);
            if let Some(conn) = pool.acquire(100) {
                println!("  Task {}: Got connection {:?}", task_id, *conn);
                thread::sleep(Duration::from_millis(30));
                println!("  Task {}: Releasing connection {:?}", task_id, conn.id);
                // Connection automatically released on drop
            } else {
                println!("  Task {}: Timed out waiting for connection!", task_id);
            }
        }));
        thread::sleep(Duration::from_millis(5));
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!();
}

fn main() {
    println!("RTOS Patterns in Rust\n");
    println!("=====================\n");

    demo_task_management();
    demo_message_queue();
    demo_semaphore();
    demo_event_flags();
    demo_software_timer();
    demo_embassy_style();
    demo_channel();
    demo_signal();
    demo_priority_mutex();
    demo_resource_pool();

    println!("=== Summary ===\n");
    println!("FreeRTOS-style patterns demonstrated:");
    println!("  • Task Control Blocks (TCB)");
    println!("  • Message Queues (xQueue)");
    println!("  • Semaphores (counting and binary)");
    println!("  • Event Flags (Event Groups)");
    println!("  • Software Timers");
    println!();
    println!("Embassy-style async patterns:");
    println!("  • Executor and task spawning");
    println!("  • Channels for async communication");
    println!("  • Signals for notifications");
    println!();
    println!("Advanced patterns:");
    println!("  • Priority Mutex (priority inheritance)");
    println!("  • Resource Pools");
    println!();
    println!("For real RTOS development, see Embassy or FreeRTOS bindings.");
}
