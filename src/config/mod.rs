use log::LevelFilter;

/// Configuration for the logger
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
    /// Create a new logger configuration with default settings
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Create a new configuration builder
    pub fn builder() -> LoggerConfigBuilder {
        LoggerConfigBuilder::default()
    }
    
    /// Create a new configuration optimized for production use
    pub fn production() -> Self {
        Self {
            show_file_info: false,
            show_date_in_stdout: false,
            use_colors: true,
            level: LevelFilter::Info,
        }
    }
    
    /// Create a new configuration optimized for development use
    pub fn development() -> Self {
        Self {
            show_file_info: true,
            show_date_in_stdout: false,
            use_colors: true,
            level: LevelFilter::Debug,
        }
    }
}

/// Builder for LoggerConfig
#[derive(Debug, Default)]
pub struct LoggerConfigBuilder {
    config: LoggerConfig,
}

impl LoggerConfigBuilder {
    /// Set whether to show file and line information
    pub fn show_file_info(mut self, show: bool) -> Self {
        self.config.show_file_info = show;
        self
    }
    
    /// Set whether to show date in stdout logs
    pub fn show_date_in_stdout(mut self, show: bool) -> Self {
        self.config.show_date_in_stdout = show;
        self
    }
    
    /// Set whether to use colors in stdout logs
    pub fn use_colors(mut self, use_colors: bool) -> Self {
        self.config.use_colors = use_colors;
        self
    }
    
    /// Set the minimum log level
    pub fn level(mut self, level: LevelFilter) -> Self {
        self.config.level = level;
        self
    }
    
    /// Build the configuration
    pub fn build(self) -> LoggerConfig {
        self.config
    }
}