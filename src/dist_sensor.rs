extern crate hc_sr04;
use hc_sr04::{HcSr04, Result, Unit};
use std::{thread, time::Duration};

fn run() -> Result<()> {
    // TRIGGER on GPIO Pin 7 & ECHO on GPIO Pin 11.
    println!("start program");
    let mut ultrasonic = HcSr04::new(4, 17, None).unwrap();
	
    println!("start loop");

    loop {
	println!("start 1");
        match ultrasonic.measure_distance(Unit::Centimeters).unwrap() {
            Some(dist) => println!("Distance: {:.1}cm", dist),
            None => println!("Object out of range"),
        }
        println!("end 1");
        thread::sleep(Duration::from_secs(1));
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {err}");
    }
}