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

impl<T> From<Vec<T>> for LinkedList<T> {
    fn from(value: Vec<T>) -> Self {
        let mut list = LinkedList::new();
        for v in value {
            list.push_right(v);
        }
        list
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

impl<T: PartialEq> LinkedList<T> {
    pub fn find(&self, val: &T) -> Option<&T> {
        let mut current_link = &self.head;

        while let Some(current_node) = current_link {
            if current_node.val == *val {
                return Some(&current_node.val);
            }
            current_link = &current_node.next;
        }
        None
    }

    // take-inspect-replace logic
    pub fn delete(&mut self, val: &T) -> Option<T> {
        // previous ptr to update the tail if necessary
        let mut prev_node_ptr: *mut Node<T> = ptr::null_mut();
        let mut current_link = &mut self.head;

        // dont borrow, take ownership. Take out of the list to manipulate
        while let Some(mut current_node) = current_link.take() {
            if current_node.val == *val {
                *current_link = current_node.next.take();
                
                let deleted_node_ptr: *const Node<T> = &*current_node;
                if ptr::eq(self.tail, deleted_node_ptr) {
                    self.tail = prev_node_ptr;
                }

                return Some(current_node.val);
            }

            // update previous pointer
            prev_node_ptr = &mut *current_node;
            
            // put back on the list
            *current_link = Some(current_node);

            // update to the next
            current_link = &mut current_link.as_mut().unwrap().next;
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Constructor tests ---
    #[test]
    fn test_new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.head.is_none());
        assert!(list.tail.is_null());
    }

    // --- From<Vec<T>> tests ---
    #[test]
    fn test_from_vec() {
        let list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.head.as_ref().unwrap().val, 1);
        unsafe {
            assert_eq!((*list.tail).val, 3);
        }
        assert_eq!(list.find(&1), Some(&1));
        assert_eq!(list.find(&2), Some(&2));
        assert_eq!(list.find(&3), Some(&3));
    }

    #[test]
    fn test_from_empty_vec() {
        let list = LinkedList::from(Vec::<i32>::new());
        assert!(list.head.is_none());
        assert!(list.tail.is_null());
    }

    // --- Push operation tests ---
    #[test]
    fn test_push_left_to_empty_list() {
        let mut list = LinkedList::new();
        list.push_left(1);
        assert_eq!(list.head.as_ref().unwrap().val, 1);
        unsafe {
            assert_eq!((*list.tail).val, 1);
        }
    }

    #[test]
    fn test_push_left_updates_head() {
        let mut list = LinkedList::new();
        list.push_left(1);
        list.push_left(2);
        assert_eq!(list.head.as_ref().unwrap().val, 2);
        unsafe {
            assert_eq!((*list.tail).val, 1);
        }
    }

    #[test]
    fn test_push_right_to_empty_list() {
        let mut list = LinkedList::new();
        list.push_right(1);
        assert_eq!(list.head.as_ref().unwrap().val, 1);
        unsafe {
            assert_eq!((*list.tail).val, 1);
        }
    }

    #[test]
    fn test_push_right_updates_tail() {
        let mut list = LinkedList::new();
        list.push_right(1);
        list.push_right(2);
        assert_eq!(list.head.as_ref().unwrap().val, 1);
        unsafe {
            assert_eq!((*list.tail).val, 2);
        }
    }

    // --- Find operation tests ---
    #[test]
    fn test_find_in_empty_list() {
        let list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.find(&1), None);
    }

    #[test]
    fn test_find_non_existent_element() {
        let list = LinkedList::from(vec![1, 2]);
        assert_eq!(list.find(&3), None);
    }

    #[test]
    fn test_find_head_element() {
        let list = LinkedList::from(vec![1, 2]);
        assert_eq!(list.find(&1), Some(&1));
    }

    #[test]
    fn test_find_tail_element() {
        let list = LinkedList::from(vec![1, 2]);
        assert_eq!(list.find(&2), Some(&2));
    }

    #[test]
    fn test_find_middle_element() {
        let list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.find(&2), Some(&2));
    }

    // --- Delete operation tests ---
    #[test]
    fn test_delete_from_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        assert_eq!(list.delete(&1), None);
    }

    #[test]
    fn test_delete_non_existent_element() {
        let mut list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.delete(&4), None);
    }

    #[test]
    fn test_delete_only_element_in_list() {
        let mut list = LinkedList::from(vec![1]);
        assert_eq!(list.delete(&1), Some(1));
        assert!(list.head.is_none());
        assert!(list.tail.is_null());
    }

    #[test]
    fn test_delete_head_element_updates_head() {
        let mut list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.delete(&1), Some(1));
        assert_eq!(list.head.as_ref().unwrap().val, 2);
        assert_eq!(list.find(&1), None);
    }

    #[test]
    fn test_delete_tail_element_updates_tail() {
        let mut list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.delete(&3), Some(3));
        unsafe {
            assert_eq!((*list.tail).val, 2);
        }
        assert_eq!(list.find(&3), None);
    }

    #[test]
    fn test_delete_middle_element() {
        let mut list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list.delete(&2), Some(2));
        assert_eq!(list.head.as_ref().unwrap().next.as_ref().unwrap().val, 3);
        assert_eq!(list.find(&2), None);
    }

    // --- Mixed operation tests ---
    #[test]
    fn test_interleaved_push_and_delete() {
        let mut list = LinkedList::new();
        list.push_right(1);
        list.push_right(2);
        list.push_left(0); // 0, 1, 2
        assert_eq!(list.delete(&1), Some(1)); // 0, 2
        assert_eq!(list.head.as_ref().unwrap().val, 0);
        unsafe {
            assert_eq!((*list.tail).val, 2);
        }
        list.push_right(3); // 0, 2, 3
        assert_eq!(list.find(&3), Some(&3));
        unsafe {
            assert_eq!((*list.tail).val, 3);
        }
        assert_eq!(list.delete(&0), Some(0)); // 2, 3
        assert_eq!(list.head.as_ref().unwrap().val, 2);
    }

    // --- Drop implementation tests ---
    #[test]
    fn test_drop_on_list_with_items() {
        use std::cell::RefCell;
        let drop_count = RefCell::new(0);

        struct DropCounter<'a> {
            _val: i32,
            count: &'a RefCell<i32>,
        }

        impl<'a> Drop for DropCounter<'a> {
            fn drop(&mut self) {
                *self.count.borrow_mut() += 1;
            }
        }

        {
            let mut list = LinkedList::new();
            list.push_right(DropCounter { _val: 1, count: &drop_count });
            list.push_right(DropCounter { _val: 2, count: &drop_count });
            list.push_right(DropCounter { _val: 3, count: &drop_count });
        } // list drops here

        assert_eq!(*drop_count.borrow(), 3);
    }
}
