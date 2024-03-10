use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    error, info, types::HashStrAny, ussd_request::USSDRequest, ussd_session::USSDSession,
    utils::FUNCTION_MAP,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDService {
    pub functions_path: String,
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
}

pub trait USSDServiceTrait {
    fn new(
        functions_path: String,
        function_name: String,
        function_url: Option<String>,
        data_key: String,
    ) -> Self;
    fn call(&self, session: &mut USSDSession, request: &USSDRequest, functions_path: String);
    fn load_function(
        &self,
        base_functions_path: String,
    ) -> Box<dyn Fn(&USSDRequest, &str) -> HashStrAny>;
}

impl USSDServiceTrait for USSDService {
    fn new(
        functions_path: String,
        function_name: String,
        function_url: Option<String>,
        data_key: String,
    ) -> Self {
        Self {
            functions_path,
            function_name,
            function_url,
            data_key,
        }
    }

    fn call(&self, session: &mut USSDSession, request: &USSDRequest, functions_path: String) {
        // Find and load the function from the functions_path
        // Logic to load the function from the function_path (You need to implement this logic)
        let loaded_function = self.load_function(functions_path);

        // Pass the function_url as an argument to the loaded function
        let result = if let Some(url) = &self.function_url {
            loaded_function(&request, url)
        } else {
            loaded_function(&request, "")
        };

        // Save the returned result in the session data with the data_key
        session.data.insert(self.data_key.clone(), result.clone());
    }

    fn load_function(
        &self,
        _base_functions_path: String,
    ) -> Box<dyn Fn(&USSDRequest, &str) -> HashStrAny> {
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
                Box::new(|_request: &USSDRequest, _url: &str| {
                    let mut result = HashMap::new();
                    result.insert(
                        "error".to_string(),
                        HashStrAny::Str("Function not found".to_string()),
                    );
                    HashStrAny::Dict(result)
                })
            }
        }
    }
}
