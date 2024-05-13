use std::sync::mpsc;
// mod serial_connection;
mod detect_color;
pub mod serial_connection_con;

fn main() {
    let (tx, rx) = mpsc::channel();
    serial_connection_con::initialize_serial(tx);
    for i in 0..10 {
        println!("Main thread is running {}", i);
        let color_values = rx.recv().unwrap();
        print!("{:?}", detect_color::logic(color_values.0, color_values.1, color_values.2));
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
    
    // let color_values = serial_connection::serial_connect();
}
