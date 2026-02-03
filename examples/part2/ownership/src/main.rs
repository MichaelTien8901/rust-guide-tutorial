//! Ownership Example
//!
//! Demonstrates Rust's ownership rules:
//! 1. Each value has exactly one owner
//! 2. When the owner goes out of scope, the value is dropped
//! 3. Ownership can be transferred (moved) or borrowed

fn main() {
    println!("=== Ownership ===\n");

    println!("--- Stack vs Heap ---");
    stack_vs_heap();

    println!("\n--- Move Semantics ---");
    move_semantics();

    println!("\n--- Clone ---");
    clone_example();

    println!("\n--- Copy Types ---");
    copy_types();

    println!("\n--- Ownership and Functions ---");
    ownership_and_functions();

    println!("\n--- Return Values and Ownership ---");
    return_values();
}

fn stack_vs_heap() {
    // Stack: fixed size, fast
    let x = 5; // Stored on stack
    let y = x; // Copy (both valid)
    println!("Stack values: x={}, y={}", x, y);

    // Heap: dynamic size, managed
    let s1 = String::from("hello"); // Stored on heap
    println!("Heap value s1: {}", s1);
    // String consists of: pointer, length, capacity (on stack)
    // Actual string data is on the heap
}

fn move_semantics() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is MOVED to s2

    // println!("{}", s1); // Error! s1 no longer valid
    println!("After move, s2 = {}", s2);

    // This prevents double-free errors
    // When s2 goes out of scope, only s2's data is freed
}

fn clone_example() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy

    // Both are valid!
    println!("s1 = {}", s1);
    println!("s2 = {}", s2);

    // Clone can be expensive for large data
    let big_vec = vec![0; 1000];
    let big_vec_clone = big_vec.clone();
    println!("Cloned vec length: {}", big_vec_clone.len());
}

fn copy_types() {
    // Types that implement Copy trait are copied, not moved
    // - All integer types
    // - bool
    // - char
    // - Floating point types
    // - Tuples of Copy types

    let x = 5;
    let y = x; // Copy, not move
    println!("Both valid: x={}, y={}", x, y);

    let tup = (1, 2, 3);
    let tup2 = tup; // Copy (all elements are Copy)
    println!("Both valid: {:?}, {:?}", tup, tup2);

    // References are Copy (but the referenced data is not)
    let s = String::from("hello");
    let r1 = &s;
    let r2 = r1; // Copy of the reference
    println!("Both refs valid: {}, {}", r1, r2);
}

fn ownership_and_functions() {
    let s = String::from("hello");

    takes_ownership(s); // s is moved into the function
                        // println!("{}", s); // Error! s no longer valid

    let x = 5;
    makes_copy(x); // x is copied
    println!("x still valid: {}", x); // x still valid
}

fn takes_ownership(s: String) {
    println!("Function received: {}", s);
} // s is dropped here

fn makes_copy(x: i32) {
    println!("Function received copy: {}", x);
}

fn return_values() {
    let s1 = gives_ownership(); // Return value moved to s1
    println!("Received ownership: {}", s1);

    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 moved in, return moved to s3
                                       // s2 is no longer valid
    println!("Got it back: {}", s3);

    // Common pattern: return tuple to give back ownership
    let s4 = String::from("hello");
    let (s5, len) = calculate_length(s4);
    println!("String '{}' has length {}", s5, len);
}

fn gives_ownership() -> String {
    let s = String::from("yours");
    s // Returned, ownership transferred to caller
}

fn takes_and_gives_back(s: String) -> String {
    s // Returned, ownership transferred back
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length) // Return both to give back ownership
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clone() {
        let s1 = String::from("test");
        let s2 = s1.clone();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_copy_types() {
        let x = 42;
        let y = x;
        assert_eq!(x, y);
    }

    #[test]
    fn test_return_ownership() {
        let s = gives_ownership();
        assert_eq!(s, "yours");
    }
}
