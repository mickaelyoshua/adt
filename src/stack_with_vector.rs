pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            elements: Vec::new(),
        }
    }
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self::default()
    }
}
