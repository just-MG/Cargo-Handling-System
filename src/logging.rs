use fern::Dispatch;
use log::LevelFilter;
use chrono::{Local, DateTime};
use std::fs::File;

pub fn setup_logging() -> Result<(), fern::InitError> {
    // Generate a timestamp string
    let now: DateTime<Local> = Local::now();
    let timestamp = now.format("%Y-%m-%d_%H-%M-%S").to_string();
    let log_file_name = format!("logs/run_{}.log", timestamp);

    // Create log file explicitly to handle errors here
    let log_file = File::create(&log_file_name)
        .expect("Failed to create log file");

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
