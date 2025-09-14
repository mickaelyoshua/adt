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

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        unimplemented!()
    }

    pub fn search(&self, val: &T) -> Option<&T> {
        unimplemented!()
    }

    pub fn insert(&mut self, val: T) {
        unimplemented!()
    }

    pub fn delete(&mut self, val: &T) -> Option<T> {
        unimplemented!()
    }
}
