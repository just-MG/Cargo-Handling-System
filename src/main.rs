mod serial_connection_con;
mod detect_color;
mod motors;
mod sorting;
mod state_machine;
use crate::state_machine::*;

use std::sync::mpsc;
use crate::serial_connection_con::get_nwst_color;


fn main() {
    // COLOR detection initialization
    let (tx_color, rx_color) = mpsc::channel();
    serial_connection_con::initialize_serial(tx_color); // Start the serial connection in a separate thread
    std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for the serial connection to initialize

    // DISTANCE sensor initialization
    // TODO: Add distance sensor initialization here

    // Motor initialization
    // TODO: Add motor initialization here

    // 'Global' variables
    let pattern_index = 0; //placeholder
    let mut machine = state_machine::StateMachine::new();

    // Robot IRL variables - all time in miliseconds
    let speed = 1; // speed of the conveyor belt
    let sorting_time = 1; // time for the sorting arms to move into positions
    let positioning_time = 1; // time for the conveyor belt to position the disc under the color sensor
    let discarding_time = 1; // time for the discarding arm to move into position
    let distance_sensor_threshold = 1; // distance sensor threshold for detecting an object
    let distance_detection_rate = 1; // wait time between each distance sensor reading
    loop {
        match &machine.current_state {
            State::Ready => {
                // Wait for the start event
                // motors::start_conveyor(speed.clone());
                let event = Event::Start;
                println!("Initialization complete");
                machine.transition(event);
            },
            State::Detecting => {
                // Wait for the disc detection event from the distance sensor
                motors::start_conveyor(speed.clone());
                loop {
                    let distance = 0; // Placeholder for the distance sensor value
                    if distance < distance_sensor_threshold { // check if an object is detected
                        break;
                    }
                    std::thread::sleep(std::time::Duration::from_millis(distance_detection_rate.clone()));
                }
                let event = Event::DiscDetected;
                println!("Disc detected");
                machine.transition(event);
            },
            State::Positioning => {
                // Position the conveyor belt based on the detected disc
                std::thread::sleep(std::time::Duration::from_millis(positioning_time.clone())); // Placeholder for positioning time
                motors::stop_conveyor();
                println!("Disc positioned");
                let event = Event::DiscPositioned;
                machine.transition(event);
        }
            State::Analyzing => {
                // Analyze the color of the detected disc
                println!("Analyzing the color");
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values.0, color_values.1, color_values.2);
                // if the disc color is unknown, reanalyze it
                if color == 2 {
                    let event = Event::DiscUnknown;
                    machine.transition(event);
                }
                // if the disc color is conveyor, go to Error
                if color == -1 {
                    machine.shared_state.disc_color = color;
                    let event = Event::Error;
                    machine.transition(event);
                }
                // if the disc color is detected and it is needed, transition to Sorting
                if sorting::check_needed(&machine.shared_state.bin_status, &pattern_index, &color) {
                    machine.shared_state.disc_color = color;
                    let event = Event::DiscNeeded;
                    machine.transition(event);
                } else {
                    // color is known, but the disc is not needed, transition to Discarding
                    let event = Event::DiscNotNeeded;
                    machine.transition(event);
                }
            },
            State::Discarding => {
                motors::discard_item();
                std::thread::sleep(std::time::Duration::from_secs(discarding_time.clone()));
                let event = Event::DiscDiscarded;
                machine.transition(event);
            },
            State::Sorting => {
                let bin = sorting::sort_disc(&machine.shared_state.bin_status, &pattern_index, &machine.shared_state.disc_color);
                match bin {
                    1 => motors::sort_arm_to_1(),
                    2 => motors::sort_arm_to_2(),
                    3 => motors::sort_arm_to_3(),
                    _ => (), // error somewhere
                }
                std::thread::sleep(std::time::Duration::from_secs(sorting_time.clone()));
                motors::start_conveyor(speed.clone());
                let event = Event::DiscSorted;
                machine.transition(event);
            },
            State::Error => {
                // Placeholder for error handling
                println!("Error occurred");
                break;
            },
            State::Reanalyzing => {
                // We retry to get the disc color
                std::thread::sleep(std::time::Duration::from_millis(500)); // wait for new measurements
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values.0, color_values.1, color_values.2);
                if color == 2 { // disk is still unknown

                }
            },
    }
    }
}