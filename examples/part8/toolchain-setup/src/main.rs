//! # Embedded Toolchain Setup Example
//!
//! Demonstrates the project structure and configuration concepts
//! for a Rust embedded project targeting the STM32F769I-DISCO.
//!
//! This example compiles as standard Rust for CI validation.
//! See the README for cross-compilation instructions.

/// Simulates the embedded project configuration layers.
///
/// In a real embedded project, these would be separate crates:
/// - PAC (Peripheral Access Crate): auto-generated register definitions
/// - HAL (Hardware Abstraction Layer): safe peripheral APIs
/// - BSP (Board Support Package): board-specific pin mappings
fn main() {
    println!("=== Embedded Project Configuration ===\n");

    // Demonstrate the PAC -> HAL -> BSP layering
    demonstrate_pac_layer();
    demonstrate_hal_layer();
    demonstrate_bsp_layer();

    // Show typical project file structure
    print_project_structure();

    // Show memory layout for STM32F769
    print_memory_layout();
}

/// PAC layer: raw register access (auto-generated from SVD)
fn demonstrate_pac_layer() {
    println!("--- PAC Layer (stm32f7) ---");
    println!("  Raw register access, unsafe, auto-generated from SVD files");
    println!("  Example: peripherals.GPIOA.odr.modify(|_, w| w.odr0().set_bit())");
    println!();
}

/// HAL layer: safe abstractions implementing embedded-hal traits
fn demonstrate_hal_layer() {
    println!("--- HAL Layer (stm32f7xx-hal) ---");
    println!("  Safe API wrapping PAC registers");
    println!("  Implements embedded-hal traits for portability");
    println!("  Example: led.set_high().ok()");
    println!();
}

/// BSP layer: board-specific configuration
fn demonstrate_bsp_layer() {
    println!("--- BSP Layer (project-level) ---");
    println!("  Pin mappings for STM32F769I-DISCO:");
    println!("    User LED (LD1): PJ13 (green)");
    println!("    User LED (LD2): PJ5  (red)");
    println!("    User Button:    PA0");
    println!("    UART Debug:     PA9 (TX) / PB7 (RX) via ST-LINK VCP");
    println!();
}

/// Show the typical embedded project file structure
fn print_project_structure() {
    println!("--- Project Structure ---");
    println!("  my-embedded-app/");
    println!("  +-- .cargo/");
    println!("  |   +-- config.toml      # Target, runner, linker flags");
    println!("  +-- src/");
    println!("  |   +-- main.rs          # #![no_std] #![no_main] entry");
    println!("  +-- memory.x             # Flash/RAM layout for linker");
    println!("  +-- Cargo.toml           # Dependencies: cortex-m-rt, HAL, defmt");
    println!("  +-- rust-toolchain.toml  # Pin Rust version (optional)");
    println!();
}

/// Show STM32F769 memory map
fn print_memory_layout() {
    println!("--- STM32F769NIH6 Memory Map ---");
    println!("  FLASH:  0x0800_0000 - 0x09FF_FFFF  (2 MB)");
    println!("  DTCM:   0x2000_0000 - 0x2000_3FFF  (16 KB, zero-wait-state)");
    println!("  SRAM1:  0x2002_0000 - 0x2004_FFFF  (368 KB)");
    println!("  SRAM2:  0x2005_0000 - 0x2005_3FFF  (16 KB)");
    println!("  ITCM:   0x0000_0000 - 0x0000_3FFF  (16 KB, instruction cache)");
    println!("  SDRAM:  0xC000_0000 - ...           (external, 128 Mbit)");
    println!();
    println!("  memory.x typically defines:");
    println!("    FLASH  : ORIGIN = 0x08000000, LENGTH = 2M");
    println!("    RAM    : ORIGIN = 0x20000000, LENGTH = 512K");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_memory_regions_valid() {
        // STM32F769 memory regions
        let flash_origin: u32 = 0x0800_0000;
        let flash_length: u32 = 2 * 1024 * 1024; // 2MB
        let ram_origin: u32 = 0x2000_0000;
        let ram_length: u32 = 512 * 1024; // 512KB

        assert_eq!(flash_origin, 0x0800_0000);
        assert_eq!(flash_length, 2_097_152);
        assert_eq!(ram_origin, 0x2000_0000);
        assert_eq!(ram_length, 524_288);

        // Verify no overlap
        let flash_end = flash_origin + flash_length;
        assert!(flash_end <= ram_origin, "Flash and RAM must not overlap");
    }

    #[test]
    fn test_target_triple() {
        // The correct target for Cortex-M7 with hard float
        let target = "thumbv7em-none-eabihf";
        assert!(target.contains("thumbv7em"), "Must be Cortex-M4/M7");
        assert!(target.contains("eabihf"), "Must use hard float ABI");
        assert!(target.contains("none"), "Must be bare metal (no OS)");
    }
}
