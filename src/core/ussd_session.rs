use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use crate::{info, types::HashStrAny};

use super::USSDRequest;

/// Structure for USSD session
/// The session stores the current screen, session data, and other session-related information.
/// It is used to maintain the state of the USSD session.
/// It can be stored and retrieved from a cache.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct USSDSession {
    pub session_id: String,
    pub data: HashMap<String, HashStrAny>,
    pub current_screen: String,
    pub displayed: HashMap<String, bool>,
    pub visited_screens: Vec<String>,
    pub last_interaction_time: SystemTime,
    pub end_session: bool,
    pub language: String,
    pub msisdn: String,
}

impl USSDSession {
    pub fn new(
        session_id: String,
        current_screen: String,
        language: String,
        msisdn: String,
    ) -> Self {
        USSDSession {
            session_id,
            data: HashMap::new(),
            current_screen,
            displayed: HashMap::new(),
            visited_screens: Vec::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language,
            msisdn,
        }
    }

    // Check if session has timed out
    pub fn has_timed_out(&self, timeout_duration: Duration) -> bool {
        self.last_interaction_time.elapsed().unwrap_or_default() > timeout_duration
    }

    // Update the session's last interaction time
    pub fn update_last_interaction_time(&mut self) {
        self.last_interaction_time = SystemTime::now();
    }

    // Restart the session
    pub fn restart(&mut self, initial_screen: &str) {
        // clear visited screens
        self.visited_screens.clear();
        self.current_screen = initial_screen.to_string();
        self.update_last_interaction_time();
        // Reset any other session-related data as needed
    }

    // Display screen history with an arrow pointing to the current screen
    pub fn display_screen_history(&self) {
        for screen in self.visited_screens.iter() {
            print!("{} \u{25B6} ", screen);
        }
    }

    // Store session
    pub fn store_session(&self, cache: &Box<dyn SessionCache>) -> Result<(), String> {
        cache.store_session(&self)
    }

    // Retrieve session
    pub fn retrieve_session(
        session_id: &str,
        cache: &Box<dyn SessionCache>,
    ) -> Result<Self, String> {
        let session = cache.retrieve_session(session_id);

        match session {
            Ok(Some(session)) => Ok(session),
            Ok(None) => Err("Session not found".to_string()),
            Err(e) => Err(e),
        }
    }

    /// Get or create a session
    pub fn get_or_create_session(
        request: &USSDRequest,
        initial_screen: &str,
        cache: &Box<dyn SessionCache>,
    ) -> Self {
        let retrieved_session = USSDSession::retrieve_session(&request.session_id, &cache);

        match retrieved_session {
            Ok(sesh) => {
                // Update last interaction time for existing session
                info!("Retrieved session {:?}", sesh);

                let mut session = sesh;
                session.update_last_interaction_time();
                session
            }
            Err(_) => {
                // Create new session
                let new_session = USSDSession {
                    session_id: request.session_id.clone(),
                    data: HashMap::new(),
                    current_screen: initial_screen.to_string(),
                    displayed: HashMap::new(),
                    visited_screens: Vec::new(),
                    last_interaction_time: SystemTime::now(),
                    end_session: false,
                    language: request.language.clone(),
                    msisdn: request.msisdn.clone(),
                };

                info!("New session {:?}", new_session);

                new_session.store_session(&cache).unwrap();
                new_session
            }
        }
    }

    /// Update the session with the current screen and last interaction time
    pub fn update_session(&mut self, session_cache: &Box<dyn SessionCache>) {
        // Store the current screen in the session's visited screens
        self.visited_screens.push(self.current_screen.clone());

        // Update the session's last interaction time
        self.update_last_interaction_time();

        // Store the session
        self.store_session(&session_cache).unwrap();
    }

    /// Fetches an item from the session data based on the given key.
    /// Returns None if the key does not exist in the session data.
    pub fn fetch_session_data<'a>(&'a self, key: &str) -> Option<&'a HashStrAny> {
        self.data.get(key)
    }
}

/// Implement this trait for storing and retrieving USSD sessions
/// from a cache.
/// The cache can be an in-memory store, a database, or any other
/// storage mechanism.
/// The cache should be thread-safe and allow for concurrent access.
/// The cache should also be able to store and retrieve USSD sessions.
/// The cache should be able to store and retrieve USSD sessions.
pub trait SessionCache: Send + Sync {
    fn store_session(&self, session: &USSDSession) -> Result<(), String>;
    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String>;
}

pub struct InMemorySessionStore {
    data: Mutex<HashMap<String, String>>,
}

// unsafe impl Send for InMemorySessionStore {}
// unsafe impl Sync for InMemorySessionStore {}

impl InMemorySessionStore {
    pub fn new() -> Self {
        Self {
            data: Mutex::new(HashMap::new()),
        }
    }
}

impl SessionCache for InMemorySessionStore {
    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        let mut data = self.data.lock().unwrap();
        data.insert(
            session.session_id.clone(),
            serde_json::to_string(session).unwrap(),
        );
        Ok(())
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        let data = self.data.lock().unwrap();
        match data.get(session_id) {
            Some(session) => Ok(Some(serde_json::from_str(session).unwrap())),
            None => Ok(None),
        }
    }
}
