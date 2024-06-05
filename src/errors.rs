// File used for all functions related to error handling, when such functions are needed
use crate::detect_color;
use crate::error_lcd;
use log::error;

/// Function that checks if the bins are full,
/// which means the robot completed it's task.
/// In case the bins are full, it will print an error message and exit the program.
pub fn check_completion(bins: &[Vec<i32>; 3]) -> () {
    if check_bins_full(bins) {
        println!("ERROR 0: Bins are full. Sorting completed.");
        error!("ERROR 0: Bins are full. Sorting completed.");
        let _ = error_lcd::display_error(0);
        std::process::exit(0);
    }
}

/// Function to check if the bins are full
/// ERROR 0
fn check_bins_full(bins: &[Vec<i32>; 3]) -> bool {
    for bin in bins {
        if bin.len() < 5 {
            return false;
        }
    }
    return true;
}

/// Function to check if the color sensor detects a colored disk.
/// Used to check for ERROR 31.
/// ERROR 31
pub fn check_color_sensor_detects(color_values: (i32, i32, i32)) -> bool {
    let color = detect_color::logic(color_values);
    if color != -1 && color != 2 {
        return true;
    }
    return false;
}
