# FStdout Logger

A simple and flexible dual-destination logger for Rust applications that implements the `log` crate's API.

[![Latest Version](https://img.shields.io/crates/v/fstdout-logger.svg)](https://crates.io/crates/fstdout-logger)
[![Documentation](https://docs.rs/fstdout-logger/badge.svg)](https://docs.rs/fstdout-logger)

## Features

- Log messages to both stdout and a file simultaneously
- Configurable log levels (Trace, Debug, Info, Warn, Error)
- Colored output for stdout (with different colors for each log level)
- Plain text output for log files (no color codes)
- Compact timestamps in stdout (only time, HH:MM:SS)
- Complete timestamps in log files (includes date)
- Optional file and line number information
- Highly configurable via simple builder API
- Compatible with the standard `log` crate macros

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
fstdout-logger = "0.1.0"
log = "0.4"
```

The crate uses the following dependencies internally:
- `colored` for terminal coloring
- `chrono` for timestamp formatting
- `thiserror` for error handling

## Usage

### Simple Usage

```rust
use fstdout_logger::init_logger;
use log::{debug, error, info, warn};

fn main() {
    // Initialize logger with defaults (colored output, Info level)
    if let Err(e) = init_logger(Some("application.log")) {
        eprintln!("Failed to initialize logger: {}", e);
        return;
    }

    // Now use the standard log macros
    info!("Application started");         // Blue
    debug!("This won't show by default"); // Suppressed (below Info level)
    warn!("Warning: resource usage high"); // Yellow
    error!("Failed to process item: {}", "invalid format"); // Red
}
```

### Using Presets

For common use cases, presets are available:

```rust
use fstdout_logger::{init_development_logger, init_production_logger};
use log::{debug, error, info};

fn main() {
    // For development: Debug level with file info and colors
    init_development_logger(Some("dev.log")).expect("Failed to initialize logger");
    
    // Or for production: Info level without file info, concise timestamps
    // init_production_logger(Some("app.log")).expect("Failed to initialize logger");
    
    debug!("Debug info shows in development mode"); // Shows in development, hidden in production
    info!("Application running");
    error!("Something went wrong!");
}
```

### Custom Configuration

For full control, use the configuration builder:

```rust
use fstdout_logger::{init_logger_with_config, LoggerConfig, LoggerConfigBuilder};
use log::{info, LevelFilter};

fn main() {
    // Create a custom configuration
    let config = LoggerConfig::builder()
        .level(LevelFilter::Info)
        .show_file_info(false)      // Hide file and line info for cleaner output
        .show_date_in_stdout(false) // Show only time in stdout (HH:MM:SS)
        .use_colors(true)           // Enable colored output
        .build();
    
    // Initialize with the custom config
    init_logger_with_config(Some("application.log"), config)
        .expect("Failed to initialize logger");
    
    info!("Customized logging experience!");
}
```

## Output Format

### Terminal Output (Default)

By default, terminal output shows a concise format with colors:

```
[HH:MM:SS LEVEL file:line] Message
```

For example:

```
[14:23:45 INFO main.rs:25] Application started
```

Log messages are colored according to log level:

- `ERROR`: Bold Red
- `WARN`: Bold Yellow
- `INFO`: Bold Blue
- `DEBUG`: Green
- `TRACE`: Default terminal color

The timestamp and file information are displayed in a dimmed color to make the log level and message stand out.

### File Output (Always Plain Text)

Log messages in files always include the date and file information:

```
[YYYY-MM-DD HH:MM:SS LEVEL file:line] Message
```

For example:

```
[2023-05-15 14:23:45 INFO main.rs:25] Application started
```

### Configuration Options

You can configure the output format through the `LoggerConfig`:

- `show_file_info` - Toggle display of file and line information
- `show_date_in_stdout` - Toggle inclusion of date in terminal output
- `use_colors` - Enable or disable colored output in terminal
- `level` - Set the minimum log level to display

## Run Examples

The crate includes examples that demonstrate its usage:

```bash
# Simple logging with custom configuration
cargo run --example basic_usage

# Stdout-only logging with minimal format
cargo run --example stdout_only

# Comparing different color and format options
cargo run --example color_options

# Demonstrating production vs development presets
cargo run --example production
```

## Full API

The logger provides several initialization functions for different use cases:

- `init_logger(path)` - Simple initialization with defaults
- `init_logger_with_level(path, level)` - Set a specific log level
- `init_logger_with_config(path, config)` - Use a custom configuration
- `init_production_logger(path)` - Use production-optimized settings
- `init_development_logger(path)` - Use development-optimized settings
- `init_stdout_logger(config)` - Initialize a stdout-only logger
- `init_simple_stdout_logger(level)` - Initialize a minimal stdout-only logger

## License

This project is licensed under the MIT License - see the LICENSE file for details.