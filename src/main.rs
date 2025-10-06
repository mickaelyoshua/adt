type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct BinarySearchTree<T> {
    root: Link<T>,
}

fn main() {
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

impl<T> BinarySearchTree<T> {
    pub fn new() -> Self {
        Self::default()
    }
}
