---
layout: default
title: Drivers
parent: Part 6 - Systems
nav_order: 5
---

# Writing Drivers in Rust

Memory-mapped I/O, interrupts, and device driver patterns.

## Memory-Mapped I/O

Hardware devices are accessed through memory addresses. Use volatile operations:

```rust
use core::ptr::{read_volatile, write_volatile};

const GPIO_BASE: usize = 0x3F20_0000;
const GPIO_FSEL0: usize = GPIO_BASE + 0x00;
const GPIO_SET0: usize = GPIO_BASE + 0x1C;
const GPIO_CLR0: usize = GPIO_BASE + 0x28;

fn set_pin_output(pin: u32) {
    let fsel = GPIO_FSEL0 + ((pin / 10) * 4) as usize;
    let shift = (pin % 10) * 3;

    unsafe {
        let mut val = read_volatile(fsel as *const u32);
        val &= !(0b111 << shift);  // Clear
        val |= 0b001 << shift;      // Set as output
        write_volatile(fsel as *mut u32, val);
    }
}

fn set_pin_high(pin: u32) {
    let reg = GPIO_SET0 + ((pin / 32) * 4) as usize;
    unsafe {
        write_volatile(reg as *mut u32, 1 << (pin % 32));
    }
}

fn set_pin_low(pin: u32) {
    let reg = GPIO_CLR0 + ((pin / 32) * 4) as usize;
    unsafe {
        write_volatile(reg as *mut u32, 1 << (pin % 32));
    }
}
```

## Register Abstraction with volatile-register

```rust
use volatile_register::{RO, RW, WO};

#[repr(C)]
struct UartRegs {
    dr: RW<u32>,      // Data register
    rsr: RO<u32>,     // Receive status
    _reserved: [u32; 4],
    fr: RO<u32>,      // Flag register
    _reserved2: u32,
    ilpr: RW<u32>,    // IrDA low-power
    ibrd: RW<u32>,    // Integer baud rate
    fbrd: RW<u32>,    // Fractional baud rate
    lcrh: RW<u32>,    // Line control
    cr: RW<u32>,      // Control register
    ifls: RW<u32>,    // FIFO level select
    imsc: RW<u32>,    // Interrupt mask
    ris: RO<u32>,     // Raw interrupt status
    mis: RO<u32>,     // Masked interrupt status
    icr: WO<u32>,     // Interrupt clear
}

struct Uart {
    regs: &'static mut UartRegs,
}

impl Uart {
    unsafe fn new(base: usize) -> Self {
        Uart {
            regs: &mut *(base as *mut UartRegs),
        }
    }

    fn write_byte(&mut self, byte: u8) {
        // Wait for TX FIFO not full
        while self.regs.fr.read() & (1 << 5) != 0 {}
        unsafe {
            self.regs.dr.write(byte as u32);
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        if self.regs.fr.read() & (1 << 4) != 0 {
            None  // RX FIFO empty
        } else {
            Some(self.regs.dr.read() as u8)
        }
    }
}
```

## Bitfield Registers

Use the `bitflags` or `modular-bitfield` crate:

```rust
use bitflags::bitflags;

bitflags! {
    struct UartFlags: u32 {
        const CTS  = 1 << 0;
        const DSR  = 1 << 1;
        const DCD  = 1 << 2;
        const BUSY = 1 << 3;
        const RXFE = 1 << 4;  // RX FIFO empty
        const TXFF = 1 << 5;  // TX FIFO full
        const RXFF = 1 << 6;  // RX FIFO full
        const TXFE = 1 << 7;  // TX FIFO empty
    }
}

impl Uart {
    fn flags(&self) -> UartFlags {
        UartFlags::from_bits_truncate(self.regs.fr.read())
    }

    fn is_tx_ready(&self) -> bool {
        !self.flags().contains(UartFlags::TXFF)
    }
}
```

## Interrupt Handling

### Bare Metal Interrupts

```rust
use core::sync::atomic::{AtomicBool, Ordering};

static BUTTON_PRESSED: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn gpio_interrupt_handler() {
    // Clear interrupt flag
    unsafe {
        write_volatile(GPIO_INTCLR as *mut u32, 1 << PIN);
    }

    BUTTON_PRESSED.store(true, Ordering::SeqCst);
}

fn main_loop() {
    loop {
        if BUTTON_PRESSED.swap(false, Ordering::SeqCst) {
            // Handle button press
        }
    }
}
```

