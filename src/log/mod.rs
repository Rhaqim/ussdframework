//! Internal logging.
//!
//! All framework logging goes through the standard [`log`](https://docs.rs/log) facade
//! under the target `"ussdframework"`. Nothing is written anywhere unless the host
//! application installs a logger (e.g. `env_logger`, `tracing-log`), and the host
//! controls verbosity, e.g. `RUST_LOG=ussdframework=warn` or `=off`.
//!
//! Never pass user input or session data to these macros: Input screens may
//! collect PINs and other secrets.

#[doc(hidden)]
pub use ::log as __log;

/// Log target used for every message emitted by the framework.
pub const TARGET: &str = "ussdframework";

#[allow(dead_code)]
pub fn logger(level: &str, message: &str) {
    let level = match level {
        "error" => ::log::Level::Error,
        "warning" => ::log::Level::Warn,
        "debug" => ::log::Level::Debug,
        "trace" => ::log::Level::Trace,
        _ => ::log::Level::Info,
    };
    ::log::log!(target: TARGET, level, "{}", message);
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
        $crate::log::__log::info!(target: $crate::log::TARGET, $($arg)*);
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
        $crate::log::__log::error!(target: $crate::log::TARGET, $($arg)*);
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
        $crate::log::__log::warn!(target: $crate::log::TARGET, $($arg)*);
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
        $crate::log::__log::debug!(target: $crate::log::TARGET, $($arg)*);
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
        $crate::log::__log::trace!(target: $crate::log::TARGET, $($arg)*);
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
