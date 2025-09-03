use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
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

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for item in value {
            list.push_right(item);
        }
        list
    }
}

impl<T> FromIterator<T> for LinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = LinkedList::new();
        for item in iter {
            list.push_right(item);
        }
        list
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_left(&mut self, val: T) {
        let layout = Layout::new::<Node<T>>();
        let new_node_ptr = unsafe { alloc(layout) };

        // if some error happen during allocation
        if new_node_ptr.is_null() {
            handle_alloc_error(layout);
        }
        
        // from *mut u8 -> *mut Node<T>
        let new_node_ptr = new_node_ptr as *mut Node<T>;

        unsafe {
            ptr::write(new_node_ptr, Node {
                val,
                next: self.head,
            });
        }

        if self.tail.is_null() {
            self.tail = new_node_ptr;
        }

        self.head = new_node_ptr;
    }

    pub fn push_right(&mut self, val: T) {
        let layout = Layout::new::<Node<T>>();
        let new_node_ptr = unsafe { alloc(layout) };

        if new_node_ptr.is_null() {
            handle_alloc_error(layout);
        }

        let new_node_ptr = new_node_ptr as *mut Node<T>;

        unsafe {
            ptr::write(new_node_ptr, Node {
                val,
                next: ptr::null_mut(),
            });
        }

        if self.head.is_null() {
            self.head = new_node_ptr;
            self.tail = new_node_ptr;
        } else {
            unsafe {
                (*self.tail).next = new_node_ptr;
            }
            self.tail = new_node_ptr;
        }
    }
}
