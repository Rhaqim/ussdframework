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
/// A USSDData value
///
/// # Example
///
/// ```
/// use ussdframework::prelude::*;
///
/// fn buy_airtime(request: &USSDRequest, args: &str) -> USSDData {
///    USSDData::Str("Airtime bought".to_string())
/// }
pub type USSDFunction = fn(&USSDSession, &str) -> USSDData;

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

/// Represents various types of USSD data.
///
/// The `USSDData` enum represents different types of data that can be exchanged
/// in the context of Unstructured Supplementary Service Data (USSD) communication.
///
/// This enum encompasses string, integer, floating-point, list, dictionary, and
/// null-like data types commonly used in USSD interactions.
///
/// # Derives
///
/// The `USSDData` enum derives `Debug`, `Clone`, `Serialize`, `Deserialize`, and `PartialEq` traits
/// to enable debugging, cloning, serialization/deserialization, and comparison of data instances.
///
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum USSDData {
    /// Represents a string value
    Str(String),
    /// Represents an integer value
    Int(i64),
    /// Represents a floating-point value
    Float(f64),
    /// Represents a list of USSD data
    List(Vec<USSDData>),
    /// Represents a list of string values
    ListStr(Vec<String>),
    /// Represents a dictionary of key-value pairs with USSDData values.
    Dict(HashMap<String, USSDData>),
    /// Represents a null-like value
    None,
}

impl USSDData {
    /// Creates a new `USSDData` instance from a JSON value.
    ///
    /// This method converts a JSON value into the corresponding `USSDData` variant.
    ///
    /// # Arguments
    ///
    /// * `value`: An optional JSON `Value` representing the data value to be converted.
    ///
    /// # Returns
    ///
    /// A `USSDData` instance representing the converted data value.
    ///
    pub fn new(value: Option<Value>) -> Self {
        match value {
            Some(val) => json_to_hash_str_any(val),
            None => USSDData::None,
        }
    }

    /// Returns the data as a string, if applicable.
    ///
    /// # Returns
    ///
    /// An optional reference to a string value, if the data is of string type.
    ///
    pub fn as_str(&self) -> Option<&str> {
        match self {
            USSDData::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Returns the data as a dictionary of key-value pairs, if applicable.
    ///
    /// # Returns
    ///
    /// An optional reference to a dictionary of key-value pairs, if the data is of dictionary type.
    ///
    pub fn as_hash_str_any(&self) -> Option<&HashMap<String, USSDData>> {
        match self {
            USSDData::Dict(d) => Some(d),
            _ => None,
        }
    }

    /// Returns the data as a list, if applicable.
    ///
    /// # Returns
    ///
    /// An optional reference to a list of USSDData values, if the data is of list type.
    ///
    pub fn as_list(&self) -> Option<&Vec<USSDData>> {
        match self {
            USSDData::List(l) => Some(l),
            _ => None,
        }
    }

    /// Creates a new `USSDData` instance representing a dictionary.
    ///
    /// The `new_dict` function creates a new `USSDData` instance representing a dictionary
    /// (i.e., a collection of key-value pairs) initialized with the provided `dict` HashMap.
    ///
    /// # Arguments
    ///
    /// * `dict`: A `HashMap<String, USSDData>` representing the dictionary data to be encapsulated
    ///           within the `USSDData` instance.
    ///
    /// # Returns
    ///
    /// A `USSDData` instance representing the dictionary data provided in the `dict` parameter.
    ///
    pub fn new_dict(dict: HashMap<String, USSDData>) -> USSDData {
        USSDData::Dict(dict)
    }

    /// Convert a JSON value to a USSDData value
    /// This function is recursive and will convert nested JSON values to USSDData values
    /// The USSDData type is a custom enum type that can represent any JSON value
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON value
    ///
    /// # Returns
    ///
    /// A USSDData value
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
        json_to_hash_str_any(json)
    }

    /// Convert a USSDData value to a JSON value
    /// This function is recursive and will convert nested USSDData values to JSON values
    /// The USSDData type is a custom enum type that can represent any JSON value
    ///
    /// # Arguments
    ///
    /// * `hash_any` - A USSDData value
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
    /// let hash_str_any = USSDData::Dict({
    ///    let mut dict = HashMap::new();
    ///   dict.insert("one".to_string(), USSDData::Str("1".to_string()));
    ///  dict.insert("two".to_string(), USSDData::Str("2".to_string()));
    /// dict
    /// });
    ///
    /// let json = hash_str_any_to_json(hash_str_any);
    /// `
    pub fn hash_str_any_to_json(&self, hash_any: USSDData) -> Value {
        match hash_any {
            USSDData::Str(s) => Value::String(s),
            USSDData::Int(i) => Value::Number(i.into()),
            USSDData::Float(f) => Value::Number((f as i64).into()),
            USSDData::List(list) => {
                let items: Vec<Value> = list
                    .into_iter()
                    .map(|item| self.hash_str_any_to_json(item))
                    .collect();
                Value::Array(items)
            }
            USSDData::ListStr(list) => Value::Array(list.into_iter().map(Value::String).collect()),
            USSDData::Dict(dict) => {
                let mut obj = serde_json::Map::new();
                for (key, value) in dict {
                    obj.insert(key, self.hash_str_any_to_json(value));
                }
                Value::Object(obj)
            }
            USSDData::None => Value::Null,
        }
    }
}

// Formatter for USSDData to convert JSON to USSDData
impl Display for USSDData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            USSDData::Str(s) => write!(f, "{}", s),
            USSDData::Int(i) => write!(f, "{}", i),
            USSDData::Float(fl) => write!(f, "{}", fl),
            USSDData::List(l) => write!(f, "{:?}", l),
            USSDData::ListStr(l) => write!(f, "{:?}", l),
            USSDData::Dict(d) => write!(f, "{:?}", d),
            USSDData::None => write!(f, "None"),
        }
    }
}

