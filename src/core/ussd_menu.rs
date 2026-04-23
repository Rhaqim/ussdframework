use config::Config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::core::{
    ussd_screens::{ScreenType, USSDScreen},
    ussd_service::USSDService,
};

/// Represents a USSD menu structure.
///
/// The `USSDMenu` struct represents the structure of a USSD (Unstructured Supplementary Service Data)
/// menu, including menus and associated services. It encapsulates a collection of menus and services
/// represented as hash maps.
///
/// This struct provides methods for creating a new menu instance and loading menu structure from a JSON file.
///
/// # Fields
///
/// * `menus`: A `HashMap<String, Screen>` representing the menus available in the USSD menu structure.
/// * `services`: A `HashMap<String, USSDService>` representing the services associated with the menu structure.
///
/// # Derives
///
/// The `USSDMenu` struct derives `Debug`, `Clone`, `Deserialize`, and `Serialize` traits
/// to enable debugging, cloning, and serialization/deserialization of menu instances.
///
#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDMenu {
    pub menus: HashMap<String, USSDScreen>,
    pub services: HashMap<String, USSDService>,
}

impl USSDMenu {
    /// Creates a new empty USSD menu instance.
    ///
    /// # Returns
    ///
    /// A `USSDMenu` instance with empty menu and service collections.
    ///
    pub fn new() -> Self {
        USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        }
    }

    /// Loads a USSD menu structure from a JSON file.
    ///
    /// This method reads the contents of the specified JSON `file_path`, parses it, and constructs
    /// a `USSDMenu` instance from the JSON data.
    ///
    /// # Arguments
    ///
    /// * `file_path`: A string representing the file path to the JSON file containing the menu structure.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `USSDMenu` instance representing the loaded menu structure
    /// (if successful), or a `Box<dyn std::error::Error>` containing an error message if an error occurs
    /// during the loading process.
    ///
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // let contents = include_str!("../data/menu.json");
        let menu: USSDMenu = serde_json::from_str(&contents)?;
        Ok(menu)
    }

    // Save menu structure to JSON file
    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }

    // load menu from config
    pub fn _load_from_config(config: &Config) -> Result<Self, Box<dyn std::error::Error>> {
        let menu: USSDMenu = config.get("menu")?;
        Ok(menu)
    }

    /// Retrieves the initial screen from the USSD menu.
    ///
    /// Returns `None` if no screen with type `Initial` exists.
    pub fn get_initial_screen(&self) -> Option<(String, &USSDScreen)> {
        self.menus.iter().find_map(|(name, screen)| {
            if let ScreenType::Initial = screen.screen_type {
                Some((name.clone(), screen))
            } else {
                None
            }
        })
    }

    /// Validates all router-screen expressions at menu-load time.
    ///
    /// Returns `Ok(())` if every expression is parseable, or `Err(errors)` with
    /// a list of human-readable problem descriptions.
    pub fn validate_router_expressions(&self) -> Result<(), Vec<String>> {
        let pattern_str = r"\{\{([\w.]+)(?:\s*(==|>|>=|<|<=)\s*\'?([\w]+)\'?)?\}\}";
        let pattern = match regex::Regex::new(pattern_str) {
            Ok(p) => p,
            Err(e) => return Err(vec![format!("Internal regex error: {}", e)]),
        };

        let mut errors: Vec<String> = Vec::new();

        for (screen_name, screen) in &self.menus {
            if screen.screen_type != ScreenType::Router {
                continue;
            }
            let Some(router_options) = &screen.router_options else {
                continue;
            };
            for option in router_options {
                if pattern.captures(&option.router_option).is_none() {
                    errors.push(format!(
                        "Screen '{}': router_option '{}' does not match pattern {{{{field op value}}}}",
                        screen_name, option.router_option
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Filters and retrieves screens and services belonging to a specific service code.
    ///
    /// This method filters the menu's screens and services to retrieve those associated with the specified `service_code`.
    ///
    /// # Arguments
    ///
    /// * `service_code`: A string representing the service code to filter screens and services.
    ///
    /// # Returns
    ///
    /// A new `USSDMenu` instance containing screens and services belonging to the specified `service_code`.
    ///
    pub fn belonging_to(&self, service_code: &str) -> USSDMenu {
        let mut menus = HashMap::new();
        let mut services = HashMap::new();

        for (name, screen) in self.menus.iter() {
            match screen.service_code {
                Some(ref code) if code == service_code => {
                    menus.insert(name.clone(), screen.clone());
                }
                None => {}
                _ => {}
            }
        }

        for (name, service) in self.services.iter() {
            match service.service_code {
                Some(ref code) if code == service_code => {
                    services.insert(name.clone(), service.clone());
                }
                None => {}
                _ => {}
            }
        }

        USSDMenu { menus, services }
    }

    // pub fn builder(service_code: &str, connection: DbConnection) -> MenuBuilder {
    //     MenuBuilder::new(service_code, connection)
    // }
}
