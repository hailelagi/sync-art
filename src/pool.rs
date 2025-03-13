// pool.rs

use std::cell::UnsafeCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicI32};
use std::sync::atomic::Ordering::{Acquire, Release};

use crate::spin::SpinLock;


struct TaskQueue<T> {
    // dispatch inner to an mpmc?
    inner: Vec<Task<T>>,
    head: AtomicI32,
    tail: AtomicI32
}

struct Task<T> {
    task: T,
    start: AtomicBool,
}

impl <T>TaskQueue<T> {
    pub fn new(depth: AtomicI32) -> Self {
        TaskQueue {
            inner: Vec::new(),
            head: AtomicI32::new(0),
            tail: AtomicI32::new(0)
        }
    }
}

struct Pool<T> {
    // mu
    lock: SpinLock<T>,
    inner: PoolInner<T>,

    // todo: // condvar notfications on wake/sleep?
}

struct PoolInner<T> {
    threads: Vec<UnsafeCell<T>>,
    thread_count: AtomicI32,
    queue_depth: TaskQueue<T>,
}
