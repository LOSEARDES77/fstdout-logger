use fstdout_logger::{init_logger_with_config, LoggerConfig};
use log::{debug, error, info, trace, warn, LevelFilter};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Initialize logger with file output and colored stdout
    let log_path = "application.log";
    
    // Create a custom configuration
    let config = LoggerConfig::builder()
        .level(LevelFilter::Trace)
        .show_file_info(true)
        .show_date_in_stdout(false)
        .use_colors(true)
        .build();
        
    if let Err(e) = init_logger_with_config(Some(log_path), config) {
        eprintln!("Failed to initialize logger: {e}");
        return;
    }

    println!("Logger initialized! Check {log_path} for log output.");
    println!("Log messages will appear both on stdout and in the log file.");

    // Log some messages
    trace!("This is a TRACE message");
    debug!("This is a DEBUG message");
    info!("This is an INFO message");
    warn!("This is a WARNING message");
    error!("This is an ERROR message");

    // Simulate some application activity
    for i in 1..=5 {
        info!("Application is running... iteration {i}");
        sleep(Duration::from_millis(500));
    }

    // Log a final message
    info!("Application finished successfully");
}