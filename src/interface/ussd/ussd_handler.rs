use std::{collections::HashMap, env};

use chrono::Utc;

use crate::types::{HashStrAny, RouterOptions};

use super::{
    ussd_request::{USSDRequest, USSDRequestTrait},
    ussd_session::{USSDSession, USSDSessionTrait},
};

pub struct USSDHandler {
    pub ussd_request: USSDRequest,
    pub handler: String,
    pub screen_content: HashMap<String, HashStrAny>,
    pub initial_screen: HashMap<String, HashStrAny>,
}

pub trait USSDHandlerTrait {
    fn new(
        ussd_request: USSDRequest,
        handler: String,
        screen_content: HashMap<String, HashStrAny>,
        initial_screen: HashMap<String, HashStrAny>,
    ) -> Self;
    fn get_by_name(&self, name: String) -> USSDHandler;
    fn handle(&self);
    fn route_options(&self, options: Option<RouterOptions>) -> (USSDRequest, String);
    fn evalute_expression(&self, items: Vec<String>, session: &USSDSession) -> Option<Vec<i32>>;
    fn context(&self, session: &USSDSession, extra_content: Option<HashMap<String, String>>) -> HashMap<String, String>;
}

impl USSDHandlerTrait for USSDHandler {
    fn new(
        ussd_request: USSDRequest,
        handler: String,
        screen_content: HashMap<String, HashStrAny>,
        initial_screen: HashMap<String, HashStrAny>,
    ) -> Self {
        USSDHandler {
            ussd_request,
            handler,
            screen_content,
            initial_screen,
        }
    }

    fn get_by_name(&self, _name: String) -> USSDHandler {
        USSDHandler {
            ussd_request: self.ussd_request.clone(),
            handler: self.handler.clone(),
            screen_content: self.screen_content.clone(),
            initial_screen: self.initial_screen.clone(),
        }
    }

    fn handle(&self) {
        println!("Handling USSD request");
    }

    fn route_options(&self, options: Option<RouterOptions>) -> (USSDRequest, String) {
        let mut statement: (USSDRequest, String) =
            (self.ussd_request.clone(), self.handler.clone());

        let route_options = match options {
            Some(options) => options,
            None => match self.screen_content.get("next_screen") {
                Some(options) => {
                    if let HashStrAny::Str(options) = options {
                        RouterOptions::String(options.to_string())
                    } else {
                        RouterOptions::String("".to_string())
                    }
                }
                None => RouterOptions::String("".to_string()),
            },
        };

        if let RouterOptions::String(next_screen) = route_options {
            statement = self.ussd_request.forward(&next_screen);
        }

        statement
    }

    fn evalute_expression(&self, items: Vec<String>, _session: &USSDSession) -> Option<Vec<i32>> {
        let mut evaluated_items: Vec<i32> = vec![];

        for _item in items {
            let evaluated_item = 0;

            // if item.contains("session.") {
            //     let session_key = item.replace("session.", "");
            //     // let session_value = session.session_data.get(&session_key).unwrap();
            //     let session_value = session.session_data.clone();

            //     if let Ok(value) = session_value.parse::<i32>() {
            //         evaluated_item = value;
            //     }
            // } else {
            //     if let Ok(value) = item.parse::<i32>() {
            //         evaluated_item = value;
            //     }
            // }

            evaluated_items.push(evaluated_item);
        }

        Some(evaluated_items)
    }

    fn context(&self, session: &USSDSession, extra_content: Option<HashMap<String, String>>) -> HashMap<String, String> {
        let mut context = session.get_session_data();

        for (key, val) in env::vars() {
            context.insert(key, val);
        }

        if let Some(extra_content) = extra_content {
            for (key, value) in extra_content {
                context.insert(key, value);
            }
        }

        let now = Utc::now().to_rfc3339();
        context.insert("now".to_string(), now);

        context
    }

}
