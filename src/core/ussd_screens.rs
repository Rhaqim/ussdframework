use crate::{
    error,
    types::{FunctionMap, USSDData},
    utils::{evaluate_expression, evaluate_expression_op},
};

use regex::Regex;
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize,
};
use std::{collections::HashMap, fmt};

use super::{ussd_service::USSDServiceTrait, USSDRequest, USSDService, USSDSession};

// ── ScreenText ────────────────────────────────────────────────────────────────
//
// Supports both the legacy plain-string format and the new per-language map:
//
//   Legacy:  "text": "Enter your phone number"
//   New:     "text": { "en": "Enter your phone number", "fr": "Entrez votre numéro" }
//
// When deserializing a plain string it is stored under the key "default".
// `get(language)` looks up the requested language then falls back to "default".

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ScreenText(pub HashMap<String, String>);

impl ScreenText {
    /// Return the text for the given language, falling back to "default", then "".
    pub fn get(&self, language: &str) -> &str {
        self.0
            .get(language)
            .or_else(|| self.0.get("default"))
            .map(|s| s.as_str())
            .unwrap_or("")
    }
}

impl Default for ScreenText {
    fn default() -> Self {
        ScreenText(HashMap::new())
    }
}

impl<'de> Deserialize<'de> for ScreenText {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct ScreenTextVisitor;

        impl<'de> Visitor<'de> for ScreenTextVisitor {
            type Value = ScreenText;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "a string or a map of language codes to strings")
            }

            fn visit_str<E: de::Error>(self, v: &str) -> Result<ScreenText, E> {
                let mut map = HashMap::new();
                map.insert("default".to_string(), v.to_string());
                Ok(ScreenText(map))
            }

            fn visit_map<A: de::MapAccess<'de>>(
                self,
                mut access: A,
            ) -> Result<ScreenText, A::Error> {
                let mut map = HashMap::new();
                while let Some((k, v)) = access.next_entry::<String, String>()? {
                    map.insert(k, v);
                }
                Ok(ScreenText(map))
            }
        }

        deserializer.deserialize_any(ScreenTextVisitor)
    }
}

// ── ScreenType ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub enum ScreenType {
    #[default]
    Initial,
    Menu,
    Input,
    Function,
    Router,
    Quit,
}

impl ScreenType {
    pub fn to_string(&self) -> String {
        match self {
            ScreenType::Initial => "Initial".to_string(),
            ScreenType::Menu => "Menu".to_string(),
            ScreenType::Input => "Input".to_string(),
            ScreenType::Function => "Function".to_string(),
            ScreenType::Router => "Router".to_string(),
            ScreenType::Quit => "Quit".to_string(),
        }
    }

    pub fn from_string(screen_type: &str) -> ScreenType {
        match screen_type {
            "Initial" => ScreenType::Initial,
            "Menu" => ScreenType::Menu,
            "Input" => ScreenType::Input,
            "Function" => ScreenType::Function,
            "Router" => ScreenType::Router,
            "Quit" => ScreenType::Quit,
            _ => {
                error!("Invalid screen type");
                ScreenType::Initial
            }
        }
    }
}

