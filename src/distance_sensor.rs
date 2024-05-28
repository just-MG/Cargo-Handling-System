extern crate hc_sr04;
use log::{debug, info};
use hc_sr04::{HcSr04, Unit};

pub fn get_distance() -> f32 {
    // TRIGGER on GPIO Pin 7 & ECHO on GPIO Pin 11.
    let mut ultrasonic = HcSr04::new(4, 17, None).unwrap();
	
    // match ultrasonic.measure_distance(Unit::Centimeters).unwrap() {
    //     Some(dist) => {info!("Distance got: {}", dist);return dist},
    //     None => {debug!("No distance result"); return 0.0},
    // }

    match ultrasonic.measure_distance(Unit::Centimeters) {
        Ok(Some(dist)) => {
            info!("Distance got: {}", dist);
            return dist;
        }
        Ok(None) => {
            debug!("No distance result");
            return 0.0;
        }
        Err(e) => {
            debug!("Error getting distance: {:?}", e);
            return 0.0;
        }
    }
}