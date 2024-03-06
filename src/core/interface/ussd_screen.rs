use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;

use super::{ussd_session::USSDSession, USSDRequest, USSDResponse, USSDService};

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuItems {
    pub option: String,
    pub display_name: String,
    pub default_next_screen: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RouterOptions {
    pub router_option: String,
    pub next_screen: String,
}

// Define an enum to represent different types of USSD screens
#[derive(Debug, Serialize)]
pub enum USSDScreen {
    Initial {
        default_next_screen: String,
    },
    Menu {
        text: String,
        default_next_screen: String,
        menu_items: HashMap<String, MenuItems>,
    },
    Input {
        text: String,
        default_next_screen: String,
        input_type: Option<String>,
        input_identifier: String,
    },
    Function {
        text: String,
        default_next_screen: String,
        function: String,
    },
    Router {
        text: String,
        default_next_screen: String,
        router: String,
        router_options: Vec<RouterOptions>
    },
    Quit {
        text: String,
        default_next_screen: String,
    },
}

impl<'de> Deserialize<'de> for USSDScreen {
    fn deserialize<D>(deserializer: D) -> Result<USSDScreen, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Debug, Deserialize)]
        struct RawUSSDScreen {
            #[serde(rename = "type")]
            screen_type: String,
            // Other fields common to all screen types
            text: String,
            default_next_screen: String,
            // Fields specific to certain screen types
            menu_items: Option<HashMap<String, MenuItems>>,
            input_type: Option<String>,
            input_identifier: Option<String>,
            function: Option<String>,
            router_options: Option<Vec<RouterOptions>>,
            router: Option<String>,
        }

        let raw_screen = RawUSSDScreen::deserialize(deserializer)?;

        match raw_screen.screen_type.as_str() {
            "Initial" => Ok(USSDScreen::Initial {
                default_next_screen: raw_screen.default_next_screen,
            }),
            "Menu" => Ok(USSDScreen::Menu {
                text: raw_screen.text,
                default_next_screen: raw_screen.default_next_screen,
                menu_items: raw_screen.menu_items.unwrap_or_default(),
            }),
            "Input" => Ok(USSDScreen::Input {
                text: raw_screen.text,
                default_next_screen: raw_screen.default_next_screen,
                input_type: Some(raw_screen.input_type.unwrap_or_default()),
                input_identifier: raw_screen.input_identifier.unwrap_or_default(),
            }),
            "Function" => Ok(USSDScreen::Function {
                text: raw_screen.text,
                default_next_screen: raw_screen.default_next_screen,
                function: raw_screen.function.unwrap_or_default(),
            }),
            "Router" => Ok(USSDScreen::Router {
                text: raw_screen.text,
                default_next_screen: raw_screen.default_next_screen,
                router_options: raw_screen.router_options.unwrap_or_default(),
                router: raw_screen.router.unwrap_or_default(),
            }),
            "Quit" => Ok(USSDScreen::Quit {
                text: raw_screen.text,
                default_next_screen: raw_screen.default_next_screen,
            }),
            _ => Err(serde::de::Error::custom("Unknown screen type")),
        }
    }
}

// Define a trait to represent actions that can be performed in a USSD session
pub trait UssdAction {
    fn validate_input(&self, input: &str) -> bool;
    fn back(&self, session: &mut USSDSession);
    fn home(&self, session: &mut USSDSession);
    fn execute(&self, request: &USSDRequest, session: &mut USSDSession, services: &HashMap<String, USSDService>) -> Option<USSDResponse>;
    fn display(&self);
}
