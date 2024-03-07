use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, Duration};


// Define a structure to represent a USSD session
#[derive(Debug, Deserialize, Serialize)]
pub struct USSDSession {
    pub session_id: String,
    pub current_screen: String,
    pub last_interaction_time: SystemTime,
    // Add any other session-related data here
}

impl USSDSession {
    // Check if session has timed out
    fn has_timed_out(&self, timeout_duration: Duration) -> bool {
        self.last_interaction_time.elapsed().unwrap_or_default() > timeout_duration
    }

    // Restart the session
    fn restart(&mut self, initial_screen: &str) {
        self.current_screen = initial_screen.to_string();
        self.last_interaction_time = SystemTime::now();
        // Reset any other session-related data as needed
    }
}

// Define the request and response structures
#[derive(Debug, Deserialize, Serialize)]
pub struct UssdRequest {
    pub session_id: String,
    pub input: String,
    pub msisdn: String,
    pub default_language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDResponse {
    pub session_id: String,
    pub message: String,
}

// Define an enum to represent different types of USSD screens
#[derive(Debug, Serialize)]
pub enum USSDScreen {
    Initial {
        title: String,
        default_next_screen: String,
    },
    Menu {
        title: String,
        default_next_screen: String,
        menu_items: HashMap<String, String>,
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

// Define a structure to hold the USSD menu data
#[derive(Debug, Deserialize, Serialize)]
pub struct USSDMenu {
    pub menus: HashMap<String, USSDScreen>,
}

impl USSDMenu {
    // Load menu structure from JSON file
    pub fn load_from_json(_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // let mut file = File::open(file_path)?;
        // let mut contents = String::new();
        // file.read_to_string(&mut contents)?;
        let contents = include_str!("../data/screens.json");
        let menu: USSDMenu = serde_json::from_str(contents)?;
        Ok(menu)
    }

    // Save menu structure to JSON file
    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
}

impl<'de> Deserialize<'de> for USSDScreen {
    fn deserialize<D>(deserializer: D) -> Result<USSDScreen, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawUSSDScreen {
            #[serde(rename = "type")]
            screen_type: String,
            // Other fields common to all screen types
            title: String,
            default_next_screen: String,
            // Fields specific to certain screen types
            menu_items: Option<HashMap<String, String>>,
            input_type: Option<String>,
            input_identifier: Option<String>,
            function_name: Option<String>,
            router_name: Option<String>,
        }

        let raw_screen = RawUSSDScreen::deserialize(deserializer)?;

        match raw_screen.screen_type.as_str() {
            "Initial" => Ok(USSDScreen::Initial {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
            
            }),
            "Menu" => Ok(USSDScreen::Menu {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                menu_items: raw_screen.menu_items.unwrap_or_default(),
            }),
            "Input" => Ok(USSDScreen::Input {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                input_type: raw_screen.input_type.unwrap_or_default(),
                input_identifier: raw_screen.input_identifier.unwrap_or_default(),
            }),
            "Function" => Ok(USSDScreen::Function {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                function_name: raw_screen.function_name.unwrap_or_default(),
            }),
            "Router" => Ok(USSDScreen::Router {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                router_name: raw_screen.router_name.unwrap_or_default(),
            }),
            "Quit" => Ok(USSDScreen::Quit),
            _ => Err(serde::de::Error::custom("Unknown screen type")),
        }
    }
}


// Define the USSDRequest struct
pub struct USSDRequest {
    pub session: USSDSession,
    pub menu: USSDMenu,
    pub timeout_duration: Duration,
}

impl USSDRequest {
    // Create a new USSDRequest
    pub fn new(session_id: String, initial_screen: String, menu: USSDMenu, timeout_duration: Duration) -> Self {
        USSDRequest {
            session: USSDSession {
                session_id,
                current_screen: initial_screen,
                last_interaction_time: SystemTime::now(),
            },
            menu,
            timeout_duration,
        }
    }

    // Handle USSD request
    pub fn handle_ussd_request(&mut self, input: &str) -> Option<String> {
        if self.session.has_timed_out(self.timeout_duration) {
            self.session.restart("InitialScreen");
        }

        if let Some(screen) = self.menu.menus.get(&self.session.current_screen) {
            let next_screen = screen.execute(&mut self.session, input);
            self.session.last_interaction_time = SystemTime::now();
            next_screen
        } else {
            None // Invalid screen
        }
    }
}


// Define a trait to represent actions that can be performed in a USSD session
pub trait UssdAction {
    fn validate_input(&self, input: &str) -> bool;
    fn execute(&self, session: &mut USSDSession, input: &str) -> Option<String>;
    fn display(&self);
}

// Implement the UssdAction trait for different screen types
impl UssdAction for USSDScreen {
    fn validate_input(&self, input: &str) -> bool {
        match self {
            USSDScreen::Initial { .. } |
            USSDScreen::Menu { .. } |
            USSDScreen::Input { .. } |
            USSDScreen::Function { .. } |
            USSDScreen::Router { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                input.chars().all(|c| c.is_digit(10))
            }
            USSDScreen::Quit => true, // No input to validate
        }
    }
    fn execute(&self, session: &mut USSDSession, input: &str) -> Option<String> {
        if !self.validate_input(input) {
            println!("Invalid input!");
            return None;
        }

        match self {
            USSDScreen::Initial { title: _, default_next_screen } => {
                // Handle initial screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            USSDScreen::Menu { title: _, default_next_screen, menu_items } => {
                // Handle menu input
                if let Some(next_screen) = menu_items.get(input) {
                    session.current_screen = next_screen.clone();
                    Some(next_screen.clone())
                } else {
                    session.current_screen = default_next_screen.clone();
                    Some(default_next_screen.clone())
                }
            }
            USSDScreen::Input { title: _, default_next_screen, input_type: _, input_identifier: _ } => {
                // Handle input screen
                // Process input and return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            USSDScreen::Function { title: _, default_next_screen, function_name: _ } => {
                // Call the corresponding function
                // Update session state based on function result
                // Return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            USSDScreen::Router { title: _, default_next_screen, router_name: _ } => {
                // Call the corresponding router
                // Update session state based on router result
                // Return the next screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            USSDScreen::Quit => {
                // End the session
                None
            }
        }
    }

    fn display(&self) {
        match self {
            USSDScreen::Menu { title, menu_items, .. } => {
                println!("Title: {}", title);
                for (index, item) in menu_items.iter() {
                    println!("{}. {}", index, item);
                }
            }
            USSDScreen::Input { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for input screen
            }
            USSDScreen::Function { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for function screen
            }
            USSDScreen::Router { title, .. } => {
                println!("Title: {}", title);
                // Additional display logic for router screen
            }
            USSDScreen::Quit => {
                // No display needed for quit screen
            }
            _ => {}
        }
    }
}
