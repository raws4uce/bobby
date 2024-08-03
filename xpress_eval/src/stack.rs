pub struct Stack<T> {
    pub vec: Vec<T>,
}

impl<T> Stack<T> {
    // Creates a new empty stack
    pub fn new() -> Stack<T> {
        Stack {
            vec: Vec::new(),
        }
    }
    
    // Returns a reference to the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.vec.last()
    }
    
    // Pushes an element onto the stack
    pub fn push(&mut self, t: T) {
        self.vec.push(t);
    }
    
    // Removes and returns the top element of the stack
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }
    
    // Checks if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }
    
    // Returns the number of elements in the stack
    pub fn len(&self) -> usize {
        self.vec.len()
    }
}