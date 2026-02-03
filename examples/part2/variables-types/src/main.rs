//! Variables and Types Example
//!
//! Demonstrates Rust's variable declarations, mutability, and basic types.

// Allow approximate constants - these are intentional examples of float literals
#![allow(clippy::approx_constant)]

fn main() {
    println!("=== Variables and Types ===\n");

    // Immutable variables (default)
    let x = 5;
    println!("Immutable x = {}", x);
    // x = 6; // Error: cannot assign twice to immutable variable

    // Mutable variables
    let mut y = 10;
    println!("Mutable y = {}", y);
    y = 20;
    println!("After mutation, y = {}", y);

    // Type annotations
    let z: i32 = 42;
    let pi: f64 = 3.14159;
    println!("Annotated types: z = {}, pi = {}", z, pi);

    // Shadowing
    let shadow = 1;
    let shadow = shadow + 1;
    let shadow = shadow * 2;
    println!("Shadowed value: {}", shadow);

    // Can change type with shadowing
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Spaces (now a number): {}", spaces);

    println!("\n--- Scalar Types ---");
    scalar_types();

    println!("\n--- Compound Types ---");
    compound_types();

    println!("\n--- Type Inference ---");
    type_inference();
}

fn scalar_types() {
    // Integers
    let byte: u8 = 255;
    let signed: i8 = -128;
    let default_int = 42; // i32 by default
    let big: i64 = 9_223_372_036_854_775_807;
    println!(
        "Integers: u8={}, i8={}, i32={}, i64={}",
        byte, signed, default_int, big
    );

    // Integer literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte_literal = b'A';
    println!(
        "Literals: dec={}, hex={}, oct={}, bin={}, byte={}",
        decimal, hex, octal, binary, byte_literal
    );

    // Floating point
    let float32: f32 = 3.14;
    let float64 = 2.71828; // f64 by default
    println!("Floats: f32={}, f64={}", float32, float64);

    // Boolean
    let t = true;
    let f: bool = false;
    println!("Booleans: t={}, f={}", t, f);

    // Character (Unicode scalar value)
    let c = 'z';
    let emoji = '😀';
    let chinese = '中';
    println!("Characters: '{}', '{}', '{}'", c, emoji, chinese);
}

fn compound_types() {
    // Tuples
    let tuple: (i32, f64, char) = (500, 6.4, 'x');
    let (a, b, c) = tuple; // Destructuring
    println!("Tuple destructured: a={}, b={}, c={}", a, b, c);
    println!(
        "Tuple indexing: .0={}, .1={}, .2={}",
        tuple.0, tuple.1, tuple.2
    );

    // Unit tuple
    let unit: () = ();
    println!("Unit type: {:?}", unit);

    // Arrays (fixed size, stack allocated)
    let arr = [1, 2, 3, 4, 5];
    let first = arr[0];
    let last = arr[arr.len() - 1];
    println!("Array: {:?}, first={}, last={}", arr, first, last);

    // Array with type annotation
    let arr_typed: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Typed array: {:?}", arr_typed);

    // Array initialization with same value
    let zeros = [0; 5]; // [0, 0, 0, 0, 0]
    println!("Initialized array: {:?}", zeros);

    // Slices
    let slice = &arr[1..3];
    println!("Slice [1..3]: {:?}", slice);
}

fn type_inference() {
    // Rust infers types from usage
    let v = vec![1, 2, 3]; // Vec<i32> inferred
    println!("Inferred Vec<i32>: {:?}", v);

    // Sometimes annotation needed
    let parsed: i32 = "42".parse().expect("Not a number");
    println!("Parsed i32: {}", parsed);

    // Or use turbofish syntax
    let parsed2 = "42".parse::<i64>().expect("Not a number");
    println!("Parsed i64 (turbofish): {}", parsed2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_shadowing() {
        let x = 5;
        let x = x + 1;
        assert_eq!(x, 6);
    }

    #[test]
    fn test_tuple_access() {
        let tup = (1, 2.0, 'a');
        assert_eq!(tup.0, 1);
        assert_eq!(tup.1, 2.0);
        assert_eq!(tup.2, 'a');
    }

    #[test]
    fn test_array_slice() {
        let arr = [1, 2, 3, 4, 5];
        let slice = &arr[1..4];
        assert_eq!(slice, &[2, 3, 4]);
    }
}
