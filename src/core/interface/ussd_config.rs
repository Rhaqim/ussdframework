use super::SessionCache;

pub struct USSDConfig {
    pub functions_path: String,
    pub session_cache: Box<dyn SessionCache>,
}