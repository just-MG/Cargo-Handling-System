extern crate hc_sr04;
use hc_sr04::{HcSr04, Unit};
use log::{debug, info};

pub fn get_distance(dd_rate: u64, dd_sample: u64) -> f32 {
    // TRIGGER on GPIO Pin 14 & ECHO on GPIO Pin 15.
    let mut ultrasonic = HcSr04::new(14, 15, None).unwrap();
    let mut distances: Vec<f32> = Vec::new();

    for _ in 0..dd_sample {
        match ultrasonic.measure_distance(Unit::Centimeters) {
            Ok(Some(dist)) => {
                distances.push(dist);
            }
            Ok(None) => {
                debug!("No distance result");
                println!("No distance result");
                return -1.0;
            }
            Err(e) => {
                debug!("Error getting distance: {:?}", e);
                return -1.0;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(dd_rate));
    }
    info!("Distances: {:?}", distances);
    println!("Distances: {:?}", distances);
    info!("Median of distances: {}", get_median(&mut distances));
    println!("Median of distances: {}", get_median(&mut distances));
    return get_median(&mut distances);
}

// Calculates median of distances. Works only for odd number of distances.
fn get_median(distances: &mut [f32]) -> f32 {
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distances[distances.len() / 2]
}
