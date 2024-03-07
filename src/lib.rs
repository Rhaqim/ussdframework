mod screens;
mod ussd_request;
mod ussd_response;

use screens::process_request;
use std::collections::HashMap;
use ussd_request::USSDRequest;

fn parse_configuration(service_code: &str) -> HashMap<String, screens::Screen> {
    // Parse JSON configuration file based on service code
    // Return HashMap of screens
    unimplemented!()
}

pub struct UssdApp {
    screens: HashMap<String, screens::Screen>,
}

impl UssdApp {
    pub fn new(service_code: &str) -> Self {
        let screens = parse_configuration(service_code);
        UssdApp { screens }
    }

    pub fn run(&self, request: USSDRequest) -> ussd_response::USSDResponse {
        process_request(&request, &self.screens)
    }
}


use crate::{USSDRequest, USSDResponse};
use std::collections::HashMap;

pub fn process_request(request: &USSDRequest, screens: &HashMap<String, Screen>) -> USSDResponse {
    // Initialize variables to store response data
    let mut current_screen_name = "InitialScreen".to_string();
    let mut message = String::new();

    // Process USSDRequest, return screens and session
    let (mut session, screens) = process_ussd_request(request, screens);

    // Process USSD request
    loop {
        // Get the current screen
        let current_screen = match screens.get(&current_screen_name) {
            Some(screen) => screen,
            None => return USSDResponse {
                msisdn: request.msisdn.clone(),
                session_id: request.session_id.clone(),
                message: "Invalid screen name".to_string(),
            },
        };

        // Append screen text to message
        message.push_str(&current_screen.text);

        // Process different types of screens
        match current_screen.screen_type {
            ScreenType::Menu => {
                // Append menu items to message
                if let Some(menu_items) = &current_screen.menu_items {
                    for (option, display_name) in menu_items {
                        message.push_str(&format!("\n{}. {}", option, display_name));
                    }
                } else {
                    message.push_str("\nNo menu items found");
                }

                // Process user input
                let selected_option = &request.input.trim();
                if let Some(next_screen_name) = current_screen.menu_items.as_ref().and_then(|items| items.get(selected_option)) {
                    // Navigate to the next screen based on the selected option
                    current_screen_name = next_screen_name.clone();
                } else {
                    message.push_str("\nInvalid option, please try again");
                }
            }
            ScreenType::Input => {
                // Handle input screen logic
                // For simplicity, let's echo back the input
                message.push_str("\nEnter your input: ");
                
                // Move to the next screen
                current_screen_name = current_screen.default_next_screen.clone();
            }
            ScreenType::Function => {
                // Handle function screen logic
                if let Some(function_name) = &current_screen.function {
                    // Call the function
                    let response_message = call_function(function_name, &request);
                    message.push_str(&response_message);
                } else {
                    message.push_str("\nNo function specified");
                }

                // Move to the next screen
                current_screen_name = current_screen.default_next_screen.clone();
            }
            ScreenType::Router => {
                // Handle router screen logic
                if let Some(router_options) = &current_screen.router_options {
                    for option in router_options {
                        if evaluate_router_option(&option.router_option, &request) {
                            // Navigate to the next screen based on the router option
                            current_screen_name = option.next_screen.clone();
                        }
                    }
                } else {
                    // Navigate to the default next screen
                    current_screen_name = current_screen.default_next_screen.clone(); 
                }
            }
            ScreenType::Quit => {
                // Quit the session
                return USSDResponse {
                    msisdn: request.msisdn.clone(),
                    session_id: request.session_id.clone(),
                    message: "Thank you for using the system".to_string(),
                };
            }
        }
    }

    // Return USSD response
    USSDResponse {
        msisdn: request.msisdn.clone(),
        session_id: request.session_id.clone(),
        message,
    }
}

fn process_ussd_request(request: &USSDRequest, screens: &HashMap<String, Screen>) -> (Session, HashMap<String, Screen>) {
    // Implement logic to process USSD request and return session and screens
    // For simplicity, let's assume it always returns a new session and all screens
    (Session::new(), screens.clone())
}

// Dummy function to call service
fn call_function(function_name: &str, request: &USSDRequest) -> String {
    // Implement logic to call the function
    format!("\nCalling function: {}", function_name)
}

// Dummy function to evaluate router option
fn evaluate_router_option(router_option: &str, request: &USSDRequest) -> bool {
    // Implement logic to evaluate router option
    // For simplicity, let's assume it always evaluates to true
    true
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
