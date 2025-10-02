use std::mem::MaybeUninit;

#[derive(Default)]
pub struct CircularQueue<T> {
    vals: Vec<MaybeUninit<T>>,
    head: usize,
    tail: usize,
    is_empty: bool,
}

impl<T> CircularQueue<T> {
    pub fn new() -> Self {
        Self {
            vals: Vec::new(),
            head: 0,
            tail: 0,
            is_empty: true,
        }
    }

    pub fn is_full(&self) -> bool {
        self.head == self.tail && !self.is_empty
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }

    pub fn len(&self) -> usize {
        if self.is_empty {
            0
        } else if self.tail >= self.head {
            self.tail - self.head
        } else {
            self.vals.len() - self.head + self.tail
        }
    }

    pub fn grow(&mut self) {
        let len = self.len(); // number of elements
        let old_capacity = self.vals.len(); // total capacity of the vector
        let new_capacity = if old_capacity == 0 { 4 } else { old_capacity * 2 }; // new capacity

        let mut new_vals: Vec<MaybeUninit<T>> = Vec::with_capacity(new_capacity);
        unsafe { new_vals.set_len(new_capacity); } // allow indexing

        if self.is_empty {
            self.vals = new_vals;
            return
        }

        if self.head < self.tail {
            let mut i: usize = 0;
            let mut j: usize = self.head;
            while j < self.tail {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }
        } else {
            let mut i: usize = 0;
            let mut j: usize = self.head;
            while j < old_capacity {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }

            j = 0;
            while j < self.tail {
                unsafe {
                    let val = self.vals[j].assume_init_read();
                    new_vals[i].write(val);
                }
                i += 1;
                j += 1;
            }
        }

        self.head = 0;
        self.vals = new_vals;
        self.tail = len;
    }

    pub fn enqueue(&mut self, val: T) {
        

        unimplemented!()
    }
}

