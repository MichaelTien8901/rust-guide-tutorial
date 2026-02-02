//! Error Handling Example
//!
//! Demonstrates Rust's error handling with Result, Option,
//! the ? operator, and custom error types.

use std::fs::File;
use std::io::{self, Read};

fn main() {
    println!("=== Error Handling ===\n");

    println!("--- Result Type ---");
    result_type();

    println!("\n--- The ? Operator ---");
    question_mark_operator();

    println!("\n--- Option for Absence ---");
    option_handling();

    println!("\n--- Custom Errors ---");
    custom_errors();

    println!("\n--- Error Conversion ---");
    error_conversion();

    println!("\n--- Recoverable vs Unrecoverable ---");
    recoverable_vs_unrecoverable();
}

fn result_type() {
    // Result<T, E> is Ok(T) or Err(E)
    let result: Result<i32, &str> = Ok(42);
    let error: Result<i32, &str> = Err("something went wrong");

    println!("result: {:?}", result);
    println!("error: {:?}", error);

    // Handling with match
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(file) => println!("File opened: {:?}", file),
        Err(e) => println!("Failed to open file: {}", e),
    }

    // Common Result methods
    let ok: Result<i32, &str> = Ok(5);

    // unwrap - panics on Err
    // ok.unwrap();

    // expect - panics with message
    // ok.expect("Failed to get value");

    // unwrap_or - default on Err
    println!("unwrap_or: {}", ok.unwrap_or(0));

    // unwrap_or_else - compute default
    let err: Result<i32, &str> = Err("error");
    let val = err.unwrap_or_else(|e| {
        println!("Error: {}", e);
        0
    });
    println!("unwrap_or_else result: {}", val);

    // map - transform Ok value
    let doubled = ok.map(|x| x * 2);
    println!("mapped: {:?}", doubled);

    // map_err - transform Err value
    let mapped_err = err.map_err(|e| format!("Error: {}", e));
    println!("mapped_err: {:?}", mapped_err);

    // and_then - chain operations
    let result = ok.and_then(|x| {
        if x > 0 {
            Ok(x * 2)
        } else {
            Err("negative number")
        }
    });
    println!("and_then: {:?}", result);

    // is_ok / is_err
    println!("is_ok: {}, is_err: {}", ok.is_ok(), ok.is_err());
}

fn question_mark_operator() {
    // The ? operator propagates errors
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {}", e),
    }

    // Chaining with ?
    match read_username_short() {
        Ok(username) => println!("Username (short): {}", username),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut file = File::open("username.txt")?; // Returns early if Err
    let mut username = String::new();
    file.read_to_string(&mut username)?; // Returns early if Err
    Ok(username)
}

fn read_username_short() -> Result<String, io::Error> {
    // Even shorter with method chaining
    let mut username = String::new();
    File::open("username.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn option_handling() {
    // Option for values that might not exist
    let numbers = vec![1, 2, 3];

    // get returns Option
    let first = numbers.get(0);
    let tenth = numbers.get(10);

    println!("first: {:?}", first);
    println!("tenth: {:?}", tenth);

    // ? works with Option in functions returning Option
    fn get_first_char(s: &str) -> Option<char> {
        s.chars().next()
    }

    fn get_first_initial(name: &str) -> Option<char> {
        // ? returns None early if None
        let first_name = name.split_whitespace().next()?;
        get_first_char(first_name)
    }

    println!("Initial: {:?}", get_first_initial("John Doe"));
    println!("Initial: {:?}", get_first_initial(""));

    // Converting Option to Result
    let opt: Option<i32> = Some(42);
    let result: Result<i32, &str> = opt.ok_or("no value");
    println!("Option to Result: {:?}", result);
}

fn custom_errors() {
    // Define custom error type
    #[derive(Debug)]
    enum MathError {
        DivisionByZero,
        NegativeSquareRoot,
        Overflow,
    }

    fn divide(a: i32, b: i32) -> Result<i32, MathError> {
        if b == 0 {
            Err(MathError::DivisionByZero)
        } else {
            Ok(a / b)
        }
    }

    fn sqrt(x: f64) -> Result<f64, MathError> {
        if x < 0.0 {
            Err(MathError::NegativeSquareRoot)
        } else {
            Ok(x.sqrt())
        }
    }

    // Using custom errors
    match divide(10, 2) {
        Ok(result) => println!("10 / 2 = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {:?}", e),
    }

    match sqrt(-4.0) {
        Ok(result) => println!("sqrt = {}", result),
        Err(e) => println!("Error: {:?}", e),
    }
}

fn error_conversion() {
    // Box<dyn Error> for generic error handling
    use std::error::Error;

    fn fallible_operation() -> Result<String, Box<dyn Error>> {
        // Can return different error types
        let _file = File::open("test.txt")?;
        let _num: i32 = "not a number".parse()?;
        Ok("success".to_string())
    }

    match fallible_operation() {
        Ok(s) => println!("Success: {}", s),
        Err(e) => println!("Error (boxed): {}", e),
    }

    // From trait for automatic conversion
    #[derive(Debug)]
    enum AppError {
        Io(io::Error),
        Parse(std::num::ParseIntError),
    }

    impl From<io::Error> for AppError {
        fn from(e: io::Error) -> Self {
            AppError::Io(e)
        }
    }

    impl From<std::num::ParseIntError> for AppError {
        fn from(e: std::num::ParseIntError) -> Self {
            AppError::Parse(e)
        }
    }

    fn app_operation() -> Result<i32, AppError> {
        // ? automatically converts errors
        let content = "42";
        let num: i32 = content.parse()?; // ParseIntError -> AppError
        Ok(num)
    }

    match app_operation() {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("App error: {:?}", e),
    }
}

fn recoverable_vs_unrecoverable() {
    // Recoverable: use Result
    fn might_fail(succeed: bool) -> Result<i32, &'static str> {
        if succeed {
            Ok(42)
        } else {
            Err("operation failed")
        }
    }

    // Handle or propagate
    let result = might_fail(true);
    if let Ok(value) = result {
        println!("Recovered value: {}", value);
    }

    // Unrecoverable: panic!
    // - Use for bugs, not expected failures
    // - Use for prototyping (unwrap, expect)

    // panic!("This would crash the program");

    // assert! macros for invariants
    let x = 5;
    assert!(x > 0, "x must be positive");
    assert_eq!(x, 5, "x must be 5");
    assert_ne!(x, 0, "x must not be zero");
    println!("All assertions passed!");

    // When to use which:
    // - Result: expected failures (file not found, parse error)
    // - panic!: bugs, violated invariants, unrecoverable state
    // - Option: absence of value (no error, just missing)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_methods() {
        let ok: Result<i32, &str> = Ok(5);
        let err: Result<i32, &str> = Err("error");

        assert!(ok.is_ok());
        assert!(err.is_err());
        assert_eq!(ok.unwrap(), 5);
        assert_eq!(err.unwrap_or(0), 0);
    }

    #[test]
    fn test_option_conversion() {
        let some: Option<i32> = Some(42);
        let none: Option<i32> = None;

        assert_eq!(some.ok_or("error"), Ok(42));
        assert_eq!(none.ok_or("error"), Err("error"));
    }

    #[test]
    #[should_panic(expected = "explicit panic")]
    fn test_panic() {
        panic!("explicit panic");
    }
}
