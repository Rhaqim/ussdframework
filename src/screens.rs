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
    let menus: USSDMenu = USSDMenu::load_from_json("data/menu.json").unwrap();

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


/// Evaluates the router options.
///
/// This function currently returns `true` for all router options.
/// Evaluate the router option, should contain `{{` and `}}` to be evaluated
///
/// Example:
///
/// ```
/// let router_option = "{{user.age > 18}}";
/// ```
/// The first part user is the data_key stored in the session data
/// The second part is the condition to be evaluated
///
/// # Arguments
///
/// * `session` - The reference to the `USSDSession` struct.
/// * `router_option` - The string representing the router option to be evaluated.
///
/// # Returns
///
/// A boolean value indicating whether the router option evaluates to true or false.
fn evaluate_router_options(session: &USSDSession, request: &USSDRequest, router_option: &str) -> bool {
    // Check if the router option contains `{{` and `}}` to indicate an expression
    if router_option.contains("{{") && router_option.contains("}}") {
        // Extract the expression inside `{{ }}`
        let expression = router_option
            .trim_start_matches("{{")
            .trim_end_matches("}}")
            .trim();

        // Here you would implement your logic to parse and evaluate the expression.
        // This implementation currently returns `true` for all router options.
        // You need to replace this with your actual implementation.

        // Parse and evaluate the expression
        match parse_and_evaluate_expression(&session, expression) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error evaluating router option: {}", e);
                false
            }
        };

        // For demonstration purposes, return true
        true
    } else {
        // If the router option doesn't contain `{{` and `}}`, print an error message and return false
        eprintln!("Error evaluating router option: Expression not found");
        false
    }
}

fn parse_and_evaluate_expression(session: &USSDSession, expression: &str) -> Result<bool, String> {
    // Split the expression into parts
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression: Must consist of three parts separated by whitespace".to_string());
    }

    // Extract the data key from the expression
    let data_key = parts[0];

    let mut data_key_parts: Vec<&str> = Vec::new();

    // check if data key contains a dot and then split it
    if data_key.contains('.') {
        data_key_parts = data_key.split('.').collect();
        if data_key_parts.len() != 2 {
            return Err("Invalid expression: Data key must consist of one or two parts separated by a dot".to_string());
        }
    } else {
        data_key_parts.push(data_key);
    }

    // Check if the data key exists in the session data
    if let Some(data_value) = session.data.get(data_key_parts[0]) {
        // Get the operator and right operand from the expression
        let operator = parts[1];
        let right_operand: i32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => return Err("Invalid expression: Right operand is not a valid integer".to_string()),
        };

        // Evaluate the expression based on the data value type
        match data_value {
            HashStrAny::Str(string_value) => {
                // If the data value is a string, parse it as an integer and compare
                if let Ok(left_operand) = string_value.parse::<i32>() {
                    match operator {
                        "==" => Ok(left_operand == right_operand),
                        ">" => Ok(left_operand > right_operand),
                        "<" => Ok(left_operand < right_operand),
                        _ => Err(format!("Unsupported operator: {}", operator)),
                    }
                } else {
                    // If the string cannot be parsed as an integer, return an error
                    Err("Invalid expression: Left operand is not a valid integer".to_string())
                }
            }
            HashStrAny::Dict(dict_value) => {
                // If the data value is a dictionary, extract nested values
                let mut current_value: &HashStrAny = data_value;
                for key_part in parts[0].split('.') {
                    if let HashStrAny::Dict(inner_dict) = current_value {
                        if let Some(next_value) = inner_dict.get(key_part) {
                            current_value = next_value;
                        } else {
                            // If the key does not exist in the dictionary, return an error
                            return Err(format!("Key '{}' not found in dictionary", key_part));
                        }
                    } else {
                        // If any intermediate value is not a dictionary, return an error
                        return Err("Invalid expression: Intermediate value is not a dictionary".to_string());
                    }
                }

                // After navigating through the nested structure, compare the value if it's an integer
                if let HashStrAny::Int(left_operand) = current_value {
                    match operator {
                        "==" => Ok(*left_operand == right_operand),
                        ">" => Ok(*left_operand > right_operand),
                        "<" => Ok(*left_operand < right_operand),
                        _ => Err(format!("Unsupported operator: {}", operator)),
                    }
                } else {
                    // If the final value is not an integer, return an error
                    Err("Invalid expression: Final value is not an integer".to_string())
                }
            }
            _ => Err("Unsupported data value type".to_string()), // Unsupported data value type
        }
    } else {
        // If the data key doesn't exist in the session data, return an error
        Err(format!("Key '{}' not found in session data", data_key_parts[0]))
    }
}
