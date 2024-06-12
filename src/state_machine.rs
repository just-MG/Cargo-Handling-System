// State machine struct includes the shared state
/// The `StateMachine` struct represents the robot's state machine with the `current state` and `shared state`.
/// `shared_state` stores data that is shared across different states.
pub struct StateMachine {
    pub current_state: State,
    pub shared_state: SharedState,
}

impl StateMachine {
    /// Creates a new instance of `StateMachine`.
    ///
    /// # Returns
    /// * `StateMachine` - A new state machine instance with the initial state set to `Detecting`.
    pub fn new() -> Self {
        StateMachine {
            current_state: State::Detecting,
            shared_state: SharedState {
                bin_status: [Vec::new(), Vec::new(), Vec::new()], // current state of the output bins
                prev_state: State::Detecting,
                disc_color: 2,
                error: -1,
            },
        }
    }

    /// Handles state transitions based on the given event.
    ///
    /// # Arguments
    /// * `event` - An `Event` that triggers a state transition.
    pub fn transition(&mut self, event: Event) {
        use State::*;
        match (self.current_state, event) {
            (Detecting, Event::DiscDetected) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Positioning;
            }
            (Positioning, Event::DiscPositioned) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Analyzing;
            }
            (Analyzing, Event::DiscNeeded) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Sorting;
            }
            (Analyzing, Event::DiscNotNeeded) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Discarding;
            }
            (Analyzing, Event::DiscUnknown) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Reanalyzing;
            }
            (Discarding, Event::DiscDiscarded) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Detecting;
            }
            (Sorting, Event::DiscSorted) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Detecting;
            }
            (Reanalyzing, Event::DiscNeeded) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Sorting;
            }
            (Reanalyzing, Event::DiscNotNeeded) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Discarding;
            }
            (Error, Event::ErrorCallBack) => {
		    let temp = self.current_state;
                self.current_state = self.shared_state.prev_state;
                self.shared_state.prev_state = temp;
            }
            (Error, Event::Restart) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Detecting;
            }
            (Detecting, Event::Error) => {
                self.shared_state.prev_state = self.current_state;
		        self.current_state = Error;
            }
            (Reanalyzing, Event::Error) => {
                self.shared_state.prev_state = self.current_state;
                self.current_state = Error;
            }
            (Analyzing, Event::Error) => {
                self.shared_state.prev_state = self.current_state;
		        self.current_state = Error;
            }
            _ => (),
        }
    }
}

// Define events
/// `Event` represents various events that can trigger state transitions in the robot.
pub enum Event {
    DiscDetected,
    DiscPositioned,
    DiscNeeded,
    DiscNotNeeded,
    DiscUnknown,
    DiscDiscarded,
    DiscSorted,
    Error,
    ErrorCallBack,
    Restart,
}

// Define the shared state
/// `SharedState` contains the state that is shared across different states in the state machine.
/// It includes the current state of the output bins, the previous state, the detected disk color, and any errors.
pub struct SharedState {
    pub bin_status: [Vec<i32>; 3],
    pub prev_state: State,
    pub disc_color: i32,
    pub error: i32,
}

// Define the states with access to the shared state
/// `State` represents the different states the state machine can be in.
#[derive(Clone, Copy)]
pub enum State {
    Detecting,
    Positioning,
    Analyzing,
    Reanalyzing,
    Discarding,
    Sorting,
    Error,
}
