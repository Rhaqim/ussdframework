use crate::core::USSDSession;
use crate::types::{FunctionMap, USSDData, USSDFunction};
use crate::{debug, info};
use std::collections::HashSet;
use std::sync::{Arc, Mutex, MutexGuard};

lazy_static::lazy_static! {
    // Define a lazy static variable to store registered functions
    pub static ref FUNCTION_MAP: Arc<Mutex<FunctionMap>> = Arc::new(Mutex::new(FunctionMap::new()));
    pub static ref REGISTERED_FUNCTIONS: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

}

/// Registers a USSD function with the provided path.
///
/// The `register_function` function is used to register a USSD (Unstructured Supplementary Service Data) function 
/// with the specified path. USSD functions are callbacks that handle USSD requests and return the appropriate 
/// USSD data. Once registered, the function can be invoked by its path.
///
/// # Arguments
///
/// * `path`: A string representing the unique path for the USSD function. This path is used to identify 
///           the function when invoking it.
/// * `function_ptr`: A function pointer representing the USSD function to be registered. This function 
///                   should have the signature `fn(&USSDSession, &str) -> USSDData`, where `USSDSession` 
///                   represents the session data and `USSDData` represents the data to be returned 
///                   as a response.
/// * `function_map_guard`: A mutable reference to a `MutexGuard<HashMap<String, USSDFunction>>` representing 
///                          the guarded HashMap containing the registered USSD functions. This guard is used 
///                          to insert the new function into the map.
///
/// # Example
///
/// ```
/// use my_module::{register_function, USSDSession, USSDData};
///
/// fn my_function(session: &USSDSession, input: &str) -> USSDData {
///     // Implementation of the USSD function
///     unimplemented!()
/// }
///
/// // Registering the function with a specific path
/// register_function("/my-path", my_function, &mut function_map_guard);
/// ```
///
/// # Panics
///
/// This function panics if it fails to acquire the lock on the function map.
///
/// # Safety
///
/// This function is safe to call as long as the provided `function_map_guard` is a mutable reference 
/// to a locked `MutexGuard<HashMap<String, USSDFunction>>`.
///
pub fn register_function(path: &str, function_ptr: USSDFunction, function_map_guard: &mut MutexGuard<FunctionMap>) {
    info!("Registering function: {}", path);

    let function_name = path.to_string();
    function_map_guard.insert(function_name, function_ptr);
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
/// let text = "Hello {{name}}";
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
                USSDData::Dict(inner_data) => {
                    if field.is_empty() {
                        // If field is empty, directly look up the object key
                        if let Some(USSDData::Str(value)) = inner_data.get(object) {
                            return value.to_string(); // Replace with value if found
                        }
                    } else {
                        // If field is not empty, look up object key first, then field key
                        if let Some(inner_value) = inner_data.get(field) {
                            match inner_value {
                                USSDData::Str(value) => {
                                    return value.to_string(); // Replace with value if found
                                }
                                _ => {}
                            }
                        }
                    }
                }
                USSDData::Str(value) => {
                    if field.is_empty() {
                        return value.to_string(); // Replace with value if found
                    }
                }
                _ => {}
            }
        }
        caps.get(0).unwrap().as_str().to_string() // Keep original if not found
    });
    evaluated_text.to_string()
}

/// Function to evaluate an expression with an operator
///
/// This function takes a string and a session and evaluates the string as an expression with an operator
///
/// # Arguments
///
/// * `session` - A reference to a USSD session
/// * `text` - A string to be evaluated
///
/// # Returns
///
/// * A boolean representing the result of the evaluation
///
/// # Example
///
/// ```rust
/// use crate::utils::evaluate_expression_op;
/// use crate::ussd_session::USSDSession;
///
/// let session = USSDSession::new();
/// let text = "Is John? {{name == 'John'}}";
/// let result = evaluate_expression_op(&session, text);
/// assert_eq!(result, true);
/// ```
pub fn evaluate_expression_op(session: &USSDSession, text: &str) -> bool {
    let pattern_str = r"\{\{(\w+)(?:\.(\w+))?(?:\s*(==|>|>=|<|<=)\s*\'?(\w+)\'?)?\}\}";
    let pattern = regex::Regex::new(pattern_str).unwrap();
    let matched = pattern.captures(text);
    if let Some(caps) = matched {
        let object = caps.get(1).unwrap().as_str();
        let field = caps.get(2).map_or("", |m| m.as_str());
        let operator = caps.get(3).map_or("", |m| m.as_str());
        let value = caps.get(4).map_or("", |m| m.as_str());

        info!(
            "Evaluating expression: object: {}, field: {}, operator: {}, value: {}",
            object, field, operator, value
        );

        if let Some(data_object) = session.data.get(object) {
            match data_object {
                USSDData::Dict(inner_data) => {
                    if field.is_empty() {
                        debug!("Field is empty");
                        if let Some(USSDData::Str(data_value)) = inner_data.get(object) {
                            return compare_strings(operator, data_value, value);
                        }
                    } else {
                        if let Some(inner_value) = inner_data.get(field) {
                            match inner_value {
                                USSDData::Str(data_value) => {
                                    return compare_strings(operator, data_value, value);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                USSDData::Str(data_value) => {
                    return compare_strings(operator, data_value, value);
                }
                _ => {}
            }
        }
    }
    false
}

/// Function to compare strings
/// This function takes an operator and two strings and compares them
/// # Arguments
/// * `operator` - A string representing the operator
/// * `left` - A string representing the left operand
/// * `right` - A string representing the right operand
/// # Returns
/// * A boolean representing the result of the comparison
/// # Example
/// ```rust
/// use crate::utils::compare_strings;
/// let result = compare_strings("==", "John", "John");
/// assert_eq!(result, true);
/// ```
fn compare_strings(operator: &str, left: &str, right: &str) -> bool {
    info!(
        "Comparing strings: left: {}, right: {}, operator: {}",
        left, right, operator
    );
    match operator {
        "==" => left == right,
        ">" => left > right,
        ">=" => left >= right,
        "<" => left < right,
        "<=" => left <= right,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::USSDData;
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
        data.insert("name".to_string(), USSDData::Str("John".to_string()));
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
        inner_data.insert("age".to_string(), USSDData::Str("30".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "You're {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "You're 30");
    }

    #[test]
    fn test_evaluate_expression_nested_field() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), USSDData::Str("John".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
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
        inner_data.insert("name".to_string(), USSDData::Str("john".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
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
        inner_data.insert("name".to_string(), USSDData::Str("john".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello {{session.age}}");
    }

    #[test]
    fn test_evaluate_expression_nested_field_not_found_object_not_dict() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("session".to_string(), USSDData::Str("john".to_string()));
        session.data = data;

        let text = "Hello {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello {{session.age}}");
    }

    #[test]
    fn test_evaluate_expression_op() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("name".to_string(), USSDData::Str("John".to_string()));
        session.data = data;

        let text = "Is John? {{name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_gt() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 > 20? {{age > '20'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_ge() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 >= 30? {{age >= '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_lt() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 < 40? {{age < '40'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_le() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 <= 30? {{age <= '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 == 30? {{age == '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_not_found_object_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 == 30? {{name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, false);
    }

    #[test]
    fn test_evaluate_expression_op_not_found_object_not_dict() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), USSDData::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 == 30? {{age.name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, false);
    }

    #[test]
    fn test_evaluate_expression_op_not_found_object_not_str() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), USSDData::new_dict(HashMap::new()));
        data.insert("age".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Is 30 == 30? {{age.name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, false);
    }
}
