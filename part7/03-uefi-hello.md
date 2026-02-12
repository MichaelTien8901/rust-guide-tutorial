---
layout: default
title: UEFI Hello World
parent: Part 7 - UEFI
nav_order: 3
---

# UEFI Hello World

Creating your first UEFI application in Rust.

## Minimal Application

The simplest UEFI application:

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    Status::SUCCESS
}
```

This does nothing but demonstrates the basic structure.

## Hello World with Text Output

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::proto::console::text::Output;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Clear the screen
    system_table.stdout().clear().unwrap();

    // Print hello world
    system_table
        .stdout()
        .output_string(cstr16!("Hello, UEFI World!\r\n"))
        .unwrap();

    // Wait for a key press
    system_table.stdin().reset(false).unwrap();
    loop {
        if let Ok(Some(_)) = system_table.stdin().read_key() {
            break;
        }
    }

    Status::SUCCESS
}
```

## Using the Logging Framework

The `uefi` crate integrates with Rust's `log` crate:

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    // Initialize UEFI helpers including logging
    uefi::helpers::init().unwrap();

    log::info!("Hello from UEFI!");
    log::debug!("Debug message");
    log::warn!("Warning message");
    log::error!("Error message");

    // Logging output goes to the console

    Status::SUCCESS
}
```

## Understanding the Entry Point

### The `#[entry]` Macro

The `#[entry]` attribute generates the actual UEFI entry point:

```rust
// What you write:
#[entry]
fn main(image_handle: Handle, system_table: SystemTable<Boot>) -> Status {
    Status::SUCCESS
}

// What gets generated (conceptually):
#[no_mangle]
pub extern "efiapi" fn efi_main(
    image_handle: Handle,
    system_table: *mut c_void
) -> Status {
    // Safety wrapper code
    // Calls your main function
}
```

### Entry Point Variations

```rust
// Simplest form - no parameters
#[entry]
fn main() -> Status {
    Status::SUCCESS
}

// With handle only
#[entry]
fn main(image: Handle) -> Status {
    Status::SUCCESS
}

// Full form
#[entry]
fn main(image: Handle, st: SystemTable<Boot>) -> Status {
    Status::SUCCESS
}
```

## Printing with Formatting

```rust
#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use uefi::prelude::*;

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let version = 1;
    let name = "Rust UEFI";

    log::info!("{} version {}", name, version);

    // Using format! with allocation
    let message = format!("Formatted: {} v{}\n", name, version);
    log::info!("{}", message);

    Status::SUCCESS
}
```

## Reading System Information

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::table::cfg;

#[entry]
fn main(image: Handle, st: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    // Firmware vendor and revision
    log::info!("Firmware Vendor: {}", st.firmware_vendor());
    log::info!("Firmware Revision: {:#x}", st.firmware_revision());

    // UEFI revision
    let rev = st.uefi_revision();
    log::info!("UEFI {}.{}", rev.major(), rev.minor());

    // Configuration tables
    for table in st.config_table() {
        if table.guid == cfg::ACPI2_GUID {
            log::info!("Found ACPI 2.0 table at {:?}", table.address);
        } else if table.guid == cfg::SMBIOS3_GUID {
            log::info!("Found SMBIOS 3.0 table at {:?}", table.address);
        }
    }

    Status::SUCCESS
}
```

## Interactive Input

```rust
#![no_main]
#![no_std]

extern crate alloc;

use alloc::string::String;
use uefi::prelude::*;
use uefi::proto::console::text::Key;

