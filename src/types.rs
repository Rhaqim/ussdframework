use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RouterOptions {
    List(Vec<RouterOptions>),
    ListStr(Vec<String>),
    String(String),
}

impl Display for RouterOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RouterOptions::List(options) => {
                write!(f, "{:?}", options)
            }
            RouterOptions::ListStr(options) => {
                write!(f, "{:?}", options)
            }
            RouterOptions::String(option) => {
                write!(f, "{}", option)
            }
        }
    }
}

// add to_string method to RouterOptions for String
// impl RouterOptions {
//     pub fn to_string(&self) -> String {
//         match self {
//             RouterOptions::List(options) => {
//                 format!("{:?}", options)
//             }
//             RouterOptions::ListStr(options) => {
//                 format!("{:?}", options)
//             }
//             RouterOptions::String(option) => {
//                 option.to_string()
//             }
//         }
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashStrAny {
    Str(String),
    Int(i32),
    Float(f32),
    List(Vec<HashStrAny>),
    ListStr(Vec<String>),
    Dict(HashMap<String, HashStrAny>),
    None,
}

impl HashStrAny {
    // Helper function to create a new HashStrAny::Dict variant
    pub fn new_dict(dict: HashMap<String, HashStrAny>) -> HashStrAny {
        HashStrAny::Dict(dict)
    }
}

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

    // First item in the stack
    pub fn first(&self) -> Option<&T> {
        self.items.first()
    }
}
