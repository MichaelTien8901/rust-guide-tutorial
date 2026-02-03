//! Embedded HAL Example
//!
//! Demonstrates the embedded-hal trait patterns for hardware abstraction.
//!
//! # Embedded HAL Architecture
//! ```text
//!     ┌─────────────────────────────────────────────────────────────┐
//!     │                  Embedded HAL Stack                         │
//!     ├─────────────────────────────────────────────────────────────┤
//!     │                                                             │
//!     │  ┌─────────────────────────────────────────────────────┐    │
//!     │  │              Application / Driver                    │    │
//!     │  │         (Generic over HAL traits)                    │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │              embedded-hal Traits                     │    │
//!     │  │    InputPin, OutputPin, SpiBus, I2c, Delay, etc.    │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │            HAL Implementation                        │    │
//!     │  │    (stm32f4xx-hal, nrf52-hal, rp2040-hal, etc.)     │    │
//!     │  └───────────────────────┬─────────────────────────────┘    │
//!     │                          │                                  │
//!     │  ┌───────────────────────▼─────────────────────────────┐    │
//!     │  │              Hardware Registers                      │    │
//!     │  └─────────────────────────────────────────────────────┘    │
//!     │                                                             │
//!     └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! Note: This example simulates embedded-hal traits in a std environment.

use std::cell::RefCell;
use std::time::{Duration, Instant};

fn main() {
    println!("=== Embedded HAL Concepts ===\n");

    println!("--- GPIO Traits ---");
    gpio_traits();

    println!("\n--- SPI Traits ---");
    spi_traits();

    println!("\n--- I2C Traits ---");
    i2c_traits();

    println!("\n--- Delay Traits ---");
    delay_traits();

    println!("\n--- PWM Traits ---");
    pwm_traits();

    println!("\n--- Generic Driver Example ---");
    generic_driver_example();
}

// ============================================
// GPIO Traits
// ============================================

/// Error type for GPIO operations
#[derive(Debug, Clone, Copy)]
struct GpioError;

/// Simulated InputPin trait (from embedded-hal)
trait InputPin {
    type Error;
    fn is_high(&mut self) -> Result<bool, Self::Error>;
    fn is_low(&mut self) -> Result<bool, Self::Error>;
}

/// Simulated OutputPin trait (from embedded-hal)
trait OutputPin {
    type Error;
    fn set_high(&mut self) -> Result<(), Self::Error>;
    fn set_low(&mut self) -> Result<(), Self::Error>;
}

/// Simulated StatefulOutputPin trait
trait StatefulOutputPin: OutputPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error>;
    fn is_set_low(&mut self) -> Result<bool, Self::Error>;
}

/// Simulated ToggleableOutputPin trait
trait ToggleableOutputPin: OutputPin {
    fn toggle(&mut self) -> Result<(), Self::Error>;
}

/// Mock GPIO pin for demonstration
struct MockPin {
    name: String,
    state: bool,
}

impl MockPin {
    fn new(name: &str) -> Self {
        MockPin {
            name: name.to_string(),
            state: false,
        }
    }
}

impl InputPin for MockPin {
    type Error = GpioError;

    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state)
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.state)
    }
}

impl OutputPin for MockPin {
    type Error = GpioError;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = true;
        println!("    {} -> HIGH", self.name);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = false;
        println!("    {} -> LOW", self.name);
        Ok(())
    }
}

impl StatefulOutputPin for MockPin {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(self.state)
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(!self.state)
    }
}

impl ToggleableOutputPin for MockPin {
    fn toggle(&mut self) -> Result<(), Self::Error> {
        self.state = !self.state;
        println!(
            "    {} -> {} (toggled)",
            self.name,
            if self.state { "HIGH" } else { "LOW" }
        );
        Ok(())
    }
}

fn gpio_traits() {
    let mut led = MockPin::new("LED");
    let mut button = MockPin::new("BUTTON");

    println!("  OutputPin operations:");
    led.set_high().unwrap();
    led.set_low().unwrap();

    println!("\n  ToggleableOutputPin:");
    led.toggle().unwrap();
    led.toggle().unwrap();

    println!("\n  InputPin operations:");
    button.state = true; // Simulate button press
    println!("    BUTTON is_high: {}", button.is_high().unwrap());
    println!("    BUTTON is_low: {}", button.is_low().unwrap());
}

// ============================================
// SPI Traits
// ============================================

/// SPI error type
#[derive(Debug, Clone, Copy)]
enum SpiError {
    BusError,
    ChipSelectError,
}

/// Simulated SpiBus trait (embedded-hal 1.0)
trait SpiBus {
    type Error;

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error>;
    fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;
    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}

