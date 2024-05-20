use crate::info;
use crate::types::{FunctionMap, USSDFunction};
use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};

lazy_static::lazy_static! {
    // Define a lazy static variable to store registered functions
    pub static ref FUNCTION_MAP: Arc<Mutex<FunctionMap>> = Arc::new(Mutex::new(FunctionMap::new()));
    pub static ref REGISTERED_FUNCTIONS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

}

/// Registers a USSD function with the provided path.
///
/// The `register_function` function is used to register a USSD (Unstructured Supplementary Service Data) function
/// with the specified path. USSD functions are callbacks that handle USSD requests and return the appropriate
/// USSD data. Once registered, the function can be invoked by its path.
///
/// # Arguments
///
/// * `path`: A string representing the unique path for the USSD function. This path is used to identify
///           the function when invoking it.
/// * `function_ptr`: A function pointer representing the USSD function to be registered. This function
///                   should have the signature `fn(&USSDSession, &str) -> USSDData`, where `USSDSession`
///                   represents the session data and `USSDData` represents the data to be returned
///                   as a response.
/// * `function_map_guard`: A mutable reference to a `MutexGuard<HashMap<String, USSDFunction>>` representing
///                          the guarded HashMap containing the registered USSD functions. This guard is used
///                          to insert the new function into the map.
///
/// # Example
///
/// ```
/// use my_module::{register_function, USSDSession, USSDData};
///
/// fn my_function(session: &USSDSession, input: &str) -> USSDData {
///     // Implementation of the USSD function
///     unimplemented!()
/// }
///
/// // Registering the function with a specific path
/// register_function("/my-path", my_function, &mut function_map_guard);
/// ```
///
/// # Panics
///
/// This function panics if it fails to acquire the lock on the function map.
///
/// # Safety
///
/// This function is safe to call as long as the provided `function_map_guard` is a mutable reference
/// to a locked `MutexGuard<HashMap<String, USSDFunction>>`.
///
pub fn register_function(
    path: &str,
    function_ptr: USSDFunction,
    function_map_guard: &mut MutexGuard<FunctionMap>,
) {
    info!("Registering function: {}", path);

    let function_name = path.to_string();
    function_map_guard.insert(function_name, function_ptr);
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::core::USSDSession;
    use crate::types::USSDData;

    fn test_function(_session: &USSDSession, _input: &str) -> USSDData {
        // Implementation of the test function
        unimplemented!()
    }

    #[test]
    fn test_register_function() {
        // Create a new function map
        let mut function_map_guard = FUNCTION_MAP.lock().unwrap();

        // Register the test function
        register_function("/test-path", test_function, &mut function_map_guard);

        // Check if the function is registered
        assert!(function_map_guard.contains_key("/test-path"));
    }

    #[test]
    fn test_register_function_no_panic() {
        // Create a new function map
        let mut function_map_guard = FUNCTION_MAP.lock().unwrap();
    
        let mut registered_function_set = REGISTERED_FUNCTIONS.lock().unwrap();
    
        // Register the test function
        register_function("/test-path", test_function, &mut function_map_guard);
    
        // Add the function path to the registered functions set
        registered_function_set.insert("/test-path".to_string());
    
        // Attempt to register the same function again and check if it already exists
        let path = "/test-path";
        if registered_function_set.contains(path) {
            println!("Function path '{}' is already registered.", path);
        } else {
            register_function(path, test_function, &mut function_map_guard);
            registered_function_set.insert(path.to_string());
        }
    
        // Verify that only one instance of '/test-path' exists
        assert!(registered_function_set.contains(path));
        assert_eq!(registered_function_set.len(), 1);
    }
}
