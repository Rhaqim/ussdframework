use crate::{
    types::HashStrAny,
    ussd_request::USSDRequest,
    ussd_response::USSDResponse,
    ussd_session::{SessionCache, USSDSession},
    USSDMenu,
    info,
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
    pub menu_items: Option<HashMap<String, MenuItems>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<RouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    // Additional fields based on screen type
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MenuItems {
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouterOption {
    pub router_option: String,
    pub next_screen: String,
}

/// Entry point for processing USSD requests.
///
/// # Arguments
///
/// * `request` - The USSD request.
/// * `functions_path` - The path to the functions used by the USSD application.
/// * `session_cache` - The session cache implementation used by the USSD application.
/// * `screens` - The USSD menu screens.
///
/// # Returns
///
/// The USSD response.
///
/// # Example
///
/// ```
/// use crate::ussd_service::process_request;
/// use crate::ussd_request::USSDRequest;
/// use crate::ussd_response::USSDResponse;
/// use crate::ussd_session::SessionCache;
///
/// let request = USSDRequest {
///    msisdn
///    session_id
///    input
/// };
///
/// let functions_path = "path/to/functions".to_string();
/// let session_cache = Box::new(SessionCacheImpl::new());
/// let screens = USSDMenu::new();
///
/// let response = process_request(&request, &functions_path, &session_cache, &screens);
/// ```
pub fn process_request(
    request: &USSDRequest,
    functions_path: &String,
    session_cache: &Box<dyn SessionCache>,
    screens: &USSDMenu,
) -> USSDResponse {
    let mut message = String::new();

    // Get the initial screen
    let (initial_screen, _) = screens.get_initial_screen();

    // Generate or retrieve the session
    let mut session = USSDSession::get_or_create_session(request, &initial_screen, session_cache);

    // Create a response object
    let mut response: USSDResponse = USSDResponse {
        msisdn: request.msisdn.clone(),
        session_id: request.session_id.clone(),
        message: "Something went wrong, please try again later".to_string(),
    };

    // Display screen history
    session.display_screen_history();

    let current_screen = session.current_screen.clone();

    info!("\nCurrent screen: {}", current_screen);
    info!("\nRequest: {:?}", request);

    match screens.menus.get(&current_screen) {
        Some(screen) => {
            screen.execute(&mut session, request, functions_path.clone(), &mut message);

            // Store the current screen in the session's visited screens
            session.visited_screens.push(session.current_screen.clone());

            // Update the session's last interaction time
            session.update_last_interaction_time();

            // Store the session
            session.store_session(&session_cache).unwrap();

            response.message = message;

            return response;
        }
        None => {
            return response;
        }
    };
}

fn back(session: &mut USSDSession) {
    // switch to the previous screen
    if let Some(prev_screen) = session.visited_screens.pop() {
        session.current_screen = prev_screen;
    }
}

fn home(session: &mut USSDSession) {
    // Switch to the initial screen
    session.current_screen = session.visited_screens.first().unwrap().clone();
}

pub trait USSDAction {
    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        function_path: String,
        message: &mut String,
    );
}

impl USSDAction for Screen {
    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        function_path: String,
        message: &mut String,
    ) {
        let input = request.input.trim();

        // if input is 0, go back
        if input == "0" {
            back(session);
        }

        // if input is 00, go home
        if input == "00" {
            home(session);
        }

        message.push_str(&self.text);

        match self.screen_type {
            ScreenType::Initial => {
                session.current_screen = self.default_next_screen.clone();
            }
            ScreenType::Menu => {
                // Append menu items to message
                if let Some(menu_items) = &self.menu_items {
                    for (index, (_, value)) in menu_items.iter().enumerate() {
                        message.push_str(&format!("\n{}. {}", index + 1, value.display_name));
                    }
                } else {
                    message.push_str("\nNo menu items found");
                }

                // Process user input
                if let Some(selected_option) = input.parse::<usize>().ok() {
                    let menu_items_len = self.menu_items.as_ref().unwrap().len();
                    if selected_option > 0 && selected_option <= menu_items_len {
                        let next_screen = self
                            .menu_items
                            .as_ref()
                            .unwrap()
                            .values()
                            .nth(selected_option - 1)
                            .unwrap()
                            .next_screen
                            .clone();
                        session.current_screen = next_screen;
                    } else {
                        message.push_str("\nInvalid option, please try again");
                    }
                } else {
                    message.push_str("\nInvalid option, please try again");
                }
            }
            ScreenType::Input => {
                if let Some(input_identifier) = &self.input_identifier {
                    session.data.insert(
                        input_identifier.to_string(),
                        HashStrAny::Str(input.to_string()),
                    );
                }
                session.current_screen = self.default_next_screen.clone();
            }
            ScreenType::Function => {
                if let Some(function_name) = &self.function {
                    // Call the function
                    let response_message = call_function(function_name, function_path, &request);
                    message.push_str(&response_message);
                }
                session.current_screen = self.default_next_screen.clone();
            }
            ScreenType::Router => {
                if let Some(router_options) = &self.router_options {
                    for option in router_options {
                        if evaluate_router_option(session, &option.router_option) {
                            // Navigate to the next screen based on the router option
                            session.current_screen = option.next_screen.clone();
                        }
                    }
                } else {
                    // Navigate to the default next screen
                    session.current_screen = self.default_next_screen.clone();
                }
            }
            ScreenType::Quit => {
                message.push_str(&self.text);

                // Quit the session
                session.end_session = true;
            }
        }
    }
}

// Dummy function to call service
fn call_function(_function_name: &str, _functions_path: String, _request: &USSDRequest) -> String {
    // Implement logic to call the function
    // For simplicity, let's assume it always returns a response message
    "Function called successfully".to_string()
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
fn evaluate_router_option(session: &USSDSession, router_option: &str) -> bool {
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
        return Err(
            "Invalid expression: Must consist of three parts separated by whitespace".to_string(),
        );
    }

    // Extract the data key from the expression
    let data_key = parts[0];

    let mut data_key_parts: Vec<&str> = Vec::new();

    // check if data key contains a dot and then split it
    if data_key.contains('.') {
        data_key_parts = data_key.split('.').collect();
        if data_key_parts.len() != 2 {
            return Err(
                "Invalid expression: Data key must consist of one or two parts separated by a dot"
                    .to_string(),
            );
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
            Err(_) => {
                return Err("Invalid expression: Right operand is not a valid integer".to_string())
            }
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
            HashStrAny::Dict(_dict_value) => {
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
                        return Err("Invalid expression: Intermediate value is not a dictionary"
                            .to_string());
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
        Err(format!(
            "Key '{}' not found in session data",
            data_key_parts[0]
        ))
    }
}
