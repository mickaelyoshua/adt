#[derive(PartialEq, Eq, Debug)]
pub struct PriorityQueue<T> {
	vals: Vec<T>,
}

impl<T: PartialOrd> PriorityQueue<T> {
    pub fn new() -> Self {
        PriorityQueue { vals: Vec::new() }
    }

    pub fn from_vec(values: Vec<T>) -> Self {
        let mut q = PriorityQueue { vals: values };

        if q.vals.len() <= 1 {
            return q;
        }
        
        let non_leaf_node_index = q.parent(q.vals.len()-1).unwrap();
        for i in (0..=non_leaf_node_index).rev() {
            q.bubble_down(i);
        }
        q

    }

    fn parent(&self, index: usize) -> Option<usize> {
        if index == 0 {
            return None;
        }
        Some((index-1)/2)
    }

    fn left_child(&self, index: usize) -> Option<usize> {
        let child = index*2 + 1;
        if child >= self.vals.len() {
            return None;
        }
        Some(child)
    }

    fn right_child(&self, index: usize) -> Option<usize> {
        let child = index*2 + 2;
        if child >= self.vals.len() {
            return None;
        }
        Some(child)
    }

    fn bubble_up(&mut self, index: usize) {
        if let Some(parent_index) = self.parent(index) && self.vals[parent_index] > self.vals[index] {
            self.vals.swap(index, parent_index);
            self.bubble_up(parent_index);
        }
    }

    pub fn insert(&mut self, val: T) {
        self.vals.push(val);
        let val_index = self.vals.len() - 1;
        self.bubble_up(val_index);
    }

    fn bubble_down(&mut self, index: usize) {
        let mut smallest = index;

        if let Some(left_child) = self.left_child(index) && self.vals[smallest] > self.vals[left_child] {
            smallest = left_child;
        }

        if let Some(right_child) = self.right_child(index) && self.vals[smallest] > self.vals[right_child] {
            smallest = right_child;
        }

        if smallest != index {
            self.vals.swap(index, smallest);
            self.bubble_down(smallest);
        }
    }

    pub fn pop(&mut self) -> Option<T> { // extract_min
        if self.vals.len() <= 1 {
            return self.vals.pop();
        }

        let min = self.vals.swap_remove(0);
        self.bubble_down(0);
        Some(min)
    }
}

impl<T: PartialOrd> Default for PriorityQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_is_empty() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_insert_and_pop_single() {
        let mut pq = PriorityQueue::new();
        pq.insert(10);
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_pop_from_empty() {
        let mut pq: PriorityQueue<i32> = PriorityQueue::new();
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_insert_multiple_and_pop_in_order() {
        let mut pq = PriorityQueue::new();
        pq.insert(10);
        pq.insert(5);
        pq.insert(20);
        pq.insert(8);

        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(8));
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), Some(20));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_insert_descending_and_pop_in_order() {
        let mut pq = PriorityQueue::new();
        pq.insert(20);
        pq.insert(15);
        pq.insert(10);
        pq.insert(5);

        assert_eq!(pq.pop(), Some(5));
        assert_eq!(pq.pop(), Some(10));
        assert_eq!(pq.pop(), Some(15));
        assert_eq!(pq.pop(), Some(20));
        assert_eq!(pq.pop(), None);
    }

    #[test]
    fn test_mixed_operations() {
        let mut pq = PriorityQueue::new();
        pq.insert(10);
        pq.insert(5);
        assert_eq!(pq.pop(), Some(5));

        pq.insert(20);
        pq.insert(8);
        assert_eq!(pq.pop(), Some(8));
        assert_eq!(pq.pop(), Some(10));

        pq.insert(1);
        assert_eq!(pq.pop(), Some(1));
        assert_eq!(pq.pop(), Some(20));
        assert_eq!(pq.pop(), None);
    }
}

