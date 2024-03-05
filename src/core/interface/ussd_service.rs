use serde::{Deserialize, Serialize};

use super::UssdSession;

#[derive(Debug, Deserialize, Serialize)]
pub struct USSDService {
    pub functions_path: String,
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
}

pub trait USSDServiceTrait {
    fn new(functions_path: String, function_name: String, function_url: Option<String>, data_key: String) -> Self;
    fn call(&self, session: &UssdSession) -> String;
}

impl USSDServiceTrait for USSDService {
    fn new(functions_path: String, function_name: String, function_url: Option<String>, data_key: String) -> Self {
        Self {
            functions_path,
            function_name,
            function_url,
            data_key,
        }
    }

    fn call(&self, session: &UssdSession) -> String {
        let function = self.function_name.clone();
        let data_key = self.data_key.clone();
        let function_url = self.function_url.clone();
        let functions_path = self.functions_path.clone();

        if let Some(url) = function_url {
            // Call the function using the URL
            // Example: call_function_from_url(url, session, data_key)
            format!("Calling function {} from URL: {}", function, url)
        } else {
            // Call the function from the local file
            // Example: call_function_from_file(functions_path, function, session, data_key)
            format!("Calling function {} from file: {}", function, functions_path)
        }
    }
}