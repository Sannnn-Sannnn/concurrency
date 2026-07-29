use std::env;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::thread;
use std::time::Instant;

mod blocking_queue;
mod non_blocking_queue;

use blocking_queue::BlockingQueue;
use non_blocking_queue::NonBlockingQueue;

fn main() {
    // ----------------------------------------
    // Parse CLI args
    // ----------------------------------------

    let args: Vec<String> = env::args().collect();

    let mut producers = 4usize;
    let mut consumers = 4usize;
    let mut items = 100000usize;
    let mut queue_type = String::from("blocking");

    let mut i = 1;

    while i < args.len() {
        match args[i].as_str() {
            "--producers" => {
                producers = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--consumers" => {
                consumers = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--items" => {
                items = args[i + 1].parse().unwrap();
                i += 2;
            }
            "--queue" => {
                queue_type = args[i + 1].clone();
                i += 2;
            }
            _ => {
                i += 1;
            }
        }
    }

    println!("-----------------------------------");
    println!("Queue type : {}", queue_type);
    println!("Producers  : {}", producers);
    println!("Consumers  : {}", consumers);
    println!("Items/thread : {}", items);
    println!("-----------------------------------");

    let total_items = producers * items;

    let produced = Arc::new(AtomicUsize::new(0));
    let consumed = Arc::new(AtomicUsize::new(0));

    let start = Instant::now();

    // ============================================================
    // BLOCKING QUEUE
    // ============================================================

    if queue_type == "blocking" {
        let queue = Arc::new(BlockingQueue::<Option<usize>>::new());

        let mut producer_handles = vec![];
        let mut consumer_handles = vec![];

        // ----------------------------------------
        // Producers
        // ----------------------------------------

        for producer_id in 0..producers {
            let queue = Arc::clone(&queue);
            let produced = Arc::clone(&produced);

            let handle = thread::spawn(move || {
                for i in 0..items {
                    let value = producer_id * items + i;

                    queue.push(Some(value));

                    produced.fetch_add(1, Ordering::Relaxed);
                }
            });

            producer_handles.push(handle);
        }

        // ----------------------------------------
        // Consumers
        // ----------------------------------------

        for _ in 0..consumers {
            let queue = Arc::clone(&queue);
            let consumed = Arc::clone(&consumed);

            let handle = thread::spawn(move || {
                loop {
                    match queue.pop() {
                        Some(value) => {
                            let _ = value;

                            consumed.fetch_add(1, Ordering::Relaxed);
                        }

                        None => {
                            // Poison pill received
                            break;
                        }
                    }
                }
            });

            consumer_handles.push(handle);
        }

        // ----------------------------------------
        // Wait producers
        // ----------------------------------------

        for handle in producer_handles {
            handle.join().unwrap();
        }

        // ----------------------------------------
        // Send poison pills
        // ----------------------------------------

        for _ in 0..consumers {
            queue.push(None);
        }

        // ----------------------------------------
        // Wait consumers
        // ----------------------------------------

        for handle in consumer_handles {
            handle.join().unwrap();
        }
    }
    // ============================================================
    // NON BLOCKING QUEUE
    // ============================================================

    else if queue_type == "nonblocking" {
        let queue = Arc::new(NonBlockingQueue::<usize>::new());

        let mut handles = vec![];

        // ----------------------------------------
        // Producers
        // ----------------------------------------

        for producer_id in 0..producers {
            let queue = Arc::clone(&queue);
            let produced = Arc::clone(&produced);

            let handle = thread::spawn(move || {
                for i in 0..items {
                    let value = producer_id * items + i;

                    queue.push(value);

                    produced.fetch_add(1, Ordering::Relaxed);
                }
            });

            handles.push(handle);
        }

        // ----------------------------------------
        // Consumers
        // ----------------------------------------

        for _ in 0..consumers {
            let queue = Arc::clone(&queue);
            let consumed = Arc::clone(&consumed);

            let handle = thread::spawn(move || {
                loop {
                    let current = consumed.load(Ordering::Relaxed);

                    if current >= total_items {
                        break;
                    }

                    if queue.pop().is_some() {
                        consumed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });

            handles.push(handle);
        }

        // ----------------------------------------
        // Join
        // ----------------------------------------

        for handle in handles {
            handle.join().unwrap();
        }
    }

    else {
        panic!("Unknown queue type");
    }

    let elapsed = start.elapsed();

    println!();
    println!("Produced : {}", produced.load(Ordering::Relaxed));
    println!("Consumed : {}", consumed.load(Ordering::Relaxed));
    println!("Expected : {}", total_items);
    println!("Elapsed  : {:.2?}", elapsed);

    // ----------------------------------------
    // Validation
    // ----------------------------------------

    if consumed.load(Ordering::Relaxed) == total_items {
        println!();
        println!("SUCCESS: all elements consumed");
    } else {
        println!();
        println!("ERROR: mismatch detected");
    }
}