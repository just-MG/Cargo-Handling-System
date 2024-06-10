extern crate hc_sr04;
use hc_sr04::{HcSr04, Unit};
use log::{debug, info};

/// Measures the distance using an HC-SR04 ultrasonic sensor and returns the median distance.
///
/// # Arguments
///
/// * `dd_rate` - The delay rate between each distance measurement in milliseconds.
/// * `dd_sample` - The number of distance samples to take.
///
/// # Returns
///
/// A `f32` representing the median distance measured in centimeters. If there is an error or no result,
/// returns `-1.0`.
///
/// # Description
///
/// This function initializes the HC-SR04 sensor with the trigger on GPIO pin 14 and the echo on GPIO pin 15.
/// It then takes a specified number of distance samples, waits for a specified delay between samples,
/// and stores the results. If a valid distance measurement is obtained, it is added to the list of distances.
/// If there is no result or an error occurs, it logs the issue and returns `-1.0`.
/// Finally, it calculates and returns the median of the collected distances.
pub fn get_distance(dd_rate: u64, dd_sample: u64) -> f32 {
    // TRIGGER on GPIO Pin 14 & ECHO on GPIO Pin 15.
    let mut ultrasonic = HcSr04::new(14, 15, None).unwrap();
    let mut distances: Vec<f32> = Vec::new();

    // Take dd_sample number of distance samples with a delay of dd_rate milliseconds between each sample.
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

/// Calculates the median of a list of distances.
///
/// # Arguments
///
/// * `distances` - A mutable slice of `f32` values representing the distances.
///
/// # Returns
///
/// A `f32` representing the median value of the distances.
///
/// # Description
///
/// This function sorts the list of distances in ascending order and returns the middle value.
/// It assumes the number of distances is odd, ensuring there is a single middle value to return.
fn get_median(distances: &mut [f32]) -> f32 {
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap());
    distances[distances.len() / 2]
}
