//! Configuration options for the logger.
//!
//! This module provides the [`LoggerConfig`] struct and [`LoggerConfigBuilder`]
//! for configuring the behavior of the logger.

use log::LevelFilter;

/// Configuration for the logger.
///
/// This struct controls the behavior and appearance of logs, including:
/// - Minimum log level to display
/// - Whether to show file and line information
/// - Whether to show dates in stdout logs
/// - Whether to use colors in stdout output
///
/// # Examples
///
/// ```
/// use fstdout_logger::LoggerConfig;
/// use log::LevelFilter;
///
/// // Create with default settings
/// let default_config = LoggerConfig::default();
///
/// // Create using a builder
/// let custom_config = LoggerConfig::builder()
///     .level(LevelFilter::Debug)
///     .show_file_info(false)
///     .build();
///
/// // Create using presets
/// let prod_config = LoggerConfig::production();
/// let dev_config = LoggerConfig::development();
/// ```
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// Whether to show file and line information in log messages
    pub show_file_info: bool,

    /// Whether to show date in stdout logs (always shown in file logs)
    pub show_date_in_stdout: bool,

    /// Whether to use colors in stdout logs
    pub use_colors: bool,

    /// Minimum log level to display
    pub level: LevelFilter,
}

impl Default for LoggerConfig {
    /// Creates a default configuration with:
    /// - `show_file_info`: `true` - Show file/line information
    /// - `show_date_in_stdout`: `false` - Only show time in stdout
    /// - `use_colors`: `true` - Use colors in stdout output
    /// - `level`: `Info` - Only show Info level and above
    fn default() -> Self {
        Self {
            show_file_info: true,
            show_date_in_stdout: false,
            use_colors: true,
            level: LevelFilter::Info,
        }
    }
}

impl LoggerConfig {
    /// Create a new logger configuration with default settings.
    ///
    /// This is equivalent to calling `LoggerConfig::default()`.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new configuration builder.
    ///
    /// This returns a [`LoggerConfigBuilder`] that can be used to construct
    /// a custom configuration with a fluent API.
    ///
    /// # Example
    ///
    /// ```
    /// use fstdout_logger::LoggerConfig;
    /// use log::LevelFilter;
    ///
    /// let config = LoggerConfig::builder()
    ///     .level(LevelFilter::Debug)
    ///     .show_file_info(false)
    ///     .build();
    /// ```
    pub fn builder() -> LoggerConfigBuilder {
        LoggerConfigBuilder::default()
    }

    /// Create a new configuration optimized for production use.
    ///
    /// Production settings:
    /// - `show_file_info`: `false` - Hide file/line for cleaner logs
    /// - `show_date_in_stdout`: `false` - Only show time in stdout
    /// - `use_colors`: `true` - Keep colors for readability
    /// - `level`: `Info` - Hide Debug/Trace logs in production
    pub fn production() -> Self {
        Self {
            show_file_info: false,
            show_date_in_stdout: false,
            use_colors: true,
            level: LevelFilter::Info,
        }
    }

    /// Create a new configuration optimized for development use.
    ///
    /// Development settings:
    /// - `show_file_info`: `true` - Show file/line for debugging
    /// - `show_date_in_stdout`: `false` - Only show time in stdout
    /// - `use_colors`: `true` - Use colors for readability
    /// - `level`: `Debug` - Show Debug logs (but not Trace)
    pub fn development() -> Self {
        Self {
            show_file_info: true,
            show_date_in_stdout: false,
            use_colors: true,
            level: LevelFilter::Debug,
        }
    }
}

/// Builder for constructing a [`LoggerConfig`] using a fluent API.
///
/// This follows the builder pattern to provide a clean way to create
/// custom logger configurations.
///
/// # Example
///
/// ```
/// use fstdout_logger::LoggerConfigBuilder;
/// use log::LevelFilter;
///
/// let config = LoggerConfigBuilder::default()
///     .level(LevelFilter::Warn)
///     .show_file_info(true)
///     .show_date_in_stdout(true)
///     .use_colors(false)
///     .build();
/// ```
#[derive(Debug, Default)]
pub struct LoggerConfigBuilder {
    config: LoggerConfig,
}

impl LoggerConfigBuilder {
    /// Set whether to show file and line information in log messages.
    ///
    /// When enabled, each log message will include the source file and line
    /// number where the log was created. This is useful for debugging but
    /// can make logs more verbose for production use.
    ///
    /// Default: `true`
    pub fn show_file_info(mut self, show: bool) -> Self {
        self.config.show_file_info = show;
        self
    }

    /// Set whether to show date in stdout logs.
    ///
    /// When enabled, stdout logs will include the full date (YYYY-MM-DD).
    /// When disabled, only the time (HH:MM:SS) will be shown.
    /// Note: Log files always include the full date regardless of this setting.
    ///
    /// Default: `false`
    pub fn show_date_in_stdout(mut self, show: bool) -> Self {
        self.config.show_date_in_stdout = show;
        self
    }

    /// Set whether to use colors in stdout logs.
    ///
    /// When enabled, different log levels will be displayed in different colors:
    /// - ERROR: Red
    /// - WARN: Yellow
    /// - INFO: Blue
    /// - DEBUG: Green
    /// - TRACE: Default terminal color
    ///
    /// Note: Log files never include color codes regardless of this setting.
    ///
    /// Default: `true`
    pub fn use_colors(mut self, use_colors: bool) -> Self {
        self.config.use_colors = use_colors;
        self
    }

    /// Set the minimum log level to display.
    ///
    /// This filters log messages based on their level:
    /// - `Error`: Only errors
    /// - `Warn`: Errors and warnings
    /// - `Info`: Errors, warnings, and info
    /// - `Debug`: Errors, warnings, info, and debug
    /// - `Trace`: All log levels
    ///
    /// Default: `Info`
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.config.level = level;
        self
    }

    /// Build the final configuration.
    ///
    /// This consumes the builder and returns a [`LoggerConfig`].
    pub fn build(self) -> LoggerConfig {
        self.config
    }
}
