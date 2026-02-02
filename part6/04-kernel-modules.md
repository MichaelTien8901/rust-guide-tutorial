---
layout: default
title: Kernel Modules
parent: Part 6 - Systems
nav_order: 4
---

# Linux Kernel Modules in Rust

Writing kernel modules using Rust-for-Linux.

## What is Rust-for-Linux?

Rust-for-Linux is the official effort to enable Rust as a second language for Linux kernel development. It provides:

- Safe abstractions over kernel APIs
- Compile-time safety guarantees
- Integration with existing C code

{: .note }
Rust support in the Linux kernel is still evolving. Check the latest kernel documentation for current APIs.

## Requirements

- Linux kernel 6.1+ with Rust support enabled
- Rust toolchain (specific version required by kernel)
- LLVM/Clang for kernel builds

```bash
# Check if kernel has Rust support
make LLVM=1 rustavailable
```

## Minimal Kernel Module

```rust
// SPDX-License-Identifier: GPL-2.0

//! Minimal Rust kernel module

use kernel::prelude::*;

module! {
    type: MinimalModule,
    name: "minimal",
    author: "Your Name",
    description: "A minimal Rust kernel module",
    license: "GPL",
}

struct MinimalModule;

impl kernel::Module for MinimalModule {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Minimal Rust module loaded\n");
        Ok(MinimalModule)
    }
}

impl Drop for MinimalModule {
    fn drop(&mut self) {
        pr_info!("Minimal Rust module unloaded\n");
    }
}
```

## The module! Macro

```rust
module! {
    type: MyModule,           // Module struct type
    name: "my_module",        // Module name
    author: "Author Name",    // Author
    description: "Description", // Description
    license: "GPL",           // License (required)
    params: {                 // Module parameters (optional)
        my_param: i32 {
            default: 42,
            permissions: 0o644,
            description: "An example parameter",
        },
    },
}
```

## Kernel Printing

```rust
use kernel::prelude::*;

fn example() {
    // Different log levels
    pr_emerg!("Emergency message\n");
    pr_alert!("Alert message\n");
    pr_crit!("Critical message\n");
    pr_err!("Error message\n");
    pr_warn!("Warning message\n");
    pr_notice!("Notice message\n");
    pr_info!("Info message\n");
    pr_debug!("Debug message\n");

    // With formatting
    let value = 42;
    pr_info!("Value is {}\n", value);
}
```

## Error Handling

The kernel uses its own error type:

```rust
use kernel::prelude::*;
use kernel::error::code;

fn may_fail() -> Result {
    // Return kernel error codes
    if something_wrong {
        return Err(code::EINVAL);
    }
    Ok(())
}

fn use_result() -> Result<i32> {
    let result = may_fail()?;
    Ok(42)
}
```

## Memory Allocation

```rust
use kernel::prelude::*;

fn allocate() -> Result {
    // Box equivalent (GFP_KERNEL allocation)
    let boxed = Box::try_new(42)?;

    // Vec equivalent
    let mut vec = Vec::try_with_capacity(10)?;
    vec.try_push(1)?;
    vec.try_push(2)?;

    Ok(())
}
```

## Synchronization Primitives

```rust
use kernel::sync::{Mutex, SpinLock};
use kernel::prelude::*;

struct SharedData {
    value: u32,
}

struct MyModule {
    // Mutex for sleeping contexts
    data: Mutex<SharedData>,

    // SpinLock for interrupt contexts
    counter: SpinLock<u32>,
}

impl MyModule {
    fn update_value(&self, new_value: u32) {
        let mut guard = self.data.lock();
        guard.value = new_value;
    }

    fn increment_counter(&self) {
        let mut guard = self.counter.lock();
        *guard += 1;
    }
}
```

## Character Device

```rust
use kernel::prelude::*;
use kernel::file::{self, File, Operations};
use kernel::io_buffer::{IoBufferReader, IoBufferWriter};

struct MyDevice;

#[vtable]
impl Operations for MyDevice {
    type Data = ();
    type OpenData = ();

    fn open(_context: &(), _file: &File) -> Result<Self::Data> {
        pr_info!("Device opened\n");
        Ok(())
    }

    fn read(
        _data: (),
        _file: &File,
        writer: &mut impl IoBufferWriter,
        _offset: u64,
    ) -> Result<usize> {
        let message = b"Hello from Rust!\n";
        writer.write_slice(message)?;
        Ok(message.len())
    }

    fn write(
        _data: (),
        _file: &File,
        reader: &mut impl IoBufferReader,
        _offset: u64,
    ) -> Result<usize> {
        let len = reader.len();
        pr_info!("Received {} bytes\n", len);
        Ok(len)
    }
}
```

## Building the Module

Create a `Kbuild` file:

```make
obj-m := my_module.o
```

Build with the kernel:

```bash
make -C /lib/modules/$(uname -r)/build M=$(pwd) modules
```

## Loading and Testing

```bash
# Load module
sudo insmod my_module.ko

# Check kernel log
dmesg | tail

# List loaded modules
lsmod | grep my_module

# Unload module
sudo rmmod my_module
```

## Safety Considerations

| Aspect | Handling |
|--------|----------|
| Memory allocation | Use `try_` variants, handle errors |
| Null pointers | Wrapped in Option/Result |
| Data races | Mutex/SpinLock required |
| Integer overflow | Checked by default in debug |
| Buffer overflows | Slice bounds checked |

## Current Limitations

1. **API stability** - Rust APIs are still evolving
2. **Coverage** - Not all kernel subsystems have Rust bindings
3. **Toolchain** - Specific Rust version required
4. **Debug info** - Some limitations with debugging

## Resources

- [Rust for Linux documentation](https://rust-for-linux.com/)
- [Kernel Rust documentation](https://rust-for-linux.github.io/docs/kernel/)
- [Linux kernel Rust samples](https://github.com/Rust-for-Linux/linux/tree/rust/samples/rust)

## Summary

| Component | Purpose |
|-----------|---------|
| `module!` macro | Module definition |
| `pr_info!` etc. | Kernel logging |
| `Result` | Error handling |
| `Mutex`/`SpinLock` | Synchronization |
| `Registration` | Device registration |

## Next Steps

Learn about [Drivers]({% link part6/05-drivers.md %}) and memory-mapped I/O.
