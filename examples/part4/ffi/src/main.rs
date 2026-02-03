//! Foreign Function Interface (FFI) Example
//!
//! Demonstrates calling C from Rust and exposing Rust to C.
//!
//! # FFI Data Flow
//! ```text
//!     ┌──────────────┐         ┌──────────────┐
//!     │   Rust Code  │ ──────► │   C Library  │
//!     │              │ extern  │              │
//!     │  safe code   │  "C"    │  libc, etc.  │
//!     └──────────────┘         └──────────────┘
//!            │                        │
//!            ▼                        ▼
//!     ┌──────────────┐         ┌──────────────┐
//!     │ #[repr(C)]   │         │ C-compatible │
//!     │ structs      │ ◄────── │ data types   │
//!     └──────────────┘         └──────────────┘
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_void};

// Link against the C standard library
#[link(name = "c")]
extern "C" {
    // Call C's abs function
    fn abs(input: c_int) -> c_int;

    // C memory allocation
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);

    // String functions
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    println!("=== Foreign Function Interface ===\n");

    println!("--- Calling C Functions ---");
    calling_c_functions();

    println!("\n--- C Strings ---");
    c_strings();

    println!("\n--- C-Compatible Structs ---");
    c_compatible_structs();

    println!("\n--- Exposing Rust to C ---");
    exposing_rust_to_c();

    println!("\n--- Callbacks ---");
    callbacks_example();

    println!("\n--- Safe Wrappers ---");
    safe_wrappers();
}

/// Calling C Functions from Rust
///
/// External functions are inherently unsafe because Rust
/// cannot verify the C code's safety guarantees.
fn calling_c_functions() {
    // Calling C's abs function
    unsafe {
        let result = abs(-42);
        println!("  abs(-42) = {}", result);
    }

    // Using libc crate for portable C functions
    unsafe {
        // Get process ID
        let pid = libc::getpid();
        println!("  Process ID: {}", pid);

        // Get current time
        let time = libc::time(std::ptr::null_mut());
        println!("  Unix timestamp: {}", time);
    }

    // Memory allocation through C
    unsafe {
        let size = 100;
        let ptr = malloc(size);
        if !ptr.is_null() {
            println!("  Allocated {} bytes at {:?}", size, ptr);
            free(ptr);
            println!("  Memory freed");
        }
    }
}

/// Working with C Strings
///
/// C strings are null-terminated, Rust strings are not.
/// CString: Rust-owned null-terminated string
/// CStr: Borrowed null-terminated string
fn c_strings() {
    // Creating a CString from Rust
    let rust_string = "Hello from Rust!";
    let c_string = CString::new(rust_string).expect("CString::new failed");

    // Get raw pointer for C functions
    let ptr = c_string.as_ptr();
    unsafe {
        let len = strlen(ptr);
        println!("  C string: {:?}", c_string);
        println!("  strlen() = {}", len);
    }

    // CStr from raw pointer (borrowed)
    let raw_ptr = c_string.as_ptr();
    unsafe {
        let c_str = CStr::from_ptr(raw_ptr);
        println!("  CStr: {:?}", c_str);

        // Convert back to Rust string
        let rust_str = c_str.to_str().expect("Invalid UTF-8");
        println!("  Back to Rust: {}", rust_str);
    }

    // Handling null bytes (C strings can't contain them)
    let has_null = "Hello\0World";
    match CString::new(has_null) {
        Ok(_) => println!("  Unexpected success"),
        Err(e) => println!("  CString error (expected): {}", e),
    }
}

/// C-Compatible Structs
///
/// #[repr(C)] ensures the struct has C-compatible memory layout.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Point {
    x: f64,
    y: f64,
}

#[repr(C)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn c_compatible_structs() {
    let p = Point { x: 1.0, y: 2.0 };
    println!("  Point: {:?}", p);
    println!("  Point size: {} bytes", std::mem::size_of::<Point>());

    let rect = Rectangle {
        top_left: Point { x: 0.0, y: 10.0 },
        bottom_right: Point { x: 10.0, y: 0.0 },
    };
    println!("  Rectangle: {:?}", rect);
    println!(
        "  Rectangle size: {} bytes",
        std::mem::size_of::<Rectangle>()
    );

    // repr(C) enum with explicit discriminants
    #[repr(C)]
    #[derive(Debug)]
    enum Status {
        Ok = 0,
        Error = 1,
        Pending = 2,
    }

    println!("  Status::Ok = {}", Status::Ok as i32);
    println!("  Status::Error = {}", Status::Error as i32);
}

/// Functions Exposed to C
///
/// Use #[no_mangle] to prevent name mangling.
/// Use extern "C" for C calling convention.

