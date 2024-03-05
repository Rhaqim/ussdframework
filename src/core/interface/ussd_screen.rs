use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::ussd_session::UssdSession;

#[derive(Debug, Serialize)]
pub struct MenuItems {
    pub option: String,
    pub display_name: String,
    pub default_next_screen: String,
}

impl<'de> Deserialize<'de> for MenuItems {
    fn deserialize<D>(deserializer: D) -> Result<MenuItems, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawMenuItems {
            option: String,
            display_name: String,
            default_next_screen: String,
        }

        let raw_menu_items = RawMenuItems::deserialize(deserializer)?;

        Ok(MenuItems {
            option: raw_menu_items.option,
            display_name: raw_menu_items.display_name,
            default_next_screen: raw_menu_items.default_next_screen,
        })
    }
}

// Define an enum to represent different types of USSD screens
#[derive(Debug, Serialize)]
pub enum UssdScreen {
    Initial {
        default_next_screen: String,
    },
    Menu {
        title: String,
        default_next_screen: String,
        menu_items: HashMap<String, MenuItems>,
    },
    Input {
        title: String,
        default_next_screen: String,
        input_type: Option<String>,
        input_identifier: String,
    },
    Function {
        title: String,
        default_next_screen: String,
        function: String,
        data_key: String,
    },
    Router {
        title: String,
        default_next_screen: String,
        router: String,
        router_options: HashMap<String, String>,
    },
    Quit {
        title: String,
        default_next_screen: String,
    },
}

impl<'de> Deserialize<'de> for UssdScreen {
    fn deserialize<D>(deserializer: D) -> Result<UssdScreen, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawUssdScreen {
            #[serde(rename = "type")]
            screen_type: String,
            // Other fields common to all screen types
            title: String,
            default_next_screen: String,
            // Fields specific to certain screen types
            menu_items: Option<HashMap<String, MenuItems>>,
            input_type: Option<String>,
            input_identifier: Option<String>,
            function: Option<String>,
            data_key: Option<String>,
            router_options: Option<HashMap<String, String>>,
            router: Option<String>,
        }

        let raw_screen = RawUssdScreen::deserialize(deserializer)?;

        match raw_screen.screen_type.as_str() {
            "Initial" => Ok(UssdScreen::Initial {
                default_next_screen: raw_screen.default_next_screen,
            }),
            "Menu" => Ok(UssdScreen::Menu {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                menu_items: raw_screen.menu_items.unwrap_or_default(),
            }),
            "Input" => Ok(UssdScreen::Input {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                input_type: Some(raw_screen.input_type.unwrap_or_default()),
                input_identifier: raw_screen.input_identifier.unwrap_or_default(),
            }),
            "Function" => Ok(UssdScreen::Function {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                function: raw_screen.function.unwrap_or_default(),
                data_key: raw_screen.data_key.unwrap_or_default(),
            }),
            "Router" => Ok(UssdScreen::Router {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
                router_options: raw_screen.router_options.unwrap_or_default(),
                router: raw_screen.router.unwrap_or_default(),
            }),
            "Quit" => Ok(UssdScreen::Quit {
                title: raw_screen.title,
                default_next_screen: raw_screen.default_next_screen,
            }),
            _ => Err(serde::de::Error::custom("Unknown screen type")),
        }
    }
}

// Define a trait to represent actions that can be performed in a USSD session
pub trait UssdAction {
    fn validate_input(&self, input: &str) -> bool;
    fn back(&self, session: &mut UssdSession);
    fn home(&self, session: &mut UssdSession);
    fn execute(&self, session: &mut UssdSession, input: &str) -> Option<String>;
    fn display(&self);
}
