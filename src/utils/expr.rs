use crate::core::USSDSession;
use crate::types::USSDData;
use crate::{debug, info};

fn get_nested_value(data: &USSDData, field: &[&str]) -> Option<String> {
    match data {
        USSDData::Str(value) => Some(value.clone()),
        USSDData::Dict(inner_data) => {
            if field.is_empty() {
                return None;
            }

            if let Some(inner_value) = inner_data.get(field[0]) {
                get_nested_value(inner_value, &field[1..])
            } else {
                None
            }
        }
        USSDData::ListStr(value) => Some(value.join(", ")),
        _ => None,
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
    // let pattern = regex::Regex::new(r"\{\{(\w+)(?:\.(\w+))?\}\}").unwrap(); // Updated regex pattern
    let pattern = regex::Regex::new(r"\{\{([\w.]+)\}\}").unwrap();

    let evaluated_text = pattern.replace_all(text, |caps: &regex::Captures| {
        // Closure to handle expression evaluation
        let _field = caps.get(1).unwrap().as_str(); // Extract the entire field string including dots

        let _field_parts: Vec<&str> = _field.split('.').collect();

        let object = _field_parts[0]; // Extract the object name

        let fields = &_field_parts[1..]; // Extract the field name if exists

        info!(
            "Evaluating expression: field: {} object: {} nested fields: {:?}",
            _field, object, fields
        );

        if let Some(data_object) = session.data.get(object) {
            match data_object {
                USSDData::Dict(inner_data) => {
                    if fields.is_empty() {
                        // If field is empty, directly look up the object key
                        if let Some(inner_value) = inner_data.get(object) {
                            return get_nested_value(inner_value, &[object])
                                .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string());
                        }
                    } else {
                        return get_nested_value(data_object, fields)
                            .unwrap_or_else(|| caps.get(0).unwrap().as_str().to_string());
                    }
                }
                USSDData::Str(value) => {
                    if fields.is_empty() {
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
    // let pattern_str = r"\{\{(\w+)(?:\.(\w+))?(?:\s*(==|>|>=|<|<=)\s*\'?(\w+)\'?)?\}\}";
    let pattern_str = r"\{\{([\w.]+)(?:\s*(==|>|>=|<|<=)\s*\'?(\w+)\'?)?\}\}";
    let pattern = regex::Regex::new(pattern_str).unwrap();

    let matched = pattern.captures(text);

    if let Some(caps) = matched {
        // let object = caps.get(1).unwrap().as_str();
        // let field = caps.get(2).map_or("", |m| m.as_str());
        // let operator = caps.get(3).map_or("", |m| m.as_str());
        // let value = caps.get(4).map_or("", |m| m.as_str());

        let field = caps.get(1).unwrap().as_str(); // Extract the entire field string including dots

        let _field_parts: Vec<&str> = field.split('.').collect();

        let object = _field_parts[0]; // Extract the object name

        let fields = &_field_parts[1..]; // Extract the field name if exists

        let operator = caps.get(2).map_or("", |m| m.as_str());
        let value = caps.get(3).map_or("", |m| m.as_str());

        debug!(
            "Field: {}, object: {}, fields: {:?}, operator: {}, value: {}",
            field, object, fields, operator, value
        );

        info!(
            "Evaluating expression: object: {}, field: {}, operator: {}, value: {}",
            object, field, operator, value
        );

        if let Some(data_object) = session.data.get(object) {
            match data_object {
                USSDData::Dict(inner_data) => {
                    if fields.is_empty() {
                        debug!("Field is empty");
                        if let Some(inner_value) = inner_data.get(object) {
                            let val = get_nested_value(inner_value, &[object]);
                            if let Some(val) = val {
                                return compare_strings(operator, &val, value);
                            }
                        }
                    } else {
                        debug!("Field is not empty");
                        let val = get_nested_value(data_object, fields);
                        if let Some(val) = val {
                            return compare_strings(operator, &val, value);
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
    fn test_evaluate_expression_deep_nested_field() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        let mut inner_inner_data = HashMap::new();
        inner_inner_data.insert("name".to_string(), USSDData::Str("John".to_string()));
        inner_data.insert("person".to_string(), USSDData::new_dict(inner_inner_data));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.person.name}}";
        let evaluated_text = evaluate_expression(text, &session);
        assert_eq!(evaluated_text, "Hello John");
    }

    #[test]
    fn test_evaluate_expression_deep_nested_field_3() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        let mut inner_inner_data = HashMap::new();
        let mut inner_inner_inner_data = HashMap::new();
        inner_inner_inner_data.insert("name".to_string(), USSDData::Str("John".to_string()));
        inner_inner_data.insert(
            "person".to_string(),
            USSDData::new_dict(inner_inner_inner_data),
        );
        inner_data.insert("session".to_string(), USSDData::new_dict(inner_inner_data));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Hello {{session.session.person.name}}";
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
    fn test_evaluate_expression_op_nested() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("age".to_string(), USSDData::Str("30".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Is 30? {{session.age == '30'}}";
        let evaluated_text = evaluate_expression_op(&session, text);
        assert_eq!(evaluated_text, true);
    }

    #[test]
    fn test_evaluate_expression_op_nested_field() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("name".to_string(), USSDData::Str("John".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Is John? {{session.name == 'John'}}";
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
    fn test_evaluate_expression_op_gt_nested() {
        let mut session = new_session();
        let mut data = HashMap::new();
        let mut inner_data = HashMap::new();
        inner_data.insert("age".to_string(), USSDData::Str("30".to_string()));
        data.insert("session".to_string(), USSDData::new_dict(inner_data));
        session.data = data;

        let text = "Is 30 > 20? {{session.age > '20'}}";
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
