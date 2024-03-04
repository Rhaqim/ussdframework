
extern crate tokio;

mod core;
mod helper;
mod interface;
mod log;
mod model;
mod types;

use interface::{USSDRequest, UssdMenu, UssdAction};
use std::{io::{self, prelude::*}, time::Duration};

#[tokio::main]
async fn main() {
    // Load menu from JSON, YAML, or database
    let menu = UssdMenu::load_from_json("src/data/menu.json").unwrap();

    // Define session timeout duration
    let timeout_duration = Duration::from_secs(60); // Example: 60 seconds

    // Create USSDRequest instance
    let mut ussd_request = USSDRequest::new("session_id_123".to_string(), "InitialScreen".to_string(), menu, timeout_duration);

    loop {
        // if screen is initial, execute the initial screen
        if ussd_request.session.current_screen == "InitialScreen" {
            if let Some(next_screen) = ussd_request.handle_ussd_request("") {
                if next_screen == "Quit" {
                    println!("Session ended.");
                    break;
                }
            } else {
                println!("Invalid input or screen!");
            }
            continue;
        }

        let current_screen = ussd_request.session.current_screen.clone();
        if let Some(screen) = ussd_request.menu.menus.get(&current_screen) {

            screen.display();
            
            print!("Enter your choice: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");
            let input = input.trim();
            if let Some(next_screen) = ussd_request.handle_ussd_request(input) {
                if next_screen == "Quit" {
                    println!("Session ended.");
                    break;
                }
            } else {
                println!("Invalid input or screen!");
            }
        } else {
            println!("Invalid screen!");
            break;
        }
    }
}