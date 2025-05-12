use fstdout_logger::examples::{
    show_colored_log_examples, show_file_info_examples, show_plain_log_examples,
};

fn main() {
    // Demonstrate both plain text and colored log formats
    show_plain_log_examples();
    show_colored_log_examples();
    show_file_info_examples();

    println!("Note: This demonstration shows how logs appear with different configurations.");
    println!("In actual usage, you would use one of these approaches:");
    println!("  - init_logger(path) // Simple initialization with defaults");
    println!("  - init_logger_with_level(path, level) // Set specific log level");
    println!("  - init_simple_stdout_logger(level) // Simple stdout-only logger");
    println!("  - init_production_logger(path) // Production optimized (no file info)");
    println!("  - init_development_logger(path) // Development optimized (with file info)");
    println!("\nOr create a custom configuration:");
    println!("  let config = LoggerConfig::builder()");
    println!("    .level(LevelFilter::Debug)");
    println!("    .show_file_info(false)  // Hide file info for cleaner output");
    println!("    .show_date_in_stdout(false) // Show only time in stdout");
    println!("    .use_colors(true)  // Enable colored output");
    println!("    .build();");
    println!("  init_logger_with_config(path, config)");
}
