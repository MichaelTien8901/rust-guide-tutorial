//! Cross-Compilation Concepts and Target Configuration
//!
//! This example demonstrates patterns and concepts for cross-compiling
//! Rust code to different target architectures and platforms.
//!
//! Note: This runs on the host. Real cross-compilation targets
//! specific hardware with appropriate toolchains.

use std::mem;

// ============================================================================
// Target Triple Components
// ============================================================================

/// Demonstrates the structure of a Rust target triple.
/// Format: <arch>-<vendor>-<os>-<env>
///
/// Examples:
/// - x86_64-unknown-linux-gnu
/// - aarch64-apple-darwin
/// - thumbv7em-none-eabihf
/// - wasm32-unknown-unknown
pub mod target_triple {
    /// Common architectures.
    #[derive(Debug, Clone, Copy)]
    pub enum Architecture {
        X86_64,    // 64-bit x86 (Intel/AMD)
        Aarch64,   // 64-bit ARM
        Arm,       // 32-bit ARM
        Thumbv7em, // ARM Cortex-M4/M7
        Thumbv6m,  // ARM Cortex-M0/M0+
        Riscv32,   // 32-bit RISC-V
        Riscv64,   // 64-bit RISC-V
        Wasm32,    // WebAssembly
        Mips,      // MIPS
        PowerPc,   // PowerPC
    }

    /// Common vendors.
    #[derive(Debug, Clone, Copy)]
    pub enum Vendor {
        Unknown,
        Apple,
        Pc,
        None, // For bare-metal
    }

    /// Common operating systems.
    #[derive(Debug, Clone, Copy)]
    pub enum Os {
        Linux,
        Windows,
        Macos,
        FreeBsd,
        None, // Bare-metal
        Uefi,
        Unknown, // WebAssembly
    }

    /// Common environments/ABIs.
    #[derive(Debug, Clone, Copy)]
    pub enum Environment {
        Gnu,
        Musl,
        Msvc,
        Eabi,
        Eabihf, // Hardware float
        None,
    }

    /// Represents a target triple.
    #[derive(Debug)]
    pub struct TargetTriple {
        pub arch: Architecture,
        pub vendor: Vendor,
        pub os: Os,
        pub env: Environment,
    }

    impl TargetTriple {
        pub fn describe(&self) -> String {
            format!(
                "Architecture: {:?}, Vendor: {:?}, OS: {:?}, Environment: {:?}",
                self.arch, self.vendor, self.os, self.env
            )
        }
    }

    /// Common embedded targets.
    pub fn common_embedded_targets() -> Vec<(&'static str, &'static str)> {
        vec![
            ("thumbv6m-none-eabi", "Cortex-M0, M0+ (ARMv6-M)"),
            ("thumbv7m-none-eabi", "Cortex-M3 (ARMv7-M)"),
            ("thumbv7em-none-eabi", "Cortex-M4, M7 no FPU (ARMv7E-M)"),
            (
                "thumbv7em-none-eabihf",
                "Cortex-M4F, M7 with FPU (ARMv7E-M)",
            ),
            ("thumbv8m.main-none-eabi", "Cortex-M33 (ARMv8-M)"),
            ("riscv32imac-unknown-none-elf", "RISC-V 32-bit"),
            ("riscv64gc-unknown-none-elf", "RISC-V 64-bit"),
        ]
    }

    /// Common cross-compilation targets.
    pub fn common_cross_targets() -> Vec<(&'static str, &'static str)> {
        vec![
            ("aarch64-unknown-linux-gnu", "64-bit Linux ARM"),
            ("aarch64-apple-darwin", "Apple Silicon Mac"),
            ("arm-unknown-linux-gnueabihf", "32-bit Linux ARM hard float"),
            ("x86_64-unknown-linux-musl", "Linux with musl libc (static)"),
            ("x86_64-pc-windows-gnu", "Windows with MinGW"),
            ("wasm32-unknown-unknown", "WebAssembly"),
            ("wasm32-wasi", "WebAssembly with WASI"),
        ]
    }
}

