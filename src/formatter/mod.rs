//! Log message formatting functionality.
//!
//! This module is responsible for formatting log messages for display in the terminal
//! and for writing to log files. It handles colored output, timestamp formatting,
//! and determining which information to include in log messages.

use colored::{ColoredString, Colorize};
use log::{Level, Record};

use crate::config::LoggerConfig;

/// Handles log formatting for both stdout and file outputs.
///
/// This struct is responsible for:
/// - Formatting log messages differently for stdout and file outputs
/// - Applying colors to terminal output when enabled
/// - Including/excluding file information based on configuration
/// - Managing date/time formatting in log messages
pub struct LogFormatter {
    /// The configuration that controls formatting behavior
    config: LoggerConfig,
}

impl LogFormatter {
    /// Create a new formatter with the given configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration options that control formatting behavior
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Get the appropriate color for a log level.
    ///
    /// Returns a `ColoredString` with the appropriate color for the given log level,
    /// or a plain string if colors are disabled.
    ///
    /// # Colors Used
    ///
    /// - `Error`: Bold Red
    /// - `Warn`: Bold Yellow
    /// - `Info`: Bold Blue
    /// - `Debug`: Green
    /// - `Trace`: Normal terminal color
    ///
    /// # Arguments
    ///
    /// * `level` - The log level to get the color for
    fn get_level_color(&self, level: Level) -> ColoredString {
        if !self.config.use_colors {
            return level.as_str().normal();
        }

        match level {
            Level::Error => "ERROR".red().bold(),
            Level::Warn => "WARN".yellow().bold(),
            Level::Info => "INFO".blue().bold(),
            Level::Debug => "DEBUG".green(),
            Level::Trace => "TRACE".normal(),
        }
    }

    /// Format a log record for stdout
    pub fn format_stdout(&self, record: &Record) -> String {
        let now = chrono::Local::now();

        // Format timestamp (HH:MM:SS) without date for stdout
        let timestamp = if self.config.show_date_in_stdout {
            now.format("%Y-%m-%d %H:%M:%S").to_string()
        } else {
            now.format("%H:%M:%S").to_string()
        };

        // Get colored log level
        let level_str = self.get_level_color(record.level());

        // Format with or without file info
        if self.config.show_file_info {
            let file = record.file().unwrap_or("unknown");
            let line = record.line().unwrap_or(0);

            if self.config.use_colors {
                let file_info = format!("{file}:{line}").bright_black();
                format!(
                    "[{} {} {}] {}",
                    timestamp.bright_black(),
                    level_str,
                    file_info,
                    record.args()
                )
            } else {
                format!(
                    "[{} {} {}:{}] {}",
                    timestamp,
                    level_str,
                    file,
                    line,
                    record.args()
                )
            }
        } else {
            // Simpler format without file info
            if self.config.use_colors {
                format!(
                    "[{} {}] {}",
                    timestamp.bright_black(),
                    level_str,
                    record.args()
                )
            } else {
                format!("[{} {}] {}", timestamp, level_str, record.args())
            }
        }
    }

    /// Format a log record for file output.
    ///
    /// This creates a formatted log message for writing to a log file.
    /// It always includes:
    /// - Full date and time (YYYY-MM-DD HH:MM:SS)
    /// - File and line information
    /// - Plain text (no color codes)
    ///
    /// # Format
    ///
    /// `[YYYY-MM-DD HH:MM:SS LEVEL file:line] message\n`
    ///
    /// # Arguments
    ///
    /// * `record` - The log record to format
    ///
    /// # Returns
    ///
    /// A formatted string ready for writing to a file (includes trailing newline)
    pub fn format_file(&self, record: &Record) -> String {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
        let file = record.file().unwrap_or("unknown");
        let line = record.line().unwrap_or(0);

        format!(
            "[{} {} {}:{}] {}\n",
            timestamp,
            record.level(),
            file,
            line,
            record.args()
        )
    }
}
