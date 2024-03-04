use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};

use super::ussd_session::UssdSession;

#[derive(Debug, Serialize, Deserialize)]
pub struct MenuItems {
    pub option: String,
    pub display_name: String,
    pub default_next_screen: String,
}

// Define an enum to represent different types of USSD screens
#[derive(Debug, Serialize)]
pub enum UssdScreen {
    Initial {
        default_next_screen: String,
    },
    Menu {
        title: String,
        default_next_screen: String,
        menu_items: HashMap<String, MenuItems>,
    },
    Input {
        title: String,
        default_next_screen: String,
        input_type: String,
        input_identifier: String,
    },
    Function {
        title: String,
        default_next_screen: String,
        function_name: String,
    },
    Router {
        title: String,
        default_next_screen: String,
        router_name: String,
    },
    Quit,
}

impl<'de> Deserialize<'de> for UssdScreen {
    fn deserialize<D>(deserializer: D) -> Result<UssdScreen, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawUssdScreen {
            #[serde(rename = "type")]
            screen_type: String,
            // Other fields common to all screen types
            title: String,
            default_next_screen: String,
            // Fields specific to certain screen types
            menu_items: Option<HashMap<String, MenuItems>>,
            input_type: Option<String>,
            input_identifier: Option<String>,
            function_name: Option<String>,
            router_name: Option<String>,
        }

        let raw_screen = RawUssdScreen::deserialize(deserializer)?;

        match raw_screen.screen_type.as_str() {
            "Initial" => Ok(UssdScreen::Initial {
                default_next_screen: raw_screen.default_next_screen,
            
            }),
            "Menu" => Ok(UssdScreen::Menu {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                menu_items: raw_screen.menu_items.unwrap_or_default(),
            }),
            "Input" => Ok(UssdScreen::Input {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                input_type: raw_screen.input_type.unwrap_or_default(),
                input_identifier: raw_screen.input_identifier.unwrap_or_default(),
            }),
            "Function" => Ok(UssdScreen::Function {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                function_name: raw_screen.function_name.unwrap_or_default(),
            }),
            "Router" => Ok(UssdScreen::Router {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                router_name: raw_screen.router_name.unwrap_or_default(),
            }),
            "Quit" => Ok(UssdScreen::Quit),
            _ => Err(serde::de::Error::custom("Unknown screen type")),
        }
    }
}



// Define a trait to represent actions that can be performed in a USSD session
pub trait UssdAction {
    fn validate_input(&self, input: &str) -> bool;
    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String>;
    fn display(&self);
}

// Implement the UssdAction trait for different screen types
impl UssdAction for UssdScreen {
    fn validate_input(&self, input: &str) -> bool {
        match self {
            UssdScreen::Initial { .. } |
            UssdScreen::Menu { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                // Validate input based on menu items
                input.chars().all(|c| c.is_digit(10))
            }
            UssdScreen::Input { .. } |
            UssdScreen::Function { .. } |
            UssdScreen::Router { .. } |
            UssdScreen::Quit => true, // No input to validate
        }
    }
    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String> {
        if !self.validate_input(input) {
            println!("Invalid input!");
            return None;
        }

        match self {
            UssdScreen::Initial { default_next_screen } => {
                // Handle initial screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Menu { title: _, default_next_screen, menu_items } => {
                // Handle menu input
                if let Some(next_screen) = menu_items.get(input) {
                    session.current_screen = next_screen.default_next_screen.clone();
                    Some(next_screen.default_next_screen.clone())
                } else {
                    session.current_screen = default_next_screen.clone();
                    Some(default_next_screen.clone())
                }
            }
            UssdScreen::Input { title: _, default_next_screen, input_type: _, input_identifier: _ } => {
                // Handle input screen
                // Process input and return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Function { title: _, default_next_screen, function_name: _ } => {
                // Call the corresponding function
                // Update session state based on function result
                // Return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Router { title: _, default_next_screen, router_name: _ } => {
                // Call the corresponding router
                // Update session state based on router result
                // Return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Quit => {
                // End the session
                None
            }
        }
    }

    fn display(&self) {
        match self {
            UssdScreen::Menu { title, menu_items, .. } => {
                println!("Title: {}", title);
                for (_, item) in menu_items.iter() {
                    println!("{}. {}", item.option, item.display_name);
                }
            }
            UssdScreen::Input { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for input screen
            }
            UssdScreen::Function { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for function screen
            }
            UssdScreen::Router { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for router screen
            }
            UssdScreen::Quit => {
                // No display needed for quit screen
            }
            _ => {}
        }
    }
}
