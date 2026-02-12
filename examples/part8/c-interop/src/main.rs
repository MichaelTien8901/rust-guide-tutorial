//! # C Interoperability Example
//!
//! Demonstrates FFI patterns for embedded Rust: extern "C" declarations,
//! safe wrappers with ownership and Drop, typestate for hardware state machines,
//! error code conversion, and simulated bindgen output.
//!
//! Compiles as standard Rust for CI. Simulates embedded C interop concepts.

use std::marker::PhantomData;

fn main() {
    println!("=== Rust and C Interoperability ===\n");
    demonstrate_extern_c_calls();
    demonstrate_safe_wrapper();
    demonstrate_typestate();
    demonstrate_error_conversion();
    demonstrate_bindgen_output();
}

// --- Simulated extern "C" function declarations and calls ---

extern "C" fn simulated_hal_init() -> i32 { 0 }
extern "C" fn simulated_hal_get_tick() -> u32 { 12345 }
extern "C" fn simulated_gpio_read(_port: u32, _pin: u16) -> i32 { 1 }

fn demonstrate_extern_c_calls() {
    println!("--- Calling C from Rust (simulated extern \"C\") ---");
    let status = simulated_hal_init();
    println!("  HAL_Init() -> {} (0 = OK)", status);
    println!("  HAL_GetTick() -> {} ms", simulated_hal_get_tick());
    let pin = simulated_gpio_read(0x4002_1C00, 13);
    println!("  GPIO_ReadPin(GPIOJ, 13) -> {} ({})\n",
        pin, if pin != 0 { "HIGH" } else { "LOW" });
}

// --- Safe wrapper: struct wrapping a raw resource with Drop ---

struct RawHandle { _id: u32, initialized: bool }

static CLEANUP_COUNT: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(0);