### Cortex-M Interrupts

```rust
use cortex_m::peripheral::NVIC;
use stm32f4xx_hal::pac::interrupt;

#[interrupt]
fn EXTI0() {
    // Handle external interrupt 0
    // Clear pending bit
}

fn enable_interrupt() {
    unsafe {
        NVIC::unmask(stm32f4xx_hal::pac::Interrupt::EXTI0);
    }
}
```

## DMA (Direct Memory Access)

```rust
struct DmaChannel {
    control: &'static mut DmaControlRegs,
}

#[repr(C)]
struct DmaControlRegs {
    src_addr: RW<u32>,
    dst_addr: RW<u32>,
    count: RW<u32>,
    config: RW<u32>,
    status: RO<u32>,
}

impl DmaChannel {
    fn transfer(&mut self, src: *const u8, dst: *mut u8, len: usize) {
        unsafe {
            self.control.src_addr.write(src as u32);
            self.control.dst_addr.write(dst as u32);
            self.control.count.write(len as u32);

            // Start transfer
            self.control.config.write(
                (1 << 0) |  // Enable
                (1 << 1)    // Memory-to-memory
            );
        }
    }

    fn is_complete(&self) -> bool {
        self.control.status.read() & (1 << 0) != 0
    }

    fn wait(&self) {
        while !self.is_complete() {
            core::hint::spin_loop();
        }
    }
}
```

## Safe Driver Abstraction

Wrap unsafe operations in safe APIs:

```rust
pub struct Led {
    pin: u8,
}

impl Led {
    pub fn new(pin: u8) -> Self {
        set_pin_output(pin as u32);
        Led { pin }
    }

    pub fn on(&mut self) {
        set_pin_high(self.pin as u32);
    }

    pub fn off(&mut self) {
        set_pin_low(self.pin as u32);
    }

    pub fn toggle(&mut self) {
        // Read current state and toggle
    }
}

impl Drop for Led {
    fn drop(&mut self) {
        self.off();  // Ensure LED is off when dropped
    }
}
```

## Driver with Ownership

Use Rust's ownership to prevent misuse:

```rust
pub struct SpiDriver {
    regs: &'static mut SpiRegs,
}

pub struct SpiDevice<'a> {
    driver: &'a mut SpiDriver,
    cs_pin: u8,
}

impl SpiDriver {
    pub fn device(&mut self, cs_pin: u8) -> SpiDevice<'_> {
        SpiDevice {
            driver: self,
            cs_pin,
        }
    }
}

impl<'a> SpiDevice<'a> {
    pub fn transfer(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
        // Assert CS
        set_pin_low(self.cs_pin as u32);

        for byte in data.iter_mut() {
            *byte = self.driver.transfer_byte(*byte)?;
        }

        // Deassert CS
        set_pin_high(self.cs_pin as u32);
        Ok(())
    }
}
```

## Error Handling

```rust
#[derive(Debug)]
pub enum DriverError {
    Timeout,
    BusError,
    InvalidParameter,
    DeviceNotReady,
}

impl From<DriverError> for u32 {
    fn from(e: DriverError) -> u32 {
        match e {
            DriverError::Timeout => 1,
            DriverError::BusError => 2,
            DriverError::InvalidParameter => 3,
            DriverError::DeviceNotReady => 4,
        }
    }
}

fn read_with_timeout(timeout_us: u32) -> Result<u8, DriverError> {
    let start = get_time_us();
    while get_time_us() - start < timeout_us {
        if data_available() {
            return Ok(read_data());
        }
    }
    Err(DriverError::Timeout)
}
```

## Summary

| Pattern | Purpose |
|---------|---------|
| Volatile access | Hardware register I/O |
| Bitflags | Type-safe register fields |
| Ownership | Prevent resource conflicts |
| RAII/Drop | Automatic cleanup |
| Result | Error handling |

## Best Practices

1. **Always use volatile** for hardware access
2. **Wrap unsafe** in safe abstractions
3. **Use ownership** to manage resources
4. **Document memory maps** and register layouts
5. **Handle errors** gracefully

## See Also

- [Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/main/examples/part6/drivers)

## Next Steps

Learn about [Real-Time]({% link part6/06-real-time.md %}) constraints and heapless programming.
