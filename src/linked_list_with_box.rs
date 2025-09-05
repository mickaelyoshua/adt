use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current_link = self.head.take();
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut()
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_right(&mut self, val: T) {
        let mut new_boxed_node = Box::new(Node {
            val,
            next: None,
        });

        let new_boxed_node_ptr: *mut Node<T> = &mut *new_boxed_node;

        if self.head.is_none() {
            self.head = Some(new_boxed_node);
        } else {
            unsafe { (*self.tail).next = Some(new_boxed_node) };
        }
        
        self.tail = new_boxed_node_ptr;
    }

    pub fn push_left(&mut self, val: T) {
        let new_boxed_node = Box::new(Node {
            val,
            next: self.head.take(), // if it is already None will get None
        });

        self.head = Some(new_boxed_node);

        if self.tail.is_null() {
            // just put a node in head so unwrap is safe.
            // Double deref because is getting a ref with "as_mut" and to deref the Box pointer
            let new_boxed_node_ptr: *mut Node<T> = &mut **self.head.as_mut().unwrap();
            self.tail = new_boxed_node_ptr;
        }
    }
}