mod c_uart {
    use super::RawHandle;
    pub fn init(base: u32, baud: u32) -> *mut RawHandle {
        Box::into_raw(Box::new(RawHandle { _id: base ^ baud, initialized: true }))
    }
    pub fn send(h: *mut RawHandle, data: &[u8]) -> i32 {
        if h.is_null() { return -1; }
        if !unsafe { &*h }.initialized { return -2; }
        data.len() as i32
    }
    pub fn deinit(h: *mut RawHandle) {
        if !h.is_null() {
            let _ = unsafe { Box::from_raw(h) };
            super::CLEANUP_COUNT.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UartError { Timeout, NotInitialized, Unknown(i32) }
impl UartError {
    fn from_code(code: i32) -> Self {
        match code { -1 => Self::Timeout, -2 => Self::NotInitialized, c => Self::Unknown(c) }
    }
}

struct Uart { handle: *mut RawHandle }
impl Uart {
    fn new(base: u32, baud: u32) -> Option<Self> {
        let h = c_uart::init(base, baud);
        if h.is_null() { None } else { Some(Uart { handle: h }) }
    }
    fn send(&mut self, data: &[u8]) -> Result<usize, UartError> {
        let rc = c_uart::send(self.handle, data);
        if rc >= 0 { Ok(rc as usize) } else { Err(UartError::from_code(rc)) }
    }
}
impl Drop for Uart {
    fn drop(&mut self) { c_uart::deinit(self.handle); }
}

fn demonstrate_safe_wrapper() {
    println!("--- Safe Wrapper Pattern (ownership + Drop) ---");
    let before = CLEANUP_COUNT.load(std::sync::atomic::Ordering::SeqCst);
    {
        let mut uart = Uart::new(0x4000_4400, 115_200).expect("UART init failed");
        println!("  Uart::new(0x40004400, 115200) -> OK");
        let sent = uart.send(b"Hello, embedded!").unwrap();
        println!("  uart.send(\"Hello, embedded!\") -> {} bytes", sent);
    } // Drop calls deinit here
    let after = CLEANUP_COUNT.load(std::sync::atomic::Ordering::SeqCst);
    println!("  Drop called deinit: cleanup count {} -> {}\n", before, after);
}

// --- Typestate pattern for a simulated hardware peripheral ---

struct Disabled; struct Configured; struct Enabled;

struct Spi<S> { base: u32, _state: PhantomData<S> }

impl Spi<Disabled> {
    fn new(base: u32) -> Self { Spi { base, _state: PhantomData } }
    fn configure(self, _div: u8) -> Spi<Configured> { Spi { base: self.base, _state: PhantomData } }
}
impl Spi<Configured> {
    fn enable(self) -> Spi<Enabled> { Spi { base: self.base, _state: PhantomData } }
}
impl Spi<Enabled> {
    fn transfer(&self, tx: &[u8]) -> Vec<u8> { tx.iter().map(|b| b.wrapping_add(1)).collect() }
    fn disable(self) -> Spi<Configured> { Spi { base: self.base, _state: PhantomData } }
}

fn demonstrate_typestate() {
    println!("--- Typestate Pattern (compile-time state machine) ---");
    let spi = Spi::<Disabled>::new(0x4001_3000);
    println!("  Spi::new() -> Disabled");
    let spi = spi.configure(4);
    println!("  .configure(4) -> Configured");
    let spi = spi.enable();
    println!("  .enable() -> Enabled");
    let rx = spi.transfer(&[0x01, 0x02, 0x03]);
    println!("  .transfer([01,02,03]) -> {:?}", rx);
    let _spi = spi.disable();
    println!("  .disable() -> Configured");
    // Spi::<Disabled>::new(0).transfer(&[1]); // Would NOT compile!
    println!();
}

// --- Error code to Result conversion ---

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum I2cError { Nack, BusError, Timeout, Unknown(i32) }
impl I2cError {
    fn from_code(c: i32) -> Self {
        match c { -1 => Self::Nack, -2 => Self::BusError, -3 => Self::Timeout, o => Self::Unknown(o) }
    }
}
fn check_i2c(code: i32) -> Result<(), I2cError> {
    if code == 0 { Ok(()) } else { Err(I2cError::from_code(code)) }
}

fn demonstrate_error_conversion() {
    println!("--- Error Code to Result Conversion ---");
    for &code in &[0, -1, -2, -3, -99] {
        println!("  code {:>3} -> {:?}", code, check_i2c(code));
    }
    println!();
}

// --- Simulated bindgen output format ---

#[allow(non_camel_case_types, non_upper_case_globals, non_snake_case, dead_code)]
mod ffi_bindings {
    pub type c_uint = u32;
    pub const GPIO_PIN_13: u16 = 0x2000;
    pub const GPIO_MODE_OUTPUT_PP: c_uint = 0x01;
    pub const GPIO_NOPULL: c_uint = 0x00;
    pub const GPIO_SPEED_HIGH: c_uint = 0x02;

    #[repr(C)]
    #[derive(Default, Clone)]
    pub struct GPIO_InitTypeDef {
        pub Pin: u32, pub Mode: c_uint, pub Pull: c_uint,
        pub Speed: c_uint, pub Alternate: c_uint,
    }

    pub fn HAL_GPIO_Init(cfg: &GPIO_InitTypeDef) -> i32 {
        if cfg.Pin == 0 { -1 } else { 0 }
    }
}

fn demonstrate_bindgen_output() {
    println!("--- Simulated Bindgen Output ---");
    use ffi_bindings::*;
    let cfg = GPIO_InitTypeDef {
        Pin: GPIO_PIN_13 as u32, Mode: GPIO_MODE_OUTPUT_PP,
        Pull: GPIO_NOPULL, Speed: GPIO_SPEED_HIGH, Alternate: 0,
    };
    println!("  GPIO_InitTypeDef {{ Pin: 0x{:04X}, Mode: 0x{:02X} }}", cfg.Pin, cfg.Mode);
    println!("  HAL_GPIO_Init() -> {} (0 = OK)", HAL_GPIO_Init(&cfg));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extern_c_calls() {
        assert_eq!(simulated_hal_init(), 0);
        assert!(simulated_hal_get_tick() > 0);
    }

    #[test]
    fn test_safe_wrapper_lifecycle() {
        let before = CLEANUP_COUNT.load(std::sync::atomic::Ordering::SeqCst);
        {
            let mut uart = Uart::new(0x4000, 9600).unwrap();
            assert_eq!(uart.send(b"test").unwrap(), 4);
        }
        assert_eq!(CLEANUP_COUNT.load(std::sync::atomic::Ordering::SeqCst), before + 1);
    }

    #[test]
    fn test_error_conversion() {
        assert_eq!(UartError::from_code(-1), UartError::Timeout);
        assert_eq!(UartError::from_code(-2), UartError::NotInitialized);
        assert!(check_i2c(0).is_ok());
        assert_eq!(check_i2c(-1), Err(I2cError::Nack));
        assert_eq!(check_i2c(-42), Err(I2cError::Unknown(-42)));
    }

    #[test]
    fn test_typestate_transitions() {
        let spi = Spi::<Disabled>::new(0x4001_3000);
        let spi = spi.configure(8).enable();
        assert_eq!(spi.transfer(&[0x00, 0xFF]), vec![0x01, 0x00]);
        let _spi = spi.disable();
    }

    #[test]
    fn test_bindgen_gpio_init() {
        use ffi_bindings::*;
        let good = GPIO_InitTypeDef { Pin: GPIO_PIN_13 as u32, ..Default::default() };
        assert_eq!(HAL_GPIO_Init(&good), 0);
        assert_eq!(HAL_GPIO_Init(&GPIO_InitTypeDef::default()), -1);
    }
}
