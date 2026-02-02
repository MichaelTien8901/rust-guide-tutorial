//! Lifetimes Example
//!
//! Demonstrates lifetime annotations in Rust.
//! Lifetimes ensure references are valid for as long as they're used.

fn main() {
    println!("=== Lifetimes ===\n");

    println!("--- Basic Lifetimes ---");
    basic_lifetimes();

    println!("\n--- Lifetime Annotations ---");
    lifetime_annotations();

    println!("\n--- Struct Lifetimes ---");
    struct_lifetimes();

    println!("\n--- Static Lifetime ---");
    static_lifetime();

    println!("\n--- Lifetime Elision ---");
    lifetime_elision();
}

fn basic_lifetimes() {
    // Every reference has a lifetime
    // Usually inferred by the compiler

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    // This works because r's lifetime is within x's scope

    // This would NOT work (dangling reference):
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // x dropped here
    // println!("{}", r); // Error: x doesn't live long enough
}

fn lifetime_annotations() {
    let string1 = String::from("long string");
    let string2 = String::from("short");

    // longest requires lifetime annotations
    let result = longest(&string1, &string2);
    println!("Longest: {}", result);

    // Lifetime ensures result is valid
    {
        let string3 = String::from("xyz");
        let result2 = longest(&string1, &string3);
        println!("Longest (inner scope): {}", result2);
    }

    // This pattern also works
    let result3;
    {
        let string4 = String::from("actually longest string");
        result3 = longest(&string1, &string4);
        println!("Result while string4 valid: {}", result3);
    }
    // Can't use result3 here if it referenced string4
}

// Lifetime annotation syntax: 'a is a lifetime parameter
// This says: the returned reference will be valid as long as
// BOTH input references are valid
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Only one input needs lifetime if output only relates to one
fn longest_with_announcement<'a>(x: &'a str, y: &str, ann: &str) -> &'a str {
    println!("Announcement: {}", ann);
    // Only returns x, so only x's lifetime matters
    let _ = y; // y not returned
    x
}

fn struct_lifetimes() {
    // Structs can hold references with lifetime annotations
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.part);
    println!("Level: {}", excerpt.level());
    println!("Announce: {}", excerpt.announce_and_return("Here's an excerpt"));
}

// Struct holding a reference needs lifetime annotation
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// Methods on struct with lifetime
impl<'a> ImportantExcerpt<'a> {
    // Elision rule: &self gets its own lifetime
    fn level(&self) -> i32 {
        3
    }

    // Elision rule: output lifetime = &self lifetime
    fn announce_and_return(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn static_lifetime() {
    // 'static means the reference lives for entire program
    let s: &'static str = "I have a static lifetime";
    println!("Static string: {}", s);

    // String literals are 'static (stored in binary)
    let literal = "Hello, world!";
    println!("Literal (static): {}", literal);

    // Be careful with 'static in function signatures
    // It's often not what you want
}

fn lifetime_elision() {
    // Rust has lifetime elision rules that let you omit annotations
    // in common cases

    // Rule 1: Each reference parameter gets its own lifetime
    // fn foo(x: &i32, y: &i32) becomes fn foo<'a, 'b>(x: &'a i32, y: &'b i32)

    // Rule 2: If exactly one input lifetime, output gets that lifetime
    // fn foo(x: &i32) -> &i32 becomes fn foo<'a>(x: &'a i32) -> &'a i32

    // Rule 3: If &self or &mut self, output gets self's lifetime

    let s = String::from("hello");
    let first = first_word(&s);
    println!("First word (elided lifetime): {}", first);
}

// Lifetimes elided here - compiler infers them
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

// Explicit version would be:
// fn first_word<'a>(s: &'a str) -> &'a str { ... }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest() {
        assert_eq!(longest("abc", "ab"), "abc");
        assert_eq!(longest("ab", "abc"), "abc");
        assert_eq!(longest("abc", "xyz"), "abc"); // Same length, returns first
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn test_excerpt() {
        let text = String::from("Hello. World.");
        let excerpt = ImportantExcerpt { part: "Hello" };
        assert_eq!(excerpt.part, "Hello");
        assert_eq!(excerpt.level(), 3);
        drop(text); // text not needed for excerpt in this test
    }
}
