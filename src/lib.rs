//! # FStdout Logger
//!
//! A flexible logger implementation for Rust that logs to both stdout and a file,
//! with support for colored console output and customizable formatting.
//!
//! ## Key Features
//!
//! - Log to both stdout and a file simultaneously
//! - Colored terminal output (configurable)
//! - Minimal stdout formatting (timestamp without date by default)
//! - Full file logging with timestamps and source location
//! - Multiple configuration options and presets
//!
//! ## Basic Usage
//!
//! ```rust
//! use fstdout_logger::init_logger;
//! use log::info;
//!
//! // Initialize with defaults (Info level, colors enabled, file info shown)
//! init_logger(Some("application.log")).expect("Failed to initialize logger");
//!
//! info!("Application started");
//! ```
//!
//! ## Configuration Options
//!
//! The logger can be customized using the `LoggerConfig` struct:
//!
//! ```rust
//! use fstdout_logger::{init_logger_with_config, LoggerConfig};
//! use log::LevelFilter;
//!
//! // Create a custom configuration
//! let config = LoggerConfig::builder()
//!     .level(LevelFilter::Debug)
//!     .show_file_info(false)      // Don't show file paths in stdout
//!     .show_date_in_stdout(false) // Show only time, not date in stdout
//!     .use_colors(true)           // Use colored output in terminal
//!     .build();
//!
//! init_logger_with_config(Some("debug.log"), config).expect("Failed to initialize logger");
//! ```
//!
//! ## Presets
//!
//! The library provides convenient presets for common scenarios:
//!
//! ```rust
//! // For development (Debug level, file info shown)
//! fstdout_logger::init_development_logger(Some("dev.log")).expect("Failed to initialize logger");
//!
//! // For production (Info level, no file info)
//! fstdout_logger::init_production_logger(Some("app.log")).expect("Failed to initialize logger");
//! ```

