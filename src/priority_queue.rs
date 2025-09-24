#[derive(PartialEq, Eq)]
pub struct PriorityQueue<T> {
	vals: Vec<T>,
}

impl<T: PartialOrd> PriorityQueue<T> {
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

    fn bubble_up(&mut self, val_index: usize) {
        if let Some(parent_index) = self.parent(val_index) && self.vals[parent_index] > self.vals[val_index] {
                self.vals.swap(val_index, parent_index);
                self.bubble_up(parent_index);
        }
    }

    pub fn insert(&mut self, val: T) {
        self.vals.push(val);
        let val_index = self.vals.len() - 1;

        // Recursive version
        self.bubble_up(val_index);

        // Loop version
        // while let Some(parent_index) = self.parent(val_index) {
        //     if self.vals[parent_index] > self.vals[val_index] {
        //         self.vals.swap(val_index, parent_index);
        //         val_index = parent_index;
        //     } else {
        //         break;
        //     }
        // }
    }

    pub fn pop(&mut self) -> Option<T> { // extract_min
        if self.vals.len() <= 1 {
            return self.vals.pop();
        }
        unimplemented!()
    }
}

