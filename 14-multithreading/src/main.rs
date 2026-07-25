//! Threads execute concurrently; ownership and synchronization keep sharing safe.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Barrier, Condvar, Mutex, RwLock, mpsc};
use std::thread;

fn heading(number: u8, title: &str) {
    println!("\n=== {number:02}: {title} ===");
}

fn main() {
    example_01_spawn_and_join();
    example_02_move_capture();
    example_03_scoped_threads();
    example_04_channels();
    example_05_multiple_producers();
    example_06_mutex_and_rwlock();
    example_07_atomics();
    example_08_barrier_and_condition_variable();
    example_09_worker_queue_and_panic_handling();
    example_10_deadlock_avoidance();
}

fn example_01_spawn_and_join() {
    heading(1, "spawn and join");

    let worker = thread::spawn(|| (1..=5).sum::<i32>());
    println!(
        "worker returned {}",
        worker.join().expect("worker should finish")
    );
}

fn example_02_move_capture() {
    heading(2, "move capture");

    let mut values = vec![2, 4, 6];
    values.push(8); // Keep this growable collection owned by the worker.
    let worker = thread::spawn(move || values.iter().product::<i32>());
    println!("moved values product={}", worker.join().unwrap());
}

fn example_03_scoped_threads() {
    heading(3, "scoped threads borrow local data");

    let values = [1, 2, 3, 4, 5, 6];
    let (left, right) = values.split_at(3);
    let total = thread::scope(|scope| {
        let left_worker = scope.spawn(|| left.iter().sum::<i32>());
        let right_worker = scope.spawn(|| right.iter().sum::<i32>());
        left_worker.join().unwrap() + right_worker.join().unwrap()
    });
    println!("scoped total={total}");
}

fn example_04_channels() {
    heading(4, "channels transfer messages");

    let (sender, receiver) = mpsc::channel();
    thread::spawn(move || sender.send(String::from("owned message")).unwrap());
    println!("received={}", receiver.recv().unwrap());
}

fn example_05_multiple_producers() {
    heading(5, "multiple producers");

    let (sender, receiver) = mpsc::channel();
    let workers: Vec<_> = (0..3)
        .map(|worker_id| {
            let sender = sender.clone();
            thread::spawn(move || sender.send(worker_id * 10).unwrap())
        })
        .collect();
    drop(sender); // Close the channel after all cloned senders finish.

    let mut values: Vec<_> = receiver.iter().collect();
    values.sort_unstable(); // Arrival order is intentionally unspecified.
    for worker in workers {
        worker.join().unwrap();
    }
    println!("messages={values:?}");
}

fn example_06_mutex_and_rwlock() {
    heading(6, "Arc<Mutex<_>> and RwLock");

    let counter = Arc::new(Mutex::new(0_u32));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || *counter.lock().unwrap() += 1)
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    println!("mutex counter={}", *counter.lock().unwrap());

    let settings = Arc::new(RwLock::new(String::from("v1")));
    *settings.write().unwrap() = "v2".into();
    let first = settings.read().unwrap();
    let second = settings.read().unwrap(); // Multiple readers may coexist.
    println!("rwlock readers: {first}, {second}");
}

fn example_07_atomics() {
    heading(7, "atomics");

    let counter = Arc::new(AtomicUsize::new(0));
    let workers: Vec<_> = (0..4)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..1_000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            })
        })
        .collect();
    for worker in workers {
        worker.join().unwrap();
    }
    println!("atomic counter={}", counter.load(Ordering::Relaxed));
}

fn example_08_barrier_and_condition_variable() {
    heading(8, "barrier and condition variable");

    let barrier = Arc::new(Barrier::new(3));
    let workers: Vec<_> = (0..2)
        .map(|id| {
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                println!("worker {id} ready");
                barrier.wait();
            })
        })
        .collect();
    barrier.wait(); // Main releases all three participants.
    for worker in workers {
        worker.join().unwrap();
    }

    let ready = Arc::new((Mutex::new(false), Condvar::new()));
    let worker_ready = Arc::clone(&ready);
    let worker = thread::spawn(move || {
        let (lock, signal) = &*worker_ready;
        *lock.lock().unwrap() = true;
        signal.notify_one();
    });
    let (lock, signal) = &*ready;
    let guard = signal
        .wait_while(lock.lock().unwrap(), |done| !*done)
        .unwrap();
    println!("condition satisfied={guard}");
    worker.join().unwrap();
}

enum Job {
    Square(i32),
    Stop,
}

fn example_09_worker_queue_and_panic_handling() {
    heading(9, "worker queue and panic handling");

    let (job_sender, job_receiver) = mpsc::channel();
    let (result_sender, result_receiver) = mpsc::channel();
    let worker = thread::spawn(move || {
        while let Ok(job) = job_receiver.recv() {
            match job {
                Job::Square(value) => result_sender.send(value * value).unwrap(),
                Job::Stop => break,
            }
        }
    });
    for value in [2, 3, 4] {
        job_sender.send(Job::Square(value)).unwrap();
    }
    job_sender.send(Job::Stop).unwrap();
    worker.join().unwrap();
    println!(
        "worker results={:?}",
        result_receiver.iter().collect::<Vec<_>>()
    );

    let panicking = thread::spawn(|| panic!("demonstration panic"));
    println!("panic captured by join={}", panicking.join().is_err());
}

fn example_10_deadlock_avoidance() {
    heading(10, "deadlock avoidance");

    println!("Acquire multiple locks in one documented global order.");
    println!("Keep lock guards short and never wait on a channel while holding a lock.");
    println!("Prefer messages when a task can own the mutable state.");
}
