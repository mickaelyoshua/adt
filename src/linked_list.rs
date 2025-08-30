type Link<T> = Option<Box<Node<T>>>;

pub struct LinkedList<T> {
    head: Link<T>,
}

struct Node<T> {
    val: T,
    next: Link<T>,
}

pub enum SearchType {
    Loop,
    Recursive,
}

// Standard, conventional way to provide a default value
impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        LinkedList { head: None }
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<T: PartialEq> LinkedList<T> {
    // SEARCH
    pub fn search(&self, val: &T, search_type: SearchType) -> bool {
        match search_type {
            SearchType::Loop => self.search_loop(val),
            SearchType::Recursive => Self::search_recursive(&self.head, val),
        }
    }

    fn search_loop(&self, val: &T) -> bool {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.val == *val {
                return true;
            }
            current = &node.next;
        }
        false
    }

    fn search_recursive(link: &Link<T>, val: &T) -> bool {
        match link {
            None => false,
            Some(node) => {
                if node.val == *val {
                    true
                } else {
                    Self::search_recursive(&node.next, val)
                }
            }
        }
    }
}