use log::{LevelFilter, Log, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;
use thiserror::Error;

mod config;
pub mod examples;
pub mod formatter;

pub use config::{LoggerConfig, LoggerConfigBuilder};
pub use formatter::LogFormatter;

/// Errors that can occur when using the logger.
#[derive(Error, Debug)]
pub enum LogError {
    /// I/O errors when opening or writing to log files.
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Errors when setting up the global logger.
    #[error("Failed to set logger")]
    Logger,
}

/// The main logger implementation that outputs to stdout and optionally to a file.
///
/// This struct implements the [`Log`] trait from the standard `log` crate,
/// handling log messages by:
///
/// 1. Writing to stdout with optional colors and formatting
/// 2. Writing to a file (if configured) with full details
///
/// # Example
///
/// ```rust
/// use fstdout_logger::{FStdoutLogger, LoggerConfig};
/// use log::LevelFilter;
///
/// // Creating a logger directly (usually done via helper functions)
/// let logger = FStdoutLogger::with_config(
///     Some("app.log"),
///     LoggerConfig::default()
/// ).expect("Failed to create logger");
///
/// // Initialize as the global logger
/// logger.init_with_level(LevelFilter::Info).expect("Failed to initialize logger");
/// ```
pub struct FStdoutLogger {
    /// Optional file to log to
    log_file: Option<Mutex<File>>,

    /// Formatter for log messages
    formatter: LogFormatter,
}

impl FStdoutLogger {
    /// Create a new logger with default configuration.
    ///
    /// This is a convenience method that uses [`LoggerConfig::default()`].
    ///
    /// # Arguments
    ///
    /// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
    ///
    /// # Returns
    ///
    /// A new logger instance or an error if the log file couldn't be opened.
    pub fn new<P: AsRef<Path>>(file_path: Option<P>) -> Result<Self, LogError> {
        Self::with_config(file_path, LoggerConfig::default())
    }

    /// Create a new logger with custom configuration.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
    /// * `config` - Configuration options for the logger.
    ///
    /// # Returns
    ///
    /// A new logger instance or an error if the log file couldn't be opened.
    pub fn with_config<P: AsRef<Path>>(
        file_path: Option<P>,
        config: LoggerConfig,
    ) -> Result<Self, LogError> {
        let log_file = match file_path {
            Some(path) => {
                let file = OpenOptions::new().create(true).append(true).open(path)?;
                Some(Mutex::new(file))
            }
            None => None,
        };

        Ok(Self {
            log_file,
            formatter: LogFormatter::new(config),
        })
    }

    /// Initialize the logger with the default configuration.
    ///
    /// This sets the maximum log level to `Trace` to enable all logs,
    /// but actual filtering will happen according to the `level` setting
    /// in the logger's configuration.
    ///
    /// # Returns
    ///
    /// `Ok(())` if initialization succeeded, or an error if it failed.
    pub fn init(self) -> Result<(), LogError> {
        if log::set_logger(Box::leak(Box::new(self))).is_err() {
            return Err(LogError::Logger);
        }
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }

    /// Initialize the logger with a specific log level.
    ///
    /// This sets the global maximum log level, overriding the level
    /// in the logger's configuration.
    ///
    /// # Arguments
    ///
    /// * `level` - The minimum log level to display.
    ///
    /// # Returns
    ///
    /// `Ok(())` if initialization succeeded, or an error if it failed.
    pub fn init_with_level(self, level: LevelFilter) -> Result<(), LogError> {
        if log::set_logger(Box::leak(Box::new(self))).is_err() {
            return Err(LogError::Logger);
        }
        log::set_max_level(level);
        Ok(())
    }
}

/// Implementation of the `Log` trait for `FStdoutLogger`.
///
/// This handles:
/// - Checking if a log message should be processed
/// - Formatting messages differently for stdout and file
/// - Writing to both destinations
/// - Flushing output streams
impl Log for FStdoutLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if !self.enabled(record.metadata()) {
            return;
        }

        // Format for stdout (with or without colors)
        let stdout_formatted = format!("{}\n", self.formatter.format_stdout(record));

        // Log to stdout
        print!("{stdout_formatted}");

        // Log to file if configured
        if let Some(file) = &self.log_file {
            if let Ok(mut file) = file.lock() {
                // Format for file (always without colors)
                let file_formatted = self.formatter.format_file(record);

                // Ignore errors when writing to file as we don't want to crash the application
                let _ = file.write_all(file_formatted.as_bytes());
            }
        }
    }

    fn flush(&self) {
        // Flush stdout
        let _ = io::stdout().flush();

        // Flush file if configured
        if let Some(file) = &self.log_file {
            if let Ok(mut file) = file.lock() {
                let _ = file.flush();
            }
        }
    }
}

//
// Helper functions for easily initializing the logger
//

/// Initialize a logger with default configuration.
///
/// This uses [`LoggerConfig::default()`] which sets:
/// - `Info` as the minimum log level
/// - File information shown in logs
/// - No date in stdout output (only time)
/// - Colors enabled for terminal output
///
/// # Arguments
///
/// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::init_logger;
/// use log::info;
///
/// init_logger(Some("app.log")).expect("Failed to initialize logger");
/// info!("Logger initialized with default settings");
///
/// ```
pub fn init_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    FStdoutLogger::new(file_path)?.init()
}

/// Initialize a logger with a specific log level.
///
/// This uses the default configuration but overrides the log level.
///
/// # Arguments
///
/// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
/// * `level` - The minimum log level to display.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::init_logger_with_level;
/// use log::LevelFilter;
///
/// init_logger_with_level(Some("debug.log"), LevelFilter::Debug)
///     .expect("Failed to initialize logger");
/// ```
pub fn init_logger_with_level<P: AsRef<Path>>(
    file_path: Option<P>,
    level: LevelFilter,
) -> Result<(), LogError> {
    FStdoutLogger::new(file_path)?.init_with_level(level)
}

/// Initialize a logger with custom configuration.
///
/// This gives full control over all configuration options.
///
/// # Arguments
///
/// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
/// * `config` - Configuration options for the logger.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::{init_logger_with_config, LoggerConfig};
/// use log::LevelFilter;
///
/// // Create a custom configuration
/// let config = LoggerConfig::builder()
///     .level(LevelFilter::Debug)
///     .show_file_info(false)
///     .use_colors(true)
///     .build();
///
/// init_logger_with_config(Some("app.log"), config)
///     .expect("Failed to initialize logger");
/// ```
pub fn init_logger_with_config<P: AsRef<Path>>(
    file_path: Option<P>,
    config: LoggerConfig,
) -> Result<(), LogError> {
    let level = config.level;
    FStdoutLogger::with_config(file_path, config)?.init_with_level(level)
}

