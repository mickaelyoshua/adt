use std::cmp::{ max, Ordering };

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

    // Helper method to check if the tree maintains AVL property
    fn is_balanced(&self) -> bool {
        Self::check_balance(&self.root)
    }

    fn check_balance(node: &Link<T>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let bf = n.balance_factor();
                if (-1..1).contains(&bf) {
                    return false;
                }
                Self::check_balance(&n.left) && Self::check_balance(&n.right)
            }
        }
    }

    // Helper to get the height of the tree
    fn tree_height(&self) -> usize {
        height(&self.root)
    }

    // Helper to verify heights are correctly maintained
    fn verify_heights(&self) -> bool {
        Self::check_heights(&self.root)
    }

    fn check_heights(node: &Link<T>) -> bool {
        match node {
            None => true,
            Some(n) => {
                let expected = 1 + max(height(&n.left), height(&n.right));
                if n.height != expected {
                    return false;
                }
                Self::check_heights(&n.left) && Self::check_heights(&n.right)
            }
        }
    }

    // Helper to collect values in-order for testing
    fn in_order(&self) -> Vec<&T> {
        let mut result = Vec::new();
        Self::in_order_helper(&self.root, &mut result);
        result
    }

    fn in_order_helper<'a>(node: &'a Link<T>, result: &mut Vec<&'a T>) {
        if let Some(n) = node {
            Self::in_order_helper(&n.left, result);
            result.push(&n.val);
            Self::in_order_helper(&n.right, result);
        }
    }
}

impl<T: Ord> AvlTree<T> {
    pub fn insert(&mut self, value: T) {
        let root = self.root.take();
        self.root = Some(Self::insert_recursive(root, value));
    }

