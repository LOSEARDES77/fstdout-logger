use fstdout_logger::LoggerConfig;
use log::{LevelFilter, debug, error, info, warn};
use std::fmt::Write;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Step 1: Compare production vs development logger output
    println!("=== PRODUCTION LOGGER SIMULATION ===");
    // Production config: no file info, INFO level minimum
    let prod_config = LoggerConfig::production();
    println!("Production settings:");
    println!(" - Log level: Info (no Debug or Trace messages)");
    println!(" - File info: Hidden (cleaner logs)");
    println!(" - Colors: Enabled (for better readability)");
    println!(" - Date in console: Hidden (time only for brevity)\n");

    // Manually format some logs with production settings
    format_and_print_logs("Production", &prod_config);

    // Reset logs
    println!("\n\n=== DEVELOPMENT LOGGER SIMULATION ===");
    // Development config: with file info, DEBUG level minimum
    let dev_config = LoggerConfig::development();
    println!("Development settings:");
    println!(" - Log level: Debug (includes debug messages)");
    println!(" - File info: Shown (helps with debugging)");
    println!(" - Colors: Enabled (for better readability)");
    println!(" - Date in console: Hidden (time only for brevity)\n");

    // Manually format logs with development settings
    format_and_print_logs("Development", &dev_config);

    println!("\nNote: In production mode, logs are more concise (no file info, only time)");
    println!(
        "but the log file still contains complete information including date and file details."
    );

    // Step 2: Initialize a real logger for demonstration
    println!("\n=== ACTUAL LOGGER IMPLEMENTATION ===");
    println!("Initializing a real logger with production settings...");
    if let Err(e) = fstdout_logger::init_production_logger(Some("prod.log")) {
        eprintln!("Failed to initialize logger: {e}");
        return;
    }

    // Log some real messages
    info!("This message was logged with the actual logger");
    debug!("Debug info won't appear in production mode");
    warn!("Warnings will appear");
    error!("Errors will appear too");

    println!("\nCheck 'prod.log' to see the file output format with full details!");
}

// Helper function to format and print example log messages
fn format_and_print_logs(context: &str, config: &LoggerConfig) {
    let now = chrono::Local::now();
    let timestamp = if config.show_date_in_stdout {
        now.format("%Y-%m-%d %H:%M:%S").to_string()
    } else {
        now.format("%H:%M:%S").to_string()
    };

    // Helper function to format a log message
    let format_log = |level: &str, colored: bool, file_info: bool, msg: &str| {
        let mut output = String::new();

        // Format the level with appropriate color if needed
        let level_str = match (colored, level) {
            (true, "ERROR") => "\x1b[1;31mERROR\x1b[0m", // Bold Red
            (true, "WARN") => "\x1b[1;33mWARN\x1b[0m",   // Bold Yellow
            (true, "INFO") => "\x1b[1;34mINFO\x1b[0m",   // Bold Blue
            (true, "DEBUG") => "\x1b[32mDEBUG\x1b[0m",   // Green
            (true, "TRACE") => "TRACE",                  // Normal
            (_, _) => level,                             // No color
        };

        // Format timestamp
        let time_part = if colored {
            format!("\x1b[90m{timestamp}\x1b[0m") // Dimmed
        } else {
            timestamp.clone()
        };

        // Include file info if needed
        if file_info {
            let file_str = if colored {
                "\x1b[90mexamples/production.rs:42\x1b[0m" // Dimmed
            } else {
                "examples/production.rs:42"
            };

            write!(output, "[{time_part} {level_str} {file_str}] {msg}").unwrap();
        } else {
            write!(output, "[{time_part} {level_str}] {msg}").unwrap();
        }

        output
    };

    // Process start
    let msg = format!("{context} process starting up");
    println!(
        "{}",
        format_log("INFO", config.use_colors, config.show_file_info, &msg)
    );

    // Debug message (will only show in development mode)
    let debug_msg = "Initializing subsystems with memory pool of 1024 bytes";
    if config.level <= LevelFilter::Debug {
        println!(
            "{}",
            format_log("DEBUG", config.use_colors, config.show_file_info, debug_msg)
        );
    }

    // Processing loop with simulated delay
    for i in 1..=3 {
        let msg = format!("Processing batch #{i}");
        println!(
            "{}",
            format_log("INFO", config.use_colors, config.show_file_info, &msg)
        );

        if i == 2 {
            let warn_msg = "Resource usage above threshold (82%)";
            println!(
                "{}",
                format_log("WARN", config.use_colors, config.show_file_info, warn_msg)
            );
        }
        sleep(Duration::from_millis(100)); // Shorter delay for demo
    }

    // Error example
    let error_msg = "Failed to connect to database: timeout after 5 seconds";
    println!(
        "{}",
        format_log("ERROR", config.use_colors, config.show_file_info, error_msg)
    );

    // Process end
    let end_msg = format!("{context} process completed");
    println!(
        "{}",
        format_log("INFO", config.use_colors, config.show_file_info, &end_msg)
    );
}
