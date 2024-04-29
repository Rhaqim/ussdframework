use std::{collections::HashMap, fmt::Display};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::core::USSDSession;

/// Function signature for USSD functions
/// The function signature is a function that takes a USSDRequest and a string as arguments
///
/// # Arguments
///
/// * `request` - The USSD request
/// * `args` - Additional arguments
///
/// # Returns
///
/// A HashStrAny value
///
/// # Example
///
/// ```
/// use ussdframework::prelude::*;
///
/// fn buy_airtime(request: &USSDRequest, args: &str) -> HashStrAny {
///    HashStrAny::Str("Airtime bought".to_string())
/// }
pub type USSDFunction = fn(&USSDSession, &str) -> HashStrAny;

/// Key-value map of USSD functions
pub type FunctionMap = HashMap<String, USSDFunction>;

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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HashStrAny {
    Str(String),
    Int(i64),
    Float(f64),
    List(Vec<HashStrAny>),
    ListStr(Vec<String>),
    Dict(HashMap<String, HashStrAny>),
    None,
}

impl HashStrAny {
    pub fn new() -> HashStrAny {
        HashStrAny::None
    }
    // Helper function to create a new HashStrAny::Dict variant
    pub fn new_dict(dict: HashMap<String, HashStrAny>) -> HashStrAny {
        HashStrAny::Dict(dict)
    }

    /// Convert a JSON value to a HashStrAny value
    /// This function is recursive and will convert nested JSON values to HashStrAny values
    /// The HashStrAny type is a custom enum type that can represent any JSON value
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON value
    ///
    /// # Returns
    ///
    /// A HashStrAny value
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use serde_json::Value;
    ///
    /// let json = json!({
    ///    "one": "1",
    ///   "two": "2"
    /// });
    ///
    /// let hash_str_any = json_to_hash_str_any(json);
    /// ```
    pub fn json_to_hash_str_any(&self, json: Value) -> Self {
        match json {
            Value::Null => HashStrAny::None,
            Value::Bool(b) => HashStrAny::Str(b.to_string()),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    HashStrAny::Int(i as i64)
                } else if let Some(f) = n.as_f64() {
                    HashStrAny::Float(f as f64)
                } else {
                    HashStrAny::None
                }
            }
            Value::String(s) => HashStrAny::Str(s),
            Value::Array(arr) => {
                let mut list = Vec::new();
                for val in arr {
                    list.push(self.json_to_hash_str_any(val));
                }
                HashStrAny::List(list)
            }
            Value::Object(obj) => {
                let mut dict = HashMap::new();
                for (key, val) in obj {
                    dict.insert(key, self.json_to_hash_str_any(val));
                }
                HashStrAny::Dict(dict)
            }
        }
    }

    /// Convert a HashStrAny value to a JSON value
    /// This function is recursive and will convert nested HashStrAny values to JSON values
    /// The HashStrAny type is a custom enum type that can represent any JSON value
    ///
    /// # Arguments
    ///
    /// * `hash_any` - A HashStrAny value
    ///
    /// # Returns
    ///
    /// A JSON value
    ///
    /// # Example
    ///
    /// ```
    /// use serde_json::json;
    /// use serde_json::Value;
    ///
    /// let hash_str_any = HashStrAny::Dict({
    ///    let mut dict = HashMap::new();
    ///   dict.insert("one".to_string(), HashStrAny::Str("1".to_string()));
    ///  dict.insert("two".to_string(), HashStrAny::Str("2".to_string()));
    /// dict
    /// });
    ///
    /// let json = hash_str_any_to_json(hash_str_any);
    /// `
    pub fn hash_str_any_to_json(&self, hash_any: HashStrAny) -> Value {
        match hash_any {
            HashStrAny::Str(s) => Value::String(s),
            HashStrAny::Int(i) => Value::Number(i.into()),
            HashStrAny::Float(f) => Value::Number((f as i64).into()),
            HashStrAny::List(list) => {
                let items: Vec<Value> = list
                    .into_iter()
                    .map(|item| self.hash_str_any_to_json(item))
                    .collect();
                Value::Array(items)
            }
            HashStrAny::ListStr(list) => {
                Value::Array(list.into_iter().map(Value::String).collect())
            }
            HashStrAny::Dict(dict) => {
                let mut obj = serde_json::Map::new();
                for (key, value) in dict {
                    obj.insert(key, self.hash_str_any_to_json(value));
                }
                Value::Object(obj)
            }
            HashStrAny::None => Value::Null,
        }
    }
}

// Formatter for HashStrAny to convert JSON to HashStrAny
impl Display for HashStrAny {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HashStrAny::Str(s) => write!(f, "{}", s),
            HashStrAny::Int(i) => write!(f, "{}", i),
            HashStrAny::Float(fl) => write!(f, "{}", fl),
            HashStrAny::List(l) => write!(f, "{:?}", l),
            HashStrAny::ListStr(l) => write!(f, "{:?}", l),
            HashStrAny::Dict(d) => write!(f, "{:?}", d),
            HashStrAny::None => write!(f, "None"),
        }
    }
}

