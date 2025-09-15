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

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self::default()
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
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        unimplemented!()
    }

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
