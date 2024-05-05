use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{error, info, types::USSDData, utils::FUNCTION_MAP};

use super::USSDSession;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDService {
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
    pub service_code: Option<String>,
}

pub trait USSDServiceTrait {
    fn call(&self, session: &mut USSDSession);
    fn load_function(&self) -> Box<dyn Fn(&USSDSession, &str) -> USSDData>;
}

impl USSDServiceTrait for USSDService {
    fn call(&self, session: &mut USSDSession) {
        // Find and load the function from the functions_path
        // Logic to load the function from the function_path (You need to implement this logic)
        let loaded_function = self.load_function();

        // Clone the session to pass to the loaded function
        let new_session: USSDSession = session.clone();

        // Pass the function_url as an argument to the loaded function
        let result = match &self.function_url {
            Some(url) => loaded_function(&new_session, url),
            None => loaded_function(&new_session, ""),
        };

        // Save the returned result in the session data with the data_key
        session.data.insert(self.data_key.clone(), result.clone());
    }

    fn load_function(&self) -> Box<dyn Fn(&USSDSession, &str) -> USSDData> {
        // Load the function from the functions_path
        let func = FUNCTION_MAP
            .lock()
            .unwrap()
            .get(&self.function_name)
            .cloned();

        match func {
            Some(f) => {
                info!("Function found: {}", self.function_name);
                Box::new(f.clone())
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
