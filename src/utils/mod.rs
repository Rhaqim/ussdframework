use std::collections::HashMap;

use crate::info;
use crate::prelude::USSDRequest;
use crate::types::HashStrAny;
use crate::ussd_session::USSDSession;
use std::sync::Mutex;
// Define a type to store registered functions
type FunctionMap = HashMap<String, fn(&USSDRequest, &str) -> HashStrAny>;

lazy_static::lazy_static! {
    // Define a lazy static variable to store registered functions
    pub static ref FUNCTION_MAP: Mutex<FunctionMap> = Mutex::new(FunctionMap::new());
}

// Function to register functions
pub fn register_function(path: &str, function_ptr: fn(&USSDRequest, &str) -> HashStrAny) {
    info!("Registering function: {}", path);
    let function_name = path.to_string();
    FUNCTION_MAP
        .lock()
        .unwrap()
        .insert(function_name, function_ptr);
}

pub fn register_functions(functions_map: HashMap<String, fn(&USSDRequest, &str) -> HashStrAny>) {
    for (path, function) in functions_map {
        register_function(&path, function);
    }
}

/// Function to evaluate an expression
///
/// This function takes a string and a session and evaluates the string as an expression
///
/// # Arguments
///
/// * `text` - A string to be evaluated
/// * `session` - A reference to a USSD session
///
/// # Returns
///
/// * A string with the evaluated expression
///
/// # Example
///
/// ```rust
/// use crate::utils::evaluate_expression;
/// use crate::ussd_session::USSDSession;
/// use std::collections::HashMap;
///
/// let mut session = USSDSession::new();
/// let mut data = HashMap::new();
/// data.insert("name".to_string(), "John".to_string());
/// session.data = data;
///
/// let text = "Hello {{session.name}}";
/// let evaluated_text = evaluate_expression(text, &session);
/// assert_eq!(evaluated_text, "Hello John");
/// ```
pub fn evaluate_expression(text: &str, session: &USSDSession) -> String {
    let pattern = regex::Regex::new(r"\{\{(\w+)(?:\.(\w+))?\}\}").unwrap(); // Updated regex pattern
    let evaluated_text = pattern.replace_all(text, |caps: &regex::Captures| {
        // Closure to handle expression evaluation
        let object = caps.get(1).unwrap().as_str(); // Extract the object name
        let field = caps.get(2).map_or("", |m| m.as_str()); // Extract the field name if exists

        info!(
            "Evaluating expression: object: {}, field: {}",
            object, field
        );

        if let Some(data_object) = session.data.get(object) {
            match data_object {
                HashStrAny::Dict(inner_data) => {
                    if field.is_empty() {
                        // If field is empty, directly look up the object key
                        if let Some(HashStrAny::Str(value)) = inner_data.get(object) {
                            return value.to_string(); // Replace with value if found
                        }
                    } else {
                        // If field is not empty, look up object key first, then field key
                        if let Some(inner_value) = inner_data.get(field) {
                            match inner_value {
                                HashStrAny::Str(value) => {
                                    return value.to_string(); // Replace with value if found
                                }
                                _ => {}
                            }
                        }
                    }
                }
                HashStrAny::Str(value) => {
                    return value.to_string(); // Replace with value if found
                }
                _ => {}
            }
        }
        caps.get(0).unwrap().as_str().to_string() // Keep original if not found
    });
    evaluated_text.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::HashStrAny;
    use std::collections::HashMap;

    fn new_session() -> USSDSession {
        USSDSession::new(
            "1234".to_string(),
            "home".to_string(),
            "en".to_string(),
            "1234".to_string(),
        )
    }

    #[test]
    fn test_evaluate_expression() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("name".to_string(), HashStrAny::Str("John".to_string()));
        session.data = data;

        let text = "Hello {{name}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello John");
    }

    #[test]
    fn test_evaluate_expression_nested() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), HashStrAny::Str("John".to_string()));
        data.insert("session".to_string(), HashStrAny::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.name}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello John");
    }

    #[test]
    fn test_evaluate_expression_nested_field() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), HashStrAny::Str("John".to_string()));
        data.insert("session".to_string(), HashStrAny::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.name}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello John");
    }

    #[test]
    fn test_evaluate_expression_nested_field_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), HashStrAny::Str("john".to_string()));
        data.insert("session".to_string(), HashStrAny::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello {{session.age}}");
    }

    #[test]
    fn test_evaluate_expression_nested_field_not_found_object_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), HashStrAny::Str("john".to_string()));
        data.insert("session".to_string(), HashStrAny::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello {{session.age}}");
    }
}
