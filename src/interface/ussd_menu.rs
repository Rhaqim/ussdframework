use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use super::ussd_screen::UssdScreen;


// Define a structure to hold the USSD menu data
#[derive(Debug, Deserialize, Serialize)]
pub struct UssdMenu {
    pub menus: HashMap<String, UssdScreen>,
}

impl UssdMenu {
    // Load menu structure from JSON file
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // let contents = include_str!("../data/menu.json");
        let menu: UssdMenu = serde_json::from_str(&contents)?;
        Ok(menu)
    }

    // Save menu structure to JSON file
    pub fn save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json_str = serde_json::to_string(self)?;
        let mut file = File::create(file_path)?;
        file.write_all(json_str.as_bytes())?;
        Ok(())
    }
}