#[no_mangle]
pub extern "C" fn rust_add(a: c_int, b: c_int) -> c_int {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// # Safety
/// `name` must be a valid null-terminated C string pointer, or null.
#[no_mangle]
pub unsafe extern "C" fn rust_greet(name: *const c_char) {
    if name.is_null() {
        println!("  Hello, stranger!");
        return;
    }

    let c_str = CStr::from_ptr(name);
    match c_str.to_str() {
        Ok(s) => println!("  Hello, {}!", s),
        Err(_) => println!("  Hello, (invalid UTF-8)!"),
    }
}

fn exposing_rust_to_c() {
    // These functions could be called from C code
    let sum = rust_add(10, 20);
    println!("  rust_add(10, 20) = {}", sum);

    let product = rust_multiply(3.0, 4.5);
    println!("  rust_multiply(3.0, 4.5) = {}", product);

    let name = CString::new("World").unwrap();
    // SAFETY: name.as_ptr() returns a valid null-terminated C string
    unsafe {
        rust_greet(name.as_ptr());
        rust_greet(std::ptr::null());
    }
}

/// Callbacks from C to Rust
///
/// Pass Rust functions to C code as function pointers.
type Callback = extern "C" fn(c_int) -> c_int;

// Simulate a C function that takes a callback
fn call_with_callback(value: c_int, callback: Callback) -> c_int {
    callback(value)
}

extern "C" fn double_value(x: c_int) -> c_int {
    x * 2
}

extern "C" fn square_value(x: c_int) -> c_int {
    x * x
}

fn callbacks_example() {
    let result1 = call_with_callback(5, double_value);
    println!("  double_value(5) = {}", result1);

    let result2 = call_with_callback(5, square_value);
    println!("  square_value(5) = {}", result2);

    // Closures with extern "C" are not directly supported,
    // but you can use static functions or Box<dyn Fn>
}

/// Safe Wrappers Around Unsafe FFI
///
/// The pattern: expose a safe Rust API that internally
/// uses unsafe FFI calls.
fn safe_wrappers() {
    // Safe wrapper for C string length
    fn safe_strlen(s: &str) -> Option<usize> {
        let c_string = CString::new(s).ok()?;
        unsafe { Some(strlen(c_string.as_ptr())) }
    }

    println!("  safe_strlen(\"hello\") = {:?}", safe_strlen("hello"));
    println!(
        "  safe_strlen(\"hi\\0there\") = {:?}",
        safe_strlen("hi\0there")
    );

    // Safe wrapper for Point operations
    impl Point {
        fn new(x: f64, y: f64) -> Self {
            Point { x, y }
        }

        fn distance(&self, other: &Point) -> f64 {
            let dx = self.x - other.x;
            let dy = self.y - other.y;
            (dx * dx + dy * dy).sqrt()
        }
    }

    let p1 = Point::new(0.0, 0.0);
    let p2 = Point::new(3.0, 4.0);
    println!("  Distance: {}", p1.distance(&p2));

    // RAII wrapper for C resources
    struct CBuffer {
        ptr: *mut c_void,
        size: usize,
    }

    impl CBuffer {
        fn new(size: usize) -> Option<Self> {
            unsafe {
                let ptr = malloc(size);
                if ptr.is_null() {
                    None
                } else {
                    Some(CBuffer { ptr, size })
                }
            }
        }

        fn as_ptr(&self) -> *mut c_void {
            self.ptr
        }

        fn size(&self) -> usize {
            self.size
        }
    }

    impl Drop for CBuffer {
        fn drop(&mut self) {
            unsafe {
                free(self.ptr);
            }
        }
    }

    if let Some(buffer) = CBuffer::new(256) {
        println!(
            "  CBuffer allocated: {} bytes at {:?}",
            buffer.size(),
            buffer.as_ptr()
        );
        // Automatically freed when buffer goes out of scope
    }
    println!("  CBuffer automatically freed (RAII)");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_abs() {
        unsafe {
            assert_eq!(abs(-5), 5);
            assert_eq!(abs(5), 5);
            assert_eq!(abs(0), 0);
        }
    }

    #[test]
    fn test_cstring() {
        let s = CString::new("test").unwrap();
        unsafe {
            assert_eq!(strlen(s.as_ptr()), 4);
        }
    }

    #[test]
    fn test_rust_add() {
        assert_eq!(rust_add(2, 3), 5);
        assert_eq!(rust_add(-1, 1), 0);
    }

    #[test]
    fn test_repr_c_struct() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(std::mem::size_of::<Point>(), 16); // 2 * f64
    }

    #[test]
    fn test_callbacks() {
        assert_eq!(call_with_callback(3, double_value), 6);
        assert_eq!(call_with_callback(3, square_value), 9);
    }
}
