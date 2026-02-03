//! Advanced Types Example
//!
//! Demonstrates newtype pattern, type aliases, and dynamically sized types.
//!
//! # Type System Features
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                  Advanced Type Features                 │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  Newtype:       struct Meters(f64);                     │
//!     │  ├── Type safety at zero runtime cost                   │
//!     │  ├── Implement foreign traits on foreign types          │
//!     │  └── Hide implementation details                        │
//!     │                                                         │
//!     │  Type Alias:    type Kilometers = f64;                  │
//!     │  ├── Shorthand for complex types                        │
//!     │  └── No type safety (just a synonym)                    │
//!     │                                                         │
//!     │  DST:           str, [T], dyn Trait                     │
//!     │  ├── Size not known at compile time                     │
//!     │  └── Must be behind pointer (&str, Box<[T]>)            │
//!     │                                                         │
//!     │  Never Type:    ! (never returns)                       │
//!     │  ├── Functions that diverge                             │
//!     │  └── Useful for exhaustive matching                     │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use std::fmt::{Display, Formatter};
use std::ops::{Add, Deref, Mul};

fn main() {
    println!("=== Advanced Types ===\n");

    println!("--- Newtype Pattern ---");
    newtype_pattern();

    println!("\n--- Type Aliases ---");
    type_aliases();

    println!("\n--- Dynamically Sized Types ---");
    dynamically_sized_types();

    println!("\n--- The Never Type ---");
    never_type();

    println!("\n--- Newtype for Traits ---");
    newtype_for_traits();

    println!("\n--- Zero-Cost Abstractions ---");
    zero_cost_abstractions();
}

// ============================================
// Newtype Pattern
// ============================================

/// Newtype for type safety
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Meters(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Kilometers(f64);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Miles(f64);

impl Meters {
    fn new(value: f64) -> Self {
        Meters(value)
    }

    fn to_kilometers(self) -> Kilometers {
        Kilometers(self.0 / 1000.0)
    }

    fn to_miles(self) -> Miles {
        Miles(self.0 / 1609.344)
    }
}

impl Add for Meters {
    type Output = Meters;
    fn add(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

impl Display for Meters {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}m", self.0)
    }
}

// Conversion from Kilometers to Meters
impl From<Kilometers> for Meters {
    fn from(km: Kilometers) -> Meters {
        Meters(km.0 * 1000.0)
    }
}

fn newtype_pattern() {
    let distance = Meters::new(5000.0);
    println!("  Distance: {}", distance);
    println!("  As kilometers: {:?}", distance.to_kilometers());
    println!("  As miles: {:?}", distance.to_miles());

    // Type safety: can't accidentally mix units
    let m1 = Meters::new(100.0);
    let m2 = Meters::new(50.0);
    let sum = m1 + m2;
    println!("  {} + {} = {}", m1, m2, sum);

    // This won't compile (type mismatch):
    // let invalid = Meters::new(100.0) + Kilometers(1.0);

    // Must explicitly convert
    let km = Kilometers(5.0);
    let meters: Meters = km.into();
    println!("  5km = {}", meters);
}

// ============================================
// Type Aliases
// ============================================

/// Type alias for complex types
type Thunk = Box<dyn Fn() + Send + 'static>;

/// Type alias for Results with common error type
type IoResult<T> = Result<T, std::io::Error>;

/// Type alias for nested generics
type StringMap<V> = std::collections::HashMap<String, V>;

fn type_aliases() {
    // Thunk alias makes code cleaner
    fn create_thunk() -> Thunk {
        Box::new(|| println!("    Hello from thunk!"))
    }

    let thunk = create_thunk();
    thunk();

    // Using IoResult
    fn read_file(path: &str) -> IoResult<String> {
        std::fs::read_to_string(path)
    }

    match read_file("/nonexistent") {
        Ok(content) => println!("  Content: {}", content),
        Err(_) => println!("  Could not read file (expected)"),
    }

    // StringMap alias
    let mut map: StringMap<i32> = StringMap::new();
    map.insert("one".to_string(), 1);
    map.insert("two".to_string(), 2);
    println!("  StringMap: {:?}", map);

    // Note: type aliases are NOT distinct types
    type Age = u32;
    type Year = u32;

    let age: Age = 30;
    let year: Year = 2024;
    // This compiles (both are u32) - no type safety!
    let sum = age + year;
    println!("  age + year = {} (type alias has no safety)", sum);
}

// ============================================
// Dynamically Sized Types
// ============================================

fn dynamically_sized_types() {
    // str is a DST - must use &str or Box<str>
    let s: &str = "Hello";
    println!("  &str: {}", s);

    // [T] is a DST - must use &[T] or Box<[T]>
    let slice: &[i32] = &[1, 2, 3];
    println!("  &[i32]: {:?}", slice);

    // Convert Vec to Box<[T]>
    let boxed_slice: Box<[i32]> = vec![1, 2, 3].into_boxed_slice();
    println!("  Box<[i32]>: {:?}", boxed_slice);

    // dyn Trait is a DST
    trait Speak {
        fn speak(&self);
    }

    struct Dog;
    impl Speak for Dog {
        fn speak(&self) {
            println!("    Woof!");
        }
    }

    // Must use reference or Box
    let speaker: &dyn Speak = &Dog;
    speaker.speak();

    let boxed_speaker: Box<dyn Speak> = Box::new(Dog);
    boxed_speaker.speak();

    // Sized trait and ?Sized
    fn sized_only<T: Sized>(t: T) {
        let _ = std::mem::size_of_val(&t);
    }

    fn accepts_unsized<T: ?Sized>(t: &T) {
        // T might not be Sized
        println!("  Size at runtime: {} bytes", std::mem::size_of_val(t));
    }

    accepts_unsized("hello"); // &str works
    accepts_unsized(&[1, 2, 3][..]); // &[i32] works
}

// ============================================
// The Never Type
// ============================================

fn never_type() {
    // ! is the never type - functions that never return

    // panic! returns !
    fn always_panic() -> ! {
        panic!("This function never returns!");
    }

    // loop without break returns !
    fn infinite() -> ! {
        loop {
            // Never exits
        }
    }

    // Useful in match expressions
    let value: Option<i32> = Some(5);
    let number = match value {
        Some(n) => n,
        None => {
            // panic! returns !, which coerces to i32
            panic!("No value!");
        }
    };
    println!("  Match with panic: {}", number);

    // continue returns !
    let numbers = vec![1, 2, 3, 4, 5];
    let result: Vec<i32> = numbers
        .iter()
        .map(|&n| {
            if n % 2 == 0 {
                n * 2
            } else {
                // Would normally be incompatible, but ! coerces
                // continue // not in loop context
                n
            }
        })
        .collect();
    println!("  Processed: {:?}", result);

    // Useful for exhaustive error handling
    fn process(input: &str) -> i32 {
        input.parse().unwrap_or_else(|_| {
            eprintln!("    Invalid input, using default");
            0 // Returns i32
        })
    }
    println!("  process(\"42\"): {}", process("42"));
    println!("  process(\"invalid\"): {}", process("invalid"));
}

// ============================================
// Newtype for Implementing Foreign Traits
// ============================================

// Can't implement Display for Vec<T> directly (orphan rule)
// But can with newtype
struct Wrapper<T>(Vec<T>);

impl<T: Display> Display for Wrapper<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", item)?;
        }
        write!(f, "]")
    }
}

