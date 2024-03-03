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
impl RouterOptions {
    pub fn to_string(&self) -> String {
        match self {
            RouterOptions::List(options) => {
                format!("{:?}", options)
            }
            RouterOptions::ListStr(options) => {
                format!("{:?}", options)
            }
            RouterOptions::String(option) => {
                option.to_string()
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HashStrAny {
    Str(String),
    Int(i32),
    Float(f32),
    Bool(bool),
    List(Vec<HashStrAny>),
    ListStr(Vec<String>),
    Dict(HashMap<String, HashStrAny>),
    None,
}
