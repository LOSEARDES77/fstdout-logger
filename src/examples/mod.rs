use colored::Colorize;

/// Display example log messages with colors to demonstrate the output format
pub fn show_colored_log_examples() {
    println!("=== COLORED LOG EXAMPLES ===");

    let timestamp = chrono::Local::now().format("%H:%M:%S");
    let colored_time = timestamp.to_string().bright_black();

    println!(
        "[{} {}] This is a TRACE message",
        colored_time,
        "TRACE".normal()
    );

    println!(
        "[{} {}] This is a DEBUG message",
        colored_time,
        "DEBUG".green()
    );

    println!(
        "[{} {}] This is an INFO message",
        colored_time,
        "INFO".blue().bold()
    );

    println!(
        "[{} {}] This is a WARNING message",
        colored_time,
        "WARN".yellow().bold()
    );

    println!(
        "[{} {}] This is an ERROR message",
        colored_time,
        "ERROR".red().bold()
    );

    println!(" ");
}

/// Display example log messages without colors to demonstrate the output format
pub fn show_plain_log_examples() {
    println!("=== PLAIN TEXT LOG EXAMPLES ===");

    let timestamp = chrono::Local::now().format("%H:%M:%S");

    println!("[{timestamp} TRACE] This is a TRACE message");

    println!("[{timestamp} DEBUG] This is a DEBUG message");

    println!("[{timestamp} INFO] This is an INFO message");

    println!("[{timestamp} WARN] This is a WARNING message");

    println!("[{timestamp} ERROR] This is an ERROR message");

    println!(" ");
}

/// Display example log messages with file information
pub fn show_file_info_examples() {
    println!("=== LOGS WITH FILE INFO ===");

    let timestamp = chrono::Local::now().format("%H:%M:%S");
    let file = "examples/show_colors.rs";
    let line = 42;

    println!(
        "[{} {} {}:{}] This is a log message with file info",
        timestamp.to_string().bright_black(),
        "INFO".blue().bold(),
        file.bright_black(),
        line
    );

    println!("=== LOGS WITHOUT FILE INFO ===");

    println!(
        "[{} {}] This is a log message without file info",
        timestamp.to_string().bright_black(),
        "INFO".blue().bold()
    );

    println!(" ");
}
