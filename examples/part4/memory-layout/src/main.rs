//! Memory Layout Example
//!
//! Demonstrates struct layout and repr attributes.
//!
//! # Memory Layout Concepts
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                   Memory Layout                         │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  Alignment: Data must be at addresses divisible by      │
//!     │            its alignment requirement                    │
//!     │                                                         │
//!     │  Padding:   Extra bytes inserted to maintain alignment  │
//!     │                                                         │
//!     │  Size:      Total bytes including padding               │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//!
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                  repr Attributes                        │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  repr(Rust)     - Default, compiler may reorder fields  │
//!     │  repr(C)        - C-compatible layout, stable ABI       │
//!     │  repr(packed)   - No padding, may cause unaligned access│
//!     │  repr(align(N)) - Minimum alignment of N bytes          │
//!     │  repr(transparent) - Same layout as single field        │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use std::mem::{align_of, size_of};

fn main() {
    println!("=== Memory Layout ===\n");

    println!("--- Primitive Sizes and Alignment ---");
    primitive_layout();

    println!("\n--- Struct Layout (Default) ---");
    default_struct_layout();

    println!("\n--- repr(C) Layout ---");
    repr_c_layout();

    println!("\n--- repr(packed) Layout ---");
    repr_packed_layout();

    println!("\n--- repr(align) Layout ---");
    repr_align_layout();

    println!("\n--- repr(transparent) ---");
    repr_transparent();

    println!("\n--- Enum Layout ---");
    enum_layout();

    println!("\n--- Union Layout ---");
    union_layout();

    println!("\n--- Practical Patterns ---");
    practical_patterns();
}

// ============================================
// Primitive Types
// ============================================

fn primitive_layout() {
    macro_rules! print_layout {
        ($t:ty) => {
            println!(
                "  {:12} size: {:2} bytes, align: {:2} bytes",
                stringify!($t),
                size_of::<$t>(),
                align_of::<$t>()
            );
        };
    }

    print_layout!(u8);
    print_layout!(u16);
    print_layout!(u32);
    print_layout!(u64);
    print_layout!(u128);
    print_layout!(usize);
    println!();
    print_layout!(f32);
    print_layout!(f64);
    println!();
    print_layout!(bool);
    print_layout!(char);
    println!();
    print_layout!(*const u8);
    print_layout!(&u8);
    print_layout!(&[u8]);
    print_layout!(&str);
}

// ============================================
// Default Struct Layout
// ============================================

/// Rust may reorder fields for optimal layout
struct DefaultLayout {
    a: u8,  // 1 byte
    b: u32, // 4 bytes
    c: u16, // 2 bytes
    d: u8,  // 1 byte
}

fn default_struct_layout() {
    println!("  struct DefaultLayout {{ a: u8, b: u32, c: u16, d: u8 }}");
    println!("    Size: {} bytes", size_of::<DefaultLayout>());
    println!("    Alignment: {} bytes", align_of::<DefaultLayout>());
    println!("    (Rust may reorder fields for better packing)");

    // Optimal manual ordering
    struct ManualOptimal {
        b: u32, // 4 bytes
        c: u16, // 2 bytes
        a: u8,  // 1 byte
        d: u8,  // 1 byte
    }

    println!("\n  struct ManualOptimal {{ b: u32, c: u16, a: u8, d: u8 }}");
    println!("    Size: {} bytes", size_of::<ManualOptimal>());
    println!("    Alignment: {} bytes", align_of::<ManualOptimal>());
}

// ============================================
// repr(C) Layout
// ============================================

/// C-compatible layout - fields in declaration order
#[repr(C)]
struct CLayout {
    a: u8, // offset 0
    // 3 bytes padding
    b: u32, // offset 4
    c: u16, // offset 8
    d: u8,  // offset 10
            // 1 byte padding
}

#[repr(C)]
struct CLayoutOptimal {
    b: u32, // offset 0
    c: u16, // offset 4
    a: u8,  // offset 6
    d: u8,  // offset 7
}

fn repr_c_layout() {
    println!("  #[repr(C)]");
    println!("  struct CLayout {{ a: u8, b: u32, c: u16, d: u8 }}");
    println!("    Size: {} bytes", size_of::<CLayout>());
    println!("    Alignment: {} bytes", align_of::<CLayout>());

    // Show field offsets
    let instance = CLayout {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
    };
    let base = &instance as *const _ as usize;

    unsafe {
        println!("    Field offsets:");
        println!(
            "      a: {} (u8)",
            (&instance.a as *const _ as usize) - base
        );
        println!(
            "      b: {} (u32)",
            (&instance.b as *const _ as usize) - base
        );
        println!(
            "      c: {} (u16)",
            (&instance.c as *const _ as usize) - base
        );
        println!(
            "      d: {} (u8)",
            (&instance.d as *const _ as usize) - base
        );
    }

    println!("\n  #[repr(C)]");
    println!("  struct CLayoutOptimal {{ b: u32, c: u16, a: u8, d: u8 }}");
    println!("    Size: {} bytes", size_of::<CLayoutOptimal>());
    println!("    Alignment: {} bytes", align_of::<CLayoutOptimal>());
}