// ── USSDScreen ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDScreen {
    /// Display text, supports plain string or `{ "en": "...", "fr": "..." }` map.
    pub text: ScreenText,
    pub screen_type: ScreenType,
    pub default_next_screen: String,
    #[serde(default)]
    pub service_code: Option<String>,
    #[serde(default)]
    pub menu_items: Option<HashMap<String, USSDMenuItems>>,
    #[serde(default)]
    pub function: Option<String>,
    #[serde(default)]
    pub router_options: Option<Vec<USSDRouterOption>>,
    #[serde(default)]
    pub input_identifier: Option<String>,
    #[serde(default)]
    pub input_type: Option<String>,
    /// Optional regex pattern to validate Input screen entries.
    #[serde(default)]
    pub validation_regex: Option<String>,
    /// Optional maximum number of characters accepted for an Input screen.
    #[serde(default)]
    pub max_length: Option<usize>,
    /// Maximum failed attempts allowed on this screen before forcing a redirect.
    /// Works together with `timeout_screen`.
    #[serde(default)]
    pub max_retries: Option<u8>,
    /// Screen to redirect to when `max_retries` is exceeded. If absent the session ends.
    #[serde(default)]
    pub timeout_screen: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDMenuItems {
    pub option: String,
    pub display_name: String,
    pub next_screen: String,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDRouterOption {
    pub router_option: String,
    pub next_screen: String,
}

fn back(session: &mut USSDSession) {
    if let Some(prev_screen) = session.visited_screens.pop() {
        session.current_screen = prev_screen;
    }
}

fn home(session: &mut USSDSession) {
    session.current_screen = session.visited_screens.first().unwrap().clone();
}

pub trait USSDAction {
    fn display(&self, session: &mut USSDSession) -> Option<String>;
    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        services: &HashMap<String, USSDService>,
        function_map: &FunctionMap,
    );
}

impl USSDAction for USSDScreen {
    fn display(&self, session: &mut USSDSession) -> Option<String> {
        let lang = session.language.clone();
        let mut message = String::new();

        if let Some(error_message) = &session.error_message {
            message.push_str(error_message);
            message.push_str("\n\n");
        }

        match self.screen_type {
            ScreenType::Initial => None,
            ScreenType::Menu => {
                let text = evaluate_expression(self.text.get(&lang), session);
                message.push_str(&text);

                if let Some(menu_items) = &self.menu_items {
                    let mut sorted: Vec<(&String, &USSDMenuItems)> = menu_items.iter().collect();
                    sorted.sort_by_key(|(_, item)| item.option.parse::<usize>().unwrap_or(0));
                    for (index, (_, value)) in sorted.iter().enumerate() {
                        message.push_str(&format!("\n{}. {}", index + 1, value.display_name));
                    }
                } else {
                    message.push_str("\nNo menu items found");
                }

                Some(message)
            }
            ScreenType::Input => {
                let text = evaluate_expression(self.text.get(&lang), session);
                message.push_str(&text);
                Some(message)
            }
            ScreenType::Function => None,
            ScreenType::Router => None,
            ScreenType::Quit => {
                let text = evaluate_expression(self.text.get(&lang), session);
                message.push_str(&text);
                session.end_session = true;
                Some(message)
            }
        }
    }

