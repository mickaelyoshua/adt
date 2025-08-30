use std::ptr;

type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub enum SearchType {
    Loop,
    Recursive,
}

// Standard, conventional way to provide a default value
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList {
            head: None,
            tail: ptr::null_mut(),
            // null mutable pointer. Ignoring Rust safety guarantees
        }
    }
}

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // take ownership of head with 'take()'
        let mut current_link = self.head.take();

        // walk the list dropping each node
        while let Some(mut boxed_node) = current_link {
            current_link = boxed_node.next.take();
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // behave like a stack, always push to head
    pub fn push_left(&mut self, val: T) {
        let mut new_head = Box::new(Node {
            val,
            next: self.head.take(), // from the head take the value and leave a None
                                    // making the old head empty and this one the head
        });
        
        if self.tail.is_null() {
            self.tail = &mut *new_head;
        }

        self.head =  Some(new_head)
    }

    // append to the end - O(n)
    pub fn push_right(&mut self, val: T) {
        let mut new_tail = Box::new(Node { val, next: None });
        let new_tail_ptr: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                // unsafe dereference of raw pointer
                (*self.tail).next = Some(new_tail);
            }
        } else {
            // if list is empty
            self.head = Some(new_tail);
        }
        
        // update the tail pointer to new node
        self.tail = new_tail_ptr;
    }
}

impl<T: PartialEq> LinkedList<T> {
    // SEARCH
    pub fn find(&self, val: &T) -> Option<&T> {
        self.find_loop(val)
    }

    pub fn find_with(&self, val: &T, search_type: SearchType) -> Option<&T> {
        match search_type {
            SearchType::Loop => self.find_loop(val),
            SearchType::Recursive => Self::find_recursive(&self.head, val)
        }
    }

    fn find_loop(&self, val: &T) -> Option<&T> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.val == *val {
                return Some(&node.val);
            }
            current = &node.next;
        }
        None
    }

    fn find_recursive<'a>(link: &'a Link<T>, val: &T) -> Option<&'a T> {
        match link {
            None => None,
            Some(node) => {
                if node.val == *val {
                    Some(&node.val)
                } else {
                    Self::find_recursive(&node.next, val)
                }
            }
        }
    }

    fn contains(&self, val: &T) -> bool {
        self.find(val).is_some()
    }

    fn contains_with(&self, val: &T, search_type: SearchType) -> bool {
        self.find_with(val, search_type).is_some()
    }
}
