use std::io::{self, Read};
use std::time::Duration;
use std::thread;
use std::sync::mpsc;
use log::*;

pub fn initialize_serial(tx: mpsc::Sender<(i32, i32, i32)>) {
    thread::spawn(move || {
        let port_name: &str = "/dev/ttyUSB0"; // Get the value of the "port" argument
        let baud_rate = 9600; // Get the value of the "baud" argument and parse it as u32

        info!("Attempting to initialize serial port '{}' at baud rate '{}'", port_name, baud_rate);

        let port = serialport::new(port_name, baud_rate) // Create a new serial port instance
            .timeout(Duration::from_millis(10)) // Set the read timeout to 10 milliseconds
            .open(); // Open the serial port

        match port {
            Ok(mut port) => { // If the serial port was successfully opened
                info!("Serial port '{}' opened successfully.", port_name);
                let mut serial_buf: Vec<u8> = vec![0; 1000]; // Create a buffer to store received data
                let mut received_data: Vec<u8> = Vec::new(); // Create a vector to store the received data
                let mut color_values: Vec<Vec<i32>> = Vec::new(); // Create a vector to store the color values
                loop {
                    match port.read(serial_buf.as_mut_slice()) { // Read data from the serial port into the buffer
                        Ok(t) => {
                            debug!("Read {} bytes from serial port.", t);
                            received_data.extend_from_slice(&serial_buf[..t]); // Append the received data to the vector
                            if received_data.len() <= 19 && received_data[0] == 123 && received_data[received_data.len()-3] == 125 { // Check if the vector does not contain more than one result
                                let result = received_data.clone(); // Save the received data to a variable
                                color_values.push(convert_serial_color(result)); // Append the converted color values to the color values vector
                            } else {
                                // If the vector contains more than one result, clear the vector
                                if color_values.len() > 19 {
                                    color_values.clear();
                                }
                            }
                        },
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            warn!("Serial connection timeout.");
                        },
                        Err(e) => {
                            error!("Error reading from serial port: {:?}", e);
                        }
                    }
                    received_data.clear(); // Clear the received data vector
                    thread::sleep(Duration::from_millis(50));

                    if color_values.len() > 5 {
                        color_values.remove(0);
                    }
                    if !color_values.is_empty() {
                        tx.send(average_color_values(color_values.clone())).unwrap();
                    }
                }
            }
            Err(e) => { // If the serial port failed to open
                error!("Failed to open serial port '{}'. Error: {:?}", port_name, e); // Log the error message
                ::std::process::exit(1);
            }
        }
    });
}


fn average_color_values(color_values: Vec<Vec<i32>>) -> (i32, i32, i32) {
    let mut average_r = 0;
    let mut average_g = 0;
    let mut average_b = 0;
    let length = color_values.len();
    for vector in color_values {
        average_r += vector[0];
        average_g += vector[1];
        average_b += vector[2];
    }
    if length == 0 {
        return (-1000, -1000, -1000); // indicate no data available
    }
    // println!("{:?}", (average_r / length as i32, average_g / length as i32, average_b / length as i32));
    (average_r / length as i32, average_g / length as i32, average_b / length as i32)
}

fn convert_serial_color(serial: Vec<u8>) -> Vec<i32> {
    let mut color_values: Vec<i32> = Vec::new();
    let truncated_serial = &serial[1..serial.len()-3]; // Truncate the serial vector to remove the first and last 3 values
    let mut color: Vec<u8> = Vec::new(); // Create a vector to store the temporary color value as ASCII characters
    let mut negative: bool = false; // Create a boolean variable to store whether the color value is negative
    for &byte in truncated_serial { // Iterate over the truncated serial vector
        if byte != 82 && byte != 71 && byte != 66 && byte != 59  && byte != 45{
            color.push(byte);
        }
        if byte == 45 { // If the byte is a hyphen
            negative = true; // Set the negative flag to true
        }
        if byte == 59 { // If the byte is a semicolon
            // println!("-{:?}", color);
            let color_string: String = color.iter().map(|&c| c as char).collect(); // Convert the color vector to a string
            // println!("--{:?}", color_string);
            let color_value: i32 = color_string.parse().unwrap(); // Parse the color string as an integer
            if negative {
                color_values.push(color_value * -1); // Append the negative color value to the color values vector
                negative = false; // Reset the negative flag
            } else {
                color_values.push(color_value); // Append the color value to the color values vector
            }
            color.clear(); // Clear the temporary color vectors
        }
    }
    
    [color_values[0], color_values[1], color_values[2]].to_vec() // Return the color values as a tuple
}

pub fn get_nwst_color(rx: &mpsc::Receiver<(i32,i32,i32)>)->(i32,i32,i32) {
    let mut color_values: (i32,i32,i32) = (0,0,0);
    loop {
        match rx.try_recv() {
            Ok(_) => {
                color_values = rx.recv().unwrap();
            },
            Err(_) => {
                break;
            }
        }
    }
    info!("Newest color got: {:?}", color_values);
    color_values
}