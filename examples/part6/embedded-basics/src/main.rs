//! Embedded Programming Concepts (Simulated)
//!
//! Demonstrates embedded Rust concepts using simulation.
//!
//! # Embedded HAL Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                  Your Application                       │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │              embedded-hal Traits                        │
//!     │   (InputPin, OutputPin, SPI, I2C, UART, PWM, etc.)      │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │          Hardware Abstraction Layer (HAL)               │
//!     │     (stm32f4xx-hal, rp2040-hal, nrf52-hal, etc.)        │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │              Peripheral Access Crate (PAC)              │
//!     │          (Generated from SVD files)                     │
//!     └─────────────────────────────────────────────────────────┘
//!                            │
//!                            ▼
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                    Hardware                             │
//!     └─────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example simulates hardware for learning purposes.

use std::cell::RefCell;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::time::{Duration, Instant};

fn main() {
    println!("=== Embedded Programming Concepts ===\n");

    println!("--- GPIO Simulation ---");
    gpio_example();

    println!("\n--- Timer/Counter Simulation ---");
    timer_example();

    println!("\n--- Interrupt Handling Patterns ---");
    interrupt_patterns();

    println!("\n--- State Machine for Hardware ---");
    hardware_state_machine();

    println!("\n--- Driver Pattern ---");
    driver_pattern();

    println!("\n--- Memory-Mapped I/O Simulation ---");
    mmio_simulation();
}

// ============================================
// GPIO (General Purpose I/O) Simulation
// ============================================

/// Simulated GPIO pin states
#[derive(Debug, Clone, Copy, PartialEq)]
enum PinState {
    Low,
    High,
}

/// Simulated GPIO pin
struct GpioPin {
    number: u8,
    state: RefCell<PinState>,
    is_output: bool,
}

impl GpioPin {
    fn new(number: u8) -> Self {
        GpioPin {
            number,
            state: RefCell::new(PinState::Low),
            is_output: false,
        }
    }

    fn into_output(mut self) -> Self {
        self.is_output = true;
        self
    }

    fn into_input(mut self) -> Self {
        self.is_output = false;
        self
    }

    fn set_high(&self) {
        if self.is_output {
            *self.state.borrow_mut() = PinState::High;
        }
    }

    fn set_low(&self) {
        if self.is_output {
            *self.state.borrow_mut() = PinState::Low;
        }
    }

    fn is_high(&self) -> bool {
        *self.state.borrow() == PinState::High
    }

    fn toggle(&self) {
        if self.is_output {
            let mut state = self.state.borrow_mut();
            *state = match *state {
                PinState::Low => PinState::High,
                PinState::High => PinState::Low,
            };
        }
    }
}

fn gpio_example() {
    // Create and configure LED pin
    let led = GpioPin::new(13).into_output();

    println!("  LED pin {} configured as output", led.number);
    println!("  Initial state: {:?}", *led.state.borrow());

    led.set_high();
    println!("  After set_high: {:?}", *led.state.borrow());

    led.toggle();
    println!("  After toggle: {:?}", *led.state.borrow());

    led.set_low();
    println!("  After set_low: {:?}", *led.state.borrow());

    // Blink pattern
    println!("\n  Simulated blink pattern:");
    for i in 0..5 {
        led.toggle();
        let state = if led.is_high() { "ON" } else { "OFF" };
        println!("    Step {}: LED {}", i + 1, state);
    }
}

// ============================================
// Timer/Counter Simulation
// ============================================

/// Simulated hardware timer
struct Timer {
    counter: AtomicU32,
    prescaler: u32,
    auto_reload: u32,
    running: AtomicBool,
}

impl Timer {
    fn new() -> Self {
        Timer {
            counter: AtomicU32::new(0),
            prescaler: 1,
            auto_reload: 1000,
            running: AtomicBool::new(false),
        }
    }

    fn set_prescaler(&mut self, prescaler: u32) {
        self.prescaler = prescaler;
    }

    fn set_auto_reload(&mut self, arr: u32) {
        self.auto_reload = arr;
    }

    fn start(&self) {
        self.running.store(true, Ordering::SeqCst);
        self.counter.store(0, Ordering::SeqCst);
    }

    fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
    }

    fn tick(&self) -> bool {
        if !self.running.load(Ordering::SeqCst) {
            return false;
        }

        let current = self.counter.fetch_add(1, Ordering::SeqCst);
        if current >= self.auto_reload {
            self.counter.store(0, Ordering::SeqCst);
            true // Overflow/update event
        } else {
            false
        }
    }

    fn get_counter(&self) -> u32 {
        self.counter.load(Ordering::SeqCst)
    }
}

fn timer_example() {
    let mut timer = Timer::new();
    timer.set_prescaler(8); // Divide clock by 8
    timer.set_auto_reload(10); // Count to 10

    println!(
        "  Timer configured: prescaler={}, arr={}",
        timer.prescaler, timer.auto_reload
    );

    timer.start();

    let mut overflows = 0;
    for i in 0..25 {
        if timer.tick() {
            overflows += 1;
            println!("  Tick {}: Overflow! (count={})", i, overflows);
        }
    }

    timer.stop();
    println!("  Timer stopped. Final counter: {}", timer.get_counter());
}

