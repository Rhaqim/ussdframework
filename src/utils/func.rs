use crate::info;
use crate::types::{FunctionMap, USSDFunction};
use std::sync::MutexGuard;

/// Inserts a function into the provided map guard.
#[allow(dead_code)]
pub fn register_function(
    path: &str,
    function_ptr: USSDFunction,
    function_map_guard: &mut MutexGuard<FunctionMap>,
) {
    info!("Registering function: {}", path);
    function_map_guard.insert(path.to_string(), function_ptr);
}

