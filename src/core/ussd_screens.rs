use crate::{
    error,
    types::USSDData,
    utils::{evaluate_expression, evaluate_expression_op},
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{ussd_service::USSDServiceTrait, USSDRequest, USSDService, USSDSession};

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

impl ScreenType {
    pub fn to_string(&self) -> String {
        match self {
            ScreenType::Initial => "Initial".to_string(),
            ScreenType::Menu => "Menu".to_string(),
            ScreenType::Input => "Input".to_string(),
            ScreenType::Function => "Function".to_string(),
            ScreenType::Router => "Router".to_string(),
            ScreenType::Quit => "Quit".to_string(),
        }
    }

    pub fn from_string(screen_type: &str) -> ScreenType {
        match screen_type {
            "Initial" => ScreenType::Initial,
            "Menu" => ScreenType::Menu,
            "Input" => ScreenType::Input,
            "Function" => ScreenType::Function,
            "Router" => ScreenType::Router,
            "Quit" => ScreenType::Quit,
            _ => {
                error!("Invalid screen type");
                ScreenType::Initial
            }
        }
    }
}

// Define structure for a screen
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDScreen {
    pub text: String,
    pub screen_type: ScreenType,
    pub default_next_screen: String,
    #[serde(default)]
    pub service_code: Option<String>,
    #[serde(default)]
    pub menu_items: Option<HashMap<String, USSDMenuItems>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<USSDRouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
    // Additional fields based on screen type
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDMenuItems {
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDRouterOption {
    pub router_option: String,
    pub next_screen: String,
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
    fn display(&self, session: &USSDSession) -> Option<String>;
    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        services: &HashMap<String, USSDService>,
    );
}

impl USSDAction for USSDScreen {
    /// Displays a message corresponding to the screen type.
    ///
    /// The message construction depends on the type of screen:
    /// - For an initial screen, no message is displayed.
    /// - For a menu screen, the message concatenates the screen text with the menu items.
    /// - For an input screen, the message comprises the screen text alone.
    /// - For a function screen, no message is displayed.
    /// - For a router screen, no message is displayed.
    fn display(&self, session: &USSDSession) -> Option<String> {
        let mut message = String::new();

        // check if there's an error message in the session if there is then append to message
        if let Some(error_message) = &session.error_message {
            message.push_str(&error_message);
            message.push_str("\n\n");
        }

        match self.screen_type {
            ScreenType::Initial => None,
            ScreenType::Menu => {
                let text = evaluate_expression(&self.text, session);
                message.push_str(&text);

                if let Some(menu_items) = &self.menu_items {
                    let mut sorted_menu_items: Vec<(&String, &USSDMenuItems)> =
                        menu_items.iter().collect();
                    // Sort the menu items by their option number
                    sorted_menu_items
                        .sort_by_key(|(_, item)| item.option.parse::<usize>().unwrap());

                    for (index, (_, value)) in sorted_menu_items.iter().enumerate() {
                        message.push_str(&format!("\n{}. {}", index + 1, value.display_name));
                    }
                } else {
                    message.push_str("\nNo menu items found");
                }

                Some(message)
            }
            ScreenType::Input => {
                let text = evaluate_expression(&self.text, session);
                message.push_str(&text);
                Some(message)
            }
            ScreenType::Function => None,
            ScreenType::Router => None,
            ScreenType::Quit => {
                let text = evaluate_expression(&self.text, session);
                message.push_str(&text);
                Some(message)
            }
        }
    }

    /// Executes the specified screen action, determining the next screen based on the action type.
    ///
    /// The action can take various forms:
    /// - If it's a function, the function is called.
    /// - If it's a router, the next screen is determined based on the router option.
    /// - If it's an initial, quit, or menu screen, the next screen is set based on a default next screen.
    /// - If it's an input screen, the input is stored in the session data, and the next screen is set based on a default next screen.
    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        services: &HashMap<String, USSDService>,
    ) {
        let input = request.input.trim();

        match input {
            "0" => back(session),
            "00" => home(session),
            _ => {
                session.current_screen = match self.screen_type {
                    ScreenType::Initial => self.default_next_screen.clone(),
                    ScreenType::Menu => {
                        match input.parse::<usize>() {
                            Ok(selected_option) if selected_option > 0 => {
                                if let Some(menu_items_ref_unwrapped) = self.menu_items.as_ref() {
                                    if let Some(selected_item) = menu_items_ref_unwrapped
                                        .values()
                                        .find(|item| item.option == selected_option.to_string())
                                    {
                                        session.current_screen = selected_item.next_screen.clone();
                                        return;
                                    } else {
                                        error!("Selected menu item not found");
                                        session.error_message = Some("Invalid menu option".to_string());
                                        session.current_screen = session.current_screen.clone();
                                        return;
                                    }
                                }
                            }
                            _ => error!("Invalid menu option"),
                        }
                        self.default_next_screen.clone()
                    }
                    ScreenType::Input => {
                        if let Some(input_identifier) = &self.input_identifier {
                            session.data.insert(
                                input_identifier.to_string(),
                                USSDData::Str(input.to_string()),
                            );
                        }
                        self.default_next_screen.clone()
                    }
                    ScreenType::Function => {
                        if let Some(function_name) = &self.function {
                            call_function(session, services, function_name);
                        }
                        self.default_next_screen.clone()
                    }
                    ScreenType::Router => {
                        if let Some(router_options) = &self.router_options {
                            for option in router_options {
                                if evaluate_expression_op(session, &option.router_option) {
                                    session.current_screen = option.next_screen.clone();
                                    return;
                                }
                            }
                        }
                        self.default_next_screen.clone()
                    }
                    ScreenType::Quit => {
                        session.end_session = true;
                        self.default_next_screen.clone()
                    }
                }
            }
        }
    }
}

/// Call the function
///
/// # Arguments
///
/// * `session` - The USSD session.
/// * `request` - The USSD request.
/// * `services` - The USSD services.
/// * `function_name` - The name of the function to call.
/// * `functions_path` - The path to the functions used by the USSD application.
///
/// # Example
///
/// ```
/// use crate::ussd_screens::call_function;
/// use crate::ussd_request::USSDRequest;
/// use crate::ussd_session::USSDSession;
/// use std::collections::HashMap;
///
/// let mut session = USSDSession::new();
///
/// let services = HashMap::new();
/// let function_name = "function_name".to_string();
/// let functions_path = "path/to/functions".to_string();
///
/// call_function(&mut session, &services, &function_name, &functions_path);
/// ```
fn call_function(
    session: &mut USSDSession,
    services: &HashMap<String, USSDService>,
    function_name: &str,
) {
    let service = services.get(function_name).unwrap();

    service.call(session);
}
