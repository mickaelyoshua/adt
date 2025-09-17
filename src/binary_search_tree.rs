use std::ptr;
use std::cmp::Ordering;

type Link<T> = Option<Box<Node<T>>>;

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
