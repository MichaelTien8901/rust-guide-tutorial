//! # Writing Embedded Software Example
//!
//! Demonstrates Rust language features and patterns beneficial for
//! embedded systems development, targeting the STM32F769I-DISCO.
//!
//! This example compiles as standard Rust for CI validation.
//! Embedded-specific code is shown as documentation and patterns.

use std::cell::RefCell;
use std::sync::Mutex;

fn main() {
    println!("=== Writing Embedded Software ===\n");

    demonstrate_ownership_safety();
    demonstrate_typestate_pattern();
    demonstrate_peripheral_singleton();
    demonstrate_critical_section_pattern();
    demonstrate_hal_abstractions();
}

// ---------------------------------------------------------------------------
// Ownership for Peripheral Safety
// ---------------------------------------------------------------------------

/// Simulates how Rust ownership prevents peripheral aliasing.
fn demonstrate_ownership_safety() {
    println!("--- Ownership for Peripheral Safety ---");

    let led = GpioPin::new("PJ13", PinMode::Output);
    println!("  Created LED pin: {}", led.name);

    // Ownership is transferred — original binding is consumed
    let mut led_output = led;
    led_output.set_high();
    led_output.set_low();

    println!("  Ownership model prevents two variables from controlling the same pin");
    println!();
}

// ---------------------------------------------------------------------------
// Type-State Pattern
// ---------------------------------------------------------------------------

/// Demonstrates the type-state pattern for GPIO pin modes.
fn demonstrate_typestate_pattern() {
    println!("--- Type-State Pattern ---");

    // Pin starts unconfigured
    let pin = UnconfiguredPin { name: "PA0" };
    println!("  Pin {} is unconfigured", pin.name);

    // Convert to input — consumes the unconfigured pin
    let input = pin.into_input();
    println!("  Pin {} is now input, reading: {}", input.name, input.read());

    // Convert to output — consumes the input pin
    let mut output = input.into_output();
    println!("  Pin {} is now output", output.name);
    output.write(true);
    println!("  Pin {} set high", output.name);

    println!("  Type-state prevents calling write() on an input pin at compile time");
    println!();
}

/// Simulates how peripherals use the singleton pattern.
fn demonstrate_peripheral_singleton() {
    println!("--- Peripheral Singleton Pattern ---");

    let peripherals = SimulatedPeripherals::take();
    match peripherals {
        Some(p) => println!("  First take(): got peripherals (GPIOA, USART1, TIM2)"),
        None => println!("  First take(): None (already taken)"),
    }

    let peripherals2 = SimulatedPeripherals::take();
    match peripherals2 {
        Some(_) => println!("  Second take(): got peripherals"),
        None => println!("  Second take(): None — prevents aliasing!"),
    }
    println!();
}

/// Demonstrates the critical section pattern for shared state.
fn demonstrate_critical_section_pattern() {
    println!("--- Critical Section Pattern ---");

    // In embedded: static SHARED: Mutex<RefCell<Option<u32>>>
    // Here we simulate with std::sync::Mutex
    let shared: Mutex<RefCell<Option<u32>>> = Mutex::new(RefCell::new(None));

    // "Main thread" writes
    {
        let lock = shared.lock().unwrap();
        *lock.borrow_mut() = Some(42);
        println!("  Main wrote: 42");
    }

    // "Interrupt handler" reads
    {
        let lock = shared.lock().unwrap();
        let value = *lock.borrow();
        println!("  Interrupt reads: {:?}", value);
    }

    println!("  Mutex<RefCell<Option<T>>> ensures safe shared access");
    println!();
}

/// Shows the HAL abstraction layers.
fn demonstrate_hal_abstractions() {
    println!("--- HAL Abstraction Layers ---");
    println!("  PAC (stm32f7):      Raw register access, unsafe, auto-generated");
    println!("  HAL (stm32f7xx-hal): Safe API, implements embedded-hal traits");
    println!("  BSP (project):       Board-specific pin mappings and clock config");
    println!();
    println!("  Clock configuration example (STM32F769):");
    println!("    HSE crystal:  25 MHz");
    println!("    SYSCLK (PLL): 216 MHz");
    println!("    HCLK:         216 MHz");
    println!("    PCLK1 (APB1): 54 MHz");
    println!("    PCLK2 (APB2): 108 MHz");
    println!();
    println!("  STM32F769I-DISCO pin assignments:");
    println!("    User LED green: PJ13");
    println!("    User LED red:   PJ5");
    println!("    User button:    PA0");
    println!("    UART TX (VCP):  PA9");
    println!("    UART RX (VCP):  PB7");
}

// ---------------------------------------------------------------------------
// Simulation types (stand-ins for HAL types)
// ---------------------------------------------------------------------------

#[derive(Debug)]
enum PinMode {
    Input,
    Output,
}

struct GpioPin {
    name: &'static str,
    mode: PinMode,
    state: bool,
}

impl GpioPin {
    fn new(name: &'static str, mode: PinMode) -> Self {
        Self { name, mode, state: false }
    }
    fn set_high(&mut self) {
        self.state = true;
        println!("  {} -> HIGH", self.name);
    }
    fn set_low(&mut self) {
        self.state = false;
        println!("  {} -> LOW", self.name);
    }
}

// Type-state pattern types
struct UnconfiguredPin { name: &'static str }
struct InputPin { name: &'static str }
struct OutputPin { name: &'static str, state: bool }

impl UnconfiguredPin {
    fn into_input(self) -> InputPin {
        InputPin { name: self.name }
    }
}

impl InputPin {
    fn read(&self) -> bool { false }
    fn into_output(self) -> OutputPin {
        OutputPin { name: self.name, state: false }
    }
}

impl OutputPin {
    fn write(&mut self, high: bool) {
        self.state = high;
    }
}

// Singleton simulation
static TAKEN: Mutex<RefCell<bool>> = Mutex::new(RefCell::new(false));

struct SimulatedPeripherals;

impl SimulatedPeripherals {
    fn take() -> Option<Self> {
        let lock = TAKEN.lock().unwrap();
        let mut taken = lock.borrow_mut();
        if *taken {
            None
        } else {
            *taken = true;
            Some(SimulatedPeripherals)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_typestate_transitions() {
        let pin = UnconfiguredPin { name: "PA0" };
        let input = pin.into_input();
        assert!(!input.read());
        let mut output = input.into_output();
        output.write(true);
        assert!(output.state);
    }

    #[test]
    fn test_gpio_pin_state() {
        let mut pin = GpioPin::new("PJ13", PinMode::Output);
        assert!(!pin.state);
        pin.set_high();
        assert!(pin.state);
        pin.set_low();
        assert!(!pin.state);
    }
}
