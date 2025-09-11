pub struct Stack<T, const N: usize> {
    elements: [T; N],
    top: usize,
}

impl<T: Default + Copy, const N: usize> Default for Stack<T, N> {
    fn default() -> Self {
        Self {
            elements: [T::default(); N],
            // in order to initialize with the default value from T is necessary to include the
            // traits Default and Copy
            top: 0,
        }
    }
}

impl<T: Default + Copy, const N: usize> Stack<T, N> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn is_full(&self) -> bool {
        self.top >= N
    }

    pub fn len(&self) -> usize {
        self.top
    }

    pub fn capacity(&self) -> usize {
        N
    }

    pub fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        Some(&self.elements[self.top - 1])
    }
    
    // 'static so the err lives for the entire program
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        if self.is_full() {
            return Err("Stack is full.");
        }

        self.elements[self.top] = value;
        self.top += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.top -= 1;
        let value = self.elements[self.top];
        self.elements[self.top] = T::default();
        Some(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn test_new_stack_is_empty() {
        let stack: Stack<i32, 5> = Stack::new();
        assert!(stack.is_empty());
        assert_eq!(stack.len(), 0);
        assert_eq!(stack.capacity(), 5);
    }

    #[test]
    fn test_push_and_peek() {
        let mut stack: Stack<i32, 5> = Stack::new();
        stack.push(10).unwrap();
        
        assert!(!stack.is_empty());
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&10));
        
        // Ensure peek doesn't remove the element
        assert_eq!(stack.len(), 1);
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<i32, 5> = Stack::new();
        stack.push(10).unwrap();
        stack.push(20).unwrap();

        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.len(), 1);
        assert_eq!(stack.peek(), Some(&10));
    }

    #[test]
    fn test_lifo_order() {
        let mut stack: Stack<char, 3> = Stack::new();
        stack.push('a').unwrap();
        stack.push('b').unwrap();
        stack.push('c').unwrap();

        assert_eq!(stack.pop(), Some('c'));
        assert_eq!(stack.pop(), Some('b'));
        assert_eq!(stack.pop(), Some('a'));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_empty_stack_operations() {
        let mut stack: Stack<i32, 5> = Stack::new();
        assert_eq!(stack.pop(), None);
        assert_eq!(stack.peek(), None);
    }

    #[test]
    fn test_full_stack_operations() {
        let mut stack: Stack<i32, 3> = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();

        assert!(stack.is_full());
        assert_eq!(stack.len(), 3);

        // Pushing to a full stack should fail
        assert!(stack.push(4).is_err());
        
        // Ensure the stack wasn't modified
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.peek(), Some(&3));
    }

    #[test]
    fn test_push_after_pop_on_full_stack() {
        let mut stack: Stack<i32, 3> = Stack::new();
        stack.push(1).unwrap();
        stack.push(2).unwrap();
        stack.push(3).unwrap();

        assert!(stack.is_full());
        assert_eq!(stack.pop(), Some(3));
        assert!(!stack.is_full());

        // Should be able to push again
        assert!(stack.push(4).is_ok());
        assert_eq!(stack.peek(), Some(&4));
        assert!(stack.is_full());
    }
}
