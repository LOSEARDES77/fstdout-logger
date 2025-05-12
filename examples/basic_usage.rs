// This example demonstrates the core functionality of the fstdout-logger crate.
//
// Features shown:
// - Using the builder pattern to create a custom logger configuration
// - Logging to both stdout and a file simultaneously
// - Using all log levels (trace, debug, info, warn, error)
// - Colored console output with timestamps
// - Complete log file with timestamps and source location

use fstdout_logger::{LoggerConfig, init_logger_with_config};
use log::{LevelFilter, debug, error, info, trace, warn};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Initialize logger with file output and colored stdout
    let log_path = "application.log";

    // Create a custom configuration with the builder pattern
    // This shows how to configure every aspect of the logger
    let config = LoggerConfig::builder()
        .level(LevelFilter::Trace) // Show all log levels, including TRACE
        .show_file_info(true) // Include file and line info in logs
        .show_date_in_stdout(false) // Only show time (HH:MM:SS) in console output
        .use_colors(true) // Use colors for different log levels
        .build();

    if let Err(e) = init_logger_with_config(Some(log_path), config) {
        eprintln!("Failed to initialize logger: {e}");
        return;
    }

    println!("Logger initialized! Check {log_path} for log output.");
    println!("Log messages will appear both on stdout and in the log file.");
    println!("Notice that stdout logs show time only while the file includes dates.");

    // Log messages at different levels to demonstrate the hierarchy
    // All of these will appear because we set level to Trace
    trace!("This is a TRACE message"); // Lowest level, normally hidden
    debug!("This is a DEBUG message"); // For developer information
    info!("This is an INFO message"); // Normal application events
    warn!("This is a WARNING message"); // Important but non-critical issues
    error!("This is an ERROR message"); // Critical issues that need attention

    // Simulate some application activity
    for i in 1..=5 {
        info!("Application is running... iteration {i}");
        sleep(Duration::from_millis(500));
    }

    // Log a final message
    info!("Application finished successfully");

    println!("\nAfter running this example, check the 'application.log' file");
    println!("to see how logs are formatted differently for file output.");
}
