pub mod function;
pub mod input;
pub mod menu;
pub mod quit;
pub mod router;

use std::collections::HashMap;

use crate::info;

use super::{USSDRequest, USSDResponse, USSDScreen, USSDService, USSDSession, UssdAction};

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

    fn execute(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        services: &HashMap<String, USSDService>,
    ) -> Option<USSDResponse> {
        let input = &request.input.clone();

        let default_message = "Dear customer, an error occurred. Please try again later.";

        let mut response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message: default_message.to_string(),
        };

        // logging
        info!(
            "\nCurrent screen:\n    {:?} \n\nInput received:\n    {:?} \n",
            self, input
        );

        // validate input
        if !self.validate_input(input) {
            println!("Invalid input!");
        }

        // if input is 0, go back
        if input == "0" {
            self.back(session);
        }

        // if input is 00, go home
        if input == "00" {
            self.home(session);
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
                None
            }
            USSDScreen::Menu {
                text,
                default_next_screen,
                menu_items,
            } => {
                menu_handler(session, input, menu_items, default_next_screen);

                let mut message = text.to_string();
                for (idx, (_, value)) in menu_items.iter().enumerate() {
                    message.push_str(&format!("\n{}: {}", idx, value.display_name));
                }

                response.message = message;
                Some(response)
            }
            USSDScreen::Input {
                text,
                default_next_screen,
                input_type,
                input_identifier,
            } => {
                input_handler(
                    session,
                    input,
                    input_type,
                    input_identifier,
                    default_next_screen,
                );
                response.message = text.to_string();
                Some(response)
            }
            USSDScreen::Function {
                text,
                default_next_screen,
                function,
            } => {
                function_handler(session, function, services, default_next_screen);
                response.message = text.to_string();
                Some(response)
            }
            USSDScreen::Router {
                text,
                default_next_screen,
                router: _,
                router_options,
            } => {
                router_handler(session, router_options, default_next_screen);
                response.message = text.to_string();
                Some(response)
            }
            USSDScreen::Quit {
                text,
                default_next_screen,
            } => {
                quit_handler(session, input, default_next_screen);
                response.message = text.to_string();
                Some(response)
            }
        }
    }

    fn display(&self) {
        match self {
            USSDScreen::Menu {
                text, menu_items, ..
            } => {
                println!("\n{}", text);
                for (index, (_, value)) in menu_items.iter().enumerate() {
                    let option_idx = index + 1;
                    println!("{}. {} \n", option_idx, value.display_name);
                }
            }
            USSDScreen::Input { text, .. } => {
                println!("\n{} \n", text);
                // Additional display logic for input screen
            }
            USSDScreen::Quit { text, .. } => {
                println!("\n{} \n", text);
            }
            _ => {}
        }
    }
}
