use crate::{
    debug, error, info, trace,
    types::HashStrAny,
    utils::{evaluate_expression, evaluate_expression_op},
    USSDMenu,
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{
    ussd_service::USSDServiceTrait, SessionCache, USSDRequest, USSDResponse, USSDService,
    USSDSession,
};

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
    pub service_code: Option<String>,
    #[serde(default)]
    pub menu_items: Option<HashMap<String, MenuItems>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<RouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
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

    let mut current_screen = session.current_screen.clone();

    loop {
        if let Some(screen) = screens.menus.get(&current_screen) {
            info!(
                "\nRunning for {}\nScreen Type: {:?}\nRequest : {:?}\n",
                current_screen, screen.screen_type, request
            );

            // Execute the screen action for Function, Router, and Initial screen types
            // They contain no display message
            // They are used to execute a function, route to another screen, or set the initial screen
            // The next screen is set based on the action
            match screen.screen_type {
                ScreenType::Function | ScreenType::Router | ScreenType::Initial => {
                    screen.execute(
                        &mut session,
                        request,
                        functions_path.clone(),
                        &screens.services,
                    );
                }

                // Display the screen message and execute the screen action for Menu and Input screen types
                // They contain a display message
                // The next screen is set based on the action
                // It checks if the current screen has been displayed
                // If not, it displays the message and sets the current screen as displayed and also routes back to the current screen
                _ => {
                    let current_screen_displayed = session
                        .displayed
                        .entry(current_screen.clone())
                        .or_insert(false);

                    if !*current_screen_displayed {
                        debug!("Displaying message for screen: {}", current_screen);

                        response.message = screen.display(&session).unwrap_or_else(|| {
                            error!("Failed to display message for screen: {} please ensure the screen has a message", current_screen);
                            "Something went wrong, please stop".to_string()
                        });

                        session.displayed.insert(current_screen.clone(), true);
                        session.current_screen = current_screen.clone();
                        session.update_session(session_cache);

                        break;
                    } else {
                        debug!("Executing action for screen: {}", current_screen);

                        screen.execute(
                            &mut session,
                            request,
                            functions_path.clone(),
                            &screens.services,
                        );

                        // remove from displayed
                        session.displayed.remove(&current_screen);
                    }
                }
            }

            current_screen = session.current_screen.clone();
            continue;
        } else {
            break;
        }
    }

    return response;
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
        function_path: String,
        services: &HashMap<String, USSDService>,
    );
}

impl USSDAction for Screen {
    fn display(&self, session: &USSDSession) -> Option<String> {
        let mut message = String::new();

        match self.screen_type {
            ScreenType::Initial => None,
            ScreenType::Menu => {
                let text = evaluate_expression(&self.text, session);
                message.push_str(&text);

                if let Some(menu_items) = &self.menu_items {
                    let mut sorted_menu_items: Vec<(&String, &MenuItems)> =
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

    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        function_path: String,
        services: &HashMap<String, USSDService>,
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

        match self.screen_type {
            ScreenType::Initial => {
                session.current_screen = self.default_next_screen.clone();
            }
            ScreenType::Menu => {
                if let Some(selected_option) = input.parse::<usize>().ok() {
                    let menu_items_ref_unwrapped = self.menu_items.as_ref().unwrap();

                    let menu_items_len = menu_items_ref_unwrapped.len();

                    if 0 < selected_option && selected_option <= menu_items_len {
                        // Find the selected menu item by its option number
                        let selected_item = menu_items_ref_unwrapped
                            .values()
                            .find(|item| item.option == selected_option.to_string());

                        trace!("Selected Items: {:?}", selected_item);

                        if let Some(selected_item) = selected_item {
                            let next_screen = selected_item.next_screen.clone();
                            session.current_screen = next_screen;
                        } else {
                            error!("Selected menu item not found");
                            session.current_screen = self.default_next_screen.clone();
                        }
                    } else {
                        error!("Invalid menu option");
                        session.current_screen = self.default_next_screen.clone();
                    }
                } else {
                    error!("Invalid input");
                    session.current_screen = self.default_next_screen.clone();
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
                    call_function(session, &request, services, function_name, function_path);
                }
                session.current_screen = self.default_next_screen.clone();
            }
            ScreenType::Router => {
                if let Some(router_options) = &self.router_options {
                    for option in router_options {
                        if evaluate_expression_op(session, &option.router_option) {
                            // Navigate to the next screen based on the router option
                            session.current_screen = option.next_screen.clone();
                        }
                        // if evaluate_router_option(session, &option.router_option) {
                        //     // Navigate to the next screen based on the router option
                        //     session.current_screen = option.next_screen.clone();
                        // }
                    }
                } else {
                    // Navigate to the default next screen
                    session.current_screen = self.default_next_screen.clone();
                }
            }
            ScreenType::Quit => {
                // Quit the session
                session.end_session = true;
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
/// let request = USSDRequest {
///    msisdn
///    session_id
///    service_code
///    input
///    language
/// };
/// let services = HashMap::new();
/// let function_name = "function_name".to_string();
/// let functions_path = "path/to/functions".to_string();
///
/// call_function(&mut session, &request, &services, &function_name, &functions_path);
/// ```
fn call_function(
    session: &mut USSDSession,
    request: &USSDRequest,
    services: &HashMap<String, USSDService>,
    function_name: &str,
    functions_path: String,
) {
    let service = services.get(function_name).unwrap();

    service.call(session, request, functions_path);
}
