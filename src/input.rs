use std::io::{self, Write};

pub fn get_input() -> [[u8;5];3] {
    println!("<------------------------------>");
    let mode = get_mode();
    match mode {
        Ok(1) => input().unwrap(),
        // Ok(2) => predefined_input().unwrap(),
        Ok(3) => {[[1,0,1,1,1],[1,0,1,0,1],[1,1,1,0,0]]},
        _ => {
            println!("Invalid input mode");
            return get_input();
        }
    }
}

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

fn input() -> io::Result<[[u8; 5]; 3]> {
    println!("<------------------------------>");
    println!("Color values: 0 - white, 1 - black");
    println!("Sample input: 1 1 0 0 0");
    println!("Discs are placed in the bins bottom to top");
    let mut arr: [[u8;5];3] = [[0;5];3];
    for i in 0..3 {
        loop {
            let mut row = String::new();
            print!("Enter the values for bin {}: ", i);
            io::stdout().flush()?;  // Flush stdout to ensure the prompt is displayed before read_line
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
    // Print the array
    println!("Real world representation:");
    for i in 0..5 {
        for j in 0..3 {
            print!("{} ", map(arr[j][4-i]));
        }
        println!();  // Print a newline at the end of each row
    }
    Ok(arr)
}

fn map (x: u8) -> char {
    match x {
        0 => 'O',
        1 => 'X',
        _ => ' '
    }
}