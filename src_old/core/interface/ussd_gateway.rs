use std::time::{Duration, SystemTime};

use crate::core::USSDSession;

use super::{USSDConfig, USSDMenu, USSDRequest, USSDResponse, USSDScreen, UssdAction};

pub struct USSDGateway {
    pub config: USSDConfig,
    pub menus: USSDMenu,
}

impl USSDGateway {
    pub fn new(config: USSDConfig, menus: USSDMenu) -> Self {
        Self { config, menus }
    }

    pub fn process_request(&self, request: USSDRequest) -> Option<USSDResponse> {
        let cache = &self.config.session_cache;

        let (initial_screen, _) = self.menus.get_initial_screen();

        let timeout_duration = Duration::from_secs(60);

        let mut session =
            USSDSession::get_or_create_session(&request, &initial_screen, timeout_duration, cache);

        if session.has_timed_out(timeout_duration) {
            session.restart(&initial_screen);
        }

        if session.current_screen == initial_screen {
            session.visited_screens.push(initial_screen.clone());
        }

        if let Some(screen) = self.menus.menus.get(&session.current_screen) {
            // Execute the screen
            match screen {
                USSDScreen::Function { .. } | USSDScreen::Router { .. } => {
                    screen.execute(&request, &mut session, &self.menus.services);
                    None
                }
                USSDScreen::Initial { default_next_screen, .. } => {
                    session.current_screen = default_next_screen.clone();
                    None
                }
                _ => {
                    // For other screens, return the response
                    let response = screen.execute(&request, &mut session, &self.menus.services);
                    session.last_interaction_time = SystemTime::now();
                    response
                }
            }
        } else {
            None // Invalid screen
        }
    }
}

// impl USSDGateway {
//     pub fn new(config: USSDConfig) -> Self {
//         Self {
//             functions_path: config.functions_path,
//             menu_source: config.menu_source,
//         }
//     }

//     pub fn initial(&self) {
//         // Load menu from JSON, YAML, or database
//         let menu = USSDMenu::load_from_json(&self.menu_source).unwrap();

//         // Define session timeout duration
//         let timeout_duration = Duration::from_secs(60);

//         pub struct RedisCache {
//             // Redis connection or any other configuration needed
//         }

//         impl SessionCache for RedisCache {
//             fn store_session(&self, session: &USSDSession) -> Result<(), String> {
//                 // Store session in Redis
//                 Ok(())
//             }

//             fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
//                 // Retrieve session from Redis
//                 Ok(None)
//             }
//         }

//         let cache = RedisCache {};

//         // Create USSDRequest instance
//         let mut ussd_request =
//             USSDRequest::new("session_id_123".to_string(), menu, timeout_duration, cache);

//         loop {
//             if ussd_request.session.current_screen == "InitialScreen" {
//                 if let Some(next_screen) = self.process_screen(&mut ussd_request) {
//                     if next_screen == "Quit" {
//                         println!("Session ended.");
//                         break;
//                     }
//                 } else {
//                     println!("Invalid input or screen!");
//                 }
//                 continue;
//             }

//             if let Some(screen) = ussd_request
//                 .menu
//                 .menus
//                 .get(&ussd_request.session.current_screen.clone())
//             {
//                 if let USSDScreen::Function { .. } | USSDScreen::Router { .. } = screen {
//                     if let Some(next_screen) = self.process_screen(&mut ussd_request) {
//                         if next_screen == "Quit" {
//                             println!("Session ended.");
//                             break;
//                         }
//                     } else {
//                         println!("Invalid input or screen!");
//                     }
//                     continue;
//                 }

//                 self.handle_response(&screen);
//                 if let Some(input) = self.read_user_input() {
//                     if let Some(next_screen) = self.process_input(&mut ussd_request, input) {
//                         if next_screen == "Quit" {
//                             println!("Session ended.");
//                             break;
//                         }
//                     } else {
//                         println!("Invalid input or screen!");
//                     }
//                 } else {
//                     println!("Failed to read input!");
//                 }
//             } else {
//                 println!("Invalid screen!");
//                 break;
//             }
//         }
//     }

//     fn process_screen(&self, ussd_request: &mut USSDRequest) -> Option<String> {
//         ussd_request.handle_ussd_request("")
//     }

//     fn handle_response(&self, screen: &USSDScreen) {
//         screen.display();
//     }

//     fn read_user_input(&self) -> Option<String> {
//         print!("Enter your choice: ");
//         io::stdout().flush().unwrap();
//         let mut input = String::new();
//         if let Ok(_) = io::stdin().read_line(&mut input) {
//             Some(input.trim().to_string())
//         } else {
//             None
//         }
//     }

//     fn process_input(&self, ussd_request: &mut USSDRequest, input: String) -> Option<String> {
//         ussd_request.handle_ussd_request(&input)
//     }
// }