/// Convert a JSON value to a HashStrAny value
/// This function is recursive and will convert nested JSON values to HashStrAny values
/// The HashStrAny type is a custom enum type that can represent any JSON value
///
/// # Arguments
///
/// * `json` - A JSON value
///
/// # Returns
///
/// A HashStrAny value
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use serde_json::Value;
///
/// let json = json!({
///    "one": "1",
///   "two": "2"
/// });
///
/// let hash_str_any = json_to_hash_str_any(json);
/// ```
pub fn json_to_hash_str_any(json: Value) -> HashStrAny {
    match json {
        Value::Null => HashStrAny::None,
        Value::Bool(b) => HashStrAny::Str(b.to_string()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                HashStrAny::Int(i as i64)
            } else if let Some(f) = n.as_f64() {
                HashStrAny::Float(f as f64)
            } else {
                HashStrAny::None
            }
        }
        Value::String(s) => HashStrAny::Str(s),
        Value::Array(arr) => {
            let mut list = Vec::new();
            for val in arr {
                list.push(json_to_hash_str_any(val));
            }
            HashStrAny::List(list)
        }
        Value::Object(obj) => {
            let mut dict = HashMap::new();
            for (key, val) in obj {
                dict.insert(key, json_to_hash_str_any(val));
            }
            HashStrAny::Dict(dict)
        }
    }
}

/// Convert a HashStrAny value to a JSON value
/// This function is recursive and will convert nested HashStrAny values to JSON values
/// The HashStrAny type is a custom enum type that can represent any JSON value
///
/// # Arguments
///
/// * `hash_any` - A HashStrAny value
///
/// # Returns
///
/// A JSON value
///
/// # Example
///
/// ```
/// use serde_json::json;
/// use serde_json::Value;
///
/// let hash_str_any = HashStrAny::Dict({
///    let mut dict = HashMap::new();
///   dict.insert("one".to_string(), HashStrAny::Str("1".to_string()));
///  dict.insert("two".to_string(), HashStrAny::Str("2".to_string()));
/// dict
/// });
///
/// let json = hash_str_any_to_json(hash_str_any);
/// `
pub fn hash_str_any_to_json(hash_any: HashStrAny) -> Value {
    match hash_any {
        HashStrAny::Str(s) => Value::String(s),
        HashStrAny::Int(i) => Value::Number(i.into()),
        HashStrAny::Float(f) => Value::Number((f as i64).into()),
        HashStrAny::List(list) => {
            let items: Vec<Value> = list
                .into_iter()
                .map(|item| hash_str_any_to_json(item))
                .collect();
            Value::Array(items)
        }
        HashStrAny::ListStr(list) => Value::Array(list.into_iter().map(Value::String).collect()),
        HashStrAny::Dict(dict) => {
            let mut obj = serde_json::Map::new();
            for (key, value) in dict {
                obj.insert(key, hash_str_any_to_json(value));
            }
            Value::Object(obj)
        }
        HashStrAny::None => Value::Null,
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

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn test_router_options() {
        let options = RouterOptions::ListStr(vec!["one".to_string(), "two".to_string()]);
        assert_eq!(options.to_string(), "[\"one\", \"two\"]");
    }

    #[test]
    fn test_hash_str_any() {
        let mut dict = HashMap::new();
        dict.insert("one".to_string(), HashStrAny::Str("1".to_string()));
        dict.insert("two".to_string(), HashStrAny::Str("2".to_string()));
        let hash_str_any = HashStrAny::new_dict(dict);
        let expected_json = json!({
            "one": "1",
            "two": "2"
        });
        let json = hash_str_any_to_json(hash_str_any);
        // Convert both JSON values to strings for comparison
        let json_str = serde_json::to_string(&json).unwrap();
        let expected_json_str = serde_json::to_string(&expected_json).unwrap();
        // assert_eq!(hash_str_any.to_string(), "{\"one\": \"1\", \"two\": \"2\"}");
        assert_eq!(json_str, expected_json_str);
    }

    #[test]
    fn test_json_to_hash_str_any() {
        let json = json!({
            "one": "1",
            "two": "2"
        });

        let hash_str_any = json_to_hash_str_any(json);

        let mut expected_dict = HashMap::new();
        expected_dict.insert("one".to_string(), HashStrAny::Str("1".to_string()));
        expected_dict.insert("two".to_string(), HashStrAny::Str("2".to_string()));

        let expected_hash_str_any = HashStrAny::Dict(expected_dict);

        assert_eq!(hash_str_any, expected_hash_str_any);
    }

    #[test]
    fn test_hash_str_any_to_json() {
        let mut dict = HashMap::new();
        dict.insert("one".to_string(), HashStrAny::Str("1".to_string()));
        dict.insert("two".to_string(), HashStrAny::Str("2".to_string()));
        let hash_str_any = HashStrAny::new_dict(dict);
        let json = hash_str_any_to_json(hash_str_any);
        let expected_json = json!({
            "one": "1",
            "two": "2"
        });
        assert_eq!(json, expected_json);
    }

    #[test]
    fn test_stack() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
    }
}
