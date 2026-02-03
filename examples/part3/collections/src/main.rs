//! Collections Example
//!
//! Demonstrates Vec, HashMap, HashSet, and String operations.

use std::collections::{HashMap, HashSet};

fn main() {
    println!("=== Collections ===\n");

    println!("--- Vec ---");
    vec_examples();

    println!("\n--- HashMap ---");
    hashmap_examples();

    println!("\n--- HashSet ---");
    hashset_examples();

    println!("\n--- String ---");
    string_examples();
}

fn vec_examples() {
    // Creating vectors
    let v1: Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3];
    let v3 = Vec::from([4, 5, 6]);

    println!("Empty: {:?}", v1);
    println!("From macro: {:?}", v2);
    println!("From array: {:?}", v3);

    // Adding elements
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("After push: {:?}", v);

    // Accessing elements
    let third = &v[2];
    println!("Third element: {}", third);

    let third_opt = v.get(2);
    println!("Third via get: {:?}", third_opt);

    let out_of_bounds = v.get(100);
    println!("Out of bounds: {:?}", out_of_bounds);

    // Iterating
    print!("Iterating: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // Mutable iteration
    for i in &mut v {
        *i *= 2;
    }
    println!("After doubling: {:?}", v);

    // Other methods
    println!("Length: {}", v.len());
    println!("Is empty: {}", v.is_empty());
    println!("Contains 4: {}", v.contains(&4));

    // Removing elements
    let popped = v.pop();
    println!("Popped: {:?}, remaining: {:?}", popped, v);

    // Slicing
    let slice = &v[0..2];
    println!("Slice: {:?}", slice);

    // Capacity
    let mut v = Vec::with_capacity(10);
    println!("Capacity: {}, len: {}", v.capacity(), v.len());
    v.extend([1, 2, 3]);
    println!(
        "After extend - capacity: {}, len: {}",
        v.capacity(),
        v.len()
    );
}

fn hashmap_examples() {
    // Creating HashMaps
    let mut scores: HashMap<String, i32> = HashMap::new();

    // Inserting
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    println!("Scores: {:?}", scores);

    // From iterator of tuples
    let teams = vec![(String::from("Green"), 25), (String::from("Yellow"), 30)];
    let scores2: HashMap<_, _> = teams.into_iter().collect();
    println!("From iter: {:?}", scores2);

    // Accessing
    let team = String::from("Blue");
    let score = scores.get(&team);
    println!("Blue team score: {:?}", score);

    // Default if missing
    let unknown = scores.get(&String::from("Unknown")).copied().unwrap_or(0);
    println!("Unknown team score: {}", unknown);

    // Iterating
    println!("All scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // Updating
    scores.insert(String::from("Blue"), 25); // Overwrites
    println!("After update: {:?}", scores);

    // Insert only if missing
    scores.entry(String::from("Blue")).or_insert(100); // Won't change
    scores.entry(String::from("Purple")).or_insert(100); // Will insert
    println!("After entry: {:?}", scores);

    // Update based on old value
    let text = "hello world wonderful world";
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    println!("Word count: {:?}", word_count);
}

fn hashset_examples() {
    // Creating HashSet
    let mut set: HashSet<i32> = HashSet::new();

    // Adding elements
    set.insert(1);
    set.insert(2);
    set.insert(3);
    set.insert(1); // Duplicate, won't be added
    println!("Set: {:?}", set);

    // From iterator
    let set2: HashSet<_> = [4, 5, 6].into_iter().collect();
    println!("Set2: {:?}", set2);

    // Operations
    println!("Contains 2: {}", set.contains(&2));
    println!("Length: {}", set.len());

    // Set operations
    let a: HashSet<_> = [1, 2, 3, 4].into_iter().collect();
    let b: HashSet<_> = [3, 4, 5, 6].into_iter().collect();

    // Union
    let union: HashSet<_> = a.union(&b).collect();
    println!("Union: {:?}", union);

    // Intersection
    let intersection: HashSet<_> = a.intersection(&b).collect();
    println!("Intersection: {:?}", intersection);

    // Difference
    let difference: HashSet<_> = a.difference(&b).collect();
    println!("A - B: {:?}", difference);

    // Symmetric difference
    let sym_diff: HashSet<_> = a.symmetric_difference(&b).collect();
    println!("Symmetric difference: {:?}", sym_diff);
}

fn string_examples() {
    // Creating strings
    let s1 = String::new();
    let s2 = String::from("hello");
    let s3 = "world".to_string();

    println!("Empty: '{}'", s1);
    println!("From: '{}'", s2);
    println!("to_string: '{}'", s3);

    // Concatenation
    let mut s = String::from("Hello");
    s.push_str(", world!");
    s.push('!');
    println!("After push: {}", s);

    // + operator (takes ownership of left)
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved, s2 is borrowed
    println!("Concatenated: {}", s3);
    // println!("{}", s1); // Error: s1 moved

    // format! macro (doesn't take ownership)
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("Formatted: {}", s);
    println!("Still valid: {} {} {}", s1, s2, s3);

    // Slicing (be careful with Unicode!)
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // First 4 bytes (2 Cyrillic chars)
    println!("Slice: {}", s);

    // Iterating
    print!("Chars: ");
    for c in "नमस्ते".chars() {
        print!("{} ", c);
    }
    println!();

    print!("Bytes: ");
    for b in "hello".bytes() {
        print!("{} ", b);
    }
    println!();

    // String methods
    let s = String::from("  Hello, World!  ");
    println!("Trim: '{}'", s.trim());
    println!("Uppercase: {}", s.to_uppercase());
    println!("Contains 'World': {}", s.contains("World"));
    println!("Replace: {}", s.replace("World", "Rust"));

    // Split
    let parts: Vec<&str> = "a,b,c".split(',').collect();
    println!("Split: {:?}", parts);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec() {
        let mut v = vec![1, 2, 3];
        v.push(4);
        assert_eq!(v.len(), 4);
        assert_eq!(v[3], 4);
    }

    #[test]
    fn test_hashmap() {
        let mut map = HashMap::new();
        map.insert("key", 42);
        assert_eq!(map.get("key"), Some(&42));
    }

    #[test]
    fn test_hashset() {
        let set: HashSet<_> = [1, 2, 2, 3].into_iter().collect();
        assert_eq!(set.len(), 3);
    }

    #[test]
    fn test_string() {
        let mut s = String::from("hello");
        s.push_str(" world");
        assert_eq!(s, "hello world");
    }
}
