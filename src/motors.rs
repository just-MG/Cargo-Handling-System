/*
Placeholder until proper implementation
*/
use log::{error, info};

// CONVEYOR
pub fn start_conveyor(speed: i32) -> () {
    println!("Starting conveyor at speed: {}", speed);
    // TODO: Implement conveyor start logic here
}

pub fn stop_conveyor() -> () {
    println!("Stopping conveyor");
    // TODO: Implement conveyor start logic here
}

// SORTING ARMS
/// direction: 0 - left, 1 - right
pub fn move_sort_arm_1(direction: i32) -> () {
    println!("Moving motor 1 to direction: {}", direction);
    // TODO: Implement motor movement logic here
}

/// direction: 0 - left, 1 - right
pub fn move_sort_arm_2(direction: i32) -> () {
    println!("Moving motor 2 to direction: {}", direction);
    // TODO: Implement motor movement logic here
}

/// Sets the sorting arms to the specified bin
pub fn sort_arm(bin: i32) -> () {
    match bin {
        1 => move_sort_arm_1(0), // Move arm 1 to left,
        2 => {
            move_sort_arm_1(0); // Move arm 1 to left
            move_sort_arm_2(1); // Move arm 2 to right
        },
        3 => {
            move_sort_arm_1(0); // Move arm 1 to left
            move_sort_arm_2(0); // Move arm 2 to left
        },
        _ => error!("Sorting error: invalid bin number"),
    }
}

// DISCARD MOTOR
pub fn discard_item() -> () {
    println!("Discarding item");
}