use std::collections::HashMap;
use std::hash::Hash;

use crate::{debug, info};
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
            object, field, operator, value);

        if let Some(data_object) = session.data.get(object) {
            match data_object {
                HashStrAny::Dict(inner_data) => {
                    if field.is_empty() {
                        debug!("Field is empty");
                        if let Some(HashStrAny::Str(data_value)) = inner_data.get(object) {
                            return compare_strings(operator, data_value, value);
                        }
                    } else {
                        if let Some(inner_value) = inner_data.get(field) {
                            match inner_value {
                                HashStrAny::Str(data_value) => {
                                    return compare_strings(operator, data_value, value);
                                }
                                _ => {}
                            }
                        }
                    }
                }
                HashStrAny::Str(data_value) => {
                    return compare_strings(operator, data_value, value);
                }
                _ => {}
            }
        }
    }
    false
}

fn compare_strings(operator: &str, left: &str, right: &str) -> bool {
    info!("Comparing strings: left: {}, right: {}, operator: {}", left, right, operator);
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
        inner_data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        data.insert("session".to_string(), HashStrAny::new_dict(inner_data));
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

    #[test]
    fn test_evaluate_expression_nested_field_not_found_object_not_dict() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("session".to_string(), HashStrAny::Str("john".to_string()));
        session.data = data;

        let text = "Hello {{session.age}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello {{session.age}}");
    }

    #[test]
    fn test_evaluate_expression_op() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("name".to_string(), HashStrAny::Str("John".to_string()));
        session.data = data;

        let text = "Is John? {{name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_gt() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 > 20? {{age > '20'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_ge() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 >= 30? {{age >= '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_lt() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 < 40? {{age < '40'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_le() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 <= 30? {{age <= '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 == 30? {{age == '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_not_found_object_not_found() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
        session.data = data;

        let text = "Is 30 == 30? {{name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, false);
    }

    #[test]
    fn test_evaluate_expression_op_not_found_object_not_dict() {
        let mut session = new_session();
        let mut data = HashMap::new();
        data.insert("age".to_string(), HashStrAny::Str("30".to_string()));
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
        inner_data.insert("name".to_string(), HashStrAny::new_dict(HashMap::new()));
        data.insert("age".to_string(), HashStrAny::new_dict(inner_data));
        session.data = data;

        let text = "Is 30 == 30? {{age.name == 'John'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, false);
    }
}


/// Evaluates the router options.
///
/// This function currently returns `true` for all router options.
/// Evaluate the router option, should contain `{{` and `}}` to be evaluated
///
/// Example:
///
/// ```
/// let router_option = "{{user.age > 18}}";
/// ```
/// The first part user is the data_key stored in the session data
/// The second part is the condition to be evaluated
///
/// # Arguments
///
/// * `session` - The reference to the `USSDSession` struct.
/// * `router_option` - The string representing the router option to be evaluated.
///
/// # Returns
///
/// A boolean value indicating whether the router option evaluates to true or false.
fn evaluate_router_option(session: &USSDSession, router_option: &str) -> bool {
    // Check if the router option contains `{{` and `}}` to indicate an expression
    if router_option.contains("{{") && router_option.contains("}}") {
        // Extract the expression inside `{{ }}`
        let expression = router_option
            .trim_start_matches("{{")
            .trim_end_matches("}}")
            .trim();

        // Here you would implement your logic to parse and evaluate the expression.
        // This implementation currently returns `true` for all router options.
        // You need to replace this with your actual implementation.

        // Parse and evaluate the expression
        match parse_and_evaluate_expression(&session, expression) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("Error evaluating router option: {}", e);
                false
            }
        };

        // For demonstration purposes, return true
        true
    } else {
        // If the router option doesn't contain `{{` and `}}`, print an error message and return false
        eprintln!("Error evaluating router option: Expression not found");
        false
    }
}

fn parse_and_evaluate_expression(session: &USSDSession, expression: &str) -> Result<bool, String> {
    // Split the expression into parts
    let parts: Vec<&str> = expression.split_whitespace().collect();
    if parts.len() != 3 {
        return Err(
            "Invalid expression: Must consist of three parts separated by whitespace".to_string(),
        );
    }

    // Extract the data key from the expression
    let data_key = parts[0];

    let mut data_key_parts: Vec<&str> = Vec::new();

    // check if data key contains a dot and then split it
    if data_key.contains('.') {
        data_key_parts = data_key.split('.').collect();
        if data_key_parts.len() != 2 {
            return Err(
                "Invalid expression: Data key must consist of one or two parts separated by a dot"
                    .to_string(),
            );
        }
    } else {
        data_key_parts.push(data_key);
    }

    // Check if the data key exists in the session data
    if let Some(data_value) = session.data.get(data_key_parts[0]) {
        // Get the operator and right operand from the expression
        let operator = parts[1];
        let right_operand: i32 = match parts[2].parse() {
            Ok(num) => num,
            Err(_) => {
                return Err("Invalid expression: Right operand is not a valid integer".to_string())
            }
        };

        // Evaluate the expression based on the data value type
        match data_value {
            HashStrAny::Str(string_value) => {
                // If the data value is a string, parse it as an integer and compare
                if let Ok(left_operand) = string_value.parse::<i32>() {
                    match operator {
                        "==" => Ok(left_operand == right_operand),
                        ">" => Ok(left_operand > right_operand),
                        "<" => Ok(left_operand < right_operand),
                        _ => Err(format!("Unsupported operator: {}", operator)),
                    }
                } else {
                    // If the string cannot be parsed as an integer, return an error
                    Err("Invalid expression: Left operand is not a valid integer".to_string())
                }
            }
            HashStrAny::Dict(_dict_value) => {
                // If the data value is a dictionary, extract nested values
                let mut current_value: &HashStrAny = data_value;
                for key_part in parts[0].split('.') {
                    if let HashStrAny::Dict(inner_dict) = current_value {
                        if let Some(next_value) = inner_dict.get(key_part) {
                            current_value = next_value;
                        } else {
                            // If the key does not exist in the dictionary, return an error
                            return Err(format!("Key '{}' not found in dictionary", key_part));
                        }
                    } else {
                        // If any intermediate value is not a dictionary, return an error
                        return Err("Invalid expression: Intermediate value is not a dictionary"
                            .to_string());
                    }
                }

                // After navigating through the nested structure, compare the value if it's an integer
                if let HashStrAny::Int(left_operand) = current_value {
                    match operator {
                        "==" => Ok(*left_operand == right_operand),
                        ">" => Ok(*left_operand > right_operand),
                        "<" => Ok(*left_operand < right_operand),
                        _ => Err(format!("Unsupported operator: {}", operator)),
                    }
                } else {
                    // If the final value is not an integer, return an error
                    Err("Invalid expression: Final value is not an integer".to_string())
                }
            }
            _ => Err("Unsupported data value type".to_string()), // Unsupported data value type
        }
    } else {
        // If the data key doesn't exist in the session data, return an error
        Err(format!(
            "Key '{}' not found in session data",
            data_key_parts[0]
        ))
    }
}