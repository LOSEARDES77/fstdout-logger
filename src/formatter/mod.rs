use colored::{ColoredString, Colorize};
use log::{Level, Record};

use crate::config::LoggerConfig;

/// Handles log formatting for both stdout and file outputs
pub struct LogFormatter {
    config: LoggerConfig,
}

impl LogFormatter {
    /// Create a new formatter with the given configuration
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// Get the appropriate color for a log level
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

    /// Format a log record for file output (always includes date and file info, no colors)
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