// ============================================================================
// Conditional Compilation with cfg
// ============================================================================

/// Demonstrates various cfg attributes for conditional compilation.
pub mod conditional {
    /// OS-specific code.
    #[cfg(target_os = "linux")]
    pub fn os_specific() -> &'static str {
        "Running on Linux"
    }

    #[cfg(target_os = "macos")]
    pub fn os_specific() -> &'static str {
        "Running on macOS"
    }

    #[cfg(target_os = "windows")]
    pub fn os_specific() -> &'static str {
        "Running on Windows"
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    pub fn os_specific() -> &'static str {
        "Running on other OS"
    }

    /// Architecture-specific code.
    #[cfg(target_arch = "x86_64")]
    pub fn arch_specific() -> &'static str {
        "64-bit x86 architecture"
    }

    #[cfg(target_arch = "aarch64")]
    pub fn arch_specific() -> &'static str {
        "64-bit ARM architecture"
    }

    #[cfg(target_arch = "arm")]
    pub fn arch_specific() -> &'static str {
        "32-bit ARM architecture"
    }

    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64", target_arch = "arm")))]
    pub fn arch_specific() -> &'static str {
        "Other architecture"
    }

    /// Pointer width checks.
    #[cfg(target_pointer_width = "64")]
    pub fn pointer_width() -> usize {
        64
    }

    #[cfg(target_pointer_width = "32")]
    pub fn pointer_width() -> usize {
        32
    }

    /// Endianness checks.
    #[cfg(target_endian = "little")]
    pub fn endianness() -> &'static str {
        "little endian"
    }

    #[cfg(target_endian = "big")]
    pub fn endianness() -> &'static str {
        "big endian"
    }

    /// Feature-based conditional compilation.
    #[cfg(feature = "advanced")]
    pub fn advanced_feature() -> &'static str {
        "Advanced feature enabled"
    }

    #[cfg(not(feature = "advanced"))]
    pub fn advanced_feature() -> &'static str {
        "Advanced feature disabled"
    }

    /// Using cfg_if pattern (without the macro).
    pub fn platform_optimization() -> &'static str {
        #[cfg(all(target_arch = "x86_64", target_feature = "avx2"))]
        {
            return "Using AVX2 SIMD instructions";
        }

        #[cfg(all(target_arch = "x86_64", target_feature = "sse4.1"))]
        {
            return "Using SSE4.1 SIMD instructions";
        }

        #[cfg(target_arch = "aarch64")]
        {
            return "Using NEON SIMD instructions";
        }

        #[allow(unreachable_code)]
        "Using scalar implementation"
    }
}

// ============================================================================
// Size and Alignment Considerations
// ============================================================================

/// Demonstrates size and alignment differences across platforms.
pub mod memory_layout {
    use std::mem::{align_of, size_of};

    /// A struct that may have different sizes on different platforms.
    #[repr(C)]
    pub struct CrossPlatformStruct {
        pub byte: u8,
        pub word: u16,
        pub dword: u32,
        pub pointer: *const (),
        pub long: u64,
    }

    /// Fixed-size struct using explicit types.
    #[repr(C)]
    pub struct FixedSizeStruct {
        pub a: u8,
        pub b: u16,
        pub c: u32,
        pub d: u64,
    }

    /// Packed struct for wire protocols.
    #[repr(C, packed)]
    #[derive(Clone, Copy)]
    pub struct PackedStruct {
        pub a: u8,
        pub b: u32,
        pub c: u16,
    }

    /// Print layout information.
    pub fn print_layouts() {
        println!("Memory Layout Information:");
        println!(
            "  usize: {} bytes, align {}",
            size_of::<usize>(),
            align_of::<usize>()
        );
        println!(
            "  pointer: {} bytes, align {}",
            size_of::<*const ()>(),
            align_of::<*const ()>()
        );
        println!(
            "  CrossPlatformStruct: {} bytes, align {}",
            size_of::<CrossPlatformStruct>(),
            align_of::<CrossPlatformStruct>()
        );
        println!(
            "  FixedSizeStruct: {} bytes, align {}",
            size_of::<FixedSizeStruct>(),
            align_of::<FixedSizeStruct>()
        );
        println!(
            "  PackedStruct: {} bytes, align {}",
            size_of::<PackedStruct>(),
            align_of::<PackedStruct>()
        );
    }
}

