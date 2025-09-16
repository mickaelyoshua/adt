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

impl<T: PartialEq + PartialOrd> BinarySearchTree<T> {
    pub fn search(&self, val: &T) -> Option<&T> {
        let mut current_node = &self.root;
        while let Some(node) = current_node {
            if node.val == *val {
                return Some(&node.val);
            } else if node.val < *val {
                current_node = &node.right;
            } else {
                current_node = &node.left;
            }
        }
        None
    }

    pub fn insert(&mut self, val: T) {
        unimplemented!()
    }

    pub fn delete(&mut self, val: &T) -> Option<T> {
        unimplemented!()
    }
}
