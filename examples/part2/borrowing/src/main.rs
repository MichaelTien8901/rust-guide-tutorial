//! Borrowing and References Example
//!
//! Demonstrates Rust's borrowing rules:
//! 1. You can have either ONE mutable reference OR any number of immutable references
//! 2. References must always be valid (no dangling references)

fn main() {
    println!("=== Borrowing and References ===\n");

    println!("--- Immutable References ---");
    immutable_references();

    println!("\n--- Mutable References ---");
    mutable_references();

    println!("\n--- Borrowing Rules ---");
    borrowing_rules();

    println!("\n--- Slices as Borrows ---");
    slices_as_borrows();

    println!("\n--- Reference Patterns ---");
    reference_patterns();
}

fn immutable_references() {
    let s = String::from("hello");

    // Create immutable reference with &
    let len = calculate_length(&s);

    // s is still valid because we only borrowed it
    println!("The length of '{}' is {}", s, len);

    // Multiple immutable references are OK
    let r1 = &s;
    let r2 = &s;
    let r3 = &s;
    println!("Three refs: {}, {}, {}", r1, r2, r3);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope, but since it's a reference,
    // the original String is not dropped
}

fn mutable_references() {
    let mut s = String::from("hello");

    println!("Before: {}", s);

    // Create mutable reference with &mut
    change(&mut s);

    println!("After: {}", s);
}

fn change(s: &mut String) {
    s.push_str(", world!");
}

fn borrowing_rules() {
    let mut s = String::from("hello");

    // Rule 1: Only ONE mutable reference at a time
    {
        let r1 = &mut s;
        r1.push_str("!");
        println!("Mutable ref: {}", r1);
    } // r1 goes out of scope, so we can make a new reference

    // Now we can create another mutable reference
    let r2 = &mut s;
    r2.push_str("!");
    println!("New mutable ref: {}", r2);

    // Rule 2: Can't have mutable ref while immutable refs exist
    let mut s2 = String::from("hello");

    let r1 = &s2;
    let r2 = &s2;
    println!("Immutable refs: {} and {}", r1, r2);
    // r1 and r2 are no longer used after this point (NLL)

    // Now we can have a mutable reference
    let r3 = &mut s2;
    r3.push_str("!");
    println!("Mutable ref after immutable refs done: {}", r3);
}

fn slices_as_borrows() {
    let s = String::from("hello world");

    // String slices are references to part of a String
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("Slices: '{}' '{}'", hello, world);

    // Slice of whole string
    let whole = &s[..];
    println!("Whole slice: '{}'", whole);

    // First word function using slices
    let word = first_word(&s);
    println!("First word: '{}'", word);

    // Array slices
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3];
    println!("Array slice: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn reference_patterns() {
    // Auto-dereferencing
    let s = String::from("hello");
    let r = &s;

    // These are equivalent due to auto-deref
    println!("Length via ref: {}", r.len());
    println!("Length via deref: {}", (*r).len());

    // References in structs
    // (Note: requires lifetime annotations in real code)

    // References and methods
    let mut vec = vec![1, 2, 3];

    // &self - immutable borrow
    println!("Vec length: {}", vec.len());

    // &mut self - mutable borrow
    vec.push(4);
    println!("After push: {:?}", vec);

    // self - takes ownership
    // vec.into_iter() would consume vec

    // Reborrowing
    let mut x = 5;
    let r1 = &mut x;
    // let r2 = &mut x; // Error! Can't have two mut refs

    // But can reborrow through first ref
    *r1 += 1;
    println!("After reborrow: {}", r1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutable_borrow() {
        let s = String::from("hello");
        let len = calculate_length(&s);
        assert_eq!(len, 5);
        assert_eq!(s, "hello"); // s still valid
    }

    #[test]
    fn test_mutable_borrow() {
        let mut s = String::from("hello");
        change(&mut s);
        assert_eq!(s, "hello, world!");
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("hello"), "hello");
        assert_eq!(first_word(""), "");
    }
}
