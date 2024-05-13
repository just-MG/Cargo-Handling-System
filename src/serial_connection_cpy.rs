use std::io::{self};
use std::time::Duration;


pub fn serial_connect() -> () {

    let port_name: &str = "/dev/ttyUSB0"; // Get the value of the "port" argument
    let baud_rate = 9600; // Get the value of the "baud" argument and parse it as u32

    let port = serialport::new(port_name, baud_rate) // Create a new serial port instance
        .timeout(Duration::from_millis(10)) // Set the read timeout to 10 milliseconds
        .open(); // Open the serial port

    match port {
        Ok(mut port) => { // If the serial port was successfully opened
            let mut serial_buf: Vec<u8> = vec![0; 1000]; // Create a buffer to store received data
            let mut received_data: Vec<u8> = Vec::new(); // Create a vector to store the received data
            let mut color_values: Vec<i32> = Vec::new(); // Create a vector to store the color values
            loop {
                match port.read(serial_buf.as_mut_slice()) { // Read data from the serial port into the buffer
                    Ok(t) => {
                        received_data.extend_from_slice(&serial_buf[..t]); // Append the received data to the vector
                            let result = received_data.clone(); // Save the received data to a variable
                            println!("result length{:?}", result.len());
                            println!("result raw{:?}", result);
                            for &byte in &result { // Iterate over the received data
                                let character = byte as char; // Convert the byte to a character
                                print!("{}", character); // Print the character
                            }
                    },
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (), // If a timeout occurred, do nothing
                    Err(e) => eprintln!("{:?}", e) // If an error occurred, print the error message
                }
                received_data.clear(); // Clear the received data vector
                std::thread::sleep(Duration::from_millis(150));
            }
        }
        Err(e) => { // If the serial port failed to open
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e); // Print an error message
            ::std::process::exit(1); // Exit the program with a non-zero status code
        }
    }
}
/*

*/
fn average_color_values() -> (i32, i32, i32) {
    (0, 0, 0)
}