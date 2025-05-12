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

#[derive(Error, Debug)]
pub enum LogError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Failed to set logger")]
    Logger,
}

/// A logger implementation that outputs to stdout and optionally to a file
pub struct FStdoutLogger {
    /// Optional file to log to
    log_file: Option<Mutex<File>>,

    /// Formatter for log messages
    formatter: LogFormatter,
}

impl FStdoutLogger {
    /// Create a new logger with default configuration
    pub fn new<P: AsRef<Path>>(file_path: Option<P>) -> Result<Self, LogError> {
        Self::with_config(file_path, LoggerConfig::default())
    }

    /// Create a new logger with custom configuration
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

    /// Initialize the logger with the default configuration
    pub fn init(self) -> Result<(), LogError> {
        if log::set_logger(Box::leak(Box::new(self))).is_err() {
            return Err(LogError::Logger);
        }
        log::set_max_level(LevelFilter::Trace);
        Ok(())
    }

    /// Initialize the logger with a specific log level
    pub fn init_with_level(self, level: LevelFilter) -> Result<(), LogError> {
        if log::set_logger(Box::leak(Box::new(self))).is_err() {
            return Err(LogError::Logger);
        }
        log::set_max_level(level);
        Ok(())
    }
}

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

// Helper functions

/// Initialize a logger with default configuration
pub fn init_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    FStdoutLogger::new(file_path)?.init()
}

/// Initialize a logger with a specific log level
pub fn init_logger_with_level<P: AsRef<Path>>(
    file_path: Option<P>,
    level: LevelFilter,
) -> Result<(), LogError> {
    FStdoutLogger::new(file_path)?.init_with_level(level)
}

/// Initialize a logger with custom configuration
pub fn init_logger_with_config<P: AsRef<Path>>(
    file_path: Option<P>,
    config: LoggerConfig,
) -> Result<(), LogError> {
    let level = config.level;
    FStdoutLogger::with_config(file_path, config)?.init_with_level(level)
}

/// Initialize a production-ready logger (no file info, concise format)
pub fn init_production_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    init_logger_with_config(file_path, LoggerConfig::production())
}

/// Initialize a development logger (with file info, colored output)
pub fn init_development_logger<P: AsRef<Path>>(file_path: Option<P>) -> Result<(), LogError> {
    init_logger_with_config(file_path, LoggerConfig::development())
}

/// Initialize a logger that only writes to stdout (not to a file)
pub fn init_stdout_logger(config: LoggerConfig) -> Result<(), LogError> {
    init_logger_with_config(None::<String>, config)
}

/// Initialize a minimal stdout-only logger with just the specified level
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