/// Simulated SpiDevice trait (for chip-select management)
trait SpiDevice {
    type Error;
    type Bus: SpiBus;

    fn transaction<R, F>(&mut self, f: F) -> Result<R, Self::Error>
    where
        F: FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as SpiBus>::Error>;
}

/// Mock SPI bus
struct MockSpiBus {
    name: String,
    clock_speed: u32,
}

impl MockSpiBus {
    fn new(name: &str, clock_speed: u32) -> Self {
        MockSpiBus {
            name: name.to_string(),
            clock_speed,
        }
    }
}

impl SpiBus for MockSpiBus {
    type Error = SpiError;

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        println!(
            "    {} transfer: write {} bytes, read {} bytes",
            self.name,
            write.len(),
            read.len()
        );
        // Simulate reading back inverted data
        for (i, byte) in read.iter_mut().enumerate() {
            *byte = if i < write.len() { !write[i] } else { 0xFF };
        }
        Ok(())
    }

    fn transfer_in_place(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        println!("    {} transfer_in_place: {} bytes", self.name, data.len());
        for byte in data.iter_mut() {
            *byte = !*byte;
        }
        Ok(())
    }

    fn read(&mut self, data: &mut [u8]) -> Result<(), Self::Error> {
        println!("    {} read: {} bytes", self.name, data.len());
        data.fill(0xAA);
        Ok(())
    }

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error> {
        println!(
            "    {} write: {:02X?}",
            self.name,
            &data[..data.len().min(8)]
        );
        Ok(())
    }
}

/// Mock SPI device with chip select
struct MockSpiDevice {
    bus: MockSpiBus,
    cs_pin: MockPin,
}

impl MockSpiDevice {
    fn new(bus: MockSpiBus, cs_pin: MockPin) -> Self {
        MockSpiDevice { bus, cs_pin }
    }
}

impl SpiDevice for MockSpiDevice {
    type Error = SpiError;
    type Bus = MockSpiBus;

    fn transaction<R, F>(&mut self, f: F) -> Result<R, Self::Error>
    where
        F: FnOnce(&mut Self::Bus) -> Result<R, <Self::Bus as SpiBus>::Error>,
    {
        // Assert CS (active low)
        self.cs_pin
            .set_low()
            .map_err(|_| SpiError::ChipSelectError)?;

        let result = f(&mut self.bus);

        // Deassert CS
        self.cs_pin
            .set_high()
            .map_err(|_| SpiError::ChipSelectError)?;

        result
    }
}

fn spi_traits() {
    let bus = MockSpiBus::new("SPI1", 1_000_000);
    let cs = MockPin::new("CS");
    let mut device = MockSpiDevice::new(bus, cs);

    println!("  SPI transaction with automatic CS:");
    device
        .transaction(|bus| {
            bus.write(&[0x9F])?; // Read ID command
            let mut id = [0u8; 3];
            bus.read(&mut id)?;
            println!("      Device ID: {:02X?}", id);
            Ok(())
        })
        .unwrap();

    println!("\n  SPI transfer:");
    let bus2 = MockSpiBus::new("SPI2", 2_000_000);
    let mut bus2 = bus2;
    let mut read_buf = [0u8; 4];
    bus2.transfer(&mut read_buf, &[0x01, 0x02, 0x03, 0x04])
        .unwrap();
    println!("    Read back: {:02X?}", read_buf);
}

// ============================================
// I2C Traits
// ============================================

/// I2C error type
#[derive(Debug, Clone, Copy)]
enum I2cError {
    Nack,
    BusError,
    ArbitrationLoss,
}

/// Simulated I2c trait (embedded-hal 1.0)
trait I2c {
    type Error;

    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error>;
    fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), Self::Error>;
    fn write_read(&mut self, address: u8, write: &[u8], read: &mut [u8])
        -> Result<(), Self::Error>;
}

/// Mock I2C bus
struct MockI2c {
    name: String,
    devices: std::collections::HashMap<u8, Vec<u8>>,
}

impl MockI2c {
    fn new(name: &str) -> Self {
        let mut devices = std::collections::HashMap::new();
        // Simulated temperature sensor at 0x48
        devices.insert(0x48, vec![0x00, 0x19, 0x80]); // 25.5°C
                                                      // Simulated EEPROM at 0x50
        devices.insert(0x50, vec![0xDE, 0xAD, 0xBE, 0xEF]);

        MockI2c {
            name: name.to_string(),
            devices,
        }
    }
}

impl I2c for MockI2c {
    type Error = I2cError;

