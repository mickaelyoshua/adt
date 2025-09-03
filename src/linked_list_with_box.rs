use std::ptr;

// Box is necessary because the struct Node is a recrusive struct
// So to make the compiler know how much space must be allocated it is used
// a pointer. In this case a Smart and raw pointer
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

// Define behavior when a value is going out of scope
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

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(v: Vec<T>) -> LinkedList<T> {
        let mut list = LinkedList::new();
        for item in v {
            list.push_right(item);
        }
        list
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

#[derive(Debug, PartialEq)]
pub enum DeleteError {
    NotFound,
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

    fn delete_value(&mut self, val: &T) -> Result<T,DeleteError> {
        if self.head.is_none() {
            return Err(DeleteError::NotFound);
        }
        
        let head_node = &self.head.as_ref().unwrap(); // reference so it does not take ownership yet
        if head_node.val == *val {
            let deleted_head = self.head.take().unwrap();
            self.head = deleted_head.next;

            // if after moving the ownership of the head to next, head is none
            // it means the list is now empty
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            return Ok(deleted_head.val);
        }

        let mut current = &mut self.head;

        while let Some(current_node) = current {
            // is not the current node since is not the head (checked before)
            if let Some(next_node) = &mut current_node.next { // see the next node
                if next_node.val == *val {
                    // take ownership of the node after the current
                    let delete_node = current_node.next.take().unwrap();

                    // give the ownership to the current
                    current_node.next = delete_node.next;

                    // now the next node is empty (deleted the tail)
                    if current_node.next.is_none() {
                        self.tail = &mut **current_node;
                    }

                    return Ok(delete_node.val);
                }
            }
            // update to evaluate the node after the next one
            current = &mut current_node.next;
        }
        Err(DeleteError::NotFound)
    }
}
