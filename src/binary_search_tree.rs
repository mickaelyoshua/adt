use std::ptr;
use std::cmp::Ordering;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T> {
    val: T,
    parent: *mut Node<T>,
    left: Link<T>,
    right: Link<T>,
}

pub struct BinarySearchTree<T> {
    root: Link<T>,
}

impl<T> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Drop for BinarySearchTree<T> {
    fn drop(&mut self) {
        let mut stack = Vec::new();
        if let Some(root) = self.root.take() {
            stack.push(root);
        }

        while let Some(mut node) = stack.pop() {
            if let Some(left_child) = node.left.take() {
                stack.push(left_child);
            }
            if let Some(right_child) = node.right.take() {
                stack.push(right_child);
            }
        }
    }
}

// struct to hold the value for the iteration
pub struct Iter<'a,T> {
    stack: Vec<&'a Node<T>>,
}

// implement the Iterator trait and the next method for the struct
impl<'a,T> Iterator for Iter<'a,T> {
    type Item = &'a T;

    // Get the value from the stack.
    // Go to the right branck from the poped node since all the left side was stacked on the iter
    // function.
    // Retrieve the left side of that right node.
    fn next(&mut self) -> Option<Self::Item> {
        let node = self.stack.pop()?;
        let mut current_node = node.right.as_ref();
        while let Some(n) = current_node {
            self.stack.push(n);
            current_node = n.left.as_ref();
        }
        Some(&node.val)
    }
}

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    // populate the Iter struct so can be called the next function of trait Iterator
    pub fn iter<'a>(&'a self) -> Iter<'a,T> {
        let mut iterator = Iter { stack: vec![] };
        let mut current_node = self.root.as_ref();

        while let Some(node) = current_node {
            iterator.stack.push(node);
            current_node = node.left.as_ref();
        }
        iterator
    }

    pub fn min(&self) -> Option<&T> {
        let mut node = self.root.as_ref()?;
        while let Some(left) = &node.left {
            node = left;
        }
        Some(&node.val)
    }
    
    pub fn max(&self) -> Option<&T> {
        let mut node = self.root.as_ref()?;
        while let Some(right) = &node.right {
            node = right;
        }
        Some(&node.val)
    }
}

impl<T: Ord> BinarySearchTree<T> {
    pub fn search(&self, val: &T) -> Option<&T> {
        let mut current_node = self.root.as_ref();
        while let Some(node) = current_node {
            if node.val == *val {
                return Some(&node.val);
            } else if node.val < *val {
                current_node = node.right.as_ref();
            } else {
                current_node = node.left.as_ref();
            }
        }
        None
    }

    #[allow(clippy::while_let_loop)] // while let will create a single continuous borrow
                                    // here is needed multiple (non-simultaneously) borrows and multible matches
    pub fn insert(&mut self, val: T) {
        let mut current_link: &mut Link<T> = &mut self.root;
        let mut parent: *mut Node<T> = ptr::null_mut();
        
        loop {
            let node = match current_link {
                Some(node) => node,
                None => break,
            };

            parent = &mut **node as *mut Node<T>;

            match val.cmp(&node.val) {
                Ordering::Less => current_link = &mut node.left,
                Ordering::Greater => current_link = &mut node.right,
                Ordering::Equal => return,
            }
        }

        let new_node = Box::new(Node {
            val,
            parent,
            left: None,
            right: None,
        });
        
        *current_link = Some(new_node);
    }

    pub fn delete(&mut self, val: &T) -> Option<T> {
        // The only way this works is with a raw pointer and some unsafe expressions.
        // The borrow checker did not allowed some safe code.
        // Maybe I'm just dumb and don't know how to do it with 100% safe rust.
        let mut current_link_ptr: *mut Link<T> = &mut self.root;

        loop {
            let node = unsafe {
                match (*current_link_ptr).as_mut() {
                    Some(node) => node,
                    None => break,
                }
            };
            
            match val.cmp(&node.val) {
                Ordering::Less => current_link_ptr = &mut node.left,
                Ordering::Greater => current_link_ptr = &mut node.right,
                Ordering::Equal => break,
            }
        }

        let node_to_delete = unsafe { (*current_link_ptr).take() }?;
        let parent_of_deleted = node_to_delete.parent;

        let deleted_val = match (node_to_delete.left, node_to_delete.right) {
            // Node has no children
            (None, None) => {
                // since the node where taken, it has None as its value, so the job is already done
                node_to_delete.val
            },

            // Node has one children
            (Some(mut child), None) | (None, Some(mut child)) => {
                child.parent = parent_of_deleted;
                unsafe { *current_link_ptr = Some(child); }
                node_to_delete.val
            },

            // Node has two children
            (Some(mut left), Some(right)) => {
                // GET THE MOST LEFT NODE FROM THE RIGHT TREE
                let mut right_subtree_link = Some(right);
                let mut successor_node = Self::detach_min(&mut right_subtree_link);
                successor_node.parent = parent_of_deleted;

                // UPDATE SUCCESSOR LEFT SIDE
                left.parent = &mut *successor_node;
                successor_node.left = Some(left);

                // UPDATE SUCCESSOR RIGHT SIDE
                if let Some(ref mut right_node) = right_subtree_link {
                    right_node.parent = &mut *successor_node;
                }
                successor_node.right = right_subtree_link;

                // ATTACH THE NEW SUBTREE TO THE REST OF THE TREE
                unsafe { *current_link_ptr = Some(successor_node) };

                node_to_delete.val
            }
        };

        Some(deleted_val)
    }

