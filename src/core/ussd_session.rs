use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    sync::Mutex,
    time::{Duration, SystemTime},
};

use crate::{info, types::USSDData};

use super::USSDRequest;

/// Represents a USSD session.
///
/// The `USSDSession` struct represents a session in the context of Unstructured Supplementary
/// Service Data (USSD) communication. It encapsulates various session-related data, including
/// session ID, session data, current screen, displayed screens, visited screens, last interaction
/// time, session termination status, language, and mobile subscriber ISDN (MSISDN) number.
///
/// This struct is used to maintain and manage the state of USSD sessions throughout their lifecycle.
/// It provides methods for accessing and manipulating session data.
///
/// # Fields
///
/// * `session_id`: A string representing the unique identifier of the session.
/// * `data`: A `HashMap<String, USSDData>` containing session-specific data associated with keys.
/// * `current_screen`: A string representing the identifier of the current screen within the session.
/// * `displayed`: A `HashMap<String, bool>` indicating whether each screen has been displayed.
/// * `visited_screens`: A vector of strings representing the screens visited during the session.
/// * `last_interaction_time`: A `SystemTime` representing the timestamp of the last interaction with the session.
/// * `end_session`: A boolean indicating whether the session has ended.
/// * `language`: A string representing the language preference of the session.
/// * `msisdn`: A string representing the mobile subscriber ISDN (MSISDN) number associated with the session.
///
/// # Derives
///
/// The `USSDSession` struct derives `Debug`, `Deserialize`, `Serialize`, and `Clone` traits
/// to enable debugging, serialization/deserialization, and cloning of session instances.
///
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct USSDSession {
    pub session_id: String,
    pub data: HashMap<String, USSDData>,
    pub current_screen: String,
    pub error_message: Option<String>,
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
            error_message: None,
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
                    error_message: None,
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
    pub fn fetch_session_data<'a>(&'a self, key: &str) -> Option<&'a USSDData> {
        self.data.get(key)
    }
}

/// Trait for a session cache implementation.
///
/// The `SessionCache` trait defines the interface for storing and retrieving USSD (Unstructured
/// Supplementary Service Data) sessions. Implementations of this trait are responsible for storing
/// and retrieving session data associated with session IDs. This trait provides methods for storing
/// and retrieving sessions, which should be implemented according to the specific storage mechanism.
///
/// # Safety
///
/// This trait is marked as `Send` and `Sync`, indicating that implementations must be thread-safe
/// and capable of being sent across threads. Implementations should ensure that concurrent access
/// to session data is properly synchronized to prevent data races.
///
pub trait SessionCache: Send + Sync {
    /// Stores a USSD session in the cache.
    ///
    /// This method stores the provided `session` in the session cache. The session data is associated
    /// with a session ID, which is used as the key for storage. If the session ID already exists
    /// in the cache, the existing session data should be replaced with the new session data.
    ///
    /// # Arguments
    ///
    /// * `session`: A reference to the `USSDSession` object representing the session to be stored.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the operation was successful. If successful, `Ok(())` is returned.
    /// If an error occurs during the storage process, a `String` containing an error message is returned.
    ///
    fn store_session(&self, session: &USSDSession) -> Result<(), String>;

    /// Retrieves a USSD session from the cache.
    ///
    /// This method retrieves the session associated with the provided `session_id` from the session cache.
    /// If a session with the specified ID exists in the cache, it is returned. Otherwise, `None` is returned.
    ///
    /// # Arguments
    ///
    /// * `session_id`: A string representing the unique identifier of the session to be retrieved.
    ///
    /// # Returns
    ///
    /// A `Result` containing either an `Option<USSDSession>` representing the retrieved session (if found),
    /// or a `String` containing an error message if an error occurs during the retrieval process.
    ///
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
