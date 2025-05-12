use fstdout_logger::{LoggerConfig, init_stdout_logger};
use log::{LevelFilter, debug, error, info, trace, warn};

fn main() {
    // Initialize logger with stdout only (no file output) with colors
    let config = LoggerConfig::builder()
        .level(LevelFilter::Debug)
        .show_file_info(false) // Don't show file and line information
        .use_colors(true) // Use colors in output
        .build();

    if let Err(e) = init_stdout_logger(config) {
        eprintln!("Failed to initialize logger: {e}");
        return;
    }

    println!("Logger initialized! Output will only appear on stdout.");
    println!("Notice that TRACE level messages won't show because we set Debug as minimum level.");

    // Log messages at different levels
    trace!("This is a TRACE message - you won't see this");
    debug!("This is a DEBUG message - visible");
    info!("This is an INFO message - visible");
    warn!("This is a WARNING message - visible");
    error!("This is an ERROR message - visible");

    // Log messages with dynamic content
    for i in 1..=3 {
        let value = i * 10;
        debug!("Debug calculation: {i} * 10 = {value}");
        info!("Processing item #{i} with value {value}");
    }

    info!("Example completed");
}
