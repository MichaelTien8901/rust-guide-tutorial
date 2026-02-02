//! Channels Example
//!
//! Demonstrates message passing with mpsc channels.

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== Channels ===\n");

    println!("--- Basic Channel ---");
    basic_channel();

    println!("\n--- Multiple Messages ---");
    multiple_messages();

    println!("\n--- Multiple Producers ---");
    multiple_producers();

    println!("\n--- Channel with Structs ---");
    channel_with_structs();

    println!("\n--- Sync Channel ---");
    sync_channel();

    println!("\n--- Select-like Pattern ---");
    select_pattern();
}

fn basic_channel() {
    // Create channel
    let (tx, rx) = mpsc::channel();

    // Spawn thread with sender
    thread::spawn(move || {
        let msg = String::from("Hello from thread!");
        tx.send(msg).unwrap();
        // msg is moved, can't use it here
    });

    // Receive in main thread
    let received = rx.recv().unwrap();
    println!("Received: {}", received);
}

fn multiple_messages() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("message 1"),
            String::from("message 2"),
            String::from("message 3"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
        // tx is dropped here, channel closes
    });

    // Iterate over received messages
    for received in rx {
        println!("Got: {}", received);
    }

    println!("Channel closed");
}

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    // Clone sender for each producer
    let tx1 = tx.clone();
    let tx2 = tx.clone();
    drop(tx); // Drop original, only clones remain

    thread::spawn(move || {
        let messages = vec!["A1", "A2", "A3"];
        for msg in messages {
            tx1.send(format!("Producer A: {}", msg)).unwrap();
            thread::sleep(Duration::from_millis(50));
        }
    });

    thread::spawn(move || {
        let messages = vec!["B1", "B2", "B3"];
        for msg in messages {
            tx2.send(format!("Producer B: {}", msg)).unwrap();
            thread::sleep(Duration::from_millis(70));
        }
    });

    // Receive from all producers
    for received in rx {
        println!("Got: {}", received);
    }
}

fn channel_with_structs() {
    #[derive(Debug)]
    enum Command {
        Increment(i32),
        Decrement(i32),
        Print,
        Quit,
    }

    let (tx, rx) = mpsc::channel();

    // Worker thread
    let worker = thread::spawn(move || {
        let mut value = 0;

        loop {
            match rx.recv().unwrap() {
                Command::Increment(n) => {
                    value += n;
                    println!("Incremented by {}, now {}", n, value);
                }
                Command::Decrement(n) => {
                    value -= n;
                    println!("Decremented by {}, now {}", n, value);
                }
                Command::Print => {
                    println!("Current value: {}", value);
                }
                Command::Quit => {
                    println!("Worker quitting");
                    break;
                }
            }
        }

        value
    });

    // Send commands
    tx.send(Command::Increment(10)).unwrap();
    tx.send(Command::Increment(5)).unwrap();
    tx.send(Command::Print).unwrap();
    tx.send(Command::Decrement(3)).unwrap();
    tx.send(Command::Print).unwrap();
    tx.send(Command::Quit).unwrap();

    let final_value = worker.join().unwrap();
    println!("Final value: {}", final_value);
}

fn sync_channel() {
    // Bounded channel - blocks when full
    let (tx, rx) = mpsc::sync_channel(2); // Buffer size 2

    thread::spawn(move || {
        for i in 0..5 {
            println!("Sending {}", i);
            tx.send(i).unwrap();
            println!("Sent {}", i);
        }
    });

    thread::sleep(Duration::from_millis(100));

    for received in rx {
        println!("Received: {}", received);
        thread::sleep(Duration::from_millis(50));
    }
}

fn select_pattern() {
    // Rust doesn't have built-in select, but we can use try_recv
    let (tx1, rx1) = mpsc::channel();
    let (tx2, rx2) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx1.send("from channel 1").unwrap();
    });

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(30));
        tx2.send("from channel 2").unwrap();
    });

    // Poll both channels
    let mut received = vec![];
    let deadline = std::time::Instant::now() + Duration::from_millis(200);

    while received.len() < 2 && std::time::Instant::now() < deadline {
        if let Ok(msg) = rx1.try_recv() {
            println!("Got from rx1: {}", msg);
            received.push(msg);
        }
        if let Ok(msg) = rx2.try_recv() {
            println!("Got from rx2: {}", msg);
            received.push(msg);
        }
        thread::sleep(Duration::from_millis(10));
    }

    println!("Total received: {}", received.len());

    // Alternative: recv_timeout
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        tx.send("delayed message").unwrap();
    });

    match rx.recv_timeout(Duration::from_millis(100)) {
        Ok(msg) => println!("Received with timeout: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("Timed out"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel closed"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(42).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), 42);
    }

    #[test]
    fn test_multiple_messages() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            for i in 0..3 {
                tx.send(i).unwrap();
            }
        });

        let received: Vec<i32> = rx.iter().collect();
        assert_eq!(received, vec![0, 1, 2]);
    }

    #[test]
    fn test_sync_channel() {
        let (tx, rx) = mpsc::sync_channel(1);

        thread::spawn(move || {
            tx.send(1).unwrap();
            tx.send(2).unwrap();
        });

        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 2);
    }
}
