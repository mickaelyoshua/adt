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

fn rotate_left<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    // Extract right tree
    let mut new_root_node = node.right.take().unwrap();

    // Move new root left to extracted node's right
    node.right = new_root_node.left.take();

    // Update height
    node.update_height();

    // Move extracted node to the left of new root
    new_root_node.left = Some(node);

    // Update height
    new_root_node.update_height();

    new_root_node
}

fn rotate_right<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    // Extract left tree
    let mut new_root_node = node.left.take().unwrap();
    
    // Move new root right to extracted node's left
    node.left = new_root_node.right.take();

    // Update height
    node.update_height();

    // Move extracted node to the right of new root
    new_root_node.right = Some(node);

    // Update height
    new_root_node.update_height();

    new_root_node
}

fn rebalance<T>(mut node: Box<Node<T>>) -> Box<Node<T>> {
    let bf = node.balance_factor();
    match bf {
        2 => {
            if node.left.as_ref().unwrap().balance_factor() >= 0 {
                rotate_right(node)
            } else {
                node.left = Some(rotate_left(node.left.unwrap()));
                rotate_right(node)
            }
        },
        -2 => {
            if node.right.as_ref().unwrap().balance_factor() <= 0 {
                rotate_left(node)
            } else {
                node.right = Some(rotate_right(node.right.unwrap()));
                rotate_left(node)
            }
        }
        _ => node,
    }
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

#[derive(Default)]
pub struct AvlTree<T> {
    root: Link<T>,
}

impl<T> AvlTree<T> {
    pub fn new() -> Self {
        Self { root: None }
    }
}
