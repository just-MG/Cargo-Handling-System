#[warn(unused_mut)]
mod serial_connection_con;
mod detect_color;
mod motors;
use std::{sync::mpsc, thread::current};

use crate::serial_connection_con::get_nwst_color;
mod sorting;

// Define the shared state
struct SharedState {
    bin_status: (Vec<i32>, Vec<i32>, Vec<i32>),
    prev_state: State,
    disc_color: i32
}

// Define the states with access to the shared state
#[derive(Clone, Copy)]
enum State {
    Ready,
    Detecting,
    Positioning,
    Analyzing,
    Reanalyzing,
    Discarding,
    Sorting,
    Error,
}

// State machine struct includes the shared state
struct StateMachine {
    current_state: State,
    shared_state: SharedState,
}

impl StateMachine {
    fn new() -> Self {
        StateMachine {
            current_state: State::Ready,
            shared_state: SharedState {
                bin_status: (Vec::new(), Vec::new(), Vec::new()),
                prev_state: State::Ready,
                disc_color: 2
            },
        }
    }

    // Transition function with access to shared state
    fn transition(&mut self, event: Event) {
        use State::*;
        match (self.current_state, event) {
            (Ready, Event::Start) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = Ready;
            },
            (Detecting, Event::DiscDetected) => {
                self.current_state = Positioning;
                self.shared_state.prev_state = Detecting;
            },
            (Positioning, Event::DiscPositioned) => {
                self.current_state = Analyzing;
                self.shared_state.prev_state = Positioning;
            },
            (Analyzing, Event::DiscNeeded) => {
                self.current_state = Sorting;
                self.shared_state.prev_state = Analyzing;
            },
            (Analyzing, Event::DiscNotNeeded) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = Analyzing;
            },
            (Analyzing, Event::DiscUnknown) => {
                self.current_state = Reanalyzing;
                self.shared_state.prev_state = Analyzing;
            },
            (Discarding, Event::DiscDiscarded) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = Discarding;
            },
            (Sorting, Event::DiscSorted) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = Sorting;
            },
            (Reanalyzing, Event::DiscUnknown) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = Reanalyzing;
            },
            (Reanalyzing, Event::DiscNeeded) => {
                self.current_state = Sorting;
                self.shared_state.prev_state = Reanalyzing;
            },
            (Reanalyzing, Event::DiscNotNeeded) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = Reanalyzing;
            },
            // Other transitions...
            _ => (),
        }
    }
}

// Define events
enum Event {
    Start,
    DiscDetected,
    DiscPositioned,
    DiscNeeded,
    DiscNotNeeded,
    DiscUnknown,
    DiscDiscarded,
    DiscSorted,
    MultipleElements,
    Error, // placeholder? for all errors
}


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
    let mut machine = StateMachine::new();
    loop {
        match &machine.current_state {
            State::Ready => {
                // Wait for the start event
                let event = Event::Start;
                print!("Initialization complete");
                machine.transition(event);
            },
            State::Detecting => {
                // Wait for the disc detection event from the distance sensor
                let event = Event::DiscDetected;
                print!("Disc detected");
                machine.transition(event);
            },
            State::Positioning => {
                // Position the conveyor belt based on the detected disc
                print!("Positioning the arms");
                let event = Event::DiscPositioned;
                machine.transition(event);
        }
            State::Analyzing => {
                // Analyze the color of the detected disc
                print!("Analyzing the color");
                let color_values = get_nwst_color(&rx_color);
                let color = detect_color::logic(color_values.0, color_values.1, color_values.2);
                // if the disc color is unknown, reanalyze it
                if color == 2 {
                    let event = Event::DiscUnknown;
                    machine.transition(event);
                }
                // if the disc color is conveyor, go to Error
                if color == -1 {
                    let event = Event::Error;
                    machine.transition(event);
                }
                // if the disc color is detected and it is needed, transition to Sorting
                if sorting::check_needed(&machine.shared_state.bin_status, &pattern_index, &color) {
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
                let event = Event::DiscDiscarded;
                machine.transition(event);
            },
            State::Sorting => {
                
            },
            State::Error => todo!(),
            State::Reanalyzing => todo!(),
    }
    }
}