/// Convert a JSON value to a USSDData value
/// This function is recursive and will convert nested JSON values to USSDData values
/// The USSDData type is a custom enum type that can represent any JSON value
///
/// # Arguments
///
/// * `json` - A JSON value
///
/// # Returns
///
/// A USSDData value
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
pub fn json_to_hash_str_any(json: Value) -> USSDData {
    match json {
        Value::Null => USSDData::None,
        Value::Bool(b) => USSDData::Str(b.to_string()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                USSDData::Int(i as i64)
            } else if let Some(f) = n.as_f64() {
                USSDData::Float(f as f64)
            } else {
                USSDData::None
            }
        }
        Value::String(s) => USSDData::Str(s),
        Value::Array(arr) => {
            let mut list = Vec::new();
            for val in arr {
                list.push(json_to_hash_str_any(val));
            }
            USSDData::List(list)
        }
        Value::Object(obj) => {
            let mut dict = HashMap::new();
            for (key, val) in obj {
                dict.insert(key, json_to_hash_str_any(val));
            }
            USSDData::Dict(dict)
        }
    }
}

/// Convert a USSDData value to a JSON value
/// This function is recursive and will convert nested USSDData values to JSON values
/// The USSDData type is a custom enum type that can represent any JSON value
///
/// # Arguments
///
/// * `hash_any` - A USSDData value
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
/// let hash_str_any = USSDData::Dict({
///    let mut dict = HashMap::new();
///   dict.insert("one".to_string(), USSDData::Str("1".to_string()));
///  dict.insert("two".to_string(), USSDData::Str("2".to_string()));
/// dict
/// });
///
/// let json = hash_str_any_to_json(hash_str_any);
/// `
pub fn hash_str_any_to_json(hash_any: USSDData) -> Value {
    match hash_any {
        USSDData::Str(s) => Value::String(s),
        USSDData::Int(i) => Value::Number(i.into()),
        USSDData::Float(f) => Value::Number((f as i64).into()),
        USSDData::List(list) => {
            let items: Vec<Value> = list
                .into_iter()
                .map(|item| hash_str_any_to_json(item))
                .collect();
            Value::Array(items)
        }
        USSDData::ListStr(list) => Value::Array(list.into_iter().map(Value::String).collect()),
        USSDData::Dict(dict) => {
            let mut obj = serde_json::Map::new();
            for (key, value) in dict {
                obj.insert(key, hash_str_any_to_json(value));
            }
            Value::Object(obj)
        }
        USSDData::None => Value::Null,
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Stack<T> {
    pub items: Vec<T>,
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
        dict.insert("one".to_string(), USSDData::Str("1".to_string()));
        dict.insert("two".to_string(), USSDData::Str("2".to_string()));
        let hash_str_any = USSDData::new_dict(dict);
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
        expected_dict.insert("one".to_string(), USSDData::Str("1".to_string()));
        expected_dict.insert("two".to_string(), USSDData::Str("2".to_string()));

        let expected_hash_str_any = USSDData::Dict(expected_dict);

        assert_eq!(hash_str_any, expected_hash_str_any);
    }

    #[test]
    fn test_hash_str_any_to_json() {
        let mut dict = HashMap::new();
        dict.insert("one".to_string(), USSDData::Str("1".to_string()));
        dict.insert("two".to_string(), USSDData::Str("2".to_string()));
        let hash_str_any = USSDData::new_dict(dict);
        let json = hash_str_any_to_json(hash_str_any);
        let expected_json = json!({
            "one": "1",
            "two": "2"
        });
        assert_eq!(json, expected_json);
    }
}