// ============================================
// repr(packed) Layout
// ============================================

/// Packed layout - no padding (may cause unaligned access)
#[repr(C, packed)]
struct PackedLayout {
    a: u8,  // offset 0
    b: u32, // offset 1 (unaligned!)
    c: u16, // offset 5 (unaligned!)
    d: u8,  // offset 7
}

fn repr_packed_layout() {
    println!("  #[repr(C, packed)]");
    println!("  struct PackedLayout {{ a: u8, b: u32, c: u16, d: u8 }}");
    println!("    Size: {} bytes", size_of::<PackedLayout>());
    println!("    Alignment: {} bytes", align_of::<PackedLayout>());

    let instance = PackedLayout {
        a: 1,
        b: 2,
        c: 3,
        d: 4,
    };

    // Safe access through copy
    let a = { instance.a };
    let b = { instance.b };
    let c = { instance.c };
    let d = { instance.d };

    println!("    Values: a={}, b={}, c={}, d={}", a, b, c, d);

    println!("\n  ⚠️  Warning: packed structs may have unaligned fields");
    println!("     Taking references to unaligned fields is undefined behavior!");
    println!("     Safe: Copy fields by value");
    println!("     Unsafe: &instance.b (if unaligned)");

    // Packed with explicit alignment
    #[repr(C, packed(2))]
    struct Packed2 {
        a: u8,
        b: u32,
        c: u16,
    }

    println!("\n  #[repr(C, packed(2))]");
    println!("    Size: {} bytes", size_of::<Packed2>());
    println!("    Alignment: {} bytes", align_of::<Packed2>());
}

// ============================================
// repr(align) Layout
// ============================================

/// Increased alignment for cache line optimization
#[repr(C, align(64))]
struct CacheAligned {
    data: [u8; 32],
}

/// Align for SIMD operations
#[repr(C, align(32))]
struct SimdAligned {
    values: [f32; 8],
}

fn repr_align_layout() {
    println!("  #[repr(C, align(64))]");
    println!("  struct CacheAligned {{ data: [u8; 32] }}");
    println!("    Size: {} bytes", size_of::<CacheAligned>());
    println!("    Alignment: {} bytes", align_of::<CacheAligned>());

    println!("\n  #[repr(C, align(32))]");
    println!("  struct SimdAligned {{ values: [f32; 8] }}");
    println!("    Size: {} bytes", size_of::<SimdAligned>());
    println!("    Alignment: {} bytes", align_of::<SimdAligned>());

    // Verify alignment at runtime
    let aligned = CacheAligned { data: [0; 32] };
    let addr = &aligned as *const _ as usize;
    println!("\n  Runtime address check:");
    println!("    Address: 0x{:x}", addr);
    println!("    Aligned to 64: {}", addr % 64 == 0);
}

// ============================================
// repr(transparent)
// ============================================

/// Transparent wrapper - same layout as inner type
#[repr(transparent)]
struct Wrapper(u32);

#[repr(transparent)]
struct NewtypeString(String);

fn repr_transparent() {
    println!("  #[repr(transparent)]");
    println!("  struct Wrapper(u32)");
    println!("    Wrapper size: {} bytes", size_of::<Wrapper>());
    println!("    u32 size: {} bytes", size_of::<u32>());

    println!("\n  struct NewtypeString(String)");
    println!(
        "    NewtypeString size: {} bytes",
        size_of::<NewtypeString>()
    );
    println!("    String size: {} bytes", size_of::<String>());

    // Safe to transmute between transparent wrapper and inner type
    let w = Wrapper(42);
    let raw: u32 = unsafe { std::mem::transmute(w) };
    println!("\n  Transmute Wrapper(42) to u32: {}", raw);

    println!("\n  Use cases:");
    println!("    - FFI interop (pass newtype where C expects inner type)");
    println!("    - Zero-cost type safety");
}

// ============================================
// Enum Layout
// ============================================

/// C-like enum with explicit discriminants
#[repr(u8)]
enum Status {
    Ok = 0,
    Error = 1,
    Pending = 2,
}

/// Enum with data
#[repr(C)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

