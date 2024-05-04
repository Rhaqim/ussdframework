mod admin;
mod core;
mod database;
mod interface;
mod log;
mod menu;
pub mod prelude;
pub mod types;
mod utils;

extern crate serde;

use core::{process_request, InMemorySessionStore, SessionCache, USSDRequest, USSDResponse};
use menu::USSDMenu;
use utils::{register_function, FUNCTION_MAP, REGISTERED_FUNCTIONS};

/// Represents a USSD application.
/// The USSD application is responsible for processing USSD requests and responses.
///
/// # Fields
///
/// * `functions_path` - The path to the functions used by the USSD application.
/// * `session_cache` - The session cache implementation used by the USSD application.
///
/// # Examples
///
/// ```
/// use ussdframework::prelude::*;
///
/// let app = UssdApp::new(false, None);
/// ```
pub struct UssdApp {
    pub session_cache: Box<dyn SessionCache>,
}

impl UssdApp {
    /// Creates a new instance of `UssdApp`.
    ///
    /// # Arguments
    ///
    /// * `built_in_session_manager` - A boolean value indicating whether to use the built-in session manager.
    /// * `session_cache` - The session cache implementation used by the USSD application.
    ///
    /// # Returns
    ///
    /// A new instance of `UssdApp`.
    pub fn new(
        built_in_session_manager: bool,
        session_manager: Option<Box<dyn SessionCache>>,
    ) -> UssdApp {
        let session_cache: Box<dyn SessionCache> =
            if built_in_session_manager || session_manager.is_none() {
                Box::new(InMemorySessionStore::new())
            } else {
                session_manager.unwrap()
            };

        UssdApp { session_cache }
    }

    /// Registers a batch of USSD functions provided in the `functions_map`.
    ///
    /// The `register_functions` function is responsible for registering a batch of USSD (Unstructured 
    /// Supplementary Service Data) functions provided in the `functions_map`. It iterates through 
    /// each function in the map, checks if it has already been registered, and if not, registers it 
    /// by calling the `register_function` function. Once registered, the path of the function is added 
    /// to the set of registered functions to prevent redundant registrations.
    ///
    /// # Arguments
    ///
    /// * `functions_map`: A `HashMap<String, USSDFunction>` containing the mapping of function paths to 
    ///                    USSD functions. Each function is identified by its unique path.
    ///
    /// # Example
    ///
    /// ```rust
    /// use ussdframework::prelude::*;
    ///
    /// use std::collections::HashMap;
    /// 
    /// // Define your USSD function
    /// fn my_function(request: &USSDRequest, url: &str) -> USSDData {
    ///    // Your function logic here
    ///    return USSDData::Str("Hello".to_string());
    /// }
    /// 
    /// // Define a function that returns a map of functions
    /// fn functions () -> FunctionMap {
    ///   let mut functions_map = HashMap::new();
    ///
    ///   functions_map.insert("my_function".to_string(), my_function as USSDFunction);
    ///
    ///   functions_map
    /// }
    ///
    /// // Register the functions
    /// register_functions(functions());
    ///
    /// ```
    ///
    /// # Panics
    ///
    /// This function panics if it fails to acquire the lock on either the function map or the set of 
    /// registered functions.
    ///
    /// # Safety
    ///
    /// This function is safe to call as long as the `FUNCTION_MAP` and `REGISTERED_FUNCTIONS` are correctly 
    /// initialized and accessible by the current thread.
    ///
    pub fn register_functions(&self, functions_map: types::FunctionMap) {
        let mut function_map_guard = FUNCTION_MAP.lock().expect("Failed to lock function map");
        let mut registered_functions_guard = REGISTERED_FUNCTIONS.lock().expect("Failed to lock registered functions");
    
        for (path, function) in functions_map {
            if registered_functions_guard.contains(&path) {
                // Function already registered, skip it
                continue;
            }
    
            register_function(&path, function, &mut function_map_guard);
            registered_functions_guard.insert(path);
        }
    }

    /// Runs the USSD application with the given request and screens.
    ///
    /// # Arguments
    ///
    /// * `request` - The USSD request.
    /// * `screens` - The USSD menu screens.
    ///
    /// # Returns
    ///
    /// The USSD response.
    pub fn run(&self, request: USSDRequest, screens: USSDMenu) -> USSDResponse {
        process_request(&request, &self.session_cache, &screens)
    }

    /// Displays the menu to the user.
    ///
    /// # Arguments
    ///
    /// * `ussd_response` - The USSD response containing the menu message.
    pub fn display_menu(&self, ussd_response: &USSDResponse) {
        // Display the menu to the user
        println!("{}", ussd_response.message);
    }
}