    fn detach_min(link: &mut Link<T>) -> Box<Node<T>> {
        let mut current_link = link;

        // here the first link must be Some
        // search the node where the left is None (the minimum value)
        while current_link.as_ref().unwrap().left.is_some() {
            current_link = &mut current_link.as_mut().unwrap().left;
        }

        // remove from the tree
        let mut min_node = current_link.take().unwrap();

        // attach the right node to the place of the removed node
        // if there is no right child take() will return None
        *current_link = min_node.right.take();

        // if there is a right child, link the parent as the parent of the removed node
        if let Some(right_child) = current_link.as_mut() {
            right_child.parent = min_node.parent;
        }
        
        min_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    // Helper to build a standard tree for tests
    //       10
    //      /  \
    //     5    15
    //    / \   / \
    //   3   7 12  18
    fn build_test_tree() -> BinarySearchTree<i32> {
        let mut tree = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(3);
        tree.insert(7);
        tree.insert(12);
        tree.insert(18);
        tree
    }

    #[test]
    fn test_new_empty() {
        let mut tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert!(tree.root.is_none());
        assert_eq!(tree.min(), None);
        assert_eq!(tree.max(), None);
        assert_eq!(tree.delete(&1), None);
    }

    #[test]
    fn test_insert_and_search() {
        let mut tree = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        assert_eq!(tree.search(&10), Some(&10));
        assert_eq!(tree.search(&5), Some(&5));
        assert_eq!(tree.search(&15), Some(&15));
        assert_eq!(tree.search(&99), None);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut tree = BinarySearchTree::new();
        tree.insert(10);
        tree.insert(10); // Should be ignored
        assert_eq!(tree.root.as_ref().unwrap().val, 10);
        assert!(tree.root.as_ref().unwrap().left.is_none());
        assert!(tree.root.as_ref().unwrap().right.is_none());
    }

    #[test]
    fn test_min_max() {
        let tree = build_test_tree();
        assert_eq!(tree.min(), Some(&3));
        assert_eq!(tree.max(), Some(&18));
    }

    #[test]
    fn test_iter() {
        let tree = build_test_tree();
        let mut iter = tree.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&7));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&12));
        assert_eq!(iter.next(), Some(&15));
        assert_eq!(iter.next(), Some(&18));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter_empty() {
        let tree: BinarySearchTree<i32> = BinarySearchTree::new();
        assert_eq!(tree.iter().next(), None);
    }

    #[test]
    fn test_delete_non_existent() {
        let mut tree = build_test_tree();
        assert_eq!(tree.delete(&99), None);
        assert_eq!(tree.min(), Some(&3)); // Ensure tree is unchanged
        assert_eq!(tree.max(), Some(&18));
    }

    #[test]
    fn test_delete_leaf() {
        let mut tree = build_test_tree();
        assert_eq!(tree.delete(&3), Some(3));
        assert_eq!(tree.search(&3), None);
        assert_eq!(tree.min(), Some(&5));
    }

    #[test]
    fn test_delete_node_with_one_right_child() {
        let mut tree = build_test_tree();
        tree.delete(&3);
        tree.insert(4);
        tree.delete(&5); // Delete 5, leaving 4 as the left child of 7
        assert_eq!(tree.search(&5), None);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().val, 7);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().val, 4);
    }

    #[test]
    fn test_delete_node_with_one_left_child() {
        let mut tree = build_test_tree();
        tree.delete(&7);
        assert_eq!(tree.search(&7), None);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().val, 5);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().right, None);
    }

    #[test]
    fn test_delete_node_with_two_children() {
        let mut tree = build_test_tree();
        assert_eq!(tree.delete(&5), Some(5));
        assert_eq!(tree.search(&5), None);
        // 7 should be the successor
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().val, 7);
        assert_eq!(tree.root.as_ref().unwrap().left.as_ref().unwrap().left.as_ref().unwrap().val, 3);
    }

    #[test]
    fn test_delete_root_with_two_children() {
        let mut tree = build_test_tree();
        assert_eq!(tree.delete(&10), Some(10));
        assert_eq!(tree.search(&10), None);
        // 12 should be the successor
        assert_eq!(tree.root.as_ref().unwrap().val, 12);
        assert_eq!(tree.max(), Some(&18));
        assert_eq!(tree.min(), Some(&3));
    }

    #[test]
    fn test_delete_root_until_empty() {
        let mut tree = build_test_tree();
        let values: Vec<i32> = tree.iter().copied().collect();
        for val in values {
            assert!(tree.delete(&val).is_some());
        }
        assert!(tree.root.is_none());
    }

    #[test]
    fn test_drop() {
        // This struct increments a counter when it's dropped.
        struct DropCounter<'a> {
            _id: i32,
            counter: &'a RefCell<i32>,
        }
        impl<'a> Drop for DropCounter<'a> {
            fn drop(&mut self) {
                *self.counter.borrow_mut() += 1;
            }
        }
        // We need Ord/PartialEq/Eq for the tree.
        impl<'a> PartialEq for DropCounter<'a> { fn eq(&self, other: &Self) -> bool { self._id == other._id } }
        impl<'a> Eq for DropCounter<'a> {}
        impl<'a> PartialOrd for DropCounter<'a> { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self._id.partial_cmp(&other._id) } }
        impl<'a> Ord for DropCounter<'a> { fn cmp(&self, other: &Self) -> Ordering { self._id.cmp(&other._id) } }

        let drop_counter = RefCell::new(0);
        {
            let mut tree = BinarySearchTree::new();
            for i in [10, 5, 15, 3, 7, 12, 18] {
                tree.insert(DropCounter { _id: i, counter: &drop_counter });
            }
            // Tree goes out of scope here, calling our custom `drop`.
        }
        // If our drop works, all 7 nodes should have been dropped.
        assert_eq!(*drop_counter.borrow(), 7);
    }
}
