use std::cmp::max;

// Adelson-Velsky and Landis Tree -> AVL Tree
// For every node in the tree the balance factor must be -1, 0 or 1.

type Link<T> = Option<Box<Node<T>>>;

fn height<T>(link: &Link<T>) -> usize {
    link.as_ref().map_or(0, |node| node.height)

    // match link {
    //     Some(node) => node.height,
    //     None => 0,
    // }
}

fn rotate_left<T>(mut boxed_node: Box<Node<T>>) -> Box<Node<T>> {
    unimplemented!()
}

fn rotate_right<T>(mut boxed_node: Box<Node<T>>) -> Box<Node<T>> {
    unimplemented!()
}

pub struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
    height: usize,
}

impl<T> Node<T> {
    fn update_height(&mut self) {
        let left_height = height(&self.left);
        let right_height = height(&self.right);
        self.height = 1 + max(left_height, right_height);
    }

    fn balance_factor(&self) -> i32 {
        height(&self.left) as i32 - height(&self.right) as i32
    }
}

pub struct AvlTree<T> {
    root: Link<T>,
}

impl<T> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
