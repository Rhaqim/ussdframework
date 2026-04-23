mod core;
mod log;
pub mod prelude;
pub mod types;
mod utils;

extern crate serde;

use core::{
    process_request, InMemorySessionStore, SessionCache, USSDMenu, USSDRequest, USSDResponse,
};

#[cfg(feature = "menubuilder")]
mod builder;

/// Represents a USSD application.
///
/// The application owns the registered function map (previously a process-global).
/// This makes the app self-contained, testable, and safe for multi-app processes.
pub struct UssdApp {
    pub session_cache: Box<dyn SessionCache>,
    function_map: types::FunctionMap,
}

impl UssdApp {
    /// Creates a new instance of `UssdApp`.
    ///
    /// * `built_in_session_manager` — use the in-process in-memory store.
    /// * `session_manager` — supply an external cache implementation.
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

        UssdApp {
            session_cache,
            function_map: types::FunctionMap::new(),
        }
    }

    /// Register a batch of USSD functions.
    ///
    /// Functions are stored in the app instance rather than a process global,
    /// so multiple independent `UssdApp` instances can have different function sets.
    pub fn register_functions(&mut self, functions_map: types::FunctionMap) {
        for (path, function) in functions_map {
            self.function_map.entry(path).or_insert(function);
        }
    }

    /// Runs the USSD application with the given request and screens.
    ///
    /// Router expressions in the menu are validated before processing begins.
    /// Invalid expressions are logged as errors but do not abort the request —
    /// the affected router screen will fall through to `default_next_screen`.
    pub fn run(&self, request: USSDRequest, screens: USSDMenu) -> USSDResponse {
        // Validate router expressions and log any problems at startup time.
        if let Err(expr_errors) = screens.validate_router_expressions() {
            for msg in &expr_errors {
                crate::error!("Router expression validation: {}", msg);
            }
        }

        process_request(&request, &self.session_cache, &screens, &self.function_map)
    }

    /// Displays the menu message to stdout.
    pub fn display_menu(&self, ussd_response: &USSDResponse) {
        println!("{}", ussd_response.message);
    }
}


