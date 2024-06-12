// File used for all functions related to error handling, when such functions are needed
use crate::detect_color;
use crate::error_lcd;
use log::error;
use rppal::gpio::Gpio;

/// Function that checks if the bins are full,
/// which means the robot completed it's task.
/// In case the bins are full, it will print an error message and exit the program.
pub fn check_completion(bins: &[Vec<i32>; 3]) -> () {
    if check_bins_full(bins) {
        println!("ERROR 0: Bins are full. Sorting completed.");
        error!("ERROR 0: Bins are full. Sorting completed.");
        let _ = error_lcd::display_error(&0);
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
pub fn check_color_sensor_detects(color_values: (i32, i32, i32)) -> bool {
    let color = detect_color::logic(color_values);
    if color == 1 || color == 0 {
        return true;
    }
    return false;
}

/// Function to check if the color sensor detects an erroneous color.
/// Used to check for ERROR 25.
pub fn check_color_sensor_erroneous(color_values: &(i32, i32, i32)) -> bool {
    if color_values.0 >= 500 || color_values.1 >= 500 || color_values.2 >= 500 {
        return true;
    }
    return false;
}

/// Function to check if the robot button has been pressed.
/// 
/// # Returns
/// 
/// A boolean value representing if the button has been pressed.
pub fn check_button_pressed() -> Result<bool, rppal::gpio::Error>{
    let gpio = Gpio::new()?;
    let button_pin = gpio.get(24)?.into_input_pullup();

    if button_pin.is_low(){
        return Ok(true);
    }
    return Ok(false);
}
