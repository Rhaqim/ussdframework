use crate::{
    core::{interface::ussd_screen::RouterOptions, USSDSession},
    types::HashStrAny,
};

/// Handles the routing logic for the USSD screens.
///
/// This function evaluates each router option and determines the next screen based on the evaluation result.
/// If a router option evaluates to true, the current screen is updated to the next screen specified in the option.
/// If no router option evaluates to true, the current screen is updated to the default next screen.
///
/// # Arguments
///
/// * `session` - The mutable reference to the `USSDSession` struct.
/// * `router_options` - The vector of `RouterOptions` structs representing the available router options.
/// * `default_next_screen` - The string representing the default next screen when no router option evaluates to true.
///
/// # Returns
///
/// An optional string representing the next screen. If a router option evaluates to true, the next screen is returned.
/// If no router option evaluates to true, the default next screen is returned.
pub fn router_handler(
    session: &mut USSDSession,
    router_options: &Vec<RouterOptions>,
    default_next_screen: &String,
) -> Option<String> {
    // Evaluate each router option if True, next_screen = router_option.default_next_screen

    for router_option in router_options {
        if evaluate_router_options(&session, &router_option.router_option) {
            session.current_screen = router_option.next_screen.clone();
            return Some(router_option.next_screen.clone());
        }
    }

    session.current_screen = default_next_screen.clone();
    Some(default_next_screen.clone())
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
fn evaluate_router_options(session: &USSDSession, router_option: &str) -> bool {
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
        return Err("Invalid expression: Must consist of three parts separated by whitespace".to_string());
    }

    // Extract the data key from the expression
    let data_key = parts[0];

    let mut data_key_parts: Vec<&str> = Vec::new();

    // check if data key contains a dot and then split it
    if data_key.contains('.') {
        data_key_parts = data_key.split('.').collect();
        if data_key_parts.len() != 2 {
            return Err("Invalid expression: Data key must consist of one or two parts separated by a dot".to_string());
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
            Err(_) => return Err("Invalid expression: Right operand is not a valid integer".to_string()),
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
            HashStrAny::Dict(dict_value) => {
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
                        return Err("Invalid expression: Intermediate value is not a dictionary".to_string());
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
        Err(format!("Key '{}' not found in session data", data_key_parts[0]))
    }
}
