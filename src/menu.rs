use config::Config;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use crate::{
    ussd_screens::{Screen, ScreenType},
    ussd_service::USSDService,
};

// Define a structure to hold the USSD menu data
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDMenu {
    pub menus: HashMap<String, Screen>,
    pub services: HashMap<String, USSDService>,
}

impl USSDMenu {
    // Load menu structure from JSON file
    pub fn load_from_json(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut file = File::open(file_path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        // let contents = include_str!("../data/menu.json");
        let menu: USSDMenu = serde_json::from_str(&contents)?;
        Ok(menu)
    }

    // Save menu structure to JSON file
    pub fn _save_to_json(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    // Get the Intial screen
    pub fn get_initial_screen(&self) -> (String, &Screen) {
        for (screen_name, screen) in self.menus.iter() {
            if let ScreenType::Initial = screen.screen_type {
                return (screen_name.clone(), screen);
            }
        }
        panic!("No initial screen found!");
    }
}

// pub struct MenuBuilder {
//     name: String,
//     // You can add more fields as needed
// }

// impl MenuBuilder {
//     pub fn new(name: &str) -> Self {
//         MenuBuilder {
//             name: name.to_string(),
//         }
//     }

//     pub fn prompt(&mut self, message: &str) {
//         println!("{}", message);
//         // Handle prompt logic
//     }

//     pub fn option<F>(&mut self, option: &str, label: &str, handler: F)
//     where
//         F: FnOnce(&mut MenuBuilder),
//     {
//         println!("{}. {}", option, label);
//         // Handle option logic
//         let mut sub_menu_builder = MenuBuilder::new(&format!("{}_{}", self.name, option));
//         handler(&mut sub_menu_builder);
//     }
// }
