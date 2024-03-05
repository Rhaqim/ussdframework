pub mod function;
pub mod input;
pub mod menu;
pub mod quit;
pub mod router;

use std::collections::HashMap;

use crate::info;

use super::{USSDService, UssdAction, USSDScreen, USSDSession};

use function::function_handler;
use input::input_handler;
use menu::menu_handler;
use quit::quit_handler;
use router::router_handler;

// Implement the UssdAction trait for different screen types
impl UssdAction for USSDScreen {
    fn validate_input(&self, input: &str) -> bool {
        match self {
            USSDScreen::Initial { .. } | USSDScreen::Menu { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                // Validate input based on menu items
                input.chars().all(|c| c.is_digit(10))
            }
            USSDScreen::Input { .. }
            | USSDScreen::Function { .. }
            | USSDScreen::Router { .. }
            | USSDScreen::Quit { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                input.chars().all(|c| c.is_digit(10))
            }
        }
    }

    fn back(&self, session: &mut USSDSession) {
        // switch to the previous screen
        if let Some(prev_screen) = session.visited_screens.pop() {
            session.current_screen = prev_screen;
        }
    }

    fn home(&self, session: &mut USSDSession) {
        // Switch to the initial screen
        session.current_screen = "InitialScreen".to_string();
    }

    fn execute(&self, session: &mut USSDSession, input: &str, services: &HashMap<String, USSDService>) -> Option<String> {
        // logging
        info!(
            "\nCurrent screen:\n    {:?} \n\nInput received:\n    {:?} \n",
            self, input
        );

        // validate input
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

        // display screen history
        session.display_screen_history();

        match self {
            USSDScreen::Initial {
                default_next_screen,
            } => {
                // Handle initial screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            USSDScreen::Menu {
                title: _,
                default_next_screen,
                menu_items,
            } => menu_handler(session, input, menu_items, default_next_screen),
            USSDScreen::Input {
                title: _,
                default_next_screen,
                input_type,
                input_identifier,
            } => input_handler(
                session,
                input,
                input_type,
                input_identifier,
                default_next_screen,
            ),
            USSDScreen::Function {
                title: _,
                default_next_screen,
                function,
            } => function_handler(session, function, services, default_next_screen),
            USSDScreen::Router {
                title: _,
                default_next_screen,
                router: _,
                router_options,
            } => router_handler(session, router_options, default_next_screen),
            USSDScreen::Quit {
                title: _,
                default_next_screen,
            } => {
                quit_handler(session, input, default_next_screen);
                Some(default_next_screen.clone())
            }
        }
    }

    fn display(&self) {
        match self {
            USSDScreen::Menu {
                title, menu_items, ..
            } => {
                println!("\n{} \n", title);
                for (index, (_, value)) in menu_items.iter().enumerate() {
                    let option_idx = index + 1;
                    println!("{}. {} \n", option_idx, value.display_name);
                }
            }
            USSDScreen::Input { title, .. } => {
                println!("\n{} \n", title);
                // Additional display logic for input screen
            }
            USSDScreen::Quit { title, .. } => {
                println!("\n{} \n", title);
            }
            _ => {}
        }
    }
}
