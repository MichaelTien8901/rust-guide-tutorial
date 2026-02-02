//! Unsafe Rust Example
//!
//! Demonstrates the five unsafe superpowers and safety invariants.
//!
//! # Unsafe Superpowers Flow
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │                    Unsafe Superpowers                       │
//! ├─────────────────────────────────────────────────────────────┤
//! │  1. Dereference raw pointers (*const T, *mut T)             │
//! │  2. Call unsafe functions or methods                        │
//! │  3. Access or modify mutable static variables               │
//! │  4. Implement unsafe traits                                 │
//! │  5. Access fields of unions                                 │
//! └─────────────────────────────────────────────────────────────┘
//! ```

use std::slice;

fn main() {
    println!("=== Unsafe Rust ===\n");

    println!("--- Raw Pointers ---");
    raw_pointers();

    println!("\n--- Unsafe Functions ---");
    unsafe_functions();

    println!("\n--- Mutable Static Variables ---");
    mutable_statics();

    println!("\n--- Unsafe Traits ---");
    unsafe_traits();

    println!("\n--- Unions ---");
    unions_example();

    println!("\n--- Safe Abstractions ---");
    safe_abstractions();
}

/// Raw Pointer Operations
///
/// Raw pointers can be created in safe code, but can only
/// be dereferenced in unsafe blocks.
fn raw_pointers() {
    let mut num = 5;

    // Creating raw pointers (safe)
    let r1 = &num as *const i32;      // immutable raw pointer
    let r2 = &mut num as *mut i32;    // mutable raw pointer

    println!("Raw pointers created:");
    println!("  r1 (const): {:?}", r1);
    println!("  r2 (mut):   {:?}", r2);

    // Dereferencing raw pointers (unsafe)
    unsafe {
        println!("  *r1 = {}", *r1);
        println!("  *r2 = {}", *r2);

        // Modify through mutable raw pointer
        *r2 = 10;
        println!("  After *r2 = 10: num = {}", *r1);
    }

    // Creating pointer to arbitrary address (dangerous!)
    let address = 0x012345usize;
    let _r = address as *const i32;
    println!("  Arbitrary address pointer: {:?} (not dereferenced!)", _r);

    // Pointer from reference is always valid
    let arr = [1, 2, 3, 4, 5];
    let ptr = arr.as_ptr();
    unsafe {
        println!("  Array via pointer: [{}, {}, {}]", *ptr, *ptr.add(1), *ptr.add(2));
    }
}

/// Unsafe Functions and Methods
///
/// Some operations are inherently unsafe and require the caller
/// to uphold certain invariants.
fn unsafe_functions() {
    // Calling unsafe function
    unsafe fn dangerous() {
        println!("  Called dangerous()");
    }

    unsafe {
        dangerous();
    }

    // slice::from_raw_parts - must ensure:
    // 1. Pointer is valid for len * size_of::<T>() bytes
    // 2. Pointer is properly aligned
    // 3. Memory is initialized
    // 4. No aliasing violations
    let data = vec![1, 2, 3, 4, 5];
    let ptr = data.as_ptr();
    let len = data.len();

    unsafe {
        let slice = slice::from_raw_parts(ptr, len);
        println!("  Slice from raw parts: {:?}", slice);
    }

    // Safe wrapper around unsafe code
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len, "mid out of bounds");

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut v, 3);
    println!("  Split: {:?} | {:?}", left, right);
}

/// Mutable Static Variables
///
/// Static variables have a fixed address in memory.
/// Mutable statics are unsafe because of potential data races.
static GREETING: &str = "Hello";
static mut COUNTER: u32 = 0;

fn mutable_statics() {
    println!("  Immutable static: {}", GREETING);

    // Accessing mutable static requires unsafe
    unsafe {
        COUNTER += 1;
        println!("  COUNTER after increment: {}", COUNTER);

        add_to_counter(5);
        println!("  COUNTER after add_to_counter(5): {}", COUNTER);
    }
}

/// # Safety
/// Caller must ensure no concurrent access to COUNTER
unsafe fn add_to_counter(inc: u32) {
    COUNTER += inc;
}

/// Unsafe Traits
///
/// A trait is unsafe when at least one of its methods has
/// invariants that the compiler can't verify.
unsafe trait Zeroable {
    // Implementors must ensure the type can be safely
    // represented as all zeros
}

