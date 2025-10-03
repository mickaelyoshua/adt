use std::mem::MaybeUninit;
use std::ptr::drop_in_place;

#[derive(Default)]
pub struct CircularQueue<T> {
    vals: Vec<MaybeUninit<T>>,
    head: usize,
    tail: usize,
    is_empty: bool,
}

impl<T> Drop for CircularQueue<T> {
    fn drop(&mut self) {
        if self.is_empty {
            return;
        }
        
        if self.head < self.tail {
            for i in self.head..self.tail {
                unsafe {
                    drop_in_place(self.vals[i].assume_init_mut());
                }
            }
        } else {
            let capacity = self.capacity();
            for i in self.head..capacity {
                unsafe {
                    drop_in_place(self.vals[i].assume_init_mut());
                }
            }

            for i in 0..self.tail {
                unsafe {
                    drop_in_place(self.vals[i].assume_init_mut());
                }
            }
        }
    }
}

impl<T> CircularQueue<T> {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            head: 0,
            tail: 0,
            is_empty: true,
        }
    }

    pub fn is_full(&self) -> bool {
        self.head == self.tail && !self.is_empty
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn capacity(&self) -> usize {
        self.vals.len()
    }

    pub fn len(&self) -> usize {
        if self.is_empty {
            0
        } else if self.head == self.tail {
            // Queue is full (head == tail && !is_empty)
            self.capacity()
        } else if self.tail > self.head {
            self.tail - self.head
        } else {
            self.capacity() - self.head + self.tail
        }
    }

    pub fn grow(&mut self) {
        let len = self.len(); // number of elements
        let old_capacity = self.capacity(); // total capacity of the vector
        let new_capacity = if old_capacity == 0 { 4 } else { old_capacity * 2 }; // new capacity

        let mut new_vals: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
        unsafe { new_vals.set_len(new_capacity); } // allow indexing

        if self.is_empty {
            self.vals = new_vals;
            return
        }

        if self.head < self.tail {
            let mut i: usize = 0;
            let mut j: usize = self.head;
            while j < self.tail {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }
        } else {
            let mut i: usize = 0;
            let mut j: usize = self.head;
            while j < old_capacity {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }

            j = 0;
            while j < self.tail {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }
        }

        self.head = 0;
        self.vals = new_vals;
        self.tail = len;
    }

    pub fn enqueue(&mut self, val: T) {
        if self.is_full() || self.capacity() == 0 {
            self.grow();
        }

        self.vals[self.tail].write(val);
        self.tail = (self.tail + 1) % self.capacity(); // wraps back to start when reacher the capacity

        self.is_empty = false;
    }
    
    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty {
            return None;
        }

        let val = unsafe { self.vals[self.head].assume_init_read() };

        self.head = (self.head + 1) % self.capacity();
        
        if self.head == self.tail {
            self.is_empty = true;
        }

        Some(val)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty {
            return None;
        }

        let val_ref = unsafe { self.vals[self.head].assume_init_ref() };
        Some(val_ref)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_queue_is_empty() {
        let queue: CircularQueue<i32> = CircularQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        assert_eq!(queue.capacity(), 0);
    }

    #[test]
    fn test_enqueue_dequeue_single_element() {
        let mut queue = CircularQueue::new();
        queue.enqueue(42);

        assert!(!queue.is_empty());
        assert_eq!(queue.len(), 1);
        assert_eq!(queue.capacity(), 4); // Should grow to initial capacity

        assert_eq!(queue.dequeue(), Some(42));
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn test_peek() {
        let mut queue = CircularQueue::new();

        assert_eq!(queue.peek(), None);

        queue.enqueue(10);
        queue.enqueue(20);

        assert_eq!(queue.peek(), Some(&10));
        assert_eq!(queue.len(), 2); // peek doesn't remove

        queue.dequeue();
        assert_eq!(queue.peek(), Some(&20));
    }

    #[test]
    fn test_fifo_order() {
        let mut queue = CircularQueue::new();

        for i in 0..5 {
            queue.enqueue(i);
        }

        for i in 0..5 {
            assert_eq!(queue.dequeue(), Some(i));
        }

        assert!(queue.is_empty());
    }

    #[test]
    fn test_growth() {
        let mut queue = CircularQueue::new();

        // Fill beyond initial capacity to trigger growth
        for i in 0..10 {
            queue.enqueue(i);
        }

        assert_eq!(queue.len(), 10);
        assert!(queue.capacity() >= 10);

        // Verify FIFO order after growth
        for i in 0..10 {
            assert_eq!(queue.dequeue(), Some(i));
        }
    }

    #[test]
    fn test_wraparound() {
        let mut queue = CircularQueue::new();

        // Fill to initial capacity
        for i in 0..4 {
            queue.enqueue(i);
        }

        // Remove some
        assert_eq!(queue.dequeue(), Some(0));
        assert_eq!(queue.dequeue(), Some(1));

        // Add more to cause wraparound
        queue.enqueue(4);
        queue.enqueue(5);

        // Should still maintain FIFO order
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert!(queue.is_empty());
    }

    #[test]
    fn test_growth_with_wraparound() {
        let mut queue = CircularQueue::new();

        // Fill initial capacity
        for i in 0..4 {
            queue.enqueue(i);
        }

        // Dequeue to move head forward
        queue.dequeue();
        queue.dequeue();

        // Now head > 0. Add elements to fill and grow
        for i in 4..8 {
            queue.enqueue(i);
        }

        // Verify order is preserved after growth with wraparound
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), Some(4));
        assert_eq!(queue.dequeue(), Some(5));
        assert_eq!(queue.dequeue(), Some(6));
        assert_eq!(queue.dequeue(), Some(7));
    }

    #[test]
    fn test_dequeue_empty() {
        let mut queue: CircularQueue<i32> = CircularQueue::new();
        assert_eq!(queue.dequeue(), None);

        queue.enqueue(1);
        queue.dequeue();
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_with_strings() {
        let mut queue = CircularQueue::new();

        queue.enqueue(String::from("hello"));
        queue.enqueue(String::from("world"));

        assert_eq!(queue.dequeue(), Some(String::from("hello")));
        assert_eq!(queue.dequeue(), Some(String::from("world")));
    }

    #[test]
    fn test_drop_cleans_up() {
        // This test verifies Drop is called by using a type that tracks drops
        use std::rc::Rc;

        let val1 = Rc::new(42);
        let val2 = Rc::new(100);

        let mut queue = CircularQueue::new();
        queue.enqueue(Rc::clone(&val1));
        queue.enqueue(Rc::clone(&val2));

        assert_eq!(Rc::strong_count(&val1), 2);
        assert_eq!(Rc::strong_count(&val2), 2);

        drop(queue); // Should drop both Rc instances

        assert_eq!(Rc::strong_count(&val1), 1);
        assert_eq!(Rc::strong_count(&val2), 1);
    }

    #[test]
    fn test_is_full() {
        let mut queue = CircularQueue::new();

        // New queue with capacity 0 is not full (it's empty)
        assert!(!queue.is_full());

        // After first enqueue, grows to 4
        queue.enqueue(1);
        assert!(!queue.is_full());

        // Fill to capacity
        queue.enqueue(2);
        queue.enqueue(3);
        queue.enqueue(4);

        // Now it should be full (head == tail && !is_empty)
        assert!(queue.is_full());

        // Enqueue triggers growth
        queue.enqueue(5);
        assert!(!queue.is_full());
    }
}

