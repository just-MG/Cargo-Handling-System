extern crate hc_sr04;
use log::{debug, info};
use hc_sr04::{HcSr04, Unit};

pub fn get_distance() -> f32 {
    // TRIGGER on GPIO Pin 7 & ECHO on GPIO Pin 11.
    let mut ultrasonic = HcSr04::new(4, 17, None).unwrap();
    let mut distance_results: Vec<f32> = Vec::new();

    for _ in 0..5 {
        match ultrasonic.measure_distance(Unit::Centimeters) {
            Ok(Some(dist)) => {
                info!("Distance got: {}", dist);
                distance_results.push(dist);
            }
            Ok(None) => {
                debug!("No distance result");
            }
            Err(e) => {
                debug!("Error getting distance: {:?}", e);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let average_distance: f32 = distance_results.iter().sum::<f32>() / distance_results.len() as f32;
    average_distance
}