mod screens;
mod ussd_request;
mod ussd_response;

use std::collections::HashMap;

use crate::{USSDRequest, USSDResponse};
use screens::process_request;
use ussd_request::USSDRequest;

pub struct UssdApp {}

impl UssdApp {
    pub fn new() -> Self {
        UssdApp {}
    }

    pub fn run(&self, request: USSDRequest) -> ussd_response::USSDResponse {
        process_request(&request)
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
