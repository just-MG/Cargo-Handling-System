use std::error::Error;
use std::thread;
use std::time::Duration;
use log::{info};
use std::sync::mpsc;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_17: u8 = 17; //In2 on shield
// const GPIO_0: u8 = 4; //In1 on shield
const GPIO_27: u8 = 27; //ENA on shield

pub fn initialize_motors(rx: mpsc::Receiver<(i32, u64)>) -> () {
    thread::spawn(move || {
        loop {
            let command = get_nwst_command(&rx);
            if command.0 == 1 {
                loop {
                    start_conveyor(&command.1);
                    let command = get_nwst_command(&rx);
                    if command.1 == 0 {
                        break;
                    }
                }
            }
            }
        });
}

fn start_conveyor(time: &u64) -> Result<(), Box<dyn Error>> {
    let mut pin17 = Gpio::new()?.get(GPIO_17)?.into_output();
    let mut pin27 = Gpio::new()?.get(GPIO_27)?.into_output();
    pin17.set_high();
    pin27.set_low();
    thread::sleep(Duration::from_millis(*time));
    pin17.set_low();
    Ok(())
}

fn get_nwst_command(rx: &mpsc::Receiver<(i32, u64)>)->(i32, u64) {
    let mut command: (i32,u64) = (-1,0);
    loop {
        match rx.try_recv() {
            Ok(_) => {
                command = rx.recv().unwrap();
            },
            Err(_) => {
                break;
            }
        }
    }
    return command;
}