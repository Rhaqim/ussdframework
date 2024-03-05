pub mod function;
pub mod input;
pub mod menu;
pub mod quit;
pub mod router;

use crate::info;

use super::{UssdAction, UssdScreen, UssdSession};

use function::function_handler;
use input::input_handler;
use menu::menu_handler;
use quit::quit_handler;
use router::router_handler;

// Implement the UssdAction trait for different screen types
impl UssdAction for UssdScreen {
    fn validate_input(&self, input: &str) -> bool {
        match self {
            UssdScreen::Initial { .. } | UssdScreen::Menu { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                // Validate input based on menu items
                input.chars().all(|c| c.is_digit(10))
            }
            UssdScreen::Input { .. }
            | UssdScreen::Function { .. }
            | UssdScreen::Router { .. }
            | UssdScreen::Quit { .. } => {
                // Perform input validation logic here
                // For example, check if input is numeric
                input.chars().all(|c| c.is_digit(10))
            }
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
        info!(
            "\nCurrent screen:\n    {:?} \n\nInput received:\n    {:?} \n",
            self, input
        );

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
            UssdScreen::Initial {
                default_next_screen,
            } => {
                // Handle initial screen
                session.current_screen = default_next_screen.clone();
                Some(default_next_screen.clone())
            }
            UssdScreen::Menu {
                title: _,
                default_next_screen,
                menu_items,
            } => menu_handler(session, input, menu_items, default_next_screen),
            UssdScreen::Input {
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
            UssdScreen::Function {
                title: _,
                default_next_screen,
                function_name,
            } => function_handler(session, function_name, default_next_screen),
            UssdScreen::Router {
                title: _,
                default_next_screen,
                router_name: _,
                router_options,
            } => router_handler(session, input, router_options, default_next_screen),
            UssdScreen::Quit {
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
            UssdScreen::Menu {
                title, menu_items, ..
            } => {
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
            UssdScreen::Function { .. } => {
                // println!("Title: {} \n", title);
                // Additional display logic for function screen
            }
            UssdScreen::Router { .. } => {
                // println!("Title: {} \n", title);
                // Additional display logic for router screen
            }
            UssdScreen::Quit { title, .. } => {
                println!("Title: {} \n", title);
            }
            _ => {}
        }
    }
}
