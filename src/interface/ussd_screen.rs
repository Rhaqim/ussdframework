use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};

use crate::info;

use super::ussd_session::UssdSession;

#[derive(Debug, Serialize)]
pub struct MenuItems {
    pub option: String,
    pub display_name: String,
    pub default_next_screen: String,
}

impl<'de> Deserialize<'de> for MenuItems {
    fn deserialize<D>(deserializer: D) -> Result<MenuItems, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawMenuItems {
            option: String,
            display_name: String,
            default_next_screen: String,
        }

        let raw_menu_items = RawMenuItems::deserialize(deserializer)?;

        Ok(MenuItems {
            option: raw_menu_items.option,
            display_name: raw_menu_items.display_name,
            default_next_screen: raw_menu_items.default_next_screen,
        })
    }
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
    fn back(&self, session: &mut UssdSession);
    fn home(&self, session: &mut UssdSession);
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

    fn back(&self, session: &mut UssdSession) {
        // switch to the previous screen
        if let Some(prev_screen) = session.visited_screens.pop() {
            session.current_screen = prev_screen;
        }
    }

    fn home(&self, session: &mut UssdSession) {
        // Switch to the initial screen
        session.current_screen = "InitialScreen".to_string();
    }

    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String> {
        // logging
        info!("\nCurrent screen:\n    {:?} \n\nInput received:\n    {:?} \n", self, input);

        if !self.validate_input(input) {
            println!("Invalid input!");
            return None;
        }

        // if input is 0, go back
        if input == "0" {
            self.back(session);
            return Some(session.current_screen.clone());
        }

        // if input is 00, go home
        if input == "00" {
            self.home(session);
            return Some(session.current_screen.clone());
        }

        // track visited screens
        session.visited_screens.push(session.current_screen.clone());

        match self {
            UssdScreen::Initial { default_next_screen } => {
                // Handle initial screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Menu { title: _, default_next_screen, menu_items } => {
                // iterate over the menu_items and give each item an index within the bounds of the menu_items_len
                let menu_items_len = menu_items.len();

                // if input is within the bounds of the menu_items_len, return the next screen
                if let Ok(input) = input.parse::<usize>() {
                    if input > 0 && input <= menu_items_len {
                        let next_screen = menu_items.values().nth(input - 1).unwrap().default_next_screen.clone();
                        session.current_screen = next_screen.clone();
                        return Some(next_screen);
                    }
                }

                // if input is not within the bounds of the menu_items_len, return the current screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
                
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
                println!("Title: {} \n", title);
                for (index, (_, value)) in menu_items.iter().enumerate() {
                    let option_idx = index + 1;
                    println!("{}. {} \n", option_idx, value.display_name);
                }
            }
            UssdScreen::Input { title, .. } => {
                println!("Title: {} \n", title);
                // Additional display logic for input screen
            }
            UssdScreen::Function { title, .. } => {
                println!("Title: {} \n", title);
                // Additional display logic for function screen
            }
            UssdScreen::Router { title, .. } => {
                println!("Title: {} \n", title);
                // Additional display logic for router screen
            }
            UssdScreen::Quit => {
                // No display needed for quit screen
            }
            _ => {}
        }
    }
}
