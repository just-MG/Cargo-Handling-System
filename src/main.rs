mod serial_connection;
mod detect_color;

fn main() {
    let color_values = serial_connection::serial_connect();
    print!("{:?}", detect_color::logic(color_values.0, color_values.1, color_values.2));
}
