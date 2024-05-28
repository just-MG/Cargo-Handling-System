extern crate hc_sr04;
use log::{debug, info};
use hc_sr04::{HcSr04, Unit};

pub fn get_distance(dd_rate: u64, dd_sample: u64) -> f32 {
    // TRIGGER on GPIO Pin 7 & ECHO on GPIO Pin 11.
    let mut ultrasonic = HcSr04::new(4, 17, None).unwrap();
    let mut distances: Vec<f32> = Vec::new();

    for _ in 0..dd_sample {
        match ultrasonic.measure_distance(Unit::Centimeters) {
            Ok(Some(dist)) => {
                distances.push(dist);
            }
            Ok(None) => {
                debug!("No distance result");
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
    info!("Average distance: {}", get_distance_average(&distances));
    return get_distance_average(&distances);
}

fn get_distance_average(distances :&Vec<f32>) -> f32 {
    let mut sum: f32 = 0.0;
    for distance in distances {
        sum += distance;
    }
    return sum / distances.len() as f32;
}