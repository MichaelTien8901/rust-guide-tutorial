//! Async Basics Example
//!
//! Demonstrates async/await with tokio runtime.

use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() {
    println!("=== Async Basics ===\n");

    println!("--- Basic Async/Await ---");
    basic_async().await;

    println!("\n--- Concurrent Execution ---");
    concurrent_execution().await;

    println!("\n--- Async Functions ---");
    async_functions().await;

    println!("\n--- Spawning Tasks ---");
    spawning_tasks().await;

    println!("\n--- Select ---");
    select_example().await;

    println!("\n--- Timeouts ---");
    timeout_example().await;
}

async fn basic_async() {
    // Async functions return Futures
    // await executes them

    println!("Before sleep");
    sleep(Duration::from_millis(100)).await;
    println!("After sleep");

    // Async block
    let result = async {
        sleep(Duration::from_millis(50)).await;
        42
    }
    .await;

    println!("Async block result: {}", result);
}

async fn concurrent_execution() {
    // Sequential execution
    println!("Sequential:");
    let start = std::time::Instant::now();

    slow_operation("A", 100).await;
    slow_operation("B", 100).await;

    println!("Sequential took: {:?}", start.elapsed());

    // Concurrent with join!
    println!("\nConcurrent with join!:");
    let start = std::time::Instant::now();

    let (a, b) = tokio::join!(slow_operation("A", 100), slow_operation("B", 100));

    println!("Results: {}, {}", a, b);
    println!("Concurrent took: {:?}", start.elapsed());

    // try_join! for Results
    async fn fallible(name: &str) -> Result<String, &'static str> {
        sleep(Duration::from_millis(50)).await;
        Ok(format!("{} done", name))
    }

    let result = tokio::try_join!(fallible("X"), fallible("Y"));

    match result {
        Ok((x, y)) => println!("try_join results: {}, {}", x, y),
        Err(e) => println!("Error: {}", e),
    }
}

async fn slow_operation(name: &str, ms: u64) -> String {
    println!("{} starting", name);
    sleep(Duration::from_millis(ms)).await;
    println!("{} done", name);
    format!("{} result", name)
}

async fn async_functions() {
    // Async function returning value
    async fn compute(x: i32) -> i32 {
        sleep(Duration::from_millis(10)).await;
        x * 2
    }

    let result = compute(21).await;
    println!("compute(21) = {}", result);

    // Async function with references
    async fn process_data(data: &[i32]) -> i32 {
        sleep(Duration::from_millis(10)).await;
        data.iter().sum()
    }

    let data = vec![1, 2, 3, 4, 5];
    let sum = process_data(&data).await;
    println!("Sum: {}", sum);

    // Generic async function
    async fn identity<T>(value: T) -> T {
        value
    }

    let x = identity(42).await;
    let s = identity("hello").await;
    println!("identity: {}, {}", x, s);
}

async fn spawning_tasks() {
    // Spawn independent tasks
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "Task 1 complete"
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_millis(50)).await;
        "Task 2 complete"
    });

    // Tasks run concurrently
    println!("Tasks spawned, doing other work...");

    // Wait for results
    let result1 = handle1.await.unwrap();
    let result2 = handle2.await.unwrap();

    println!("Results: {}, {}", result1, result2);

    // Spawn with move
    let data = vec![1, 2, 3];
    let handle = tokio::spawn(async move {
        // data is moved into task
        data.iter().sum::<i32>()
    });

    let sum = handle.await.unwrap();
    println!("Spawned sum: {}", sum);

    // Many spawned tasks
    let handles: Vec<_> = (0..5)
        .map(|i| {
            tokio::spawn(async move {
                sleep(Duration::from_millis(10)).await;
                i * 10
            })
        })
        .collect();

    let results: Vec<i32> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|r| r.unwrap())
        .collect();

    println!("All results: {:?}", results);
}

async fn select_example() {
    use tokio::select;

    // select! runs until one branch completes
    select! {
        _ = sleep(Duration::from_millis(100)) => {
            println!("Sleep completed first");
        }
        result = async { 42 } => {
            println!("Immediate completed first: {}", result);
        }
    }

    // Racing futures
    async fn fast() -> &'static str {
        sleep(Duration::from_millis(50)).await;
        "fast"
    }

    async fn slow() -> &'static str {
        sleep(Duration::from_millis(100)).await;
        "slow"
    }

    let winner = select! {
        result = fast() => result,
        result = slow() => result,
    };

    println!("Winner: {}", winner);
}

async fn timeout_example() {
    use tokio::time::timeout;

    // Successful operation
    let result = timeout(Duration::from_millis(100), async {
        sleep(Duration::from_millis(50)).await;
        "completed"
    })
    .await;

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(_) => println!("Timed out"),
    }

    // Timed out operation
    let result = timeout(Duration::from_millis(50), async {
        sleep(Duration::from_millis(100)).await;
        "completed"
    })
    .await;

    match result {
        Ok(value) => println!("Success: {}", value),
        Err(_) => println!("Timed out"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_async_function() {
        async fn double(x: i32) -> i32 {
            x * 2
        }

        assert_eq!(double(21).await, 42);
    }

    #[tokio::test]
    async fn test_spawn() {
        let handle = tokio::spawn(async { 42 });
        assert_eq!(handle.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_join() {
        let (a, b) = tokio::join!(async { 1 }, async { 2 });
        assert_eq!(a + b, 3);
    }
}
