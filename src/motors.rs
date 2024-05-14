/*
Placeholder until proper implementation
*/


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
pub fn move_sort_arm_1(direction: i32) -> () {
    println!("Moving motor 1 to direction: {}", direction);
    // TODO: Implement motor movement logic here
}

pub fn move_sort_arm_2(direction: i32) -> () {
    println!("Moving motor 2 to direction: {}", direction);
    // TODO: Implement motor movement logic here
}

/*
Sort to bin 1
*/
pub fn sort_arm_to_1() -> () {
    move_sort_arm_1(1); // Move motor 1 to right
    // TODO: check movement time
    std::thread::sleep(std::time::Duration::from_secs(1)); // wait until movement is complete
}
/*
Sort to bin 2
*/
pub fn sort_arm_to_2() -> () {
    move_sort_arm_1(0); // Move motor 1 to left
    move_sort_arm_2(1); // Move motor 2 to right
    // TODO: check movement time
    std::thread::sleep(std::time::Duration::from_secs(1)); // wait until movement is complete
}
/*
Sort to bin 3
*/
pub fn sort_arm_to_3() -> () {
    move_sort_arm_1(0); // Move motor 1 to left
    move_sort_arm_2(0); // Move motor 2 to left
    // TODO: check movement time
    std::thread::sleep(std::time::Duration::from_secs(1)); // wait until movement is complete
}

// DISCARD MOTOR
pub fn discard_item() -> () {
    println!("Discarding item");
    std::thread::sleep(std::time::Duration::from_secs(1)); // wait until movement is complete
}