pub mod buy_airtime;

use std::collections::HashMap;

pub use buy_airtime::*;
use ussdframework::prelude::{HashStrAny, USSDRequest};

pub fn get_functions() -> HashMap<String, fn(&USSDRequest, &str) -> HashStrAny> {
    let mut functions = HashMap::new();

    functions.insert("buy_airtime".to_string(), buy_airtime as fn(&USSDRequest, &str) -> HashStrAny);
    functions.insert("get_balance".to_string(), get_balance as fn(&USSDRequest, &str) -> HashStrAny);

    functions
}