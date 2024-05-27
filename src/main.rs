use log::{info, debug, warn, error};
mod serial_connection_con;
mod detect_color;
mod motors;
mod sorting;
mod state_machine;
mod logging;
mod dist_sensor;
mod input;

use crate::state_machine::*;
use std::sync::mpsc;
use crate::serial_connection_con::get_nwst_color;

fn main() {
    // Initialize logging
    logging::setup_logging().expect("Failed to initialize logging");
    info!("Begin initialization");

    // Get desired output from user
    info!("Getting input from user");
    let output: [[u8; 5]; 3] = input::get_input();
    info!("Input received: {:?}", output);

    // COLOR detection initialization
    info!("Initializing color detection");
    let (tx_color, rx_color) = mpsc::channel();
    serial_connection_con::initialize_serial(tx_color); // Start the serial connection in a separate thread
    std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for the serial connection to initialize
    info!("Serial connection initialized");

    // DISTANCE sensor initialization
    info!("Initializing distance sensor");
    // TODO: Add distance sensor initialization here

    // Motor initialization
    info!("Initializing motors");
    // TODO: Add motor initialization here

    // State maschine
    let mut machine = state_machine::StateMachine::new();

    // Robot IRL variables - all time in milliseconds
    let sorting_time = 1; // time for the sorting arms to move into positions
    let positioning_time = 1; // time for the conveyor belt to position the disc under the color sensor
    let discarding_time = 1; // time for the discarding arm to move into position
    let distance_sensor_threshold = 1.0; // distance sensor threshold for detecting an object
    let distance_detection_rate = 1; // wait time between each distance sensor reading

    info!("Initialization complete");
    info!("Starting state machine");
    loop {
        match &machine.current_state {
            State::Detecting => {
                motors::start_conveyor();
                info!("Conveyor started for detecting disc");
                loop {
                    let distance = dist_sensor::get_distance(); // Placeholder for the distance sensor value
                    debug!("Checking distance: {}", distance);
                    if distance < distance_sensor_threshold {
                        info!("Disc detected at distance: {}", distance);
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(distance_detection_rate.clone()));
                }
                let event = Event::DiscDetected;
                info!("Transitioning to Positioning due to disc detection");
                machine.transition(event);
            },
            State::Positioning => {
                info!("Positioning the disc");
                std::thread::sleep(std::time::Duration::from_millis(positioning_time.clone())); // Placeholder for positioning time
                motors::stop_conveyor();
                let event = Event::DiscPositioned;
                machine.transition(event);
            },
            State::Analyzing => {
                info!("Analyzing the color of the disc");
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values.0, color_values.1, color_values.2);

                if color == 2 { // color is unknown
                    warn!("Disc color unknown, reanalyzing");
                    let event = Event::DiscUnknown;
                    machine.transition(event);
                } else if color == -1 { // color is conveyor
                    error!("Error in color detection, moving to error state");
                    machine.shared_state.disc_color = color;
                    let event = Event::Error;
                    machine.transition(event);
                } else if sorting::check_needed(&machine.shared_state.bin_status, output.clone(), &color) {
                    info!("Disc needed, sorting");
                    machine.shared_state.disc_color = color;
                    let event = Event::DiscNeeded;
                    machine.transition(event);
                } else {
                    info!("Disc not needed, discarding");
                    let event = Event::DiscNotNeeded;
                    machine.transition(event);
                }
            },
            State::Discarding => {
                info!("Discarding item");
                motors::discard_item();
                std::thread::sleep(std::time::Duration::from_secs(discarding_time.clone()));
                let event = Event::DiscDiscarded;
                machine.transition(event);
            },
            State::Sorting => {
                info!("Sorting item");
                let bin = sorting::sort_disc(&machine.shared_state.bin_status, output.clone(), &machine.shared_state.disc_color);
                motors::sort_arm(bin);
                // wait for the sorting arms to move into position
                std::thread::sleep(std::time::Duration::from_secs(sorting_time.clone()));
                motors::start_conveyor();
                let event = Event::DiscSorted;
                machine.transition(event);
            },
            State::Error => {
                // handle error
                
                // use Event::ErrorCallBack to transition back to the previous state
                let event = Event::ErrorCallBack;
                machine.transition(event);
            },
            State::Reanalyzing => {
                info!("Reanalyzing disc color");
                std::thread::sleep(std::time::Duration::from_millis(500)); // wait for new measurements
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values.0, color_values.1, color_values.2);
                if sorting::check_needed(&machine.shared_state.bin_status, output.clone(), &color) {
                    info!("Disc needed after reanalysis, sorting");
                    machine.shared_state.disc_color = color;
                    let event = Event::DiscNeeded;
                    machine.transition(event);
                } else {
                    info!("Disc not needed after reanalysis, discarding");
                    let event = Event::DiscNotNeeded;
                    machine.transition(event);
                }
            },
        }
    }
}