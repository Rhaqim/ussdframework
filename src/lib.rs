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

    /// Registers application functions
    /// Takes a HashMap of functions that would be called through the journey of the USSD application
    /// The function_map is a HashMap with the key as the function path and the value as the function pointer
    /// The function path is a string that represents the path to the function, should also be the key in the menu config service
    ///
    /// # Arguments
    ///
    /// * `functions_map` - A HashMap of functions: key is the function path, value is the function pointer
    ///
    /// # Example
    ///
    /// ```rust
    /// use ussdframework::prelude::*;
    ///
    /// use std::collections::HashMap;
    ///
    /// fn my_function(request: &USSDRequest, url: &str) -> USSDData {
    ///    // Your function logic here
    ///    return USSDData::Str("Hello".to_string());
    /// }
    ///
    /// fn functions () -> FunctionMap {
    ///   let mut functions_map = HashMap::new();
    ///
    ///   functions_map.insert("my_function".to_string(), my_function as USSDFunction);
    ///
    ///   functions_map
    /// }
    ///
    /// register_functions(functions());
    ///
    /// ```
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