// ============================================
// Interrupt Handling Patterns
// ============================================

/// Simulated interrupt controller
struct InterruptController {
    pending: AtomicU32,
    enabled: AtomicU32,
}

impl InterruptController {
    const TIMER_IRQ: u32 = 1 << 0;
    const UART_IRQ: u32 = 1 << 1;
    const GPIO_IRQ: u32 = 1 << 2;

    fn new() -> Self {
        InterruptController {
            pending: AtomicU32::new(0),
            enabled: AtomicU32::new(0),
        }
    }

    fn enable(&self, irq: u32) {
        self.enabled.fetch_or(irq, Ordering::SeqCst);
    }

    fn disable(&self, irq: u32) {
        self.enabled.fetch_and(!irq, Ordering::SeqCst);
    }

    fn set_pending(&self, irq: u32) {
        self.pending.fetch_or(irq, Ordering::SeqCst);
    }

    fn clear_pending(&self, irq: u32) {
        self.pending.fetch_and(!irq, Ordering::SeqCst);
    }

    fn is_pending_and_enabled(&self, irq: u32) -> bool {
        let pending = self.pending.load(Ordering::SeqCst);
        let enabled = self.enabled.load(Ordering::SeqCst);
        (pending & enabled & irq) != 0
    }
}

fn interrupt_patterns() {
    let nvic = InterruptController::new();

    // Enable some interrupts
    nvic.enable(InterruptController::TIMER_IRQ);
    nvic.enable(InterruptController::UART_IRQ);

    println!("  Enabled: TIMER_IRQ, UART_IRQ");

    // Simulate interrupt
    nvic.set_pending(InterruptController::TIMER_IRQ);
    nvic.set_pending(InterruptController::GPIO_IRQ); // Not enabled

    // Check and handle
    if nvic.is_pending_and_enabled(InterruptController::TIMER_IRQ) {
        println!("  Handling TIMER_IRQ...");
        nvic.clear_pending(InterruptController::TIMER_IRQ);
    }

    if nvic.is_pending_and_enabled(InterruptController::GPIO_IRQ) {
        println!("  Handling GPIO_IRQ...");
    } else {
        println!("  GPIO_IRQ pending but not enabled");
    }

    // Critical section pattern
    println!("\n  Critical section pattern:");

    fn critical_section<F, R>(f: F) -> R
    where
        F: FnOnce() -> R,
    {
        // In real code: disable interrupts
        println!("    [Interrupts disabled]");
        let result = f();
        // In real code: enable interrupts
        println!("    [Interrupts enabled]");
        result
    }

    let value = critical_section(|| {
        println!("    Modifying shared state...");
        42
    });
    println!("  Critical section returned: {}", value);
}

// ============================================
// Hardware State Machine
// ============================================

#[derive(Debug, Clone, Copy, PartialEq)]
enum DeviceState {
    Idle,
    Initializing,
    Ready,
    Processing,
    Error,
}

struct Device {
    state: DeviceState,
    error_count: u32,
}

impl Device {
    fn new() -> Self {
        Device {
            state: DeviceState::Idle,
            error_count: 0,
        }
    }

    fn initialize(&mut self) -> Result<(), &'static str> {
        match self.state {
            DeviceState::Idle => {
                println!("    Initializing device...");
                self.state = DeviceState::Initializing;
                // Simulate initialization
                self.state = DeviceState::Ready;
                Ok(())
            }
            _ => Err("Cannot initialize from current state"),
        }
    }

    fn start_processing(&mut self) -> Result<(), &'static str> {
        match self.state {
            DeviceState::Ready => {
                println!("    Starting processing...");
                self.state = DeviceState::Processing;
                Ok(())
            }
            _ => Err("Device not ready"),
        }
    }

    fn complete(&mut self) {
        if self.state == DeviceState::Processing {
            println!("    Processing complete");
            self.state = DeviceState::Ready;
        }
    }

    fn report_error(&mut self) {
        self.error_count += 1;
        self.state = DeviceState::Error;
        println!("    Error reported (count: {})", self.error_count);
    }

    fn reset(&mut self) {
        println!("    Resetting device...");
        self.state = DeviceState::Idle;
    }
}

fn hardware_state_machine() {
    let mut device = Device::new();
    println!("  Initial state: {:?}", device.state);

    device.initialize().unwrap();
    println!("  After init: {:?}", device.state);

    device.start_processing().unwrap();
    println!("  After start: {:?}", device.state);

    device.complete();
    println!("  After complete: {:?}", device.state);

    device.report_error();
    println!("  After error: {:?}", device.state);

    device.reset();
    println!("  After reset: {:?}", device.state);
}

// ============================================
// Driver Pattern
// ============================================