    fn write(&mut self, address: u8, data: &[u8]) -> Result<(), Self::Error> {
        if !self.devices.contains_key(&address) {
            println!(
                "    {} write 0x{:02X}: NACK (no device)",
                self.name, address
            );
            return Err(I2cError::Nack);
        }
        println!("    {} write 0x{:02X}: {:02X?}", self.name, address, data);
        Ok(())
    }

    fn read(&mut self, address: u8, data: &mut [u8]) -> Result<(), Self::Error> {
        if let Some(device_data) = self.devices.get(&address) {
            let len = data.len().min(device_data.len());
            data[..len].copy_from_slice(&device_data[..len]);
            println!("    {} read 0x{:02X}: {:02X?}", self.name, address, data);
            Ok(())
        } else {
            println!("    {} read 0x{:02X}: NACK (no device)", self.name, address);
            Err(I2cError::Nack)
        }
    }

    fn write_read(
        &mut self,
        address: u8,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), Self::Error> {
        println!(
            "    {} write_read 0x{:02X}: write {:02X?}",
            self.name, address, write
        );
        self.read(address, read)
    }
}

fn i2c_traits() {
    let mut i2c = MockI2c::new("I2C1");

    println!("  I2C write:");
    i2c.write(0x48, &[0x00]).unwrap(); // Set pointer register

    println!("\n  I2C read:");
    let mut temp = [0u8; 2];
    i2c.read(0x48, &mut temp).unwrap();

    println!("\n  I2C write_read (combined):");
    let mut data = [0u8; 4];
    i2c.write_read(0x50, &[0x00], &mut data).unwrap();

    println!("\n  I2C to non-existent device:");
    let result = i2c.write(0x99, &[0x00]);
    println!("    Result: {:?}", result);
}

// ============================================
// Delay Traits
// ============================================

/// Simulated DelayNs trait (embedded-hal 1.0)
trait DelayNs {
    fn delay_ns(&mut self, ns: u32);

    fn delay_us(&mut self, us: u32) {
        self.delay_ns(us * 1000);
    }

    fn delay_ms(&mut self, ms: u32) {
        self.delay_ns(ms * 1_000_000);
    }
}

/// Mock delay using std::thread::sleep
struct MockDelay;

impl DelayNs for MockDelay {
    fn delay_ns(&mut self, ns: u32) {
        let duration = Duration::from_nanos(ns as u64);
        std::thread::sleep(duration);
    }
}

/// Blocking delay implementation
struct BlockingDelay {
    start: Option<Instant>,
}

impl BlockingDelay {
    fn new() -> Self {
        BlockingDelay { start: None }
    }
}

impl DelayNs for BlockingDelay {
    fn delay_ns(&mut self, ns: u32) {
        let target = Duration::from_nanos(ns as u64);
        let start = Instant::now();
        while start.elapsed() < target {
            // Busy wait
            core::hint::spin_loop();
        }
    }
}

fn delay_traits() {
    let mut delay = MockDelay;

    println!("  DelayNs operations:");

    let start = Instant::now();
    delay.delay_ms(10);
    println!("    delay_ms(10): {:?} elapsed", start.elapsed());

    let start = Instant::now();
    delay.delay_us(500);
    println!("    delay_us(500): {:?} elapsed", start.elapsed());

    println!("\n  Blocking delay (busy-wait):");
    let mut blocking = BlockingDelay::new();
    let start = Instant::now();
    blocking.delay_us(100);
    println!("    delay_us(100): {:?} elapsed", start.elapsed());
}

// ============================================
// PWM Traits
// ============================================

/// Simulated SetDutyCycle trait
trait SetDutyCycle {
    type Error;

    fn max_duty_cycle(&self) -> u16;
    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error>;

    fn set_duty_cycle_percent(&mut self, percent: u8) -> Result<(), Self::Error> {
        let max = self.max_duty_cycle() as u32;
        let duty = (max * percent as u32 / 100) as u16;
        self.set_duty_cycle(duty)
    }

    fn set_duty_cycle_fully_on(&mut self) -> Result<(), Self::Error> {
        self.set_duty_cycle(self.max_duty_cycle())
    }

    fn set_duty_cycle_fully_off(&mut self) -> Result<(), Self::Error> {
        self.set_duty_cycle(0)
    }
}

/// Mock PWM channel
struct MockPwm {
    name: String,
    duty: u16,
    max_duty: u16,
    frequency: u32,
}

impl MockPwm {
    fn new(name: &str, frequency: u32) -> Self {
        MockPwm {
            name: name.to_string(),
            duty: 0,
            max_duty: 65535,
            frequency,
        }
    }
}

impl SetDutyCycle for MockPwm {
    type Error = ();

    fn max_duty_cycle(&self) -> u16 {
        self.max_duty
    }

