// colorise log output
fn log_info(message: &str) {
    println!("\x1b[1;32m{}\x1b[0m", message);
}

fn log_error(message: &str) {
    println!("\x1b[1;31m{}\x1b[0m", message)
}

fn log_warning(message: &str) {
    println!("\x1b[1;33m{}\x1b[0m", message)
}

fn log_debug(message: &str) {
    println!("\x1b[1;34m{}\x1b[0m", message)
}

fn log_trace(message: &str) {
    println!("\x1b[1;35m{}\x1b[0m", message)
}

pub fn logger(level: &str, message: &str) {
    match level {
        "info" => log_info(message),
        "error" => log_error(message),
        "warning" => log_warning(message),
        "debug" => log_debug(message),
        "trace" => log_trace(message),
        _ => log_info(message),
    }
}

pub struct Logger {
    pub scope: String,
}

pub trait USSDLogger {
    fn info(&self, message: &str);
    fn error(&self, message: &str);
    fn warning(&self, message: &str);
    fn debug(&self, message: &str);
    fn trace(&self, message: &str);
}

impl USSDLogger for Logger {
    fn info(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("info", &msg);
    }

    fn error(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("error", &msg);
    }

    fn warning(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("warning", &msg);
    }

    fn debug(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("debug", &msg);
    }

    fn trace(&self, message: &str) {
        let msg = self.scope.clone() + ": " + message;
        logger("trace", &msg);
    }
}

#[macro_export]
/// Macro for logging an info message.
///
/// This macro takes a format string and arguments, and logs a warning message using the `logger` function from the `log` module.
/// The format string and arguments are passed to the `format!` macro to create the final log message.
///
/// # Examples
///
/// ```
/// info!("Invalid input: {}", input);
/// ```
macro_rules! info {
    ($($arg:tt)*) => ({
        $crate::log::logger("info", &format!($($arg)*));
    })
}

#[macro_export]
/// Macro for logging an error message.
///
/// This macro takes a format string and arguments, and logs a warning message using the `logger` function from the `log` module.
/// The format string and arguments are passed to the `format!` macro to create the final log message.
///
/// # Examples
///
/// ```
/// error!("Invalid input: {}", input);
/// ```
macro_rules! error {
    ($($arg:tt)*) => ({
        $crate::log::logger("error", &format!($($arg)*));
    })
}

#[macro_export]
/// Macro for logging a warning message.
///
/// This macro takes a format string and arguments, and logs a warning message using the `logger` function from the `log` module.
/// The format string and arguments are passed to the `format!` macro to create the final log message.
///
/// # Examples
///
/// ```
/// warning!("Invalid input: {}", input);
/// ```
macro_rules! warning {
    ($($arg:tt)*) => ({
        $crate::log::logger("warning", &format!($($arg)*));
    })
}

#[macro_export]
/// Macro for logging a debug message.
///
/// This macro takes a format string and arguments, and logs a warning message using the `logger` function from the `log` module.
/// The format string and arguments are passed to the `format!` macro to create the final log message.
///
/// # Examples
///
/// ```
/// debug!("Invalid input: {}", input);
/// ```
macro_rules! debug {
    ($($arg:tt)*) => ({
        $crate::log::logger("debug", &format!($($arg)*));
    })
}

#[macro_export]
/// Macro for logging a warning message.
///
/// This macro takes a format string and arguments, and logs a warning message using the `logger` function from the `log` module.
/// The format string and arguments are passed to the `format!` macro to create the final log message.
///
/// # Examples
///
/// ```
/// trace!("Invalid input: {}", input);
/// ```
macro_rules! trace {
    ($($arg:tt)*) => ({
        $crate::log::logger("trace", &format!($($arg)*));
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        logger("info", "test");
        logger("error", "test");
        logger("warning", "test");
        logger("debug", "test");
        logger("trace", "test");
    }

    #[test]
    fn test_logger_macro() {
        info!("test");
        error!("test");
        warning!("test");
        debug!("test");
        trace!("test");
    }
}
