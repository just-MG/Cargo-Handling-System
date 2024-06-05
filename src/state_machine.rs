// State machine struct includes the shared state
pub struct StateMachine {
    pub current_state: State,
    pub shared_state: SharedState,
}

impl StateMachine {
    pub fn new() -> Self {
        StateMachine {
            current_state: State::Detecting,
            shared_state: SharedState {
                bin_status: [Vec::new(), Vec::new(), Vec::new()],
                prev_state: State::Detecting,
                disc_color: 2,
                error: -1,
            },
        }
    }

    // Transition function with access to shared state
    pub fn transition(&mut self, event: Event) {
        use State::*;
        match (self.current_state, event) {
            (Detecting, Event::DiscDetected) => {
                self.current_state = Positioning;
                self.shared_state.prev_state = self.current_state;
            },
            (Positioning, Event::DiscPositioned) => {
                self.current_state = Analyzing;
                self.shared_state.prev_state = self.current_state;
            },
            (Analyzing, Event::DiscNeeded) => {
                self.current_state = Sorting;
                self.shared_state.prev_state = self.current_state;
            },
            (Analyzing, Event::DiscNotNeeded) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = self.current_state;
            }
            (Analyzing, Event::DiscUnknown) => {
                self.current_state = Reanalyzing;
                self.shared_state.prev_state = self.current_state;
            },
            (Discarding, Event::DiscDiscarded) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = self.current_state;
            },
            (Sorting, Event::DiscSorted) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = self.current_state;
            },
            (Reanalyzing, Event::DiscUnknown) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = self.current_state;
            },
            (Reanalyzing, Event::DiscNeeded) => {
                self.current_state = Sorting;
                self.shared_state.prev_state = self.current_state;
            },
            (Reanalyzing, Event::DiscNotNeeded) => {
                self.current_state = Discarding;
                self.shared_state.prev_state = self.current_state;
            },
            (Error, Event::ErrorCallBack) => {
                self.current_state = self.shared_state.prev_state;
                self.shared_state.prev_state = self.current_state;
            },
            (Error, Event::Restart) => {
                self.current_state = Detecting;
                self.shared_state.prev_state = self.current_state;
            },
            (_, Event::Error) => {
                self.current_state = Error;
                self.shared_state.prev_state = self.current_state;
            },
            _ => (),
        }
    }
}

// Define events
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
    Restart
}

// Define the shared state
pub struct SharedState {
    pub bin_status: [Vec<i32>; 3],
    pub prev_state: State,
    pub disc_color: i32,
    pub error: i32
}

// Define the states with access to the shared state
#[derive(Clone, Copy)]
pub enum State {
    Detecting,
    Positioning,
    Analyzing,
    Reanalyzing,
    Discarding,
    Sorting,
    Error
}
