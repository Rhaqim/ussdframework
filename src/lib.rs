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

/// Represents a USSD application.
pub struct UssdApp {
    functions_path: String,
    pub session_cache: Box<dyn SessionCache>,
}

impl UssdApp {
    /// Creates a new instance of `UssdApp`.
    ///
    /// # Arguments
    ///
    /// * `functions_path` - The path to the functions used by the USSD application.
    /// * `session_cache` - The session cache implementation used by the USSD application.
    ///
    /// # Returns
    ///
    /// A new instance of `UssdApp`.
    pub fn new(
        functions_path: String,
        built_in_session_manager: bool,
        session_manager: Option<Box<dyn SessionCache>>,
    ) -> UssdApp {
        let session_cache: Box<dyn SessionCache>;

        if built_in_session_manager || session_manager.is_none() {
            session_cache = Box::new(InMemorySessionStore::new());
        } else {
            session_cache = session_manager.unwrap();
        }

        UssdApp {
            functions_path,
            session_cache,
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
        process_request(
            &request,
            &self.functions_path,
            &self.session_cache,
            &screens,
        )
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