// Implement Deref for transparent access
impl<T> Deref for Wrapper<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn newtype_for_traits() {
    let w = Wrapper(vec![1, 2, 3, 4, 5]);

    // Use Display (our implementation)
    println!("  Custom display: {}", w);

    // Use Vec methods through Deref
    println!("  Length: {}", w.len());
    println!("  First: {:?}", w.first());

    // Hiding implementation
    struct Password(String);

    impl Password {
        fn new(s: &str) -> Self {
            Password(s.to_string())
        }
    }

    // Don't expose the internal String
    impl Display for Password {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "********")
        }
    }

    let pw = Password::new("secret123");
    println!("  Password: {}", pw);
}

// ============================================
// Zero-Cost Abstractions
// ============================================

/// Newtype with no runtime cost
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct Percentage(f64);

impl Percentage {
    fn new(value: f64) -> Option<Self> {
        if (0.0..=100.0).contains(&value) {
            Some(Percentage(value))
        } else {
            None
        }
    }

    fn value(&self) -> f64 {
        self.0
    }

    fn as_fraction(&self) -> f64 {
        self.0 / 100.0
    }
}

impl Mul<f64> for Percentage {
    type Output = f64;
    fn mul(self, rhs: f64) -> f64 {
        rhs * self.as_fraction()
    }
}

fn zero_cost_abstractions() {
    // Same size as f64
    println!("  Size of f64: {} bytes", std::mem::size_of::<f64>());
    println!(
        "  Size of Percentage: {} bytes",
        std::mem::size_of::<Percentage>()
    );

    // Type-safe percentage
    let discount = Percentage::new(20.0).unwrap();
    let price = 100.0;
    let savings = discount * price;

    println!("  {}% of {} = {}", discount.value(), price, savings);

    // Invalid percentage rejected at construction
    match Percentage::new(150.0) {
        Some(_) => println!("  Created 150%"),
        None => println!("  Rejected 150% (out of range)"),
    }

    // PhantomData for type-level markers
    use std::marker::PhantomData;

    struct Id<T> {
        value: u64,
        _marker: PhantomData<T>,
    }

    impl<T> Id<T> {
        fn new(value: u64) -> Self {
            Id {
                value,
                _marker: PhantomData,
            }
        }
    }

    struct User;
    struct Product;

    let user_id: Id<User> = Id::new(1);
    let product_id: Id<Product> = Id::new(1);

    // Can't mix them up, even though both are u64 internally
    // user_id == product_id  // Won't compile!

    println!("  User ID: {}", user_id.value);
    println!("  Product ID: {}", product_id.value);
    println!(
        "  PhantomData size: {} bytes",
        std::mem::size_of::<PhantomData<User>>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_conversion() {
        let m = Meters::new(1609.344);
        let miles = m.to_miles();
        assert!((miles.0 - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_meters_addition() {
        let m1 = Meters::new(100.0);
        let m2 = Meters::new(50.0);
        assert_eq!((m1 + m2).0, 150.0);
    }

    #[test]
    fn test_percentage_valid() {
        assert!(Percentage::new(50.0).is_some());
        assert!(Percentage::new(0.0).is_some());
        assert!(Percentage::new(100.0).is_some());
    }

    #[test]
    fn test_percentage_invalid() {
        assert!(Percentage::new(-1.0).is_none());
        assert!(Percentage::new(101.0).is_none());
    }

    #[test]
    fn test_percentage_calculation() {
        let pct = Percentage::new(25.0).unwrap();
        let result = pct * 200.0;
        assert!((result - 50.0).abs() < 0.001);
    }

    #[test]
    fn test_wrapper_display() {
        let w = Wrapper(vec![1, 2, 3]);
        let s = format!("{}", w);
        assert_eq!(s, "[1, 2, 3]");
    }
}
