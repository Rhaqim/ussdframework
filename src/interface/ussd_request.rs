use std::time::{Duration, SystemTime};

use super::{ussd_menu::UssdMenu, ussd_session::UssdSession, ussd_screen::UssdAction};

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