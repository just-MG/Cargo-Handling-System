use std::io::{self, Write};

pub fn input() -> io::Result<[[u8; 5]; 3]> {
    let mut arr = [[0u8; 5]; 3];
    println!("Enter values 0 for black and 1 for white");
    println!("arr[bin] ");
    let mut arr: [[u8;5];3] = [[0;5];3];
    for i in 0..3 {
        loop {
            let mut row = String::new();
            println!("Enter the values for arr[{}]: ", i);
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
    for i in 0..3 {
        for j in 0..5 {
            print!("{} ", arr[i][j]);
        }
        println!();  // Print a newline at the end of each row
    }

    Ok(arr)
}