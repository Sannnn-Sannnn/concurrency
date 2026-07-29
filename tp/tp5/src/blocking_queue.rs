use std::collections::VecDeque;
use std::sync::{ Mutex, Condvar };

// Wrapper -> Queue + sync
pub struct BlockingQueue<T> {
    queue: Mutex<VecDeque<T>>,
    not_empty: Condvar
}

impl<T> BlockingQueue<T> {
    pub fn new() -> BlockingQueue<T> {
        Self {
            queue: Mutex::new(VecDeque::new()),
            not_empty: Condvar::new()
        }
    }

    // Produce
    pub fn push(&self, value: T) {
        // Acquire lock
        let mut queue = self.queue.lock().unwrap();
        // Add element
        queue.push_back(value);
        // Notify one consumer
        self.not_empty.notify_one();
    }

    // Consume
    pub fn pop(&self) -> T {
        // Acquire lock
        let mut queue = self.queue.lock().unwrap();
        // Loop in case of spurious wakeup
        loop {
            // Try to consume element
            if let Some(x) = queue.pop_front() {
                return x;
            }
            // Could not consume, so wait for condvar
            queue = self.not_empty.wait(queue).unwrap();
        }
    }
}