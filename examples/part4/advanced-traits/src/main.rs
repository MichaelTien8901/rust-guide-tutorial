//! Advanced Traits Example
//!
//! Demonstrates associated types, supertraits, and advanced trait patterns.
//!
//! # Trait Relationship Hierarchy
//! ```text
//!     ┌─────────────────────────────────────────────────────────┐
//!     │                   Trait Features                        │
//!     ├─────────────────────────────────────────────────────────┤
//!     │                                                         │
//!     │  Associated Types:    type Item;                        │
//!     │  ├── Single concrete type per impl                      │
//!     │  └── Cleaner than generics for many cases               │
//!     │                                                         │
//!     │  Supertraits:         trait A: B + C { }                │
//!     │  ├── A requires B and C                                 │
//!     │  └── Can use methods from B and C                       │
//!     │                                                         │
//!     │  Default Types:       type Item = i32;                  │
//!     │  ├── Can be overridden                                  │
//!     │  └── Useful for common cases                            │
//!     │                                                         │
//!     │  GATs:                type Item<'a>;                    │
//!     │  ├── Generic Associated Types                           │
//!     │  └── Lifetime/type params on associated types           │
//!     │                                                         │
//!     └─────────────────────────────────────────────────────────┘
//! ```

use std::fmt::{Debug, Display};
use std::ops::Add;

fn main() {
    println!("=== Advanced Traits ===\n");

    println!("--- Associated Types ---");
    associated_types();

    println!("\n--- Supertraits ---");
    supertraits();

    println!("\n--- Default Type Parameters ---");
    default_type_params();

    println!("\n--- Fully Qualified Syntax ---");
    fully_qualified_syntax();

    println!("\n--- Blanket Implementations ---");
    blanket_implementations();

    println!("\n--- Generic Associated Types ---");
    generic_associated_types();

    println!("\n--- Trait Objects Advanced ---");
    trait_objects_advanced();
}

// ============================================
// Associated Types
// ============================================

/// Container trait with associated type
trait Container {
    type Item;

    fn add(&mut self, item: Self::Item);
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
}

impl<T> Container for Stack<T> {
    type Item = T;

    fn add(&mut self, item: T) {
        self.items.push(item);
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }

    fn len(&self) -> usize {
        self.items.len()
    }
}

/// Compare: Generic parameter vs Associated Type
trait GenericIterator<T> {
    fn next(&mut self) -> Option<T>;
}

// With generics, same type can implement for multiple T
struct MultiIterator;
impl GenericIterator<i32> for MultiIterator {
    fn next(&mut self) -> Option<i32> { Some(1) }
}
impl GenericIterator<String> for MultiIterator {
    fn next(&mut self) -> Option<String> { Some("hello".into()) }
}

// With associated type, only one implementation per type
trait AssociatedIterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

fn associated_types() {
    let mut stack: Stack<i32> = Stack::new();
    stack.add(1);
    stack.add(2);
    stack.add(3);

    println!("  Stack length: {}", stack.len());
    println!("  Stack[1]: {:?}", stack.get(1));

    // Function using associated type
    fn print_container<C: Container>(c: &C)
    where
        C::Item: Debug
    {
        for i in 0..c.len() {
            println!("    [{}]: {:?}", i, c.get(i));
        }
    }

    print_container(&stack);
}

// ============================================
// Supertraits
// ============================================

/// Trait with supertrait requirements
trait Printable: Display + Debug {
    fn print_formatted(&self) {
        println!("  Display: {}", self);
        println!("  Debug: {:?}", self);
    }
}

// Must implement Display and Debug to implement Printable
#[derive(Debug)]
struct Document {
    title: String,
    content: String,
}

impl Display for Document {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.title, self.content)
    }
}

impl Printable for Document {}

/// Supertrait chain
trait Named {
    fn name(&self) -> &str;
}

trait Aged {
    fn age(&self) -> u32;
}

trait Person: Named + Aged {
    fn introduce(&self) {
        println!("  I'm {}, {} years old", self.name(), self.age());
    }
}

struct Employee {
    name: String,
    age: u32,
    department: String,
}

impl Named for Employee {
    fn name(&self) -> &str { &self.name }
}

impl Aged for Employee {
    fn age(&self) -> u32 { self.age }
}

impl Person for Employee {}

fn supertraits() {
    let doc = Document {
        title: "Readme".into(),
        content: "Hello World".into(),
    };
    doc.print_formatted();

    let emp = Employee {
        name: "Alice".into(),
        age: 30,
        department: "Engineering".into(),
    };
    emp.introduce();
}

// ============================================
// Default Type Parameters
// ============================================

/// Trait with default type parameter
trait Combine<Rhs = Self> {
    type Output;
    fn combine(self, rhs: Rhs) -> Self::Output;
}

#[derive(Debug, Clone)]
struct Meters(f64);

#[derive(Debug, Clone)]
struct Feet(f64);

// Default: combine with same type
impl Combine for Meters {
    type Output = Meters;
    fn combine(self, rhs: Meters) -> Meters {
        Meters(self.0 + rhs.0)
    }
}

// Custom: combine with different type
impl Combine<Feet> for Meters {
    type Output = Meters;
    fn combine(self, rhs: Feet) -> Meters {
        Meters(self.0 + rhs.0 * 0.3048)
    }
}

