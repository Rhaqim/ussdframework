#[derive(Debug)]
pub struct USSDConfig {
    pub functions_path: String,
    pub menu_source: String,
    pub timeout_duration: u64,
}

impl USSDConfig {
    pub fn new(functions_path: String, menu_source: String, timeout_duration: u64) -> Self {
        Self {
            functions_path,
            menu_source,
            timeout_duration
        }
    }
}