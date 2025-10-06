use std::fmt::Display;

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    val: T,
    left: Link<T>,
    right: Link<T>,
}

pub struct BinaryTree<T> {
    root: Link<T>,
}

fn main() {
    let tree = BinaryTree {
        root: Some(Box::new(Node {
            val: 'A',
            left: Some(Box::new(Node {
                val: 'B',
                left: Some(Box::new(Node {
                    val: 'D',
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(Node {
                    val: 'E',
                    left: Some(Box::new(Node {
                        val: 'H',
                        left: None,
                        right: None,
                    })),
                    right: Some(Box::new(Node {
                        val: 'I',
                        left: None,
                        right: None,
                    })),
                })),
            })),
            right: Some(Box::new(Node {
                val: 'C',
                left: Some(Box::new(Node {
                    val: 'F',
                    left: None,
                    right: Some(Box::new(Node {
                        val: 'K',
                        left: None,
                        right: None,
                    })),
                })),
                right: None,
            })),
        })),
    };

    tree.show_tree();
    println!();
    tree.traverse();
}

impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        Self { root: None }
    }
}

impl<T> Drop for BinaryTree<T> {
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

impl<T: Display> BinaryTree<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn traverse(&self) {
        if self.root.is_none() {
            return
        }
        Self::traverse_recursive(&self.root);
    }

    fn traverse_recursive(link: &Link<T>) {
        if let Some(node) = link {
            Self::traverse_recursive(&node.left);
            print!("{} ", node.val);
            Self::traverse_recursive(&node.right);
        }
    }
}

impl<T: Display> BinaryTree<T> {
    pub fn show_tree(&self) {
        if self.root.is_none(){
            return
        }
        Self::show_tree_recursive(&self.root);
    }

    fn show_tree_recursive(link: &Link<T>) {
        if let Some(node) = link {
            print!("{}", node.val);
            print!("(");
            Self::show_tree_recursive(&node.left);
            Self::show_tree_recursive(&node.right);
            print!(")");
        }
    }
}
