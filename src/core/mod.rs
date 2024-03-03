use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::time::{SystemTime, Duration};


// Define a structure to represent a USSD session
#[derive(Debug, Deserialize, Serialize)]
pub struct UssdSession {
    pub session_id: String,
    pub current_screen: String,
    pub last_interaction_time: SystemTime,
    // Add any other session-related data here
}

impl UssdSession {
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
pub struct UssdResponse {
    pub session_id: String,
    pub message: String,
}

// Define an enum to represent different types of USSD screens
#[derive(Debug, Deserialize, Serialize)]
pub enum UssdScreen {
    Initial {
        title: String,
        defualt_next_screen: String,
    },
    Menu {
        title: String,
        defualt_next_screen: String,
        menu_items: HashMap<String, String>,
    },
    Input {
        title: String,
        defualt_next_screen: String,
        input_type: String,
        input_identifier: String,
    },
    Function {
        title: String,
        defualt_next_screen: String,
        function_name: String,
    },
    Router {
        title: String,
        defualt_next_screen: String,
        router_name: String,
    },
    Quit,
}

// Define a structure to hold the USSD menu data
#[derive(Debug, Deserialize, Serialize)]
pub struct UssdMenu {
    pub menus: HashMap<String, UssdScreen>,
}

impl UssdMenu {
    // Load menu structure from JSON file
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let menu: UssdMenu = serde_json::from_str(&contents)?;
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

// Define the USSDRequest struct
pub struct USSDRequest {
    pub session: UssdSession,
    pub menu: UssdMenu,
    pub timeout_duration: Duration,
}

impl USSDRequest {
    // Create a new USSDRequest
    pub fn new(session_id: String, initial_screen: String, menu: UssdMenu, timeout_duration: Duration) -> Self {
        USSDRequest {
            session: UssdSession {
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
    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String>;
    fn display(&self);
}

// Implement the UssdAction trait for different screen types
impl UssdAction for UssdScreen {
    fn validate_input(&self, input: &str) -> bool {
        match self {
            UssdScreen::Initial { .. } |
            UssdScreen::Menu { .. } |
            UssdScreen::Input { .. } |
            UssdScreen::Function { .. } |
            UssdScreen::Router { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                input.chars().all(|c| c.is_digit(10))
            }
            UssdScreen::Quit => true, // No input to validate
        }
    }
    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String> {
        if !self.validate_input(input) {
            println!("Invalid input!");
            return None;
        }

        match self {
            UssdScreen::Initial { title: _, defualt_next_screen } => {
                // Handle initial screen
                session.current_screen = defualt_next_screen.clone();
                Some(defualt_next_screen.clone())
            }
            UssdScreen::Menu { title: _, defualt_next_screen, menu_items } => {
                // Handle menu input
                if let Some(next_screen) = menu_items.get(input) {
                    session.current_screen = next_screen.clone();
                    Some(next_screen.clone())
                } else {
                    session.current_screen = defualt_next_screen.clone();
                    Some(defualt_next_screen.clone())
                }
            }
            UssdScreen::Input { title: _, defualt_next_screen, input_type: _, input_identifier: _ } => {
                // Handle input screen
                // Process input and return the next screen
                session.current_screen = defualt_next_screen.clone();
                Some(defualt_next_screen.clone())
            }
            UssdScreen::Function { title: _, defualt_next_screen, function_name: _ } => {
                // Call the corresponding function
                // Update session state based on function result
                // Return the next screen
                session.current_screen = defualt_next_screen.clone();
                Some(defualt_next_screen.clone())
            }
            UssdScreen::Router { title: _, defualt_next_screen, router_name: _ } => {
                // Call the corresponding router
                // Update session state based on router result
                // Return the next screen
                session.current_screen = defualt_next_screen.clone();
                Some(defualt_next_screen.clone())
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
                for (index, item) in menu_items.iter() {
                    println!("{}. {}", index, item);
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
