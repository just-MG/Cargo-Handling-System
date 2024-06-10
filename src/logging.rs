use chrono::{DateTime, Local};
use fern::Dispatch;
use log::LevelFilter;
use std::fs::File;

/// Sets up logging for the robot using the `fern` crate.
///
/// # Description
///
/// This function configures logging for the robot to output log messages to a file with a
/// timestamp in its name. The log file is created in the "logs" directory with a filename pattern
/// of "run_YYYY-MM-DD_HH-MM-SS.log", where the timestamp is the current date and time. The log
/// messages are formatted to include the timestamp, target, log level, and the message itself.
///
/// The global logging level is set to `Debug`, ensuring that all messages at this level and above
/// are logged. If the log file cannot be created, the function will panic.
///
/// # Example
///
/// ```rust
/// setup_logging().expect("Failed to initialize logging");
/// ```
pub fn setup_logging() -> Result<(), fern::InitError> {
    // Generate a timestamp string
    let now: DateTime<Local> = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_name = format!("logs/run_{}.log", timestamp);

    // Create log file explicitly to handle errors here
    let log_file = File::create(&log_file_name).expect("Failed to create log file");

    // Configure fern
    Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                Local::now().format("[%Y-%m-%d %H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(LevelFilter::Debug) // Set the global logging level
        .chain(log_file) // Log to a file with timestamp in the name
        .apply()?;

    Ok(())
}