    fn set_duty_cycle(&mut self, duty: u16) -> Result<(), Self::Error> {
        self.duty = duty.min(self.max_duty);
        let percent = (self.duty as f32 / self.max_duty as f32) * 100.0;
        println!(
            "    {} duty: {} ({:.1}%) @ {}Hz",
            self.name, self.duty, percent, self.frequency
        );
        Ok(())
    }
}

fn pwm_traits() {
    let mut pwm = MockPwm::new("PWM_CH1", 1000);

    println!("  PWM SetDutyCycle operations:");
    pwm.set_duty_cycle_percent(50).unwrap();
    pwm.set_duty_cycle_percent(75).unwrap();
    pwm.set_duty_cycle_fully_on().unwrap();
    pwm.set_duty_cycle_fully_off().unwrap();

    println!("\n  Raw duty cycle:");
    pwm.set_duty_cycle(32768).unwrap();
}

// ============================================
// Generic Driver Example
// ============================================

/// Example: Generic LED driver using HAL traits
struct Led<P: OutputPin> {
    pin: P,
    active_low: bool,
}

impl<P: OutputPin> Led<P> {
    fn new(pin: P, active_low: bool) -> Self {
        Led { pin, active_low }
    }

    fn on(&mut self) -> Result<(), P::Error> {
        if self.active_low {
            self.pin.set_low()
        } else {
            self.pin.set_high()
        }
    }

    fn off(&mut self) -> Result<(), P::Error> {
        if self.active_low {
            self.pin.set_high()
        } else {
            self.pin.set_low()
        }
    }
}

impl<P: ToggleableOutputPin> Led<P> {
    fn toggle(&mut self) -> Result<(), P::Error> {
        self.pin.toggle()
    }
}

/// Example: Generic button driver
struct Button<P: InputPin> {
    pin: P,
    active_low: bool,
}

impl<P: InputPin> Button<P> {
    fn new(pin: P, active_low: bool) -> Self {
        Button { pin, active_low }
    }

    fn is_pressed(&mut self) -> Result<bool, P::Error> {
        let state = self.pin.is_high()?;
        Ok(if self.active_low { !state } else { state })
    }
}

/// Example: Generic temperature sensor driver
struct TempSensor<I: I2c> {
    i2c: I,
    address: u8,
}

impl<I: I2c> TempSensor<I> {
    fn new(i2c: I, address: u8) -> Self {
        TempSensor { i2c, address }
    }

    fn read_temperature(&mut self) -> Result<f32, I::Error> {
        let mut data = [0u8; 2];
        self.i2c.write_read(self.address, &[0x00], &mut data)?;

        // Convert raw data to temperature (simplified)
        let raw = ((data[0] as u16) << 8) | (data[1] as u16);
        let temp = (raw >> 4) as f32 * 0.0625;
        Ok(temp)
    }
}

fn generic_driver_example() {
    println!("  Generic LED driver:");
    let pin = MockPin::new("LED_PIN");
    let mut led = Led::new(pin, false);
    led.on().unwrap();
    led.off().unwrap();
    led.toggle().unwrap();

    println!("\n  Generic Button driver:");
    let mut button_pin = MockPin::new("BTN_PIN");
    button_pin.state = true;
    let mut button = Button::new(button_pin, true); // Active low
    println!("    Button pressed: {}", button.is_pressed().unwrap());

    println!("\n  Generic Temperature Sensor driver:");
    let i2c = MockI2c::new("I2C1");
    let mut temp_sensor = TempSensor::new(i2c, 0x48);
    match temp_sensor.read_temperature() {
        Ok(temp) => println!("    Temperature: {:.2}°C", temp),
        Err(_) => println!("    Failed to read temperature"),
    }

    println!("\n  Benefits of HAL traits:");
    println!("    - Drivers work with any HAL implementation");
    println!("    - Easy to test with mock implementations");
    println!("    - Portable across different microcontrollers");
    println!("    - Type-safe compile-time checks");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpio_output() {
        let mut pin = MockPin::new("TEST");
        assert!(pin.is_low().unwrap());

        pin.set_high().unwrap();
        assert!(pin.is_high().unwrap());

        pin.toggle().unwrap();
        assert!(pin.is_low().unwrap());
    }

    #[test]
    fn test_led_driver() {
        let pin = MockPin::new("LED");
        let mut led = Led::new(pin, false);

        led.on().unwrap();
        led.off().unwrap();
    }

    #[test]
    fn test_pwm_duty_cycle() {
        let mut pwm = MockPwm::new("PWM", 1000);

        pwm.set_duty_cycle_percent(50).unwrap();
        assert_eq!(pwm.duty, 32767);

        pwm.set_duty_cycle_fully_on().unwrap();
        assert_eq!(pwm.duty, 65535);
    }
}
