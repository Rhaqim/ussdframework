use crate::{
    prelude::{HashStrAny, USSDMenu},
    ussd_session::{SessionCache, USSDSession},
    USSDRequest, USSDResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Define types of screens
#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ScreenType {
    Initial,
    Menu,
    Input,
    Function,
    Router,
    Quit,
}

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Screen {
    pub text: String,
    pub screen_type: ScreenType,
    pub default_next_screen: String,
    #[serde(default)]
    pub menu_items: Option<HashMap<String, String>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<RouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    // Additional fields based on screen type
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouterOption {
    pub router_option: String,
    pub next_screen: String,
}

// Implement logic to process USSD requests
pub fn process_request(
    request: &USSDRequest,
    functions_path: &String,
    session_cache: &Box<dyn SessionCache>,
) -> USSDResponse {
    // Initialize variables to store response data
    let mut current_screen_name = "InitialScreen".to_string();
    let mut message = String::new();

    // Process USSDRequest, return screens and session
    let (mut session, screens) = process_ussd_request(request, &session_cache);

    let response: USSDResponse = USSDResponse {
        msisdn: request.msisdn.clone(),
        session_id: request.session_id.clone(),
        message: "Invalid screen name".to_string(),
    };

    // Process USSD request
    loop {
        // Get the current screen
        let current_screen = match screens.menus.get(&current_screen_name) {
            Some(screen) => screen,
            None => {
                return response;
            }
        };

        // Append screen text to message
        message.push_str(&current_screen.text);

        // Process different types of screens
        match current_screen.screen_type {
            ScreenType::Initial => {
                // Move to the next screen
                current_screen_name = current_screen.default_next_screen.clone();
            }
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
                if let Some(next_screen_name) = current_screen
                    .menu_items
                    .as_ref()
                    .and_then(|items| items.get(*selected_option))
                {
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

                // Process user input
                let input = &request.input.trim();

                // input identifier
                if let Some(input_identifier) = &current_screen.input_identifier {
                    process_input(input, input_identifier, &mut session);
                } else {
                    message.push_str("\nNo input identifier specified");
                }

                // Move to the next screen
                current_screen_name = current_screen.default_next_screen.clone();
            }
            ScreenType::Function => {
                // Handle function screen logic
                if let Some(function_name) = &current_screen.function {
                    // Call the function
                    let response_message =
                        call_function(function_name, functions_path.clone(), &request);
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

        session.current_screen = current_screen_name.clone();

        // Store the session
        session.store_session(&session_cache).unwrap();

        // Check if the session has ended
        if session.end_session {
            return response;
        }

        // Check if the session has timed out
        if session.has_timed_out(std::time::Duration::from_secs(60)) {
            return response;
        }

        // Update the session's last interaction time
        session.update_last_interaction_time();

        // Display screen history
        session.display_screen_history();

        // Return USSD response
        return USSDResponse {
            msisdn: request.msisdn.clone(),
            session_id: request.session_id.clone(),
            message,
        };
    }
    // Return USSD response
    // USSDResponse {
    //     msisdn: request.msisdn.clone(),
    //     session_id: request.session_id.clone(),
    //     message,
    // }
}

fn process_ussd_request(
    request: &USSDRequest,
    session_cache: &Box<dyn SessionCache>,
) -> (USSDSession, USSDMenu) {
    // Implement logic to process USSD request and return session and screens
    // For simplicity, let's assume it always returns a new session and all screens
    let menus: USSDMenu = USSDMenu::load_from_json("menu.json").unwrap();

    let (initial_screen, _) = menus.get_initial_screen();

    let session = USSDSession::get_or_create_session(
        request,
        &initial_screen,
        std::time::Duration::from_secs(60),
        session_cache,
    );

    (session, menus)
}

// Dummy function to call service
fn call_function(function_name: &str, functions_path: String, request: &USSDRequest) -> String {
    // Implement logic to call the function
    // For simplicity, let's assume it always returns a response message
    "Function called successfully".to_string()
}

// Dummy function to evaluate router option
fn evaluate_router_option(router_option: &str, request: &USSDRequest) -> bool {
    // Implement logic to evaluate router option
    // For simplicity, let's assume it always evaluates to true
    true
}

// Dummy function to process input
fn process_input(input: &str, input_identifier: &str, session: &mut USSDSession) {
    // Implement logic to process input
    // For simplicity, let's assume it always stores the input in the session
    session.data.insert(
        input_identifier.to_string(),
        HashStrAny::Str(input.to_string()),
    );
}
