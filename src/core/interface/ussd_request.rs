use std::{
    collections::HashMap,
    time::{Duration, SystemTime},
};

use crate::helper::stack::Stack;

use super::{ussd_menu::UssdMenu, ussd_screen::UssdAction, ussd_session::UssdSession};

// Define the USSDRequest struct
pub struct USSDRequest {
    pub session: UssdSession,
    pub menu: UssdMenu,
    pub timeout_duration: Duration,
}

impl USSDRequest {
    // Create a new USSDRequest
    pub fn new(session_id: String, menu: UssdMenu, timeout_duration: Duration) -> Self {
        let (start_screen, _) = menu.get_initial_screen();

        USSDRequest {
            session: UssdSession {
                session_id,
                data: HashMap::new(),
                current_screen: start_screen,
                visited_screens: Stack::new(),
                last_interaction_time: SystemTime::now(),
                end_session: false,
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

        self.session.display_screen_history();

        if let Some(screen) = self.menu.menus.get(&self.session.current_screen) {
            let next_screen = screen.execute(&mut self.session, input);
            self.session.last_interaction_time = SystemTime::now();
            next_screen
        } else {
            None // Invalid screen
        }
    }
}
