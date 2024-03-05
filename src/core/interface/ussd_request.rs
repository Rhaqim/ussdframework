use std::time::{Duration, SystemTime};

use super::{SessionCache, USSDMenu, USSDSession, UssdAction};

// Define the USSDRequest struct
pub struct USSDRequest {
    pub session: USSDSession,
    pub menu: USSDMenu,
    pub timeout_duration: Duration,
}

pub struct RedisCache {
    // Redis connection or any other configuration needed
}

impl SessionCache for RedisCache {
    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        // Store session in Redis
        Ok(())
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        // Retrieve session from Redis
        Ok(None)
    }
}

impl USSDRequest {
    // Create a new USSDRequest
    pub fn new(session_id: String, menu: USSDMenu, timeout_duration: Duration) -> Self {
        let (initial_screen, _) = menu.get_initial_screen();

        let cache = RedisCache {};

        let session = USSDSession::get_or_create_session(
            &session_id,
            &initial_screen,
            timeout_duration,
            &cache,
        );

        USSDRequest {
            session,
            menu,
            timeout_duration,
        }
    }

    // Handle USSD request
    pub fn handle_ussd_request(&mut self, input: &str) -> Option<String> {
        let (initial_screen, _) = self.menu.get_initial_screen();
        if self.session.has_timed_out(self.timeout_duration) {
            self.session.restart(&initial_screen);
        }

        if let Some(screen) = self.menu.menus.get(&self.session.current_screen) {
            let next_screen = screen.execute(&mut self.session, input, &self.menu.services);
            self.session.last_interaction_time = SystemTime::now();
            next_screen
        } else {
            None // Invalid screen
        }
    }
}
