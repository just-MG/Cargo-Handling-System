mod serial_connection_con;
mod detect_color;
mod motors;
use std::{sync::mpsc, thread::current};

#[derive(Clone, Copy)]
enum State {
    Ready,
    Detecting,
    Positioning,
    Analyzing,
    Discarding,
    Sorting,
    Error,
    Paused,
}


enum Event {
    PowerOn,
    DiscDetected,
    DiscPositioned,
    DiscAnalyzed { needed: bool },
    Discarded,
    Sorted,
    OperationError,
    Reset,
    Pause,
    Resume,
}

fn next_state(current_state: State, event: Event) -> State {
    use State::*;
    use Event::*;

    match (current_state, event) {
        (Ready, PowerOn) | (Ready, Resume) => Detecting,
        (Detecting, DiscDetected) => Positioning,
        (Positioning, DiscPositioned) => Analyzing,
        (Analyzing, DiscAnalyzed { needed }) if needed => Sorting,
        (Analyzing, DiscAnalyzed { needed: false }) => Discarding,
        (Discarding, Discarded) | (Sorting, Sorted) => Ready,
        (_, OperationError) | (_, Pause) => Error,
        (Error, Reset) => Ready,
        _ => current_state,
    }
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

    let mut current_state = State::Ready;
    loop {
        match current_state {
            State::Ready => {

            },
            State::Detecting => {

            },
            State::Positioning => {
                
            },
            State::Analyzing => {
                
            },
            State::Discarding => {
                
            },
            State::Sorting => {
                
            },
            State::Error => {
                
            },
            State::Paused => {
                
            },
        }

    }
}