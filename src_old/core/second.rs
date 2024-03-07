use serde::{Deserialize, Deserializer, Serialize};

use std::{collections::HashMap, fs::File, io::Read, time::SystemTime};

use crate::{helper::stack::Stack, types::HashStrAny};

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDRequest {
    pub input: String,
    pub msisdn: String,
    pub session_id: String,
    pub request_id: String,
    pub telco: String,
    pub service_code: String,
    pub country_code: String,
    pub language: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDResponse {
    pub msisdn: String,
    pub request_id: String,
    pub telco: String,
    pub service_code: String,
    pub country_code: String,
    pub language: String,
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDSession {
    pub session_id: String,
    pub data: HashMap<String, HashStrAny>,
    pub current_screen: String,
    pub visited_screens: Stack<String>,
    pub last_interaction_time: SystemTime,
    pub end_session: bool,
    pub language: String,
    pub msisdn: String,
}

pub trait SessionCache {
    fn store_session(&self, session: &USSDSession) -> Result<(), String>;
    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDService {
    pub functions_path: String,
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MenuItems {
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
        router_options: Vec<RouterOptions>,
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

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDMenu {
    pub menus: HashMap<String, USSDScreen>,
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
}

pub struct USSDConfig {
    pub functions_path: String,
    pub session_cache: Box<dyn SessionCache>,
}

pub struct USSDGateway {
    pub config: USSDConfig,
    pub menus: USSDMenu,
}

impl USSDGateway {
    pub fn new(config: USSDConfig, menus: USSDMenu) -> Self {
        Self { config, menus }
    }

    pub async fn process_request(&self, request: USSDRequest) -> Result<USSDResponse, String> {
        let session = self.retrieve_session(&request.session_id)?;
        let mut session = match session {
            Some(session) => session,
            None => {
                let session = USSDSession {
                    session_id: request.session_id.clone(),
                    data: HashMap::new(),
                    current_screen: "initial".to_string(),
                    visited_screens: Stack::new(),
                    last_interaction_time: SystemTime::now(),
                    end_session: false,
                    language: request.language.clone(),
                    msisdn: request.msisdn.clone(),
                };
                self.store_session(&session)?;
                session
            }
        };

        let menu = self
            .menus
            .menus
            .get(&mut session.current_screen)
            .ok_or("Menu not found")?;

        let response = match menu {
            USSDScreen::Initial {
                default_next_screen,
            } => self.process_initial_screen(&request, &mut session, default_next_screen)?,
            USSDScreen::Menu {
                text,
                default_next_screen,
                menu_items,
            } => {
                self.process_menu_screen(&request, &mut session, text, default_next_screen, menu_items)?
            }
            USSDScreen::Input {
                text,
                default_next_screen,
                input_type,
                input_identifier,
            } => self.process_input_screen(
                &request,
                &mut session,
                text,
                default_next_screen,
                input_type,
                input_identifier,
            )?,
            USSDScreen::Function {
                text,
                default_next_screen,
                function,
            } => self.process_function_screen(
                &request,
                &mut session,
                text,
                default_next_screen,
                function,
            )?,
            USSDScreen::Router {
                text,
                default_next_screen,
                router,
                router_options,
            } => self.process_router_screen(
                &request,
                &mut session,
                text,
                default_next_screen,
                router,
                router_options,
            )?,
            USSDScreen::Quit {
                text,
                default_next_screen,
            } => self.process_quit_screen(&request, &mut session, text, default_next_screen)?,
        };

        self.store_session(&mut session)?;
        Ok(response)
    }

    fn process_initial_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        default_next_screen: &str,
    ) -> Result<USSDResponse, String> {
        let menu = self
            .menus
            .menus
            .get(default_next_screen)
            .ok_or("Menu not found")?;
        let response = match menu {
            USSDScreen::Menu {
                text,
                default_next_screen,
                menu_items,
            } => {
                self.process_menu_screen(request, session, text, default_next_screen, menu_items)?
            }
            _ => return Err("Invalid menu type".to_string()),
        };
        Ok(response)
    }

    fn process_menu_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        text: &str,
        default_next_screen: &str,
        menu_items: &HashMap<String, MenuItems>,
    ) -> Result<USSDResponse, String> {
        let mut message = text.to_string();
        for (idx, (_, value)) in menu_items.iter().enumerate() {
            message.push_str(&format!("\n{}: {}", idx, value.display_name));
        }
        let response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message,
        };
        Ok(response)
    }

    fn process_input_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        text: &str,
        default_next_screen: &str,
        input_type: &Option<String>,
        input_identifier: &str,
    ) -> Result<USSDResponse, String> {
        session
        .data
        .insert(input_identifier.to_string(), HashStrAny::Str(request.input.clone()));

        let response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message: text.to_string(),
        };
        Ok(response)
    }

    fn process_function_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        text: &str,
        default_next_screen: &str,
        function: &str,
    ) -> Result<USSDResponse, String> {
        let response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message: text.to_string(),
        };
        Ok(response)
    }

    fn process_router_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        text: &str,
        default_next_screen: &str,
        router: &str,
        router_options: &Vec<RouterOptions>,
    ) -> Result<USSDResponse, String> {
        let response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message: text.to_string(),
        };
        Ok(response)
    }

    fn process_quit_screen(
        &self,
        request: &USSDRequest,
        session: &mut USSDSession,
        text: &str,
        default_next_screen: &str,
    ) -> Result<USSDResponse, String> {
        let response = USSDResponse {
            msisdn: request.msisdn.clone(),
            request_id: request.request_id.clone(),
            telco: request.telco.clone(),
            service_code: request.service_code.clone(),
            country_code: request.country_code.clone(),
            language: request.language.clone(),
            message: text.to_string(),
        };
        Ok(response)
    }

    fn store_session(&self, session: &USSDSession) -> Result<(), String> {
        self.config.session_cache.store_session(session)
    }

    fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
        self.config.session_cache.retrieve_session(session_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helper::stack::Stack;
    use std::collections::HashMap;
    use std::time::SystemTime;

    struct MockSessionCache {}

    impl SessionCache for MockSessionCache {
        fn store_session(&self, session: &USSDSession) -> Result<(), String> {
            Ok(())
        }

        fn retrieve_session(&self, session_id: &str) -> Result<Option<USSDSession>, String> {
            Ok(None)
        }
    }

    #[tokio::test]
    async fn test_process_request() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "1234567890".to_string(),
        };
        let response = gateway.process_request(request).await;
        assert!(response.is_ok());
        // assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_initial_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "initial".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let response = gateway
            .process_initial_screen(&request, &mut session, "menu")
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_menu_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "menu".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let text = "Choose an option".to_string();
        let default_next_screen = "input".to_string();
        let mut menu_items = HashMap::new();
        menu_items.insert(
            "1".to_string(),
            MenuItems {
                display_name: "Option 1".to_string(),
                default_next_screen: "input".to_string(),
            },
        );
        menu_items.insert(
            "2".to_string(),
            MenuItems {
                display_name: "Option 2".to_string(),
                default_next_screen: "input".to_string(),
            },
        );
        let response = gateway
            .process_menu_screen(&request, &mut session, &text, &default_next_screen, &menu_items)
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_input_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "input".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let text = "Enter your name".to_string();
        let default_next_screen = "menu".to_string();
        let input_type = Some("text".to_string());
        let input_identifier = "name".to_string();
        let response = gateway
            .process_input_screen(
                &request,
                &mut session,
                &text,
                &default_next_screen,
                &input_type,
                &input_identifier,
            )
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_function_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "function".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let text = "Processing".to_string();
        let default_next_screen = "menu".to_string();
        let function = "process".to_string();
        let response = gateway
            .process_function_screen(&request, &mut session, &text, &default_next_screen, &function)
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_router_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "router".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let text = "Choose an option".to_string();
        let default_next_screen = "menu".to_string();
        let router = "process".to_string();
        let router_options = vec![
            RouterOptions {
                router_option: "1".to_string(),
                next_screen: "menu".to_string(),
            },
            RouterOptions {
                router_option: "2".to_string(),
                next_screen: "menu".to_string(),
            },
        ];
        let response = gateway
            .process_router_screen(
                &request,
                &mut session,
                &text,
                &default_next_screen,
                &router,
                &router_options,
            )
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_process_quit_screen() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let request = USSDRequest {
            input: "1".to_string(),
            msisdn: "1234567890".to_string(),
            session_id: "1234567890".to_string(),
            request_id: "1234567890".to_string(),
            telco: "1234567890".to_string(),
            service_code: "1234567890".to_string(),
            country_code: "1234567890".to_string(),
            language: "en".to_string(),
        };
        let mut session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "quit".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let text = "Goodbye".to_string();
        let default_next_screen = "menu".to_string();
        let response = gateway
            .process_quit_screen(&request, &mut session, &text, &default_next_screen)
            .unwrap();
        assert_eq!(response.msisdn, "1234567890");
    }

    #[test]
    fn test_store_session() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let session = USSDSession {
            session_id: "1234567890".to_string(),
            data: HashMap::new(),
            current_screen: "quit".to_string(),
            visited_screens: Stack::new(),
            last_interaction_time: SystemTime::now(),
            end_session: false,
            language: "en".to_string(),
            msisdn: "1234567890".to_string(),
        };
        let result = gateway.store_session(&session);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_retrieve_session() {
        let config = USSDConfig {
            functions_path: "functions".to_string(),
            session_cache: Box::new(MockSessionCache {}),
        };
        let menus = USSDMenu {
            menus: HashMap::new(),
            services: HashMap::new(),
        };
        let gateway = USSDGateway::new(config, menus);

        let session_id = "1234567890".to_string();
        let result = gateway.retrieve_session(&session_id);
        assert!(result.is_ok());
    }
}