// ============================================================================
// Build Configuration Simulation
// ============================================================================

/// Simulates Cargo.toml configuration for cross-compilation.
pub mod build_config {
    /// Example .cargo/config.toml content for cross-compilation.
    pub fn cargo_config_example() -> &'static str {
        r#"
# .cargo/config.toml

# Default target for this project
[build]
target = "thumbv7em-none-eabihf"

# Target-specific linker configuration
[target.thumbv7em-none-eabihf]
rustflags = [
    "-C", "link-arg=-Tlink.x",  # Use custom linker script
    "-C", "link-arg=--nmagic",   # Don't page-align sections
]
runner = "probe-run --chip STM32F411CEUx"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"

[target.x86_64-unknown-linux-musl]
linker = "musl-gcc"
rustflags = ["-C", "target-feature=+crt-static"]

# Environment variables for cross-compilation
[env]
CC_aarch64_unknown_linux_gnu = "aarch64-linux-gnu-gcc"
AR_aarch64_unknown_linux_gnu = "aarch64-linux-gnu-ar"
"#
    }

    /// Example linker script for embedded.
    pub fn linker_script_example() -> &'static str {
        r#"
/* memory.x - Memory layout for STM32F411 */

MEMORY
{
    FLASH : ORIGIN = 0x08000000, LENGTH = 512K
    RAM   : ORIGIN = 0x20000000, LENGTH = 128K
}

/* Entry point */
ENTRY(Reset);

/* Stack configuration */
_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
    .vector_table ORIGIN(FLASH) :
    {
        LONG(_stack_start);
        KEEP(*(.vector_table.reset_vector));
        KEEP(*(.vector_table.exceptions));
    } > FLASH

    .text :
    {
        *(.text .text.*);
    } > FLASH

    .rodata :
    {
        *(.rodata .rodata.*);
    } > FLASH

    .data :
    {
        _sdata = .;
        *(.data .data.*);
        _edata = .;
    } > RAM AT > FLASH

    .bss :
    {
        _sbss = .;
        *(.bss .bss.*);
        _ebss = .;
    } > RAM
}
"#
    }

    /// Example build.rs for cross-compilation.
    pub fn build_script_example() -> &'static str {
        r#"
// build.rs

fn main() {
    // Tell Cargo to rerun if memory.x changes
    println!("cargo:rerun-if-changed=memory.x");

    // Get output directory
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_path = std::path::Path::new(&out_dir);

    // Copy linker script to output directory
    std::fs::copy("memory.x", out_path.join("memory.x")).unwrap();

    // Add linker search path
    println!("cargo:rustc-link-search={}", out_path.display());

    // Set target-specific configuration
    let target = std::env::var("TARGET").unwrap();

    if target.contains("thumbv7em") {
        // Cortex-M4 specific settings
        println!("cargo:rustc-cfg=cortex_m4");
    } else if target.contains("thumbv6m") {
        // Cortex-M0 specific settings
        println!("cargo:rustc-cfg=cortex_m0");
    }

    // Pass target arch to code
    if target.starts_with("arm") || target.starts_with("thumb") {
        println!("cargo:rustc-cfg=arm_target");
    }
}
"#
    }
}

// ============================================================================
// Size Optimization Techniques
// ============================================================================

/// Demonstrates size optimization for embedded targets.
pub mod size_optimization {
    /// Cargo.toml profile for size optimization.
    pub fn release_profile() -> &'static str {
        r#"
[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
panic = "abort"     # Abort on panic (no unwinding)
strip = true        # Strip symbols

[profile.release.package."*"]
opt-level = "z"     # Also optimize dependencies for size
"#
    }

