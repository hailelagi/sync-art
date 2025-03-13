use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

/**
 * https://pages.cs.wisc.edu/~remzi/OSTEP/threads-cv.pdf
 * https://lamport.azurewebsites.net/pubs/spec.pdf
 */

pub struct RingBuffer<T> {
    buffer: Vec<UnsafeCell<T>>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,  
}

unsafe impl<T: Send> Send for RingBuffer<T> {}
unsafe impl<T: Send> Sync for RingBuffer<T> {}

#[derive(Debug, PartialEq)]
pub enum BufferError {
    Full,
    Empty,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0 && capacity.is_power_of_two());
        
        let mut buffer = Vec::with_capacity(capacity);
        for _ in 0..capacity {
            // initialise mem as safe to r/w and assume `Drop`
            buffer.push(UnsafeCell::new(unsafe { std::mem::MaybeUninit::uninit().assume_init() }));
        }

        RingBuffer {
            buffer,
            capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, item: T) -> Result<(), BufferError> {
        unimplemented!()
    }

    pub fn pop(&self) -> Result<T, BufferError> {
        unimplemented!()
    }

    pub fn empty(&self) -> bool {
        unimplemented!()
    }

    pub fn full(&self) -> bool {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_spsc_basic() {
        let buffer = RingBuffer::new(4);
        
        assert!(buffer.empty());

        // push  d
        assert!(buffer.push(1).is_ok());
        assert!(buffer.push(2).is_ok());
        assert!(buffer.push(3).is_ok());

        // cap
        assert!(buffer.push(4).is_err());

        // pop
        assert_eq!(buffer.pop(), Ok(3));

        // len
        assert_eq!(buffer.len(), 2);

        assert_eq!(buffer.pop(), Ok(2));
        assert_eq!(buffer.pop(), Ok(1));
        assert!(buffer.pop().is_err());
    }

    #[test]
    fn test_spsc_threads() {
        let buffer = std::sync::Arc::new(RingBuffer::new(4));
        let producer_buffer = buffer.clone();
        
        let producer = thread::spawn(move || {
            for i in 0..8 {
                while producer_buffer.push(i).is_err() {
                    thread::yield_now();
                }
            }
        });

        let consumer = thread::spawn(move || {
            let mut sum = 0;
            let mut count = 0;
            
            while count < 8 {
                if let Ok(value) = buffer.pop() {
                    sum += value;
                    count += 1;
                } else {
                    thread::yield_now();
                }
            }
            
            sum
        });

        producer.join().unwrap();
        assert_eq!(consumer.join().unwrap(), 28);
    }
}