    fn execute(
        &self,
        session: &mut USSDSession,
        request: &USSDRequest,
        services: &HashMap<String, USSDService>,
        function_map: &FunctionMap,
    ) {
        let input = request.input.trim();

        match input {
            "0" => back(session),
            "00" => home(session),
            _ => {
                // Capture the screen name before any mutation.
                let screen_name = session.current_screen.clone();

                session.current_screen = match self.screen_type {
                    ScreenType::Initial => self.default_next_screen.clone(),

                    ScreenType::Menu => {
                        match input.parse::<usize>() {
                            Ok(selected_option) if selected_option > 0 => {
                                if let Some(items) = self.menu_items.as_ref() {
                                    if let Some(selected) = items
                                        .values()
                                        .find(|i| i.option == selected_option.to_string())
                                    {
                                        // Successful selection — clear attempt counter.
                                        session.screen_attempts.remove(&screen_name);
                                        session.current_screen = selected.next_screen.clone();
                                        return;
                                    } else {
                                        error!("Selected menu item not found");
                                        session.error_message =
                                            Some("Invalid menu option".to_string());
                                        self.handle_retry(session, &screen_name);
                                        return;
                                    }
                                }
                            }
                            _ => {
                                error!("Invalid menu option");
                                session.error_message = Some("Invalid menu option".to_string());
                                self.handle_retry(session, &screen_name);
                                return;
                            }
                        }
                        self.default_next_screen.clone()
                    }

                    ScreenType::Input => {
                        // Max-length check.
                        if let Some(max) = self.max_length {
                            if input.len() > max {
                                session.error_message = Some(format!(
                                    "Input too long. Maximum {} characters allowed.",
                                    max
                                ));
                                self.handle_retry(session, &screen_name);
                                return;
                            }
                        }
                        // Regex validation.
                        if let Some(pattern) = &self.validation_regex {
                            match Regex::new(pattern) {
                                Ok(re) => {
                                    if !re.is_match(input) {
                                        session.error_message =
                                            Some("Invalid input. Please try again.".to_string());
                                        self.handle_retry(session, &screen_name);
                                        return;
                                    }
                                }
                                Err(e) => {
                                    error!("Invalid validation regex '{}': {}", pattern, e);
                                }
                            }
                        }
                        // Successful input — clear attempt counter and error.
                        session.screen_attempts.remove(&screen_name);
                        session.error_message = None;
                        if let Some(id) = &self.input_identifier {
                            session
                                .data
                                .insert(id.to_string(), USSDData::Str(input.to_string()));
                        }
                        self.default_next_screen.clone()
                    }

                    ScreenType::Function => {
                        if let Some(function_name) = &self.function {
                            call_function(session, services, function_name, function_map);
                        }
                        self.default_next_screen.clone()
                    }

                    ScreenType::Router => {
                        if let Some(router_options) = &self.router_options {
                            for option in router_options {
                                match evaluate_expression_op_result(session, &option.router_option)
                                {
                                    Ok(true) => {
                                        session.current_screen = option.next_screen.clone();
                                        return;
                                    }
                                    Ok(false) => {}
                                    Err(e) => {
                                        error!(
                                            "Router expression '{}' evaluation failed: {}",
                                            option.router_option, e
                                        );
                                    }
                                }
                            }
                            // No condition matched — use default.
                            error!(
                                "No router option matched for screen '{}', falling through to default_next_screen",
                                screen_name
                            );
                        }
                        self.default_next_screen.clone()
                    }

                    ScreenType::Quit => {
                        session.end_session = true;
                        self.default_next_screen.clone()
                    }
                }
            }
        }
    }
}

impl USSDScreen {
    /// Increment the failed-attempt counter for this screen. If `max_retries` is
    /// set and exceeded, redirect to `timeout_screen` (or end the session).
    fn handle_retry(&self, session: &mut USSDSession, screen_name: &str) {
        let Some(max) = self.max_retries else {
            // No retry limit — just stay on the current screen.
            return;
        };

        let attempts = session
            .screen_attempts
            .entry(screen_name.to_string())
            .or_insert(0);
        *attempts += 1;

        if *attempts >= max {
            session.screen_attempts.remove(screen_name);
            session.error_message = None;

            if let Some(ts) = &self.timeout_screen {
                error!(
                    "Max retries ({}) exceeded on '{}', redirecting to '{}'",
                    max, screen_name, ts
                );
                session.current_screen = ts.clone();
            } else {
                error!(
                    "Max retries ({}) exceeded on '{}', ending session",
                    max, screen_name
                );
                session.end_session = true;
                session.current_screen = self.default_next_screen.clone();
            }
        }
    }
}

// ── evaluate_expression_op with error surfacing ───────────────────────────────

/// Like `evaluate_expression_op` but surfaces parse failures as `Err(String)`.
fn evaluate_expression_op_result(session: &USSDSession, text: &str) -> Result<bool, String> {
    let pattern_str = r"\{\{([\w.]+)(?:\s*(==|>|>=|<|<=)\s*\'?(\w+)\'?)?\}\}";
    let pattern = regex::Regex::new(pattern_str)
        .map_err(|e| format!("Router regex compile error: {}", e))?;

    if pattern.captures(text).is_none() {
        return Err(format!(
            "Expression '{}' does not match the expected pattern {{{{field op value}}}}",
            text
        ));
    }

    Ok(evaluate_expression_op(session, text))
}

// ── call_function ─────────────────────────────────────────────────────────────

fn call_function(
    session: &mut USSDSession,
    services: &HashMap<String, USSDService>,
    function_name: &str,
    function_map: &FunctionMap,
) {
    match services.get(function_name) {
        Some(service) => service.call(session, function_map),
        None => error!("Service '{}' not found in services map", function_name),
    }
}
