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
    fn call(&self, session: &mut UssdSession) -> String;
    fn load_function(&self) -> Box<dyn Fn(&str) -> String>;
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

    fn call(&self, session: &mut UssdSession) -> String {
        // Find and load the function from the functions_path
        // Logic to load the function from the function_path (You need to implement this logic)
        let loaded_function = self.load_function();
        
        // Pass the function_url as an argument to the loaded function
        let result = if let Some(url) = &self.function_url {
            loaded_function(url)
        } else {
            loaded_function("")
        };
        
        // Save the returned result in the session data with the data_key
        session.data.insert(self.data_key.clone(), result.clone());

        result
    }

    fn load_function(&self) -> Box<dyn Fn(&str) -> String> {
        // Logic to load the function from the function_path (You need to implement this logic)
        // For now, we will return a dummy function
        Box::new(|url| {
            format!("Function loaded with url: {}", url)
        })
    }
}