/// Option<&T> has niche optimization
fn enum_layout() {
    println!("  #[repr(u8)] enum Status {{ Ok=0, Error=1, Pending=2 }}");
    println!("    Size: {} bytes", size_of::<Status>());
    println!("    Alignment: {} bytes", align_of::<Status>());

    println!("\n  #[repr(C)] enum Message {{ Quit, Move{{x,y}}, Write(String) }}");
    println!("    Size: {} bytes", size_of::<Message>());
    println!("    Alignment: {} bytes", align_of::<Message>());

    // Niche optimization
    println!("\n  Niche optimization:");
    println!("    Option<u32> size: {} bytes", size_of::<Option<u32>>());
    println!("    Option<&u32> size: {} bytes", size_of::<Option<&u32>>());
    println!("    &u32 size: {} bytes", size_of::<&u32>());
    println!("    (Option<&T> uses null pointer for None!)");

    // NonZero optimization
    use std::num::NonZeroU32;
    println!("\n    NonZeroU32 size: {} bytes", size_of::<NonZeroU32>());
    println!(
        "    Option<NonZeroU32> size: {} bytes",
        size_of::<Option<NonZeroU32>>()
    );
}

// ============================================
// Union Layout
// ============================================

/// Union - all fields share same memory
#[repr(C)]
union IntBytes {
    int: u32,
    bytes: [u8; 4],
}

fn union_layout() {
    println!("  #[repr(C)]");
    println!("  union IntBytes {{ int: u32, bytes: [u8; 4] }}");
    println!("    Size: {} bytes", size_of::<IntBytes>());
    println!("    Alignment: {} bytes", align_of::<IntBytes>());

    let u = IntBytes { int: 0x12345678 };
    unsafe {
        println!("\n    int: 0x{:08x}", u.int);
        println!("    bytes: {:02x?}", u.bytes);

        #[cfg(target_endian = "little")]
        println!("    (little-endian: least significant byte first)");

        #[cfg(target_endian = "big")]
        println!("    (big-endian: most significant byte first)");
    }
}

// ============================================
// Practical Patterns
// ============================================

fn practical_patterns() {
    // Pattern 1: Network protocol header
    #[repr(C, packed)]
    struct PacketHeader {
        version: u8,
        flags: u8,
        length: u16,
        sequence: u32,
    }

    println!("  Network packet header (packed):");
    println!(
        "    Size: {} bytes (fixed wire format)",
        size_of::<PacketHeader>()
    );

    // Pattern 2: Cache-line aligned buffer for concurrent access
    #[repr(C, align(64))]
    struct PerCpuData {
        counter: std::sync::atomic::AtomicU64,
        _padding: [u8; 56], // Pad to full cache line
    }

    println!("\n  Per-CPU data (cache-line aligned):");
    println!("    Size: {} bytes", size_of::<PerCpuData>());
    println!(
        "    Alignment: {} bytes (prevents false sharing)",
        align_of::<PerCpuData>()
    );

    // Pattern 3: FFI struct matching C definition
    #[repr(C)]
    struct TimeVal {
        tv_sec: i64,
        tv_usec: i64,
    }

    println!("\n  FFI struct (C layout):");
    println!("    TimeVal size: {} bytes", size_of::<TimeVal>());

    // Pattern 4: Bit flags
    #[repr(transparent)]
    struct Flags(u32);

    impl Flags {
        const READ: u32 = 1 << 0;
        const WRITE: u32 = 1 << 1;
        const EXECUTE: u32 = 1 << 2;

        fn new() -> Self {
            Flags(0)
        }
        fn set(&mut self, flag: u32) {
            self.0 |= flag;
        }
        fn has(&self, flag: u32) -> bool {
            self.0 & flag != 0
        }
    }

    let mut flags = Flags::new();
    flags.set(Flags::READ);
    flags.set(Flags::EXECUTE);
    println!("\n  Bit flags (transparent wrapper):");
    println!("    Size: {} bytes", size_of::<Flags>());
    println!("    READ set: {}", flags.has(Flags::READ));
    println!("    WRITE set: {}", flags.has(Flags::WRITE));
    println!("    EXECUTE set: {}", flags.has(Flags::EXECUTE));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_sizes() {
        assert_eq!(size_of::<u8>(), 1);
        assert_eq!(size_of::<u32>(), 4);
        assert_eq!(size_of::<u64>(), 8);
    }

    #[test]
    fn test_c_layout_size() {
        // CLayout should have predictable size
        assert_eq!(size_of::<CLayout>(), 12);
        assert_eq!(size_of::<CLayoutOptimal>(), 8);
    }

    #[test]
    fn test_packed_size() {
        assert_eq!(size_of::<PackedLayout>(), 8); // No padding
    }

    #[test]
    fn test_aligned() {
        assert_eq!(align_of::<CacheAligned>(), 64);
        assert_eq!(align_of::<SimdAligned>(), 32);
    }

    #[test]
    fn test_transparent() {
        assert_eq!(size_of::<Wrapper>(), size_of::<u32>());
        assert_eq!(align_of::<Wrapper>(), align_of::<u32>());
    }

    #[test]
    fn test_option_niche() {
        // Option<&T> should be same size as &T due to niche optimization
        assert_eq!(size_of::<Option<&u32>>(), size_of::<&u32>());
    }

    #[test]
    fn test_union_size() {
        // Union size is the max of all variants
        assert_eq!(size_of::<IntBytes>(), 4);
    }
}
