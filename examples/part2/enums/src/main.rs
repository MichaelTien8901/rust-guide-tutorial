//! Enums and Pattern Matching Example
//!
//! Demonstrates enum definitions, variants with data,
//! Option, Result, and pattern matching.

fn main() {
    println!("=== Enums and Pattern Matching ===\n");

    println!("--- Basic Enums ---");
    basic_enums();

    println!("\n--- Enums with Data ---");
    enums_with_data();

    println!("\n--- Option Type ---");
    option_type();

    println!("\n--- Match Expression ---");
    match_expression();

    println!("\n--- if let ---");
    if_let_syntax();

    println!("\n--- Enum Methods ---");
    enum_methods();
}

// Basic enum
#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn basic_enums() {
    let dir = Direction::North;
    println!("Direction: {:?}", dir);

    // Using in match
    let description = match dir {
        Direction::North => "Going up",
        Direction::South => "Going down",
        Direction::East => "Going right",
        Direction::West => "Going left",
    };
    println!("Description: {}", description);
}

// Enum with associated data
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,                       // No data
    Move { x: i32, y: i32 },    // Named fields (like struct)
    Write(String),              // Single value
    ChangeColor(i32, i32, i32), // Multiple values
}

fn enums_with_data() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // Extracting data with match
    match home {
        IpAddr::V4(a, b, c, d) => {
            println!("IPv4: {}.{}.{}.{}", a, b, c, d);
        }
        IpAddr::V6(addr) => {
            println!("IPv6: {}", addr);
        }
    }

    // Message variants
    let messages = vec![
        Message::Quit,
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("Hello")),
        Message::ChangeColor(255, 0, 0),
    ];

    for msg in messages {
        println!("Message: {:?}", msg);
    }
}

fn option_type() {
    // Option<T> is either Some(T) or None
    let some_number: Option<i32> = Some(5);
    let no_number: Option<i32> = None;

    println!("some_number: {:?}", some_number);
    println!("no_number: {:?}", no_number);

    // Using Option with match
    let x = Some(5);
    let result = match x {
        Some(i) => i + 1,
        None => 0,
    };
    println!("Result: {}", result);

    // Common Option methods
    let opt = Some(42);

    // unwrap - panics if None
    println!("unwrap: {}", opt.unwrap());

    // unwrap_or - default if None
    let none: Option<i32> = None;
    println!("unwrap_or: {}", none.unwrap_or(0));

    // unwrap_or_else - compute default lazily
    println!("unwrap_or_else: {}", none.unwrap_or_else(|| 2 + 2));

    // map - transform Some value
    let doubled = opt.map(|x| x * 2);
    println!("mapped: {:?}", doubled);

    // and_then - chain operations
    let result = opt.and_then(|x| Some(x.to_string()));
    println!("and_then: {:?}", result);

    // is_some / is_none
    println!("is_some: {}, is_none: {}", opt.is_some(), opt.is_none());
}

fn match_expression() {
    // Match is exhaustive - must cover all cases
    let number = 7;

    match number {
        1 => println!("One"),
        2 | 3 | 5 | 7 | 11 => println!("Prime"),
        13..=19 => println!("Teen"),
        _ => println!("Something else"),
    }

    // Match with guards
    let pair = (2, -2);
    match pair {
        (x, y) if x == y => println!("Equal"),
        (x, y) if x + y == 0 => println!("Sum to zero"),
        (x, _) if x % 2 == 0 => println!("First is even"),
        _ => println!("No match"),
    }

    // Destructuring in match
    let point = (3, 5);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On y-axis at {}", y),
        (x, 0) => println!("On x-axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }

    // Match with bindings
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
        Message::Write(text) => println!("Write: {}", text),
        Message::ChangeColor(r, g, b) => println!("Color: rgb({}, {}, {})", r, g, b),
    }

    // @ bindings
    let num = 5;
    match num {
        n @ 1..=5 => println!("Small number: {}", n),
        n @ 6..=10 => println!("Medium number: {}", n),
        n => println!("Large number: {}", n),
    }
}

fn if_let_syntax() {
    // if let for single pattern matching
    let some_value = Some(3);

    // Verbose match
    match some_value {
        Some(3) => println!("Three!"),
        _ => (),
    }

    // Concise if let
    if let Some(3) = some_value {
        println!("Three with if let!");
    }

    // if let with else
    if let Some(x) = some_value {
        println!("Got value: {}", x);
    } else {
        println!("No value");
    }

    // while let
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }

    // let else (Rust 1.65+)
    fn get_value() -> Option<i32> {
        Some(42)
    }

    let Some(value) = get_value() else {
        println!("No value!");
        return;
    };
    println!("Got value with let-else: {}", value);
}

// Enums can have methods
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quitting..."),
            Message::Move { x, y } => println!("Moving to ({}, {})", x, y),
            Message::Write(text) => println!("Writing: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("Changing color to rgb({}, {}, {})", r, g, b)
            }
        }
    }
}

fn enum_methods() {
    let messages = vec![
        Message::Quit,
        Message::Move { x: 5, y: 10 },
        Message::Write(String::from("Hello, world!")),
        Message::ChangeColor(0, 255, 0),
    ];

    for msg in &messages {
        msg.call();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_option_methods() {
        let some = Some(5);
        let none: Option<i32> = None;

        assert_eq!(some.unwrap(), 5);
        assert_eq!(none.unwrap_or(0), 0);
        assert_eq!(some.map(|x| x * 2), Some(10));
    }

    #[test]
    fn test_pattern_matching() {
        let result = match Some(42) {
            Some(n) => n,
            None => 0,
        };
        assert_eq!(result, 42);
    }

    #[test]
    fn test_ip_addr() {
        let v4 = IpAddr::V4(192, 168, 1, 1);
        match v4 {
            IpAddr::V4(a, _, _, _) => assert_eq!(a, 192),
            _ => panic!("Expected V4"),
        }
    }
}
