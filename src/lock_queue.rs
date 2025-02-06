// queue.rs
use std::ffi::c_void;

#[repr(C)]
pub struct Queue {
    inner: *mut c_void,
}

unsafe extern "C" {
   //  fn Queue_Init(q: *mut c_void);
    fn Queue_Enqueue(q: *mut c_void, value: i32);
    fn Queue_Dequeue(q: *mut c_void, value: *mut i32) -> i32;
}

impl Queue {
    pub fn new() -> Self {
        let queue: Queue = unsafe { std::mem::zeroed() };
        // todo: get the raw C pointer

        queue
    }

    pub fn enqueue(&self, value: i32) {
        unsafe { Queue_Enqueue(self.inner, value) }
    }

    pub fn dequeue(&self) -> Option<i32> {
        let mut value = 0;
        let result = unsafe { Queue_Dequeue(self.inner, &mut value) };

        if result == 0 { Some(value) } else { None }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_operations() {

        let queue = Queue::new();

        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);

        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None); 
    }
}
