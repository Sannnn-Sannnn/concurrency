use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

struct Node<T> {
    value: Option<T>,
    next: AtomicPtr<Node<T>>,
}

pub struct NonBlockingQueue<T> {
    head: AtomicPtr<Node<T>>,
    tail: AtomicPtr<Node<T>>,
}

impl<T> NonBlockingQueue<T> {
    pub fn new() -> Self {
        // Dummy node
        // This way: head -> dummy <- tail
        let dummy = Box::into_raw(Box::new(Node {
            value: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        Self {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    // Produce
    pub fn push(&self, value: T) {
        let new_node = Box::into_raw(Box::new(Node {
            value: Some(value),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);

            let next = unsafe {
                (*tail).next.load(Ordering::Acquire)
            };

            // Tail really points to last node
            if next.is_null() {
                let result = unsafe {
                    (*tail).next.compare_exchange(
                        ptr::null_mut(),
                        new_node,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    )
                };

                // Successfully linked node
                if result.is_ok() {
                    // Try to advance tail
                    let _ = self.tail.compare_exchange(
                        tail,
                        new_node,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    );

                    return;
                }
            } else {
                // Tail is lagging behind.
                // Help advance it.
                let _ = self.tail.compare_exchange(
                    tail,
                    next,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            }
        }
    }

    // Consume
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);

            let next = unsafe {
                (*head).next.load(Ordering::Acquire)
            };

            // Queue empty
            if next.is_null() {
                return None;
            }

            // Try to move head forward
            let result = self.head.compare_exchange(
                head,
                next,
                Ordering::AcqRel,
                Ordering::Acquire,
            );

            if result.is_ok() {
                unsafe {
                    return (*next).value.take();
                }
            }
        }
    }
}