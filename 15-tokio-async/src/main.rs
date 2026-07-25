//! Tokio schedules many asynchronous tasks on a smaller set of runtime threads.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, Semaphore, mpsc, watch};
use tokio::task::JoinSet;
use tokio::time::{sleep, timeout};

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

#[tokio::main]
async fn main() {
    example_01_async_functions().await;
    example_02_spawned_tasks().await;
    example_03_join_concurrent_futures().await;
    example_04_select_and_timeout().await;
    example_05_async_channels().await;
    example_06_shared_state().await;
    example_07_semaphores().await;
    example_08_cancellation().await;
    example_09_blocking_work_and_task_errors().await;
    example_10_bounded_concurrent_work().await;
}

async fn delayed_value<T>(value: T, milliseconds: u64) -> T {
    sleep(Duration::from_millis(milliseconds)).await;
    value
}

async fn example_01_async_functions() {
    heading(1, "async functions and .await");
    println!("value={}", delayed_value(42, 10).await);
}

async fn example_02_spawned_tasks() {
    heading(2, "spawned tasks");

    let task = tokio::spawn(async { delayed_value(7, 10).await * 2 });
    match task.await {
        Ok(value) => println!("task returned {value}"),
        Err(error) => println!("task failed to join: {error}"),
    }
}

async fn example_03_join_concurrent_futures() {
    heading(3, "join! runs futures concurrently");

    let (first, second, third) = tokio::join!(
        delayed_value(1, 30),
        delayed_value(2, 20),
        delayed_value(3, 10)
    );
    println!("results={first}, {second}, {third}");
}

async fn example_04_select_and_timeout() {
    heading(4, "select! and timeout");

    tokio::select! {
        value = delayed_value("fast", 10) => println!("select winner={value}"),
        value = delayed_value("slow", 100) => println!("select winner={value}"),
    }

    let result = timeout(Duration::from_millis(20), delayed_value(99, 100)).await;
    println!("timeout elapsed={}", result.is_err());
}

async fn example_05_async_channels() {
    heading(5, "async channels");

    let (sender, mut receiver) = mpsc::channel(4);
    let producer = tokio::spawn(async move {
        for value in 1..=3 {
            sender.send(value).await.unwrap();
        }
    });

    let mut values = Vec::new();
    while let Some(value) = receiver.recv().await {
        values.push(value);
    }
    producer.await.unwrap();
    println!("received={values:?}");
}

async fn example_06_shared_state() {
    heading(6, "Arc with async Mutex and RwLock");

    let counter = Arc::new(Mutex::new(0));
    let mut tasks = JoinSet::new();
    for _ in 0..4 {
        let counter = Arc::clone(&counter);
        tasks.spawn(async move {
            *counter.lock().await += 1;
        });
    }
    while let Some(result) = tasks.join_next().await {
        result.unwrap();
    }
    println!("mutex counter={}", *counter.lock().await);

    let settings = Arc::new(RwLock::new(String::from("v1")));
    *settings.write().await = "v2".into();
    let value = settings.read().await;
    println!("rwlock value={value}");
}

async fn example_07_semaphores() {
    heading(7, "semaphores limit concurrency");

    let permits = Arc::new(Semaphore::new(2));
    let mut tasks = JoinSet::new();
    for id in 0..4 {
        let permits = Arc::clone(&permits);
        tasks.spawn(async move {
            let _permit = permits.acquire_owned().await.unwrap();
            println!("limited task {id} entered");
            sleep(Duration::from_millis(10)).await;
            id
        });
    }
    let mut completed = Vec::new();
    while let Some(result) = tasks.join_next().await {
        completed.push(result.unwrap());
    }
    completed.sort_unstable();
    println!("completed={completed:?}");
}

async fn example_08_cancellation() {
    heading(8, "cancellation with a watch channel");

    let (cancel_sender, mut cancel_receiver) = watch::channel(false);
    let worker = tokio::spawn(async move {
        let mut ticks = 0;
        loop {
            tokio::select! {
                _ = sleep(Duration::from_millis(10)) => ticks += 1,
                changed = cancel_receiver.changed() => {
                    if changed.is_err() || *cancel_receiver.borrow() {
                        break;
                    }
                }
            }
        }
        ticks
    });
    sleep(Duration::from_millis(25)).await;
    cancel_sender.send(true).unwrap();
    println!("worker stopped after {} ticks", worker.await.unwrap());
}

async fn example_09_blocking_work_and_task_errors() {
    heading(9, "blocking work and task errors");

    let calculation = tokio::task::spawn_blocking(|| (1_u64..=10_000).sum::<u64>());
    println!("blocking calculation={}", calculation.await.unwrap());

    let fallible = tokio::spawn(async { "not-a-number".parse::<u32>() });
    match fallible.await {
        Ok(Ok(value)) => println!("parsed {value}"),
        Ok(Err(error)) => println!("task returned an error: {error}"),
        Err(error) => println!("task panicked or was cancelled: {error}"),
    }
}

async fn example_10_bounded_concurrent_work() {
    heading(10, "bounded concurrent work");

    let permits = Arc::new(Semaphore::new(3));
    let mut tasks = JoinSet::new();
    for value in 1..=8 {
        let permits = Arc::clone(&permits);
        tasks.spawn(async move {
            let _permit = permits.acquire_owned().await.unwrap();
            delayed_value(value * value, 10).await
        });
    }

    let mut squares = Vec::new();
    while let Some(result) = tasks.join_next().await {
        squares.push(result.unwrap());
    }
    squares.sort_unstable();
    println!("squares={squares:?}");
}
