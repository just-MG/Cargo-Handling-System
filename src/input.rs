use std::io::{self, Write};
use crate::predefined_output;

/// Gets the input based on the user's choice of mode.
///
/// # Returns
/// A 2D array representing the bin contents.
/// * `[[u8; 5]; 3]` - The bin contents.
pub fn get_input() -> [[u8; 5]; 3] {
    println!("<------------------------------>");
    let mode = get_mode();
    match mode {
        Ok(1) => input().unwrap(),
        Ok(2) => get_user_selected_predefined_output(),
        Ok(3) => [[1, 0, 1, 1, 1],[1, 0, 1, 0, 1],[1, 1, 1, 0, 1]],
        _ => {
            println!("Invalid input mode");
            return get_input();
        }
    }
}

/// Asks the user if they want to continue.
///
/// # Returns
/// * `true` if the user wants to continue, otherwise `false`.
pub fn continue_input() -> bool {
    println!("Do you want to continue? (y/n)");
    let mut cont = String::new();
    io::stdin()
        .read_line(&mut cont)
        .expect("Failed to read line");
    match cont.trim() {
        "y" => true,
        "n" => false,
        _ => {
            println!("Invalid input. Please enter 'y' or 'n'.");
            continue_input()
        }
    }
}

/// Prompts the user to select a character or number for the robot to display.
/// Ensures the input is either a number or a capital letter.
///
/// # Returns
/// * A `char` representing the user's choice.
fn get_user_char() -> char {
    println!("Select a number or a capital letter you would like the robot to display:");
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        let input = input.trim();
        if let Some(c) = input.chars().next() {
            if c.is_ascii_digit() || (c.is_ascii_uppercase() && c.is_alphabetic()) {
                return c;
            }
        }
        println!("Invalid input. Please enter a number (0-9) or a capital letter (A-Z).");
    }
}

/// Gets a predefined output from the JSON file based on the user's selected character.
///
/// # Returns
/// * A 2D array representing the bin contents.
/// * `[[u8; 5]; 3]` - The bin contents.
fn get_user_selected_predefined_output() -> [[u8; 5]; 3] {
    let index = get_user_char();
    let output = predefined_output::get_predefined(index);
    visualise(&output);
    return output;
}

/// Prompts the user to select an input mode.
///
/// # Returns
/// * `io::Result<i32>` - The selected mode.
fn get_mode() -> io::Result<i32> {
    println!("Enter input mode: ");
    println!("1. Custom input");
    println!("2. Select from predefined inputs");
    println!("3. Debug mode");
    let mut mode = String::new();
    io::stdin().read_line(&mut mode)?;
    let mode: i32 = mode.trim().parse().expect("Invalid input");
    Ok(mode)
}

/// Prompts the user to input the bin contents.
///
/// # Returns
/// * `io::Result<[[u8; 5]; 3]>` - The bin contents.
fn input() -> io::Result<[[u8; 5]; 3]> {
    println!("<------------------------------>");
    println!("Color values: 0 - white, 1 - black");
    println!("Sample input: 1 1 0 0 0");
    println!("Discs are placed in the bins bottom to top");
    let mut arr: [[u8; 5]; 3] = [[0; 5]; 3];
    for i in 0..3 {
        loop {
            let mut row = String::new();
            print!("Enter the values for bin {}: ", i);
            io::stdout().flush()?; // Flush stdout to ensure the prompt is displayed before read_line
            io::stdin().read_line(&mut row)?;
            let row: Result<Vec<u8>, _> = row.split_whitespace().map(|s| s.parse()).collect();
            match row {
                Ok(row) if row.iter().all(|&x| x == 0 || x == 1) && row.len() == 5 => {
                    arr[i] = [row[0], row[1], row[2], row[3], row[4]];
                    break;
                }
                _ => {
                    println!("Invalid input. Please enter 5 values of 1 or 0.");
                }
            }
        }
    }
    visualise(&arr);
    Ok(arr)
}

/// Maps a binary value to a corresponding character for visualization.
///
/// # Arguments
/// * `x` - A binary value (0 or 1).
///
/// # Returns
/// * A `char` representing the visual representation of the binary value.
fn map(x: u8) -> char {
    match x {
        0 => 'O',
        1 => 'X',
        _ => ' ',
    }
}

/// Visualizes the bin contents in a readable format.
///
/// # Arguments
/// * `arr` - A 2D array representing the bin contents.
fn visualise(arr: &[[u8; 5]; 3]) {
    println!("Real world representation:");
    for i in 0..5 {
        for j in 0..3 {
            print!("{} ", map(arr[j][4 - i]));
        }
        println!(); // Print a newline at the end of each row
    }
}
