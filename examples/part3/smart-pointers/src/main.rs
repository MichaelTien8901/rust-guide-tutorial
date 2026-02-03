//! Smart Pointers Example
//!
//! Demonstrates Box, Rc, Arc, RefCell, and their combinations.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

fn main() {
    println!("=== Smart Pointers ===\n");

    println!("--- Box<T> ---");
    box_examples();

    println!("\n--- Rc<T> ---");
    rc_examples();

    println!("\n--- RefCell<T> ---");
    refcell_examples();

    println!("\n--- Rc<RefCell<T>> ---");
    rc_refcell_examples();

    println!("\n--- Arc<T> ---");
    arc_examples();

    println!("\n--- Cow<T> ---");
    cow_examples();
}

fn box_examples() {
    // Basic Box - heap allocation
    let b = Box::new(5);
    println!("Boxed value: {}", b);

    // Deref coercion
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y); // Dereference
    println!("Dereferenced: {}", *y);

    // Recursive types require Box
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);

    // Large data on heap
    struct LargeData {
        data: [u8; 1000],
    }

    let large = Box::new(LargeData { data: [0; 1000] });
    println!(
        "Large data size on stack: {} bytes",
        std::mem::size_of_val(&large)
    );

    // Trait objects
    trait Animal {
        fn speak(&self);
    }

    struct Dog;
    struct Cat;

    impl Animal for Dog {
        fn speak(&self) {
            println!("Woof!");
        }
    }

    impl Animal for Cat {
        fn speak(&self) {
            println!("Meow!");
        }
    }

    let animals: Vec<Box<dyn Animal>> = vec![Box::new(Dog), Box::new(Cat)];

    for animal in &animals {
        animal.speak();
    }
}

fn rc_examples() {
    // Single-threaded reference counting
    let a = Rc::new(5);
    println!("Reference count after creating a: {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("Reference count after clone: {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("Reference count in inner scope: {}", Rc::strong_count(&a));
    }

    println!(
        "Reference count after inner scope: {}",
        Rc::strong_count(&a)
    );

    // Shared ownership example
    #[derive(Debug)]
    struct Node {
        value: i32,
        children: Vec<Rc<Node>>,
    }

    let leaf1 = Rc::new(Node {
        value: 1,
        children: vec![],
    });
    let leaf2 = Rc::new(Node {
        value: 2,
        children: vec![],
    });

    let parent = Rc::new(Node {
        value: 0,
        children: vec![Rc::clone(&leaf1), Rc::clone(&leaf2)],
    });

    println!("Parent: {:?}", parent);
    println!("leaf1 count: {}", Rc::strong_count(&leaf1));
}

fn refcell_examples() {
    // Interior mutability
    let data = RefCell::new(5);

    println!("Initial value: {:?}", data.borrow());

    // Mutable borrow at runtime
    *data.borrow_mut() += 10;
    println!("After mutation: {:?}", data.borrow());

    // Multiple immutable borrows
    let r1 = data.borrow();
    let r2 = data.borrow();
    println!("Two borrows: {} {}", r1, r2);
    drop(r1);
    drop(r2);

    // Can't have mutable and immutable at same time (runtime panic)
    // let r3 = data.borrow();
    // let r4 = data.borrow_mut(); // Would panic!

    // Practical example: modifying through shared reference
    struct Counter {
        count: RefCell<u32>,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                count: RefCell::new(0),
            }
        }

        fn increment(&self) {
            *self.count.borrow_mut() += 1;
        }

        fn get(&self) -> u32 {
            *self.count.borrow()
        }
    }

    let counter = Counter::new();
    counter.increment();
    counter.increment();
    println!("Counter: {}", counter.get());
}

fn rc_refcell_examples() {
    // Combining Rc and RefCell for shared mutable state
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 20;

    println!("Final value: {}", value.borrow());

    // Graph-like structure
    #[derive(Debug)]
    struct GraphNode {
        value: i32,
        neighbors: RefCell<Vec<Rc<GraphNode>>>,
    }

    let node1 = Rc::new(GraphNode {
        value: 1,
        neighbors: RefCell::new(vec![]),
    });

    let node2 = Rc::new(GraphNode {
        value: 2,
        neighbors: RefCell::new(vec![]),
    });

    // Add edges
    node1.neighbors.borrow_mut().push(Rc::clone(&node2));
    node2.neighbors.borrow_mut().push(Rc::clone(&node1));

    println!("Node1 value: {}", node1.value);
    println!("Node1 neighbor count: {}", node1.neighbors.borrow().len());
}

fn arc_examples() {
    // Thread-safe reference counting
    use std::thread;

    let data = Arc::new(vec![1, 2, 3]);

    let handles: Vec<_> = (0..3)
        .map(|i| {
            let data = Arc::clone(&data);
            thread::spawn(move || {
                println!("Thread {}: {:?}", i, data);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Arc count: {}", Arc::strong_count(&data));

    // Arc + Mutex for shared mutable state
    use std::sync::Mutex;

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}

fn cow_examples() {
    use std::borrow::Cow;

    // Clone-on-write
    fn process(input: &str) -> Cow<str> {
        if input.contains("bad") {
            // Need to modify, so clone
            Cow::Owned(input.replace("bad", "good"))
        } else {
            // No modification needed, just borrow
            Cow::Borrowed(input)
        }
    }

    let good = process("This is good text");
    let fixed = process("This is bad text");

    println!("Good (borrowed): {}", good);
    println!("Fixed (owned): {}", fixed);

    // Check if owned or borrowed
    match good {
        Cow::Borrowed(_) => println!("good is borrowed"),
        Cow::Owned(_) => println!("good is owned"),
    }

    match fixed {
        Cow::Borrowed(_) => println!("fixed is borrowed"),
        Cow::Owned(_) => println!("fixed is owned"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let b = Box::new(42);
        assert_eq!(*b, 42);
    }

    #[test]
    fn test_rc() {
        let a = Rc::new(5);
        let b = Rc::clone(&a);
        assert_eq!(Rc::strong_count(&a), 2);
        drop(b);
        assert_eq!(Rc::strong_count(&a), 1);
    }

    #[test]
    fn test_refcell() {
        let cell = RefCell::new(5);
        *cell.borrow_mut() += 5;
        assert_eq!(*cell.borrow(), 10);
    }

    #[test]
    fn test_arc() {
        let arc = Arc::new(42);
        let arc2 = Arc::clone(&arc);
        assert_eq!(*arc, *arc2);
    }
}