/// Initialize a production-ready logger (no file info, concise format).
///
/// This uses [`LoggerConfig::production()`] which is optimized for
/// clean, minimal output in production environments:
/// - `Info` as the minimum log level (no debug messages)
/// - No file information shown in logs
/// - No date in stdout output (only time)
/// - Colors enabled for better readability
///
/// # Arguments
///
/// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::init_production_logger;
///
/// init_production_logger(Some("app.log"))
///     .expect("Failed to initialize production logger");
/// ```
pub fn init_production_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    init_logger_with_config(file_path, LoggerConfig::production())
}

/// Initialize a development logger (with file info, colored output).
///
/// This uses [`LoggerConfig::development()`] which is optimized for
/// detailed output during development:
/// - `Debug` as the minimum log level (shows debug messages)
/// - File information shown in logs (helps with debugging)
/// - No date in stdout output (only time)
/// - Colors enabled for better readability
///
/// # Arguments
///
/// * `file_path` - Optional path to a log file. If `None`, logs will only go to stdout.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::init_development_logger;
///
/// init_development_logger(Some("debug.log"))
///     .expect("Failed to initialize development logger");
/// ```
pub fn init_development_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    init_logger_with_config(file_path, LoggerConfig::development())
}

/// Initialize a logger that only writes to stdout (not to a file).
///
/// # Arguments
///
/// * `config` - Configuration options for the logger.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::{init_stdout_logger, LoggerConfig};
///
/// init_stdout_logger(LoggerConfig::default())
///     .expect("Failed to initialize stdout logger");
/// ```
pub fn init_stdout_logger(config: LoggerConfig) -> Result<(), LogError> {
    init_logger_with_config(None::<String>, config)
}

/// Initialize a minimal stdout-only logger with just the specified level.
///
/// This is the simplest way to get a stdout-only logger with a specific level.
///
/// # Arguments
///
/// * `level` - The minimum log level to display.
///
/// # Returns
///
/// `Ok(())` if initialization succeeded, or an error if it failed.
///
/// # Example
///
/// ```rust
/// use fstdout_logger::init_simple_stdout_logger;
/// use log::LevelFilter;
///
/// init_simple_stdout_logger(LevelFilter::Info)
///     .expect("Failed to initialize simple logger");
/// ```
pub fn init_simple_stdout_logger(level: LevelFilter) -> Result<(), LogError> {
    // Create a minimal config with the specified level
    let config = LoggerConfig {
        level,
        ..LoggerConfig::default()
    };

    // Initialize with the config
    FStdoutLogger::with_config(None::<String>, config)?.init_with_level(level)
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::{debug, error, info, trace, warn};
    use std::fs;
    use std::io::Read;

    #[test]
    fn test_stdout_logger() {
        // This test only checks that initialization doesn't fail
        let config = LoggerConfig::builder()
            .level(LevelFilter::Debug)
            .show_file_info(false)
            .build();

        let result = init_stdout_logger(config);
        assert!(result.is_ok());
    }

    #[test]
    fn test_file_logger() {
        let test_file = "test_log.txt";
        // Clean up any existing test file
        let _ = fs::remove_file(test_file);

        // Initialize logger
        let config = LoggerConfig::builder()
            .level(LevelFilter::Debug)
            .show_file_info(true)
            .use_colors(false)
            .build();

        let result = init_logger_with_config(Some(test_file), config);
        assert!(result.is_ok());

        // Log some messages
        trace!("This is a trace message");
        debug!("This is a debug message");
        info!("This is an info message");
        warn!("This is a warning message");
        error!("This is an error message");

        // Verify file contains logs
        let mut file = File::open(test_file).expect("Failed to open log file");
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .expect("Failed to read log file");

        // Debug and higher should be logged
        assert!(!contents.contains("trace message"));
        assert!(contents.contains("debug message"));
        assert!(contents.contains("info message"));
        assert!(contents.contains("warning message"));
        assert!(contents.contains("error message"));

        // Clean up
        let _ = fs::remove_file(test_file);
    }
}
