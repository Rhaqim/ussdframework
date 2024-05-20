pub mod expr;
pub mod func;

pub use expr::{evaluate_expression, evaluate_expression_op};
pub use func::{register_function, FUNCTION_MAP, REGISTERED_FUNCTIONS};
