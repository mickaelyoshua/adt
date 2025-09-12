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

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self {
            head: None,
            tail: ptr::null_mut(),
            len: 0,
        }
    }
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
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn peek(&self) -> Option<&T> {
        Some(&self.head.as_ref()?.val)
        // ? operator will make the function return None if head turns out to be None
    }

    pub fn enqueue(&mut self, val: T) {
        let mut new_boxed_node = Box::new(Node {
            val,
            next: None,
        });

        let new_boxed_node_ptr: *mut Node<T> = &mut *new_boxed_node;

        // if queue is empty
        if self.tail.is_null() {
            self.head = Some(new_boxed_node);
        } else {
            unsafe {
                (*self.tail).next = Some(new_boxed_node);
            }
        }

        self.tail = new_boxed_node_ptr;
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        let removed_node = self.head.take()?;
        self.head = removed_node.next;

        self.len -= 1;

        if self.head.is_none() {
            self.tail = ptr::null_mut();
        }

        Some(removed_node.val)
    }
}
