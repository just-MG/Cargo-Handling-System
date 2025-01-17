use log::*;
use std::io::{self, Read};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use crate::error_lcd::display_error;

/// Initializes the serial connection to the Arduino Uno and spawns a new thread to handle the data reception.
///
/// # Arguments
///
/// * `tx` - A `mpsc::Sender` used to send the averaged color values (RGB) from the serial thread to the main thread.
///
/// # Description
///
/// This function attempts to open a serial port connection to an Arduino Uno at a specified port and baud rate.
/// Once the connection is established, it reads data from the port, processes the data to extract RGB values,
/// and sends the averaged RGB values to the main thread using the provided `tx` channel. If the serial port
/// cannot be opened, the function logs the error and exits the program.
pub fn initialize_serial(tx: mpsc::Sender<(i32, i32, i32)>) {
    thread::spawn(move || {
        let port_name: &str = "/dev/ttyUSB0"; // Name of the port the Arduino Uno is connected to
        let baud_rate = 9600; // The baud rate of the connection, 9600

        info!(
            "Attempting to initialize serial port '{}' at baud rate '{}'",
            port_name, baud_rate
        );
        println!(
            "Attempting to initialize serial port '{}' at baud rate '{}'",
            port_name, baud_rate
        );

        let port = serialport::new(port_name, baud_rate) // Create a new serial port instance
            .timeout(Duration::from_millis(100)) // Set the read timeout to 100 milliseconds
            .open();

        match port {
            Ok(mut port) => {
                // If the serial port was successfully opened
                info!("Serial port '{}' opened successfully.", port_name);
                println!("Serial port '{}' opened successfully.", port_name);
                let mut serial_buf: Vec<u8> = vec![0; 1000]; // Create a buffer to store received data
                let mut received_data: Vec<u8> = Vec::new(); // Create a vector to store the received data
                let mut color_values: Vec<Vec<i32>> = Vec::new(); // Create a vector to store the color values
                loop {
                    match port.read(serial_buf.as_mut_slice()) {
                        // Read data from the serial port into the buffer
                        Ok(t) => {
                            received_data.extend_from_slice(&serial_buf[..t]); // Append the received data to the vector
                                                                               // Check if the vector does not contain more than one result
                            if received_data.len() <= 19
                                && received_data[0] == 123
                                && received_data[received_data.len() - 3] == 125
                            {
                                // Append the converted color values to the color values vector
                                color_values.push(convert_serial_color(received_data.clone()));
                            } else {
                                // If the vector contains more than one result, clear the vector
                                if received_data.len() > 19 {
                                    received_data.clear();
                                }
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                            warn!("Serial connection timeout.");
                            println!("Serial connection timeout.");
                        }
                        Err(e) => {
                            error!("Error reading from serial port: {:?}", e);
                            println!("Error reading from serial port: {:?}", e);
                        }
                    }
                    received_data.clear(); // Clear the received data vector
                    thread::sleep(Duration::from_millis(50));

                    // Assure the color_values vector contains no more than 5 newest results
                    if color_values.len() > 5 {
                        color_values.remove(0);
                    }
                    // Only send data to the main thread if there is data available
                    if !color_values.is_empty() {
                        tx.send(average_color_values(color_values.clone())).unwrap();
                    }
                }
            }
            Err(e) => {
                // If the serial port failed to open
                error!("Failed to open serial port '{}'. Error: {:?}", port_name, e);
                println!("Failed to open serial port '{}'. Error: {:?}", port_name, e);
                display_error(&22).unwrap(); // Display error code 22 on the LCD
                ::std::process::exit(22); // Exit the program, robot cannot work without the color sensor
            }
        }
    });
}

/// Calculates the average RGB values from a vector of RGB values.
///
/// # Arguments
///
/// * `color_values` - A vector of RGB values, where each RGB value is a vector of three integers (R, G, B).
///
/// # Returns
///
/// A tuple containing the average R, G, and B values as integers. If the input vector is empty, returns `(-1000, -1000, -1000)` to indicate no data is available.
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
        return (-1000, -1000, -1000); // Indicate no data available
    }
    (
        average_r / length as i32,
        average_g / length as i32,
        average_b / length as i32,
    )
}

/// Converts the data received from the Arduino Uno via the serial connection into RGB color values.
///
/// # Arguments
///
/// * `serial` - A vector of bytes received from the serial connection.
///
/// # Returns
///
/// A vector containing three integers representing the RGB color values.
fn convert_serial_color(serial: Vec<u8>) -> Vec<i32> {
    let mut color_values: Vec<i32> = Vec::new();
    let truncated_serial = &serial[1..serial.len() - 3]; // Truncate the serial vector to remove the first and last 3 values
    let mut color: Vec<u8> = Vec::new(); // Create a vector to store the temporary color value as ASCII characters
    let mut negative: bool = false; // Create a boolean variable to store whether the color value is negative
    for &byte in truncated_serial {
        // Iterate over the truncated serial vector
        if byte != 82 && byte != 71 && byte != 66 && byte != 59 && byte != 45 {
            // If the byte is not an R, G, B, semicolon, or hyphen
            color.push(byte);
        }
        if byte == 45 {
            // If the byte is a hyphen
            negative = true; // Set the negative flag to true
        }
        if byte == 59 {
            // If the byte is a semicolon
            let color_string: String = color.iter().map(|&c| c as char).collect(); // Convert the color vector to a string
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

/// Retrieves the most recent color values from the receiver channel.
///
/// # Arguments
///
/// * `rx` - A reference to an `mpsc::Receiver` that receives the averaged RGB values from the serial thread.
///
/// # Returns
///
/// A tuple containing the most recently received R, G, and B values as integers.
pub fn get_nwst_color(rx: &mpsc::Receiver<(i32, i32, i32)>) -> (i32, i32, i32) {
    let mut color_values: (i32, i32, i32) = (0, 0, 0);
    loop {
        match rx.try_recv() {
            Ok(_) => {
                color_values = rx.recv().unwrap();
            }
            Err(_) => {
                break;
            }
        }
    }
    info!("Newest color got: {:?}", color_values);
    println!("Newest color got: {:?}", color_values);
    color_values
}
