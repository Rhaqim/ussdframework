mod log;
mod menu;
pub mod prelude;
pub mod types;
mod ussd_request;
mod ussd_response;
mod ussd_screens;
mod ussd_service;
mod ussd_session;
mod utils;

extern crate serde;

use prelude::USSDMenu;
use ussd_request::USSDRequest;
use ussd_response::USSDResponse;
use ussd_screens::process_request;
use ussd_session::SessionCache;

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
    pub fn new(functions_path: String, session_cache: Box<dyn SessionCache>) -> Self {
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
    pub fn run(&self, request: USSDRequest, screens: USSDMenu) -> ussd_response::USSDResponse {
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

// pub mod prelude;
// pub mod menu;

// pub struct UssdApp {
//     // You can define any necessary fields here
// }

// impl UssdApp {
//     pub fn new() -> Self {
//         // Initialize any necessary resources
//         UssdApp {}
//     }

//     pub fn menu<F>(&mut self, name: &str, builder: F)
//     where
//         F: FnOnce(&mut menu::MenuBuilder),
//     {
//         let mut menu_builder = menu::MenuBuilder::new(name);
//         builder(&mut menu_builder);
//         // Store or process the constructed menu
//     }

//     pub fn run(&self) {
//         // Execute the USSD application
//     }
// }
