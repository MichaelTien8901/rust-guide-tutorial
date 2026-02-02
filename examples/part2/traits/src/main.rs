//! Traits Example
//!
//! Demonstrates trait definitions, implementations,
//! trait bounds, and common standard library traits.

fn main() {
    println!("=== Traits ===\n");

    println!("--- Defining Traits ---");
    defining_traits();

    println!("\n--- Implementing Traits ---");
    implementing_traits();

    println!("\n--- Trait Bounds ---");
    trait_bounds();

    println!("\n--- Default Implementations ---");
    default_implementations();

    println!("\n--- Common Std Traits ---");
    common_std_traits();

    println!("\n--- Trait Objects ---");
    trait_objects();
}

// Define a trait
trait Summary {
    fn summarize(&self) -> String;

    // Method with default implementation
    fn summarize_author(&self) -> String {
        String::from("Unknown author")
    }
}

// Structs to implement the trait on
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

fn defining_traits() {
    println!("Traits define shared behavior");
    println!("Like interfaces in other languages");
    println!("But with default implementations allowed");
}

// Implement trait for types
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn implementing_traits() {
    let article = NewsArticle {
        headline: String::from("Breaking News!"),
        location: String::from("New York"),
        author: String::from("John"),
        content: String::from("Something happened..."),
    };

    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Rust 2024 edition released!"),
        reply: false,
        retweet: false,
    };

    println!("Article: {}", article.summarize());
    println!("Article author: {}", article.summarize_author());

    println!("Tweet: {}", tweet.summarize());
    println!("Tweet author: {}", tweet.summarize_author()); // Uses default
}

fn trait_bounds() {
    // Trait as parameter (impl Trait syntax)
    fn notify(item: &impl Summary) {
        println!("Breaking news! {}", item.summarize());
    }

    // Equivalent with trait bound syntax
    fn notify_verbose<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    // Multiple trait bounds
    fn notify_and_display<T: Summary + std::fmt::Debug>(item: &T) {
        println!("Item: {:?}", item);
        println!("Summary: {}", item.summarize());
    }

    // where clause for cleaner syntax
    fn complex_function<T, U>(t: &T, u: &U) -> String
    where
        T: Summary + Clone,
        U: Summary,
    {
        format!("{} and {}", t.summarize(), u.summarize())
    }

    let tweet = Tweet {
        username: String::from("user"),
        content: String::from("Hello!"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
    notify_verbose(&tweet);
}

// Trait with default implementation
trait Greet {
    fn name(&self) -> &str;

    fn greet(&self) -> String {
        format!("Hello, {}!", self.name())
    }

    fn formal_greet(&self) -> String {
        format!("Good day, {}. How do you do?", self.name())
    }
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn name(&self) -> &str {
        &self.name
    }

    // Override default implementation
    fn greet(&self) -> String {
        format!("Hi there, {}!", self.name())
    }

    // formal_greet uses default
}

fn default_implementations() {
    let person = Person {
        name: String::from("Alice"),
    };

    println!("Custom greet: {}", person.greet());
    println!("Default formal: {}", person.formal_greet());
}

fn common_std_traits() {
    // Debug - {:?} formatting
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 1, y: 2 };
    println!("Debug: {:?}", p);

    // Clone - explicit copying
    #[derive(Clone, Debug)]
    struct Data {
        value: i32,
    }
    let d1 = Data { value: 42 };
    let d2 = d1.clone();
    println!("Original: {:?}, Clone: {:?}", d1, d2);

    // Copy - implicit copying (for small, stack-only types)
    #[derive(Copy, Clone, Debug)]
    struct SmallData {
        value: i32,
    }
    let s1 = SmallData { value: 10 };
    let s2 = s1; // Copy, not move
    println!("Both valid: {:?}, {:?}", s1, s2);

    // PartialEq, Eq - equality comparison
    #[derive(PartialEq, Eq, Debug)]
    struct Id(u32);
    let id1 = Id(1);
    let id2 = Id(1);
    let id3 = Id(2);
    println!("id1 == id2: {}", id1 == id2);
    println!("id1 == id3: {}", id1 == id3);

    // PartialOrd, Ord - ordering comparison
    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
    struct Priority(u32);
    let p1 = Priority(1);
    let p2 = Priority(2);
    println!("p1 < p2: {}", p1 < p2);

    // Default - default values
    #[derive(Default, Debug)]
    struct Config {
        debug: bool,
        level: u32,
    }
    let config = Config::default();
    println!("Default config: {:?}", config);

    // Display - user-facing formatting
    use std::fmt;

    struct Temperature(f64);

    impl fmt::Display for Temperature {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}°C", self.0)
        }
    }

    let temp = Temperature(23.5);
    println!("Temperature: {}", temp);
}

fn trait_objects() {
    // Trait objects for runtime polymorphism
    // Use when you need different types in same collection

    let article = NewsArticle {
        headline: String::from("News!"),
        location: String::from("NYC"),
        author: String::from("Reporter"),
        content: String::from("..."),
    };

    let tweet = Tweet {
        username: String::from("user"),
        content: String::from("Tweet!"),
        reply: false,
        retweet: false,
    };

    // Vec of trait objects (requires dynamic dispatch)
    let items: Vec<&dyn Summary> = vec![&article, &tweet];

    for item in items {
        println!("Summary: {}", item.summarize());
    }

    // Box<dyn Trait> for owned trait objects
    fn get_summarizer(is_tweet: bool) -> Box<dyn Summary> {
        if is_tweet {
            Box::new(Tweet {
                username: String::from("bot"),
                content: String::from("Automated tweet"),
                reply: false,
                retweet: false,
            })
        } else {
            Box::new(NewsArticle {
                headline: String::from("Auto News"),
                location: String::from("Web"),
                author: String::from("Bot"),
                content: String::from("..."),
            })
        }
    }

    let summarizer = get_summarizer(true);
    println!("Dynamic: {}", summarizer.summarize());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_summarize() {
        let tweet = Tweet {
            username: String::from("test"),
            content: String::from("testing"),
            reply: false,
            retweet: false,
        };
        assert_eq!(tweet.summarize(), "test: testing");
    }

    #[test]
    fn test_default_implementation() {
        let person = Person {
            name: String::from("Bob"),
        };
        assert!(person.formal_greet().contains("Bob"));
    }

    #[test]
    fn test_derive_traits() {
        #[derive(Clone, PartialEq, Debug)]
        struct TestStruct {
            value: i32,
        }

        let a = TestStruct { value: 42 };
        let b = a.clone();
        assert_eq!(a, b);
    }
}
