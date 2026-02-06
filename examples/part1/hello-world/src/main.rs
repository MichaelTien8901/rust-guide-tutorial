//! Hello World - Your first Rust program
//!
//! This example demonstrates:
//! - The `main` function entry point
//! - The `println!` macro for output
//! - Variables with `let`
//! - String formatting with `{}`

fn main() {
    // Simple hello world
    println!("Hello, world!");

    // Using a variable
    let name = "Rustacean";
    println!("Hello, {}!", name);

    // Interactive version - uncomment to try
    // println!("What is your name?");
    // let mut input = String::new();
    // io::stdin()
    //     .read_line(&mut input)
    //     .expect("Failed to read line");
    // let name = input.trim();
    // println!("Hello, {}!", name);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        // This test always passes - it's just to verify the project builds
        assert!(true);
    }
}
