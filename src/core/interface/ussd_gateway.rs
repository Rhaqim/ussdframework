use std::{
    io::{self, prelude::*},
    time::Duration,
};

use super::{USSDConfig, USSDMenu, USSDRequest, USSDScreen, UssdAction};

#[derive(Debug)]
pub struct USSDGateway {
    pub functions_path: String,
    pub menu_source: String,
}

impl USSDGateway {
    pub fn new(config: USSDConfig) -> Self {
        Self {
            functions_path: config.functions_path,
            menu_source: config.menu_source,
        }
    }

    pub fn initial(&self) {
        // Load menu from JSON, YAML, or database
        let menu = USSDMenu::load_from_json(&self.menu_source).unwrap();

        // Define session timeout duration
        let timeout_duration = Duration::from_secs(60);

        // Create USSDRequest instance
        let mut ussd_request =
            USSDRequest::new("session_id_123".to_string(), menu, timeout_duration);

        loop {
            if ussd_request.session.current_screen == "InitialScreen" {
                if let Some(next_screen) = self.process_screen(&mut ussd_request) {
                    if next_screen == "Quit" {
                        println!("Session ended.");
                        break;
                    }
                } else {
                    println!("Invalid input or screen!");
                }
                continue;
            }

            if let Some(screen) = ussd_request
                .menu
                .menus
                .get(&ussd_request.session.current_screen.clone())
            {
                if let USSDScreen::Function { .. } | USSDScreen::Router { .. } = screen {
                    if let Some(next_screen) = self.process_screen(&mut ussd_request) {
                        if next_screen == "Quit" {
                            println!("Session ended.");
                            break;
                        }
                    } else {
                        println!("Invalid input or screen!");
                    }
                    continue;
                }

                self.handle_response(&screen);
                if let Some(input) = self.read_user_input() {
                    if let Some(next_screen) = self.process_input(&mut ussd_request, input) {
                        if next_screen == "Quit" {
                            println!("Session ended.");
                            break;
                        }
                    } else {
                        println!("Invalid input or screen!");
                    }
                } else {
                    println!("Failed to read input!");
                }
            } else {
                println!("Invalid screen!");
                break;
            }
        }
    }

    fn process_screen(&self, ussd_request: &mut USSDRequest) -> Option<String> {
        ussd_request.handle_ussd_request("")
    }

    fn handle_response(&self, screen: &USSDScreen) {
        screen.display();
    }

    fn read_user_input(&self) -> Option<String> {
        print!("Enter your choice: ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        if let Ok(_) = io::stdin().read_line(&mut input) {
            Some(input.trim().to_string())
        } else {
            None
        }
    }

    fn process_input(&self, ussd_request: &mut USSDRequest, input: String) -> Option<String> {
        ussd_request.handle_ussd_request(&input)
    }

    // pub fn initial(&self) {
    //     // Load menu from JSON, YAML, or database
    //     let menu = USSDMenu::load_from_json(&self.menu_source).unwrap();

    //     // Define session timeout duration
    //     let timeout_duration = Duration::from_secs(60); // Example: 60 seconds

    //     // Create USSDRequest instance
    //     let mut ussd_request = USSDRequest::new("session_id_123".to_string(), menu, timeout_duration);

    //     loop {
    //         // if screen is initial, execute the initial screen
    //         if ussd_request.session.current_screen == "InitialScreen" {
    //             if let Some(next_screen) = ussd_request.handle_ussd_request("") {
    //                 if next_screen == "Quit" {
    //                     println!("Session ended.");
    //                     break;
    //                 }
    //             } else {
    //                 println!("Invalid input or screen!");
    //             }
    //             continue;
    //         }

    //         let current_screen = ussd_request.session.current_screen.clone();
    //         if let Some(screen) = ussd_request.menu.menus.get(&current_screen) {

    //             // if the current screen is either a function or a router, execute the screen
    //             if let USSDScreen::Function { .. } | USSDScreen::Router { .. } = screen {
    //                 if let Some(next_screen) = ussd_request.handle_ussd_request("") {
    //                     if next_screen == "Quit" {
    //                         println!("Session ended.");
    //                         break;
    //                     }
    //                 } else {
    //                     println!("Invalid input or screen!");
    //                 }
    //                 continue;
    //             }

    //             screen.display();

    //             print!("Enter your choice: ");
    //             io::stdout().flush().unwrap();
    //             let mut input = String::new();
    //             io::stdin()
    //                 .read_line(&mut input)
    //                 .expect("Failed to read line");
    //             let input = input.trim();
    //             if let Some(next_screen) = ussd_request.handle_ussd_request(input) {
    //                 if next_screen == "Quit" {
    //                     println!("Session ended.");
    //                     break;
    //                 }
    //             } else {
    //                 println!("Invalid input or screen!");
    //             }
    //         } else {
    //             println!("Invalid screen!");
    //             break;
    //         }
    //     }
    // }
}
