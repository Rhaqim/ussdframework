use std::collections::HashMap;

use crate::info;
use crate::prelude::USSDRequest;
use crate::types::HashStrAny;
use std::sync::Mutex;
// Define a type to store registered functions
type FunctionMap = HashMap<String, fn(&USSDRequest, &str) -> HashStrAny>;

lazy_static::lazy_static! {
    // Define a lazy static variable to store registered functions
    pub static ref FUNCTION_MAP: Mutex<FunctionMap> = Mutex::new(FunctionMap::new());
}

// Function to register functions
pub fn register_function(path: &str, function_ptr: fn(&USSDRequest, &str) -> HashStrAny) {
    info!("Registering function: {}", path);
    let function_name = path.to_string();
    FUNCTION_MAP.lock().unwrap().insert(function_name, function_ptr);
}

#[macro_export]
macro_rules! register_function {
    ($path:expr, $function_ptr:expr) => {
        $crate::utils::register_function($path, $function_ptr);
    };
}

