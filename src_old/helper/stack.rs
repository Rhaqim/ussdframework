use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stack<T> {
    pub items: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new empty stack
    pub fn new() -> Self {
        Stack { items: Vec::new() }
    }

    // Check if the stack is empty
    pub fn _is_empty(&self) -> bool {
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
    pub fn _peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Get the number of items in the stack
    pub fn _len(&self) -> usize {
        self.items.len()
    }

    // Iterate over the items in the stack
    pub fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
}
