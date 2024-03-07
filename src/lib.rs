mod screens;
mod ussd_request;
mod ussd_response;

use std::collections::HashMap;

use crate::{USSDRequest, USSDResponse};
use screens::process_request;
use ussd_request::USSDRequest;

pub struct UssdApp {
    functions_path: String,
    pub session_cache: Box<dyn SessionCache>,
}

impl UssdApp {
    pub fn new(functions_path: String, session_cache: Box<dyn SessionCache>) -> Self {
        UssdApp {
            functions_path,
            session_cache,
        }
    }

    pub fn run(&self, request: USSDRequest) -> ussd_response::USSDResponse {
        process_request(&request, &self.functions_path, &self.session_cache)
    }

    pub fn display_menu(&self, ussd_response: &USSDResponse) {
        // Display the menu to the user

        // For example, you can use the following code to display the menu:
        println!("{}", ussd_response.text);
            
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
