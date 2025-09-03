use std::alloc::{dealloc, Layout};
use std::ptr;

struct Node<T> {
    val: T,
    next: *mut Node<T>,
}

pub struct LinkedList<T> {
	head: *mut Node<T>,
	tail: *mut Node<T>,
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head;

        while !current.is_null() {
            let next_node = unsafe { (*current).next };

            let layout = Layout::new::<Node<T>>();

            unsafe { dealloc(current as *mut u8, layout) };

            current = next_node;
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList { head: ptr::null_mut(), tail: ptr::null_mut() }
    }
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self::default()
    }
}
