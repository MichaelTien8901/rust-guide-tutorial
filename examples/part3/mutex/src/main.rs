//! Mutex and RwLock Example
//!
//! Demonstrates shared mutable state with locks.

use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Mutex and RwLock ===\n");

    println!("--- Basic Mutex ---");
    basic_mutex();

    println!("\n--- Mutex with Threads ---");
    mutex_with_threads();

    println!("\n--- RwLock ---");
    rwlock_example();

    println!("\n--- Avoiding Deadlocks ---");
    avoiding_deadlocks();

    println!("\n--- Practical Patterns ---");
    practical_patterns();
}

fn basic_mutex() {
    // Mutex provides interior mutability with locking
    let m = Mutex::new(5);

    {
        // lock() returns MutexGuard
        let mut num = m.lock().unwrap();
        *num = 6;
        println!("Changed to: {}", num);
    } // MutexGuard dropped, lock released

    println!("After lock released: {:?}", m.lock().unwrap());

    // try_lock - non-blocking
    let m2 = Mutex::new(10);
    if let Ok(mut guard) = m2.try_lock() {
        *guard += 1;
        println!("try_lock succeeded: {}", guard);
    } else {
        println!("try_lock failed");
    }
}

fn mutex_with_threads() {
    // Arc + Mutex for shared state across threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Thread {} incremented to {}", i, *num);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());

    // Multiple values in mutex
    let data = Arc::new(Mutex::new(vec![]));
    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec.push(i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Collected: {:?}", data.lock().unwrap());
}

fn rwlock_example() {
    // RwLock allows multiple readers OR one writer
    let data = Arc::new(RwLock::new(vec![1, 2, 3]));

    // Multiple readers
    let data1 = Arc::clone(&data);
    let data2 = Arc::clone(&data);

    let r1 = thread::spawn(move || {
        let read = data1.read().unwrap();
        println!("Reader 1: {:?}", *read);
        thread::sleep(Duration::from_millis(50));
        println!("Reader 1 done");
    });

    let r2 = thread::spawn(move || {
        let read = data2.read().unwrap();
        println!("Reader 2: {:?}", *read);
        thread::sleep(Duration::from_millis(50));
        println!("Reader 2 done");
    });

    r1.join().unwrap();
    r2.join().unwrap();

    // Writer (exclusive access)
    {
        let mut write = data.write().unwrap();
        write.push(4);
        println!("Writer added 4: {:?}", *write);
    }

    // Read again
    println!("After write: {:?}", data.read().unwrap());
}

fn avoiding_deadlocks() {
    // Deadlock: two threads waiting for each other's locks

    // BAD: Could deadlock (commented out)
    /*
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    // Thread 1: locks a, then b
    let t1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _b = b1.lock().unwrap(); // Waits for b
    });

    // Thread 2: locks b, then a
    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);
    let t2 = thread::spawn(move || {
        let _b = b2.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        let _a = a2.lock().unwrap(); // Waits for a -> DEADLOCK
    });
    */

    // GOOD: Always lock in same order
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _a = a1.lock().unwrap();
        let _b = b1.lock().unwrap();
        println!("Thread 1: got both locks");
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let _a = a2.lock().unwrap(); // Same order!
        let _b = b2.lock().unwrap();
        println!("Thread 2: got both locks");
    });

    t1.join().unwrap();
    t2.join().unwrap();

    // GOOD: Use try_lock with retry
    let lock = Arc::new(Mutex::new(0));
    let lock2 = Arc::clone(&lock);

    thread::spawn(move || {
        loop {
            if let Ok(mut guard) = lock2.try_lock() {
                *guard += 1;
                println!("Got lock with try_lock");
                break;
            }
            thread::sleep(Duration::from_millis(1));
        }
    })
    .join()
    .unwrap();

    println!("Deadlock avoidance demonstrated");
}

fn practical_patterns() {
    // Thread-safe counter
    #[derive(Clone)]
    struct Counter {
        value: Arc<Mutex<i32>>,
    }

    impl Counter {
        fn new() -> Self {
            Counter {
                value: Arc::new(Mutex::new(0)),
            }
        }

        fn increment(&self) {
            let mut val = self.value.lock().unwrap();
            *val += 1;
        }

        fn get(&self) -> i32 {
            *self.value.lock().unwrap()
        }
    }

    let counter = Counter::new();
    let mut handles = vec![];

    for _ in 0..5 {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        }));
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Counter: {}", counter.get());

    // Thread-safe cache
    struct Cache {
        data: RwLock<std::collections::HashMap<String, String>>,
    }

    impl Cache {
        fn new() -> Self {
            Cache {
                data: RwLock::new(std::collections::HashMap::new()),
            }
        }

        fn get(&self, key: &str) -> Option<String> {
            let read = self.data.read().unwrap();
            read.get(key).cloned()
        }

        fn set(&self, key: String, value: String) {
            let mut write = self.data.write().unwrap();
            write.insert(key, value);
        }
    }

    let cache = Arc::new(Cache::new());

    let c1 = Arc::clone(&cache);
    thread::spawn(move || {
        c1.set("key1".to_string(), "value1".to_string());
    })
    .join()
    .unwrap();

    println!("Cache get: {:?}", cache.get("key1"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex() {
        let m = Mutex::new(0);
        {
            let mut guard = m.lock().unwrap();
            *guard = 42;
        }
        assert_eq!(*m.lock().unwrap(), 42);
    }

    #[test]
    fn test_arc_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let c = Arc::clone(&counter);
            handles.push(thread::spawn(move || {
                *c.lock().unwrap() += 1;
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_rwlock() {
        let lock = RwLock::new(vec![1, 2, 3]);

        // Multiple readers
        let r1 = lock.read().unwrap();
        let r2 = lock.read().unwrap();
        assert_eq!(*r1, vec![1, 2, 3]);
        assert_eq!(*r2, vec![1, 2, 3]);
        drop(r1);
        drop(r2);

        // Writer
        lock.write().unwrap().push(4);
        assert_eq!(*lock.read().unwrap(), vec![1, 2, 3, 4]);
    }
}