// Safe to implement for primitive numeric types
unsafe impl Zeroable for i32 {}
unsafe impl Zeroable for u32 {}
unsafe impl Zeroable for f64 {}

fn unsafe_traits() {
    fn zero<T: Zeroable + Default>() -> T {
        T::default()
    }

    let x: i32 = zero();
    let y: u32 = zero();
    println!("  Zeroed i32: {}, u32: {}", x, y);

    // Send and Sync are unsafe traits
    // - Send: safe to transfer between threads
    // - Sync: safe to share references between threads
    println!("  Send/Sync are auto-implemented for most types");
}

/// Unions
///
/// Unions allow multiple interpretations of the same memory.
/// Reading union fields is unsafe because Rust can't guarantee
/// which field is currently valid.
#[repr(C)]
union IntOrFloat {
    i: i32,
    f: f32,
}

fn unions_example() {
    let mut u = IntOrFloat { i: 42 };

    unsafe {
        println!("  As integer: {}", u.i);
        println!("  As float: {} (reinterpreted bits)", u.f);

        u.f = 3.14;
        println!("  After setting float to 3.14:");
        println!("    As float: {}", u.f);
        println!("    As integer: {} (raw bits)", u.i);
    }

    // Type punning example
    #[repr(C)]
    union Bytes {
        value: u32,
        bytes: [u8; 4],
    }

    let bytes = Bytes { value: 0x12345678 };
    unsafe {
        println!("  0x12345678 as bytes: {:02x?}", bytes.bytes);
    }
}

/// Safe Abstractions Over Unsafe Code
///
/// The key pattern: unsafe implementation, safe interface.
fn safe_abstractions() {
    // A safe Vec-like type built on unsafe
    struct SimpleVec<T> {
        ptr: *mut T,
        len: usize,
        cap: usize,
    }

    impl<T> SimpleVec<T> {
        fn new() -> Self {
            SimpleVec {
                ptr: std::ptr::null_mut(),
                len: 0,
                cap: 0,
            }
        }

        fn with_capacity(cap: usize) -> Self {
            let layout = std::alloc::Layout::array::<T>(cap).unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) as *mut T };
            SimpleVec { ptr, len: 0, cap }
        }

        fn push(&mut self, value: T) {
            if self.len == self.cap {
                panic!("SimpleVec is full!");
            }
            unsafe {
                self.ptr.add(self.len).write(value);
            }
            self.len += 1;
        }

        fn get(&self, index: usize) -> Option<&T> {
            if index < self.len {
                unsafe { Some(&*self.ptr.add(index)) }
            } else {
                None
            }
        }

        fn len(&self) -> usize {
            self.len
        }
    }

    impl<T> Drop for SimpleVec<T> {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe {
                    // Drop all elements
                    for i in 0..self.len {
                        std::ptr::drop_in_place(self.ptr.add(i));
                    }
                    // Deallocate memory
                    if self.cap > 0 {
                        let layout = std::alloc::Layout::array::<T>(self.cap).unwrap();
                        std::alloc::dealloc(self.ptr as *mut u8, layout);
                    }
                }
            }
        }
    }

    let mut vec = SimpleVec::with_capacity(10);
    vec.push(1);
    vec.push(2);
    vec.push(3);

    println!("  SimpleVec length: {}", vec.len());
    println!("  SimpleVec[0]: {:?}", vec.get(0));
    println!("  SimpleVec[1]: {:?}", vec.get(1));
    println!("  SimpleVec[5]: {:?}", vec.get(5));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_pointer() {
        let x = 42;
        let ptr = &x as *const i32;
        unsafe {
            assert_eq!(*ptr, 42);
        }
    }

    #[test]
    fn test_mutable_raw_pointer() {
        let mut x = 10;
        let ptr = &mut x as *mut i32;
        unsafe {
            *ptr = 20;
        }
        assert_eq!(x, 20);
    }

    #[test]
    fn test_slice_from_raw_parts() {
        let arr = [1, 2, 3, 4, 5];
        let ptr = arr.as_ptr();
        unsafe {
            let slice = std::slice::from_raw_parts(ptr.add(1), 3);
            assert_eq!(slice, &[2, 3, 4]);
        }
    }

    #[test]
    fn test_union() {
        let u = IntOrFloat { i: 0x40490FDB }; // bits for ~3.14159
        unsafe {
            let f = u.f;
            assert!((f - std::f32::consts::PI).abs() < 0.0001);
        }
    }
}
