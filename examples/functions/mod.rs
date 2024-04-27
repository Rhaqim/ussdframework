pub mod buy_airtime;

use std::collections::HashMap;

pub use buy_airtime::*;
use ussdframework::prelude::{FunctionMap, USSDFunction};

pub fn get_functions() -> FunctionMap {
    let mut functions = HashMap::new();

    functions.insert("buy_airtime".to_string(), buy_airtime as USSDFunction);
    functions.insert("get_balance".to_string(), get_balance as USSDFunction);

    functions
}
