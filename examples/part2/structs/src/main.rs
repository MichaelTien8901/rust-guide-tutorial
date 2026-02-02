//! Structs Example
//!
//! Demonstrates struct definitions, instantiation, methods,
//! and associated functions in Rust.

fn main() {
    println!("=== Structs ===\n");

    println!("--- Basic Structs ---");
    basic_structs();

    println!("\n--- Tuple Structs ---");
    tuple_structs();

    println!("\n--- Unit Structs ---");
    unit_structs();

    println!("\n--- Methods ---");
    methods();

    println!("\n--- Associated Functions ---");
    associated_functions();

    println!("\n--- Struct Update Syntax ---");
    struct_update();

    println!("\n--- Derived Traits ---");
    derived_traits();
}

// Basic struct definition
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn basic_structs() {
    // Creating an instance
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User: {}", user1.username);
    println!("Email: {}", user1.email);
    println!("Active: {}", user1.active);

    // Mutable struct instance
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("newemail@example.com");
    println!("Updated email: {}", user2.email);

    // Using a builder function
    let user3 = build_user(
        String::from("test@example.com"),
        String::from("testuser"),
    );
    println!("Built user: {}", user3.username);
}

// Field init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        email,    // Shorthand for email: email
        username, // Shorthand for username: username
        active: true,
        sign_in_count: 1,
    }
}

// Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Black RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("Origin: ({}, {}, {})", origin.0, origin.1, origin.2);

    // Destructuring
    let Color(r, g, b) = black;
    println!("Destructured: r={}, g={}, b={}", r, g, b);
}

// Unit-like struct (no fields)
struct AlwaysEqual;

fn unit_structs() {
    let _subject = AlwaysEqual;
    // Useful for implementing traits without data
    println!("Unit struct created (no data)");
}

// Struct with methods
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Method: takes &self
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method with parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Mutable method: takes &mut self
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method that consumes self
    fn into_square(self) -> Rectangle {
        let side = self.width.max(self.height);
        Rectangle {
            width: side,
            height: side,
        }
    }
}

fn methods() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rectangle: {:?}", rect1);
    println!("Area: {} square pixels", rect1.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    // Mutable method
    let mut rect3 = Rectangle {
        width: 5,
        height: 10,
    };
    println!("Before scale: {:?}", rect3);
    rect3.scale(2);
    println!("After scale(2): {:?}", rect3);

    // Consuming method
    let rect4 = Rectangle {
        width: 10,
        height: 20,
    };
    let square = rect4.into_square();
    println!("Square from rectangle: {:?}", square);
    // rect4 is no longer valid here
}

// Associated functions (no self)
impl Rectangle {
    // Associated function (constructor)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Another associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn associated_functions() {
    // Called with :: syntax
    let rect = Rectangle::new(10, 20);
    println!("Created with new: {:?}", rect);

    let square = Rectangle::square(15);
    println!("Created square: {:?}", square);
}

fn struct_update() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Struct update syntax: copy remaining fields from user1
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1 // username moved from user1!
    };

    println!("user2 email: {}", user2.email);
    println!("user2 active (from user1): {}", user2.active);

    // Note: user1.username is now invalid (moved to user2)
    // But user1.active and user1.sign_in_count are still valid (Copy types)
    println!("user1.active still valid: {}", user1.active);
}

// Structs with derived traits
#[derive(Debug, Clone, PartialEq)]
struct Person {
    name: String,
    age: u32,
}

fn derived_traits() {
    let person1 = Person {
        name: String::from("Alice"),
        age: 30,
    };

    // Debug trait: {:?} formatting
    println!("Debug: {:?}", person1);
    println!("Pretty debug: {:#?}", person1);

    // Clone trait
    let person2 = person1.clone();
    println!("Cloned: {:?}", person2);

    // PartialEq trait: comparison
    let person3 = Person {
        name: String::from("Alice"),
        age: 30,
    };
    println!("person1 == person3? {}", person1 == person3);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_area() {
        let rect = Rectangle::new(10, 20);
        assert_eq!(rect.area(), 200);
    }

    #[test]
    fn test_rectangle_can_hold() {
        let rect1 = Rectangle::new(10, 20);
        let rect2 = Rectangle::new(5, 10);
        assert!(rect1.can_hold(&rect2));
        assert!(!rect2.can_hold(&rect1));
    }

    #[test]
    fn test_rectangle_square() {
        let square = Rectangle::square(10);
        assert_eq!(square.width, 10);
        assert_eq!(square.height, 10);
    }

    #[test]
    fn test_person_equality() {
        let p1 = Person {
            name: String::from("Bob"),
            age: 25,
        };
        let p2 = p1.clone();
        assert_eq!(p1, p2);
    }
}
