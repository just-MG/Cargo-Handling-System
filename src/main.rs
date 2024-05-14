mod serial_connection_con;
mod detect_color;
use std::sync::mpsc;

fn main() {
    let (tx, rx) = mpsc::channel();
    serial_connection_con::initialize_serial(tx); // Start the serial connection in a separate thread
    std::thread::sleep(std::time::Duration::from_secs(3)); // Wait for the serial connection to initialize
    let color_values = get_nwst_color(&rx);
    println!("{:?}", detect_color::logic(color_values.0, color_values.1, color_values.2));
}

fn get_nwst_color(rx: &mpsc::Receiver<(i32,i32,i32)>)->(i32,i32,i32) {
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
    color_values
}
