//! Functions Example
//!
//! Demonstrates function definitions, parameters, return values,
//! and expressions vs statements in Rust.

fn main() {
    println!("=== Functions ===\n");

    println!("--- Basic Functions ---");
    basic_functions();

    println!("\n--- Parameters ---");
    parameters();

    println!("\n--- Return Values ---");
    return_values();

    println!("\n--- Expressions vs Statements ---");
    expressions_vs_statements();

    println!("\n--- Early Returns ---");
    early_returns();
}

fn basic_functions() {
    // Function names use snake_case
    say_hello();
    greet("World");
}

fn say_hello() {
    println!("Hello!");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn parameters() {
    // Parameters must have type annotations
    print_sum(5, 3);
    print_labeled_measurement(5, 'h');

    // Multiple parameters
    let result = add(10, 20);
    println!("10 + 20 = {}", result);
}

fn print_sum(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}

fn print_labeled_measurement(value: i32, unit: char) {
    println!("The measurement is: {}{}", value, unit);
}

fn add(a: i32, b: i32) -> i32 {
    a + b // Expression - no semicolon means return value
}

fn return_values() {
    // Return type specified with ->
    let x = five();
    println!("five() = {}", x);

    let y = plus_one(x);
    println!("plus_one(5) = {}", y);

    // Explicit return keyword
    let z = explicit_return(10);
    println!("explicit_return(10) = {}", z);

    // Multiple return values with tuple
    let (sum, product) = sum_and_product(3, 4);
    println!("sum=3+4={}, product=3*4={}", sum, product);

    // Option return for fallible operations
    let result = safe_divide(10, 2);
    println!("10 / 2 = {:?}", result);

    let result = safe_divide(10, 0);
    println!("10 / 0 = {:?}", result);
}

fn five() -> i32 {
    5 // Last expression is returned
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn explicit_return(x: i32) -> i32 {
    return x * 2; // Explicit return with semicolon
}

fn sum_and_product(a: i32, b: i32) -> (i32, i32) {
    (a + b, a * b)
}

fn safe_divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        None
    } else {
        Some(a / b)
    }
}

fn expressions_vs_statements() {
    // Statements perform actions, don't return values
    let _x = 5; // This is a statement

    // Expressions evaluate to a value
    let y = {
        let x = 3;
        x + 1 // No semicolon - this is an expression
    };
    println!("Block expression result: {}", y);

    // Adding semicolon makes it a statement (returns unit)
    let z = {
        let x = 3;
        x + 1; // Semicolon - returns ()
    };
    println!("Statement block result: {:?}", z);

    // if is an expression
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("if expression result: {}", number);

    // loop can return a value
    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("loop result: {}", loop_result);
}

fn early_returns() {
    println!("is_even(4) = {}", is_even(4));
    println!("is_even(7) = {}", is_even(7));

    println!("grade(95) = {}", grade(95));
    println!("grade(72) = {}", grade(72));
    println!("grade(55) = {}", grade(55));
}

fn is_even(n: i32) -> bool {
    if n % 2 == 0 {
        return true; // Early return
    }
    false
}

fn grade(score: u32) -> char {
    if score >= 90 {
        return 'A';
    }
    if score >= 80 {
        return 'B';
    }
    if score >= 70 {
        return 'C';
    }
    if score >= 60 {
        return 'D';
    }
    'F'
}

// Diverging function - never returns
#[allow(dead_code)]
fn diverging() -> ! {
    panic!("This function never returns!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(add(-1, 1), 0);
    }

    #[test]
    fn test_sum_and_product() {
        assert_eq!(sum_and_product(3, 4), (7, 12));
    }

    #[test]
    fn test_safe_divide() {
        assert_eq!(safe_divide(10, 2), Some(5));
        assert_eq!(safe_divide(10, 0), None);
    }

    #[test]
    fn test_is_even() {
        assert!(is_even(2));
        assert!(!is_even(3));
    }

    #[test]
    fn test_grade() {
        assert_eq!(grade(95), 'A');
        assert_eq!(grade(85), 'B');
        assert_eq!(grade(75), 'C');
        assert_eq!(grade(65), 'D');
        assert_eq!(grade(55), 'F');
    }
}
