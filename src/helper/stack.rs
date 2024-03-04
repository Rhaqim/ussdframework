use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Stack<T> {
    pub items: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new empty stack
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Clear the stack
    pub fn clear(&mut self) {
        self.items.clear();
    }

    // Push an item onto the stack
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Pop an item from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek at the top item of the stack without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Get the number of items in the stack
    pub fn len(&self) -> usize {
        self.items.len()
    }
}
