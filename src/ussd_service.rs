use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{prelude::USSDRequest, types::HashStrAny, ussd_session::USSDSession};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct USSDService {
    pub functions_path: String,
    pub function_name: String,
    pub function_url: Option<String>,
    pub data_key: String,
}

pub trait USSDServiceTrait {
    fn new(functions_path: String, function_name: String, function_url: Option<String>, data_key: String) -> Self;
    fn call(&self, session: &mut USSDSession) -> String;
    fn load_function(&self) -> Box<dyn Fn(&USSDRequest, &str) -> HashStrAny>;
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

    fn call(&self, session: &mut USSDSession) -> String {
        // Find and load the function from the functions_path
        // Logic to load the function from the function_path (You need to implement this logic)
        let loaded_function = self.load_function();

        let request = USSDRequest {
            msisdn: session.msisdn.clone(),
            session_id: session.session_id.clone(),
            input: "".to_string(),
            service_code: "".to_string(),
            language: "".to_string(),
        };
        
        // Pass the function_url as an argument to the loaded function
        let result = if let Some(url) = &self.function_url {
            loaded_function(&request, url)
        } else {
            loaded_function(&request, "")
        };
        
        // Save the returned result in the session data with the data_key
        session.data.insert(self.data_key.clone(), result.clone());

        // Return the result as a string
        format!("{:?}", result)
    }

    fn load_function(&self) -> Box<dyn Fn(&USSDRequest, &str) -> HashStrAny> {
        // Load the function from the functions_path
        // Logic to load the function from the function_path (You need to implement this logic)
        // This implementation currently returns a function that returns None for all function paths.
        // You need to replace this with your actual implementation.
    
        Box::new(|_request, _url| {
            // Creating a sample HashMap for demonstration
            let mut dict = HashMap::new();
            dict.insert("key1".to_string(), HashStrAny::Int(42));
            dict.insert("status".to_string(), HashStrAny::Str("success".to_string()));
    
            // Returning HashStrAny::Dict variant
            HashStrAny::new_dict(dict)
        })
    }
}