/// Generic sensor driver interface
trait Sensor {
    type Reading;
    type Error;

    fn read(&mut self) -> Result<Self::Reading, Self::Error>;
    fn calibrate(&mut self) -> Result<(), Self::Error>;
}

/// Simulated temperature sensor
struct TemperatureSensor {
    last_reading: f32,
    calibration_offset: f32,
}

impl TemperatureSensor {
    fn new() -> Self {
        TemperatureSensor {
            last_reading: 0.0,
            calibration_offset: 0.0,
        }
    }
}

impl Sensor for TemperatureSensor {
    type Reading = f32;
    type Error = &'static str;

    fn read(&mut self) -> Result<f32, &'static str> {
        // Simulate reading (would read from hardware register)
        self.last_reading = 25.0 + self.calibration_offset;
        Ok(self.last_reading)
    }

    fn calibrate(&mut self) -> Result<(), &'static str> {
        // Simulate calibration
        self.calibration_offset = 0.5;
        Ok(())
    }
}

fn driver_pattern() {
    let mut temp_sensor = TemperatureSensor::new();

    println!("  Reading temperature sensor...");
    let temp = temp_sensor.read().unwrap();
    println!("  Temperature: {:.1}°C", temp);

    println!("  Calibrating...");
    temp_sensor.calibrate().unwrap();

    let temp = temp_sensor.read().unwrap();
    println!("  Temperature (calibrated): {:.1}°C", temp);

    // Generic function using trait
    fn read_sensor<S: Sensor>(sensor: &mut S) -> Result<S::Reading, S::Error> {
        sensor.read()
    }

    let reading = read_sensor(&mut temp_sensor).unwrap();
    println!("  Generic read: {:.1}", reading);
}

// ============================================
// Memory-Mapped I/O Simulation
// ============================================

/// Simulated peripheral registers
#[repr(C)]
struct PeripheralRegisters {
    control: AtomicU32,
    status: AtomicU32,
    data: AtomicU32,
    config: AtomicU32,
}

impl PeripheralRegisters {
    // Control register bits
    const CTRL_ENABLE: u32 = 1 << 0;
    const CTRL_START: u32 = 1 << 1;
    const CTRL_RESET: u32 = 1 << 7;

    // Status register bits
    const STATUS_READY: u32 = 1 << 0;
    const STATUS_BUSY: u32 = 1 << 1;
    const STATUS_ERROR: u32 = 1 << 7;

    fn new() -> Self {
        PeripheralRegisters {
            control: AtomicU32::new(0),
            status: AtomicU32::new(Self::STATUS_READY),
            data: AtomicU32::new(0),
            config: AtomicU32::new(0),
        }
    }

    fn write_control(&self, value: u32) {
        self.control.store(value, Ordering::SeqCst);
    }

    fn read_status(&self) -> u32 {
        self.status.load(Ordering::SeqCst)
    }

    fn write_data(&self, value: u32) {
        self.data.store(value, Ordering::SeqCst);
    }

    fn read_data(&self) -> u32 {
        self.data.load(Ordering::SeqCst)
    }
}

fn mmio_simulation() {
    let regs = PeripheralRegisters::new();

    println!("  Initial status: 0b{:08b}", regs.read_status());

    // Enable peripheral
    regs.write_control(PeripheralRegisters::CTRL_ENABLE);
    println!("  Enabled peripheral");

    // Write data
    regs.write_data(0x12345678);
    println!("  Wrote data: 0x{:08X}", regs.read_data());

    // Start operation
    regs.write_control(PeripheralRegisters::CTRL_ENABLE | PeripheralRegisters::CTRL_START);
    println!("  Started operation");

    // Check status
    let status = regs.read_status();
    if (status & PeripheralRegisters::STATUS_READY) != 0 {
        println!("  Peripheral ready");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_toggle() {
        let led = GpioPin::new(1).into_output();

        assert!(!led.is_high());

        led.toggle();
        assert!(led.is_high());

        led.toggle();
        assert!(!led.is_high());
    }

    #[test]
    fn test_timer_overflow() {
        let mut timer = Timer::new();
        timer.set_auto_reload(5);
        timer.start();

        let mut overflows = 0;
        for _ in 0..12 {
            if timer.tick() {
                overflows += 1;
            }
        }

        assert_eq!(overflows, 2);
    }

    #[test]
    fn test_device_state_machine() {
        let mut device = Device::new();

        assert_eq!(device.state, DeviceState::Idle);

        device.initialize().unwrap();
        assert_eq!(device.state, DeviceState::Ready);

        device.start_processing().unwrap();
        assert_eq!(device.state, DeviceState::Processing);
    }

    #[test]
    fn test_sensor_driver() {
        let mut sensor = TemperatureSensor::new();

        let reading = sensor.read().unwrap();
        assert!((reading - 25.0).abs() < 0.01);

        sensor.calibrate().unwrap();
        let reading = sensor.read().unwrap();
        assert!((reading - 25.5).abs() < 0.01);
    }
}
