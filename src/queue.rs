use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct Queue<T> {
    head: Link<T>,
    tail: *mut Node<T>,
    len: usize,
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(node) = self.head.take() {
            self.head = node.next;
        }
    }
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }

    pub fn len(&self) -> usize {
        unimplemented!()
    }

    pub fn enqueue(&mut self, val: T) {
        unimplemented!()
    }

    pub fn dequeue(&mut self) -> Option<T> {
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }
}