    /// Size comparison of optimization levels.
    pub fn optimization_levels() {
        println!("Optimization Level Comparison (approximate):");
        println!("  opt-level = 0: Fastest compile, largest binary");
        println!("  opt-level = 1: Basic optimizations");
        println!("  opt-level = 2: Most optimizations (default release)");
        println!("  opt-level = 3: Aggressive optimizations, may increase size");
        println!("  opt-level = 's': Optimize for size");
        println!("  opt-level = 'z': Optimize more aggressively for size");
    }

    /// Techniques for reducing binary size.
    pub fn size_reduction_tips() {
        println!("\nSize Reduction Techniques:");
        println!("  1. Use opt-level = 'z' for size optimization");
        println!("  2. Enable LTO (Link-Time Optimization)");
        println!("  3. Set codegen-units = 1 for better optimization");
        println!("  4. Use panic = 'abort' to remove unwinding code");
        println!("  5. Strip symbols with strip = true");
        println!("  6. Avoid format strings (use ufmt for embedded)");
        println!("  7. Use #![no_std] to avoid standard library");
        println!("  8. Minimize dependencies");
        println!("  9. Use cargo-bloat to identify large functions");
        println!(" 10. Consider using min-sized-rust techniques");
    }
}

// ============================================================================
// Cross-Compilation Tooling
// ============================================================================

/// Information about cross-compilation tools.
pub mod tooling {
    /// Commands for setting up cross-compilation.
    pub fn setup_commands() {
        println!("Cross-Compilation Setup Commands:");
        println!();
        println!("# Install target support");
        println!("  rustup target add thumbv7em-none-eabihf");
        println!("  rustup target add aarch64-unknown-linux-gnu");
        println!();
        println!("# Install cross-linker (Ubuntu/Debian)");
        println!("  sudo apt install gcc-arm-none-eabi      # ARM embedded");
        println!("  sudo apt install gcc-aarch64-linux-gnu  # ARM64 Linux");
        println!();
        println!("# Install cross (Docker-based cross-compilation)");
        println!("  cargo install cross");
        println!();
        println!("# Build for target");
        println!("  cargo build --target thumbv7em-none-eabihf --release");
        println!("  cross build --target aarch64-unknown-linux-gnu");
    }

    /// Binary inspection tools.
    pub fn inspection_tools() {
        println!("\nBinary Inspection Tools:");
        println!();
        println!("# Check binary size");
        println!("  ls -la target/thumbv7em-none-eabihf/release/myapp");
        println!();
        println!("# View sections (requires binutils)");
        println!("  arm-none-eabi-size target/*/release/myapp");
        println!("  arm-none-eabi-objdump -h target/*/release/myapp");
        println!();
        println!("# Analyze bloat");
        println!("  cargo install cargo-bloat");
        println!("  cargo bloat --release --target thumbv7em-none-eabihf");
        println!();
        println!("# View symbols");
        println!("  nm target/*/release/myapp | sort -k2");
        println!();
        println!("# Disassemble");
        println!("  arm-none-eabi-objdump -d target/*/release/myapp");
    }
}

// ============================================================================
// Platform Abstraction Example
// ============================================================================

/// Example of abstracting platform-specific code.
pub mod platform_abstraction {
    /// Platform-independent interface.
    pub trait Platform {
        fn name() -> &'static str;
        fn pointer_size() -> usize;
        fn endianness() -> Endianness;
        fn has_fpu() -> bool;
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Endianness {
        Little,
        Big,
    }

    /// Current platform implementation.
    pub struct CurrentPlatform;

    impl Platform for CurrentPlatform {
        fn name() -> &'static str {
            #[cfg(target_os = "linux")]
            return "Linux";
            #[cfg(target_os = "macos")]
            return "macOS";
            #[cfg(target_os = "windows")]
            return "Windows";
            #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
            return "Unknown";
        }

        fn pointer_size() -> usize {
            std::mem::size_of::<usize>() * 8
        }

        fn endianness() -> Endianness {
            #[cfg(target_endian = "little")]
            return Endianness::Little;
            #[cfg(target_endian = "big")]
            return Endianness::Big;
        }

        fn has_fpu() -> bool {
            // Most desktop platforms have FPU
            cfg!(any(
                target_arch = "x86_64",
                target_arch = "x86",
                target_arch = "aarch64",
            ))
        }
    }

