use serde::{Deserialize, Serialize};

use crate::{
    prelude::USSDRequest, types::HashStrAny, ussd_session::USSDSession, utils::FUNCTION_MAP,
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
        base_functions_path: String,
    ) -> Box<dyn Fn(&USSDRequest, &str) -> HashStrAny> {
        // Get the function parts

        let mut parts = self.function_name.split('.');
        let file_name = parts.next().expect("Invalid function file name");
        let function_name = parts.next().expect("Invalid function name");

        let function_path = format!("{}/{}.::{}", base_functions_path, file_name, function_name);

        let function: fn(&USSDRequest, &str) -> HashStrAny = FUNCTION_MAP
            .lock()
            .unwrap()
            .get(&function_path)
            .unwrap()
            .clone();

        Box::new(function)
    }
}
