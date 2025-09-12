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
