#![allow(unused)]

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Release};

pub struct SpinLock<T> {
    lock: SpinLockInner,
    data: UnsafeCell<T>,
}

unsafe impl<T> Sync for SpinLock<T> {}

struct SpinLockInner {
    locked: AtomicBool,
}

impl SpinLockInner {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }
}

impl<T> SpinLock<T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: SpinLockInner::new(),
            data: UnsafeCell::new(data),
        }
    }

    fn lock(&self) {
        while self.lock.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.lock.locked.store(false, Release);
    }

    // move out of Arc `safely` because we're keeping track of our inner lifetime with
    // an Arc - ideally we'd replace this with a better strategy for garbage collection
    pub fn take(&self) -> T
    where
        T: Default,
    {
        self.lock();
        let old_value = std::mem::take(unsafe { &mut *self.data.get() });
        self.unlock();
        old_value
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_it_spins() {
        let spin = Arc::new(SpinLock::new(vec![1, 2, 3]));
        let mut handles = vec![];

        for i in 4..10 {
            let spin = Arc::clone(&spin);

            let frame = thread::spawn(move || {
                spin.lock();

                unsafe { (*spin.data.get()).push(i) };
                spin.unlock();
            });

            handles.push(frame);
        }
        for handle in handles {
            handle.join().unwrap();
        }

        let mut result = spin.take();
        result.sort();

        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