    fn insert_recursive(link: Link<T>, value: T) -> Box<Node<T>> {
        match link {
            None => {
                Box::new(Node {
                    val: value,
                    left: None,
                    right: None,
                    height: 1,
                })
            },
            Some(mut node) => {
                match value.cmp(&node.val) {
                    Ordering::Equal => (),
                    Ordering::Less => {
                        let left = Self::insert_recursive(node.left.take(), value);
                        node.left = Some(left);
                    },
                    Ordering::Greater => {
                        let right = Self::insert_recursive(node.right.take(), value);
                        node.right = Some(right);
                    },
                }
                node.update_height();
                rebalance(node)
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_tree() {
        let tree: AvlTree<i32> = AvlTree::new();
        assert_eq!(tree.tree_height(), 0);
        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), Vec::<&i32>::new());
    }

    #[test]
    fn test_single_insertion() {
        let mut tree = AvlTree::new();
        tree.insert(10);

        assert_eq!(tree.tree_height(), 1);
        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&10]);
    }

    #[test]
    fn test_multiple_insertions_no_rotation() {
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);

        assert_eq!(tree.tree_height(), 2);
        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&5, &10, &15]);
    }

    #[test]
    fn test_left_left_rotation() {
        // Insert in descending order to trigger right rotation (LL case)
        let mut tree = AvlTree::new();
        tree.insert(30);
        tree.insert(20);
        tree.insert(10); // This should trigger right rotation

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&10, &20, &30]);
        assert_eq!(tree.tree_height(), 2); // Should be balanced
    }

    #[test]
    fn test_right_right_rotation() {
        // Insert in ascending order to trigger left rotation (RR case)
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(20);
        tree.insert(30); // This should trigger left rotation

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&10, &20, &30]);
        assert_eq!(tree.tree_height(), 2); // Should be balanced
    }

    #[test]
    fn test_left_right_rotation() {
        // Insert pattern that triggers LR rotation
        let mut tree = AvlTree::new();
        tree.insert(30);
        tree.insert(10);
        tree.insert(20); // This should trigger LR rotation

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&10, &20, &30]);
        assert_eq!(tree.tree_height(), 2);
    }

    #[test]
    fn test_right_left_rotation() {
        // Insert pattern that triggers RL rotation
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(30);
        tree.insert(20); // This should trigger RL rotation

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        assert_eq!(tree.in_order(), vec![&10, &20, &30]);
        assert_eq!(tree.tree_height(), 2);
    }

    #[test]
    fn test_complex_insertions() {
        let mut tree = AvlTree::new();
        let values = vec![50, 25, 75, 10, 30, 60, 80, 5, 15, 27, 55, 65];

        for val in values {
            tree.insert(val);
            assert!(tree.is_balanced(), "Tree became unbalanced after inserting {}", val);
            assert!(tree.verify_heights(), "Heights incorrect after inserting {}", val);
        }

        let in_order = tree.in_order();
        let expected: Vec<&i32> = vec![&5, &10, &15, &25, &27, &30, &50, &55, &60, &65, &75, &80];
        assert_eq!(in_order, expected);
    }

    #[test]
    fn test_sequential_insertions() {
        // Insert 1..20 sequentially - should maintain balance through multiple rotations
        let mut tree = AvlTree::new();

        for i in 1..=20 {
            tree.insert(i);
            assert!(tree.is_balanced(), "Tree unbalanced after inserting {}", i);
            assert!(tree.verify_heights(), "Heights incorrect after inserting {}", i);
        }

        // Height should be log2(20) ≈ 4.32, so height should be around 5-6
        let height = tree.tree_height();
        assert!(height <= 6, "Tree height {} is too large for 20 nodes", height);

        let in_order = tree.in_order();
        let expected_values: Vec<i32> = (1..=20).collect();
        let expected: Vec<&i32> = expected_values.iter().collect();
        assert_eq!(in_order, expected);
    }

    #[test]
    fn test_reverse_sequential_insertions() {
        // Insert 20..1 in reverse - should maintain balance
        let mut tree = AvlTree::new();

        for i in (1..=20).rev() {
            tree.insert(i);
            assert!(tree.is_balanced(), "Tree unbalanced after inserting {}", i);
            assert!(tree.verify_heights(), "Heights incorrect after inserting {}", i);
        }

        let height = tree.tree_height();
        assert!(height <= 6, "Tree height {} is too large for 20 nodes", height);

        let in_order = tree.in_order();
        let expected_values: Vec<i32> = (1..=20).collect();
        let expected: Vec<&i32> = expected_values.iter().collect();
        assert_eq!(in_order, expected);
    }

    #[test]
    fn test_duplicate_insertions() {
        let mut tree = AvlTree::new();
        tree.insert(10);
        tree.insert(5);
        tree.insert(15);
        tree.insert(10); // Duplicate
        tree.insert(10); // Another duplicate

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());
        // Should only have one 10 since we ignore duplicates
        assert_eq!(tree.in_order(), vec![&5, &10, &15]);
    }

    #[test]
    fn test_height_calculation() {
        let mut tree = AvlTree::new();

        // Single node
        tree.insert(10);
        assert_eq!(tree.tree_height(), 1);

        // Two levels
        tree.insert(5);
        tree.insert(15);
        assert_eq!(tree.tree_height(), 2);

        // Three levels
        tree.insert(3);
        assert_eq!(tree.tree_height(), 3);
    }

    #[test]
    fn test_large_tree() {
        let mut tree = AvlTree::new();
        let n = 100;

        for i in 0..n {
            tree.insert(i);
        }

        assert!(tree.is_balanced());
        assert!(tree.verify_heights());

        // For 100 nodes, height should be around log2(100) ≈ 6.64
        // AVL guarantees height ≤ 1.44 * log2(n), so should be ≤ 9-10
        let height = tree.tree_height();
        assert!(height <= 10, "Tree height {} is too large for {} nodes", height, n);

        // Verify all elements are present and sorted
        let in_order = tree.in_order();
        assert_eq!(in_order.len(), n);
        for i in 0..n {
            assert_eq!(*in_order[i], i);
        }
    }

    #[test]
    fn test_alternating_insertions() {
        let mut tree = AvlTree::new();

        // Insert alternating high and low values
        for i in 0..10 {
            if i % 2 == 0 {
                tree.insert(i);
            } else {
                tree.insert(100 - i);
            }
            assert!(tree.is_balanced());
            assert!(tree.verify_heights());
        }

        let in_order = tree.in_order();
        assert_eq!(in_order.len(), 10);

        // Verify sorted order
        for i in 1..in_order.len() {
            assert!(in_order[i - 1] < in_order[i]);
        }
    }
}
