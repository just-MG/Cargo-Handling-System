use log::{info, debug, warn, error};
mod color_sensor;
mod detect_color;
mod motors;
mod sorting;
mod state_machine;
mod logging;
mod distance_sensor;
mod input;
mod errors;
mod error_lcd;

use crate::state_machine::*;
use std::sync::mpsc;
use crate::color_sensor::get_nwst_color;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::thread;
use crate::motors::{start_conveyor, stop_conveyor};

fn main() {
    // Initialize logging
    logging::setup_logging().expect("Failed to initialize logging");
    info!("Begin initialization");
    println!("Begin initialization");

    // Get desired output from user
    info!("Getting input from user");
    println!("Getting input from user");
    let output: [[u8; 5]; 3] = input::get_input();
    info!("Input received: {:?}", output);
    println!("Input received: {:?}", output);

    //Conveyor Thread shit
   let running = Arc::new(Mutex::new(false));
    let running_clone = Arc::clone(&running);

    // Start a thread to manage the conveyor belt
    thread::spawn(move || {
        loop {
            let should_run = {
                let lock = running_clone.lock().unwrap();
                *lock
            };
            if should_run {
                // Continuously call start_conveyor while it should be running
                if let Err(e) = start_conveyor() {
                    eprintln!("Failed to start conveyor: {}", e);
                }
            } else {
                // Stop the conveyor if it should not be running
                if let Err(e) = stop_conveyor() {
                    eprintln!("Failed to stop conveyor: {}", e);
                }
            }
            thread::sleep(Duration::from_millis(100)); // Adjust the sleep duration as needed
        }
    });

    // Function to start the conveyor
    fn start_conveyor_control(running: &Arc<Mutex<bool>>) {
        let mut run = running.lock().unwrap();
        *run = true;
    }

    // Function to stop the conveyor
    fn stop_conveyor_control(running: &Arc<Mutex<bool>>) {
        let mut run = running.lock().unwrap();
        *run = false;
    }
   

    // COLOR detection initialization
    info!("Initializing color detection");
    println!("Initializing color detection");
    let (tx_color, rx_color) = mpsc::channel();
    color_sensor::initialize_serial(tx_color); // Start the serial connection in a separate thread
    std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for the serial connection to initialize
    info!("Serial connection initialized");
    println!("Serial connection initialized");

    // State maschine
    let mut machine = state_machine::StateMachine::new();

    // Robot IRL variables - all time in milliseconds
    let sorting_time: u64 = 3000; // time for the sorting arms to move into positions
    let positioning_time: u64 = 2750; // time for the conveyor belt to position the disc under the color sensor
    let discarding_time: u64 = 10; // time for the discarding arm to move into position
    let conveyor_discard_time: u64 = 500; // time for the conveyor belt to move the disc to the discarding area
    let distance_sensor_threshold: f32 = 2.5; // distance sensor threshold for detecting an object
    let distance_detection_rate: u64 = 75; // wait time between each distance sensor reading
    let distance_detection_samples: u64 = 5; // number of samples taken and averaged by the distance sensor

    info!("Initialization complete");
    println!("Initialization complete");
    info!("Starting state machine");
    println!("Starting state machine");
    loop {
        match &machine.current_state {
            State::Detecting => {
                start_conveyor_control(&running);
                info!("Conveyor started for detecting disc");
                println!("Conveyor started for detecting disc");
                loop {
	                let distance = distance_sensor::get_distance(distance_detection_rate.clone(),
                        distance_detection_samples.clone()); // Placeholder for the distance sensor value
                    debug!("Checking distance: {}", distance);
                    if distance < distance_sensor_threshold {
                        info!("Disc detected at distance: {}", distance);
                        println!("Disc detected at distance: {}", distance);
                        info!("Moving separation servo down");
			            println!("Moving separation servo down");
			            motors::separate_input(1);
                        break;
                    }
                    // check if the color sensor detects a colored disk
                    // if so, the distance sensor must've failed to detect the disk
                    if errors::check_color_sensor_detects(get_nwst_color(&rx_color)) {
                        // ERROR 31
                        error!("ERROR31: Error in distance detection, moving to error state");
                        println!("ERROR31: Error in distance detection, moving to error state");
                        machine.shared_state.error = 31;
                        let event = Event::Error;
                        machine.transition(event);
                    }
                    std::thread::sleep(std::time::Duration::from_millis(distance_detection_rate.clone()));
                }
                let event = Event::DiscDetected;
                info!("Transitioning to Positioning due to disc detection");
                println!("Transitioning to Positioning due to disc detection");
                machine.transition(event);
            },
            State::Positioning => {
                info!("Positioning the disc");
                println!("Positioning the disc");
                std::thread::sleep(std::time::Duration::from_millis(positioning_time.clone())); // Placeholder for positioning time
		        stop_conveyor_control(&running);
                let event = Event::DiscPositioned;
                machine.transition(event);
            },
            State::Analyzing => {
                info!("Analyzing the color of the disc");
                println!( "Analyzing the color of the disc");
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values);

                info!("Disk color: {:?}", color);
                println!("Disk color: {:?}", color);
                if color == 2 { // color is unknown
                    warn!("Disc color unknown, reanalyzing");
                    println!("Disc color unknown, reanalyzing");
                    let event = Event::DiscUnknown;
                    machine.transition(event);
                } else if color == -1 { // color is conveyor
                    // ERROR 21
                    error!("ERROR21: Error during color detection, moving to error state");
                    println!("ERROR21: Error during color detection, moving to error state");
                    machine.shared_state.disc_color = color;
                    machine.shared_state.error = 21;
                    let event = Event::Error;
                    machine.transition(event);
                } else if sorting::check_needed(&machine.shared_state.bin_status, output.clone(), &color) {
                    // disk is needed
                    info!("Disc needed, sorting");
                    println!("Disc needed, sorting");
                    machine.shared_state.disc_color = color;
                    let event = Event::DiscNeeded;
                    machine.transition(event);
                } else {
                    info!("Disc not needed, discarding");
                    println!("Disc not needed, discarding");
                    let event = Event::DiscNotNeeded;
                    machine.transition(event);
                }
            },
            State::Discarding => {
                info!("Moving the disk to the discarding area");
                println!("Moving the disk to the discarding area");
                start_conveyor_control(&running);
                std::thread::sleep(std::time::Duration::from_millis(conveyor_discard_time.clone()));
                stop_conveyor_control(&running);
                info!("Discarding the disk");
                println!("Discarding the disk");
                let _ = motors::discard_item();
                std::thread::sleep(std::time::Duration::from_millis(discarding_time.clone()));
                let event = Event::DiscDiscarded;
                machine.transition(event);
            },
            State::Sorting => {
                info!("Sorting item");
                println!("Sorting item");
                let bin = sorting::sort_disc(&machine.shared_state.bin_status, output.clone(), &machine.shared_state.disc_color);
                motors::sort_arm(bin);
                machine.shared_state.bin_status[bin as usize].push(machine.shared_state.disc_color);
                // check if the robot completed its task
                errors::check_completion(&machine.shared_state.bin_status);
                start_conveyor_control(&running);
                let event = Event::DiscSorted;
                machine.transition(event);
            },
            State::Error => {
                match machine.shared_state.error {
                    31 => {
                        // let _ = error_lcd::display_error(31);
                        // wait for the press of a button to continue
                        // TODO: implement button press
                        // go to the default state, after the issue has been fixed (button pressed)
                        // let _ = error_lcd::display_clear();
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        let event = Event::Restart;
                        machine.transition(event);
                    },
                    21 => {
                        // let _ = error_lcd::display_error(21);
                        // wait for the press of a button to continue
                        // TODO: implement button press
                        // go to the default state, after the issue has been fixed (button pressed)
                        // let _ = error_lcd::display_clear();
                        std::thread::sleep(std::time::Duration::from_secs(5));
                        let event = Event::Restart;
                        machine.transition(event);

                    },
                    _ => {
                        error!("Unknown error occurred");
                        println!("Unknown error occurred");
                        let to_continue = input::continue_input();
                        if to_continue {
                            let event = Event::Restart;
                            machine.transition(event);
                        } else {
                            std::process::exit(1);
                        }
                    }
                }
                // use Event::ErrorCallBack to transition back to the previous state
                let event = Event::ErrorCallBack;
                machine.transition(event);
            },
            State::Reanalyzing => {
                info!("Reanalyzing disc color");
                println!("Reanalyzing disc color");
                // TODO: implement reanalysis, currently implemented just waiting for 500ms
                // maybe move a bit forward or backwards
                std::thread::sleep(std::time::Duration::from_millis(500)); // wait for new measurements
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values);
                if sorting::check_needed(&machine.shared_state.bin_status, output.clone(), &color) {
                    info!("Disc needed after reanalysis, sorting");
                    println!("Disc needed after reanalysis, sorting");
                    machine.shared_state.disc_color = color;
                    let event = Event::DiscNeeded;
                    machine.transition(event);
                } else {
                    info!("Disc not needed after reanalysis, discarding");
                    println!("Disc not needed after reanalysis, discarding");
                    let event = Event::DiscNotNeeded;
                    machine.transition(event);
                }
            },
        }
    }
}