fn default_type_params() {
    let m1 = Meters(10.0);
    let m2 = Meters(5.0);
    let combined = m1.clone().combine(m2);
    println!("  Meters + Meters: {:?}", combined);

    let m3 = Meters(10.0);
    let f1 = Feet(10.0);
    let combined = m3.combine(f1);
    println!("  Meters + Feet: {:?}", combined);
}

// ============================================
// Fully Qualified Syntax
// ============================================

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) { println!("  Flying an airplane"); }
}

impl Wizard for Human {
    fn fly(&self) { println!("  Flying on a broomstick"); }
}

impl Human {
    fn fly(&self) { println!("  Waving arms frantically"); }
}

trait Animal {
    fn name() -> String;
}

struct Dog;

impl Dog {
    fn name() -> String { "Spot".into() }
}

impl Animal for Dog {
    fn name() -> String { "puppy".into() }
}

fn fully_qualified_syntax() {
    let person = Human;

    // Ambiguous - uses inherent method
    person.fly();

    // Explicit trait method calls
    Pilot::fly(&person);
    Wizard::fly(&person);

    // For associated functions (no self)
    println!("  Dog::name() = {}", Dog::name());
    println!("  <Dog as Animal>::name() = {}", <Dog as Animal>::name());
}

// ============================================
// Blanket Implementations
// ============================================

/// Custom trait
trait Stringify {
    fn to_string_custom(&self) -> String;
}

/// Blanket impl for anything that implements Display
impl<T: Display> Stringify for T {
    fn to_string_custom(&self) -> String {
        format!("Value: {}", self)
    }
}

/// Blanket impl example from std: impl<T: Display> ToString for T

fn blanket_implementations() {
    // Works for any Display type
    println!("  {}", 42.to_string_custom());
    println!("  {}", "hello".to_string_custom());
    println!("  {}", 3.14f64.to_string_custom());

    // Conditional implementation
    trait Summarize {
        fn summarize(&self) -> String;
    }

    // Only for Vec of Debug types
    impl<T: Debug> Summarize for Vec<T> {
        fn summarize(&self) -> String {
            format!("Vec with {} items: {:?}", self.len(), self)
        }
    }

    let v = vec![1, 2, 3];
    println!("  {}", v.summarize());
}

// ============================================
// Generic Associated Types (GATs)
// ============================================

/// GAT example: lending iterator
trait LendingIterator {
    type Item<'a> where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
}

struct WindowIter<'data> {
    data: &'data [i32],
    window_size: usize,
    pos: usize,
}

impl<'data> LendingIterator for WindowIter<'data> {
    type Item<'a> = &'a [i32] where Self: 'a;

    fn next<'a>(&'a mut self) -> Option<Self::Item<'a>> {
        if self.pos + self.window_size <= self.data.len() {
            let window = &self.data[self.pos..self.pos + self.window_size];
            self.pos += 1;
            Some(window)
        } else {
            None
        }
    }
}

fn generic_associated_types() {
    let data = [1, 2, 3, 4, 5];
    let mut iter = WindowIter {
        data: &data,
        window_size: 3,
        pos: 0,
    };

    println!("  Windows of size 3:");
    while let Some(window) = iter.next() {
        println!("    {:?}", window);
    }
}

// ============================================
// Trait Objects Advanced
// ============================================

trait Drawable: CloneDrawable {
    fn draw(&self);
    fn name(&self) -> &str;
}

// Trait for cloning trait objects
trait CloneDrawable {
    fn clone_box(&self) -> Box<dyn Drawable>;
}

impl<T: Drawable + Clone + 'static> CloneDrawable for T {
    fn clone_box(&self) -> Box<dyn Drawable> {
        Box::new(self.clone())
    }
}

#[derive(Clone)]
struct Circle { radius: f64 }

#[derive(Clone)]
struct Rectangle { width: f64, height: f64 }

impl Drawable for Circle {
    fn draw(&self) { println!("    Drawing circle (r={})", self.radius); }
    fn name(&self) -> &str { "Circle" }
}

impl Drawable for Rectangle {
    fn draw(&self) { println!("    Drawing rectangle ({}x{})", self.width, self.height); }
    fn name(&self) -> &str { "Rectangle" }
}

fn trait_objects_advanced() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];

    println!("  Drawing shapes:");
    for shape in &shapes {
        shape.draw();
    }

    // Clone trait objects
    let cloned = shapes[0].clone_box();
    println!("  Cloned: {}", cloned.name());

    // Multiple trait bounds with trait objects
    fn print_debug_display(item: &(impl Debug + Display)) {
        println!("    Debug: {:?}", item);
        println!("    Display: {}", item);
    }

    print_debug_display(&42);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_associated_type() {
        let mut stack = Stack::<i32>::new();
        stack.add(1);
        stack.add(2);
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.get(0), Some(&1));
    }

    #[test]
    fn test_supertrait() {
        let doc = Document {
            title: "Test".into(),
            content: "Content".into(),
        };
        // Can call Display and Debug methods
        let display = format!("{}", doc);
        let debug = format!("{:?}", doc);
        assert!(display.contains("Test"));
        assert!(debug.contains("Document"));
    }

    #[test]
    fn test_fully_qualified() {
        assert_eq!(Dog::name(), "Spot");
        assert_eq!(<Dog as Animal>::name(), "puppy");
    }

    #[test]
    fn test_blanket_impl() {
        let s = 42.to_string_custom();
        assert!(s.contains("42"));
    }
}
