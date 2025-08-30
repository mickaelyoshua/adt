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
    pub fn find(&self, val: &T) -> Option<&T> {
        self.find_loop(val)
    }

    pub fn find_with(&self, val: &T, search_type: SearchType) -> Option<&T> {
        match search_type {
            SearchType::Loop => self.find_loop(val),
            SearchType::Recursive => Self::find_recursive(&self.head, val)
        }
    }

    fn find_loop(&self, val: &T) -> Option<&T> {
        let mut current = &self.head;

        while let Some(node) = current {
            if node.val == *val {
                return Some(&node.val);
            }
            current = &node.next;
        }
        None
    }

    fn find_recursive<'a>(link: &'a Link<T>, val: &T) -> Option<&'a T> {
        match link {
            None => None,
            Some(node) => {
                if node.val == *val {
                    Some(&node.val)
                } else {
                    Self::find_recursive(&node.next, val)
                }
            }
        }
    }

    fn contains(&self, val: &T) -> bool {
        self.find(val).is_some()
    }

    fn contains_with(&self, val: &T, search_type: SearchType) -> bool {
        self.find_with(val, search_type).is_some()
    }
}
