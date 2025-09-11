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

    pub fn push(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }
    
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_push_and_peek() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);
        
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 2);
        assert_eq!(stack.peek(), Some(&20));
        
        // Ensure peek doesn't remove the element
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut stack = Stack::new();
        stack.push(10);
        stack.push(20);

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&10));
    }

    #[test]
    fn test_lifo_order() {
        let mut stack = Stack::new();
        stack.push('a');
        stack.push('b');
        stack.push('c');

        assert_eq!(stack.pop(), Some('c'));
        assert_eq!(stack.pop(), Some('b'));
        assert_eq!(stack.pop(), Some('a'));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_empty_stack_operations() {
        let mut stack: Stack<i32> = Stack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
    }
}