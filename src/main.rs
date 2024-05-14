#[warn(unused_mut)]
mod serial_connection_con;
mod detect_color;
mod motors;
use std::{sync::mpsc, thread::current};
mod sorting;

// Define the shared state
struct SharedState {
    bin_status: (Vec<i32>, Vec<i32>, Vec<i32>),
}

// Define the states with access to the shared state
#[derive(Clone, Copy)]
enum State {
    Ready,
    Detecting,
    Positioning,
    Analyzing,
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
            },
        }
    }

    // Transition function with access to shared state
    fn transition(&mut self, event: Event) {
        use State::*;
        match (self.current_state, event) {
            (Ready, Event::Start) => self.current_state = Detecting,
            (Detecting, Event::DiscDetected) => {
                // Access or modify the shared state
                // self.shared_state.bin_status += 1;
                self.current_state = Positioning;
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
    Error,
}


fn main() {
    // COLOR detection initialization
    // let (tx_color, rx_color) = mpsc::channel();
    // serial_connection_con::initialize_serial(tx_color); // Start the serial connection in a separate thread
    // std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for the serial connection to initialize
    // DISTANCE sensor initialization
    // TODO: Add distance sensor initialization here

    // Motor initialization
    // TODO: Add motor initialization here

    // 'Global' variables

    let mut machine = StateMachine::new();
    loop {
        match &machine.current_state {
            State::Ready => {
                // Wait for the start event
                let event = Event::Start;
                print!("Starting the machine");
                machine.transition(event);
            },
            State::Detecting => {
                // Wait for the disc detection event
                let event = Event::DiscDetected;
                print!("Disc detected");
                machine.transition(event);
            },
            State::Positioning => {
                // Position the sorting arms based on the detected disc
                print!("Positioning the arms");
        }
            State::Analyzing => todo!(),
            State::Discarding => todo!(),
            State::Sorting => todo!(),
            State::Error => todo!(),
    }
    }
}