use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{error, info, types::{FunctionMap, USSDData}};

use super::USSDSession;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct USSDService {
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
    pub service_code: Option<String>,
}

pub trait USSDServiceTrait {
    fn call(&self, session: &mut USSDSession, function_map: &FunctionMap);
    fn load_function(&self, function_map: &FunctionMap) -> Box<dyn Fn(&USSDSession, &str) -> USSDData>;
}

impl USSDServiceTrait for USSDService {
    fn call(&self, session: &mut USSDSession, function_map: &FunctionMap) {
        let loaded_function = self.load_function(function_map);

        let new_session: USSDSession = session.clone();

        let result = match &self.function_url {
            Some(url) => loaded_function(&new_session, url),
            None => loaded_function(&new_session, ""),
        };

        session.data.insert(self.data_key.clone(), result.clone());
    }

    fn load_function(&self, function_map: &FunctionMap) -> Box<dyn Fn(&USSDSession, &str) -> USSDData> {
        let func = function_map.get(&self.function_name).cloned();

        match func {
            Some(f) => {
                info!("Function found: {}", self.function_name);
                Box::new(f)
            }
            None => {
                error!("Function not found: {}", self.function_name);
                Box::new(|_session: &USSDSession, _url: &str| {
                    let mut result = HashMap::new();
                    result.insert(
                        "error".to_string(),
                        USSDData::Str("Function not found".to_string()),
                    );
                    USSDData::Dict(result)
                })
            }
        }
    }
}

