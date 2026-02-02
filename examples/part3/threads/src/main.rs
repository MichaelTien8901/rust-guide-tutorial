//! Threads Example
//!
//! Demonstrates thread creation, joining, and move closures.

use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Threads ===\n");

    println!("--- Basic Thread Spawning ---");
    basic_spawning();

    println!("\n--- Join Handles ---");
    join_handles();

    println!("\n--- Move Closures ---");
    move_closures();

    println!("\n--- Thread Builder ---");
    thread_builder();

    println!("\n--- Scoped Threads ---");
    scoped_threads();

    println!("\n--- Parallel Processing ---");
    parallel_processing();
}

fn basic_spawning() {
    // Spawn a new thread
    thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Main thread continues
    for i in 1..3 {
        println!("Main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Note: spawned thread may not finish before main exits
    thread::sleep(Duration::from_millis(10));
}

fn join_handles() {
    // Get handle to wait for thread completion
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Spawned: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
        42 // Return value
    });

    // Main thread work
    for i in 1..3 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for thread and get return value
    let result = handle.join().unwrap();
    println!("Thread returned: {}", result);

    // Multiple threads
    let handles: Vec<_> = (0..3)
        .map(|i| {
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(10));
                i * 10
            })
        })
        .collect();

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    println!("All results: {:?}", results);
}

fn move_closures() {
    // move keyword transfers ownership to thread
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Vector in thread: {:?}", v);
        v.len()
    });

    // v is no longer accessible here
    // println!("{:?}", v); // Error!

    let len = handle.join().unwrap();
    println!("Vector length: {}", len);

    // Without move - would be error
    // let v2 = vec![1, 2, 3];
    // thread::spawn(|| println!("{:?}", v2)); // Error: may outlive borrowed value

    // Cloning for thread
    let original = vec![1, 2, 3];
    let cloned = original.clone();

    let handle = thread::spawn(move || {
        println!("Cloned in thread: {:?}", cloned);
    });

    println!("Original still available: {:?}", original);
    handle.join().unwrap();
}

fn thread_builder() {
    // Configure thread with Builder
    let builder = thread::Builder::new()
        .name("worker".to_string())
        .stack_size(4 * 1024 * 1024); // 4MB stack

    let handle = builder
        .spawn(|| {
            println!("Thread name: {:?}", thread::current().name());
            thread::current().id()
        })
        .unwrap();

    let id = handle.join().unwrap();
    println!("Thread ID: {:?}", id);

    // Current thread info
    println!("Main thread name: {:?}", thread::current().name());
    println!("Main thread ID: {:?}", thread::current().id());
}

fn scoped_threads() {
    // Scoped threads can borrow from local scope
    let numbers = vec![1, 2, 3, 4, 5];
    let mut results = vec![];

    thread::scope(|s| {
        // These threads can borrow numbers
        let t1 = s.spawn(|| {
            let sum: i32 = numbers.iter().sum();
            sum
        });

        let t2 = s.spawn(|| {
            let product: i32 = numbers.iter().product();
            product
        });

        results.push(t1.join().unwrap());
        results.push(t2.join().unwrap());
    });

    // numbers still accessible!
    println!("Numbers: {:?}", numbers);
    println!("Results: {:?}", results);

    // Mutable access in scoped threads
    let mut data = vec![0; 5];

    thread::scope(|s| {
        for (i, elem) in data.iter_mut().enumerate() {
            s.spawn(move || {
                *elem = i * 10;
            });
        }
    });

    println!("Modified data: {:?}", data);
}

fn parallel_processing() {
    // Process chunks in parallel
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 3;

    let results: Vec<i32> = thread::scope(|s| {
        let handles: Vec<_> = data
            .chunks(chunk_size)
            .map(|chunk| {
                s.spawn(move || {
                    let sum: i32 = chunk.iter().sum();
                    println!("Chunk {:?} sum: {}", chunk, sum);
                    sum
                })
            })
            .collect();

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    let total: i32 = results.iter().sum();
    println!("Total sum: {}", total);

    // Parallel map
    let items = vec![1, 2, 3, 4, 5];
    let squared: Vec<i32> = thread::scope(|s| {
        let handles: Vec<_> = items
            .iter()
            .map(|&x| s.spawn(move || x * x))
            .collect();

        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });

    println!("Squared: {:?}", squared);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spawn_join() {
        let handle = thread::spawn(|| 42);
        assert_eq!(handle.join().unwrap(), 42);
    }

    #[test]
    fn test_move_closure() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || v.len());
        assert_eq!(handle.join().unwrap(), 3);
    }

    #[test]
    fn test_scoped_threads() {
        let data = vec![1, 2, 3];
        let sum = thread::scope(|s| {
            s.spawn(|| data.iter().sum::<i32>()).join().unwrap()
        });
        assert_eq!(sum, 6);
    }
}