    /// Print current platform info.
    pub fn print_platform_info() {
        println!("Current Platform Information:");
        println!("  Name: {}", CurrentPlatform::name());
        println!("  Pointer size: {} bits", CurrentPlatform::pointer_size());
        println!("  Endianness: {:?}", CurrentPlatform::endianness());
        println!("  Has FPU: {}", CurrentPlatform::has_fpu());
    }
}

// ============================================================================
// Demonstration Functions
// ============================================================================

fn demo_target_triples() {
    println!("=== Target Triple Information ===\n");

    use target_triple::*;

    let current = TargetTriple {
        arch: Architecture::X86_64,
        vendor: Vendor::Unknown,
        os: Os::Linux,
        env: Environment::Gnu,
    };

    println!("Example target: x86_64-unknown-linux-gnu");
    println!("  {}", current.describe());
    println!();

    println!("Common Embedded Targets:");
    for (target, description) in common_embedded_targets() {
        println!("  {} - {}", target, description);
    }
    println!();

    println!("Common Cross-Compilation Targets:");
    for (target, description) in common_cross_targets() {
        println!("  {} - {}", target, description);
    }
    println!();
}

fn demo_conditional_compilation() {
    println!("=== Conditional Compilation ===\n");

    use conditional::*;

    println!("OS: {}", os_specific());
    println!("Architecture: {}", arch_specific());
    println!("Pointer width: {} bits", pointer_width());
    println!("Endianness: {}", endianness());
    println!("Feature status: {}", advanced_feature());
    println!("SIMD: {}", platform_optimization());
    println!();

    // Compile-time assertions
    #[cfg(target_pointer_width = "64")]
    const _: () = assert!(mem::size_of::<usize>() == 8);

    #[cfg(target_pointer_width = "32")]
    const _: () = assert!(mem::size_of::<usize>() == 4);

    println!("Compile-time pointer width check: PASSED");
    println!();
}

fn demo_memory_layout() {
    println!("=== Memory Layout ===\n");

    memory_layout::print_layouts();
    println!();
}

fn demo_build_configuration() {
    println!("=== Build Configuration Examples ===\n");

    println!(".cargo/config.toml example:");
    println!("-----------------------------");
    for line in build_config::cargo_config_example().lines().take(15) {
        println!("{}", line);
    }
    println!("  ... (truncated)");
    println!();

    println!("Linker script example (memory.x):");
    println!("----------------------------------");
    for line in build_config::linker_script_example().lines().take(12) {
        println!("{}", line);
    }
    println!("  ... (truncated)");
    println!();
}

fn demo_size_optimization() {
    println!("=== Size Optimization ===\n");

    size_optimization::optimization_levels();
    size_optimization::size_reduction_tips();
    println!();
}

fn demo_tooling() {
    println!("=== Cross-Compilation Tooling ===\n");

    tooling::setup_commands();
    tooling::inspection_tools();
    println!();
}

fn demo_platform_abstraction() {
    println!("=== Platform Abstraction ===\n");

    platform_abstraction::print_platform_info();
    println!();
}

fn main() {
    println!("Cross-Compilation Concepts in Rust\n");
    println!("===================================\n");

    demo_target_triples();
    demo_conditional_compilation();
    demo_memory_layout();
    demo_build_configuration();
    demo_size_optimization();
    demo_tooling();
    demo_platform_abstraction();

    println!("=== Summary ===\n");
    println!("Key cross-compilation concepts:");
    println!("  • Target triples: arch-vendor-os-env format");
    println!("  • Conditional compilation: #[cfg(...)] attributes");
    println!("  • Memory layout: #[repr(C)], packed structs");
    println!("  • Build configuration: .cargo/config.toml");
    println!("  • Linker scripts: Memory layout for embedded");
    println!("  • Size optimization: opt-level, LTO, panic=abort");
    println!("  • Tooling: rustup, cross, cargo-bloat");
    println!();
    println!("For actual cross-compilation, install target toolchains:");
    println!("  rustup target add <target-triple>");
}
