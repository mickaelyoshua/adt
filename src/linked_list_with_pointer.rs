use std::alloc::{alloc, dealloc, handle_alloc_error, Layout};
use std::marker::PhantomData;
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

// Structure to hold the iterator state
pub struct Iter<'a, T> {
    next: *const Node<T>,
    _marker: PhantomData<&'a T>,
    // this PhantomData make the compiler treats this structure like the 
    // 'a lifetime is beeing used, it contains a reference &'a T. That way it binds the
    // lifetime to the LinkedList created ensuring that the iterator will not live more than the
    // list
}

impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T; // defines what type the iterator yields

    fn next(&mut self) -> Option<Self::Item> {
        if !self.next.is_null() {
            unsafe {
                let node = &*self.next; // get the current node
                self.next = node.next; // update the current node to the next
                Some(&node.val)
            }
        } else {
            None
        }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // to get the iterator struct
    pub fn iter<'a>(&self) -> Iter<'a,T> {
        Iter { next: self.head, _marker: PhantomData }
    }

    pub fn push_left(&mut self, val: T) {
        let layout = Layout::new::<Node<T>>();
        let new_node_ptr = unsafe { alloc(layout) as *mut Node<T> };

        // if some error happen during allocation
        if new_node_ptr.is_null() {
            handle_alloc_error(layout);
        }

        unsafe {
            ptr::write(new_node_ptr, Node {
                val,
                next: self.head,
            });
        }

        self.head = new_node_ptr;

        if self.tail.is_null() {
            self.tail = new_node_ptr;
        }
    }

    pub fn push_right(&mut self, val: T) {
        let layout = Layout::new::<Node<T>>();
        let new_node_ptr = unsafe { alloc(layout) as *mut Node<T> };

        if new_node_ptr.is_null() {
            handle_alloc_error(layout);
        }

        unsafe {
            ptr::write(new_node_ptr, Node {
                val,
                next: ptr::null_mut(),
            });
        }

        if let Some(tail_node) = unsafe { self.tail.as_mut() } {
            tail_node.next = new_node_ptr;
        } else { // if tail is null the new node is also head
            self.head = new_node_ptr;
        }

        self.tail = new_node_ptr;
    }
}

pub enum DeleteError {
    NotFound,
}

impl<T: PartialEq> LinkedList<T> {
    pub fn find(&self, val: &T) -> Option<&T> {
        self.iter().find(|&v| v == val)
    }

    pub fn contains(&self, val: &T) -> bool {
        self.iter().any(|v| v == val)
    }

    pub fn delete(&mut self, val: &T) -> Result<T, DeleteError> {
        if self.head.is_null() {
            return Err(DeleteError::NotFound);
        }

        unsafe {
            // Case 1: Handle the head node
            if (*self.head).val == *val {
                let node_to_delete = self.head;
                let value = ptr::read(&(*node_to_delete).val);

                self.head = (*node_to_delete).next;
                if self.head.is_null() { // If list is now empty
                    self.tail = ptr::null_mut();
                }

                let layout = Layout::new::<Node<T>>();
                dealloc(node_to_delete as *mut u8, layout);
                return Ok(value);
            }

            let mut current = self.head;
            while !(*current).next.is_null() {
                let next_node_ptr = (*current).next;
                if (*next_node_ptr).val == *val {
                    let node_to_delete = next_node_ptr;
                    let value = ptr::read(&(*node_to_delete).val);

                    (*current).next = (*node_to_delete).next;

                    if self.tail == node_to_delete {
                        self.tail = current;
                    }

                    let layout = Layout::new::<Node<T>>();
                    dealloc(node_to_delete as *mut u8, layout);
                    return Ok(value);
                }
                current = (*current).next;
            }
        }

        Err(DeleteError::NotFound)
    }
}

#[cfg(test)]
mod tests {
}