#[entry]
fn main(_image: Handle, mut st: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    st.stdout().clear().unwrap();
    st.stdout()
        .output_string(cstr16!("Enter your name: "))
        .unwrap();

    let mut input = String::new();

    loop {
        // Wait for key
        st.boot_services()
            .wait_for_event(&mut [st.stdin().wait_for_key_event().unwrap()])
            .unwrap();

        if let Some(Key::Printable(c)) = st.stdin().read_key().unwrap() {
            let ch: char = c.into();

            if ch == '\r' {
                st.stdout().output_string(cstr16!("\r\n")).unwrap();
                break;
            }

            input.push(ch);

            // Echo character
            let mut buf = [0u16; 2];
            let s = ch.encode_utf16(&mut buf);
            // Note: Would need CStr16 conversion for output
        }
    }

    log::info!("Hello, {}!", input);

    Status::SUCCESS
}
```

## Memory Map Example

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi::table::boot::{MemoryDescriptor, MemoryType};

#[entry]
fn main(_image: Handle, st: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    let bt = st.boot_services();

    // Get memory map size
    let map_size = bt.memory_map_size();
    log::info!("Memory map size: {} bytes", map_size.map_size);
    log::info!("Descriptor size: {} bytes", map_size.entry_size);

    // Allocate buffer and get memory map
    let mut buffer = alloc::vec![0u8; map_size.map_size + 256];
    let (_key, descriptors) = bt.memory_map(&mut buffer).unwrap();

    let mut total_memory = 0u64;
    let mut usable_memory = 0u64;

    for desc in descriptors {
        let pages = desc.page_count;
        let bytes = pages * 4096;
        total_memory += bytes;

        match desc.ty {
            MemoryType::CONVENTIONAL
            | MemoryType::BOOT_SERVICES_CODE
            | MemoryType::BOOT_SERVICES_DATA => {
                usable_memory += bytes;
            }
            _ => {}
        }
    }

    log::info!("Total memory: {} MB", total_memory / 1024 / 1024);
    log::info!("Usable memory: {} MB", usable_memory / 1024 / 1024);

    Status::SUCCESS
}
```

## Timer and Delays

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main(_image: Handle, st: SystemTable<Boot>) -> Status {
    uefi::helpers::init().unwrap();

    log::info!("Starting countdown...");

    for i in (1..=5).rev() {
        log::info!("{}...", i);
        // Stall for 1 second (1,000,000 microseconds)
        st.boot_services().stall(1_000_000);
    }

    log::info!("Done!");

    Status::SUCCESS
}
```

## Error Handling

```rust
#![no_main]
#![no_std]

use uefi::prelude::*;

#[entry]
fn main() -> Status {
    // Using unwrap - panics on error
    uefi::helpers::init().unwrap();

    // Using ? operator with Result
    if let Err(e) = do_something() {
        log::error!("Error: {:?}", e);
        return Status::DEVICE_ERROR;
    }

    Status::SUCCESS
}

fn do_something() -> uefi::Result {
    uefi::helpers::init()?;

    // Simulate an operation that might fail
    log::info!("Doing something...");

    Ok(())
}
```

## Complete Example

```rust
#![no_main]
#![no_std]

extern crate alloc;

use uefi::prelude::*;

#[entry]
fn main(image: Handle, mut st: SystemTable<Boot>) -> Status {
    // Initialize helpers (logging, allocator)
    uefi::helpers::init().unwrap();

    // Clear screen
    st.stdout().clear().unwrap();

    // Print banner
    log::info!("========================================");
    log::info!("     Rust UEFI Hello World App");
    log::info!("========================================");
    log::info!("");

    // System info
    log::info!("Firmware: {}", st.firmware_vendor());
    log::info!("UEFI {}.{}",
        st.uefi_revision().major(),
        st.uefi_revision().minor());

    // Memory info
    let mem = st.boot_services().memory_map_size();
    log::info!("Memory descriptors: ~{}",
        mem.map_size / mem.entry_size);

    log::info!("");
    log::info!("Press any key to exit...");

    // Wait for key
    st.stdin().reset(false).unwrap();
    st.boot_services()
        .wait_for_event(&mut [st.stdin().wait_for_key_event().unwrap()])
        .unwrap();

    log::info!("Goodbye!");

    Status::SUCCESS
}
```

## Building and Running

```bash
# Build
cargo build --release

# Create ESP structure
mkdir -p esp/EFI/BOOT
cp target/x86_64-unknown-uefi/release/uefi-app.efi esp/EFI/BOOT/BOOTX64.EFI

# Run in QEMU
qemu-system-x86_64 \
    -nodefaults \
    -machine q35 \
    -bios /usr/share/OVMF/OVMF_CODE.fd \
    -drive format=raw,file=fat:rw:esp \
    -serial stdio
```

## Summary

| Concept | Description |
|---------|-------------|
| `#[entry]` | UEFI entry point macro |
| `SystemTable<Boot>` | Access to boot services |
| `Status` | Return value for UEFI functions |
| `cstr16!` | UTF-16 string literal macro |
| `uefi::helpers::init()` | Initialize logging and allocator |

## See Also

- [Example Code](https://github.com/MichaelTien8901/rust-guide-tutorial/tree/main/examples/part7/uefi-hello)

## Next Steps

Learn about [Boot Services]({% link part7/04-boot-services.md %}) for memory and protocol APIs.
