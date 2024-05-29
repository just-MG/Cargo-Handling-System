use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_17: u8 = 17;
//const GPIO_0: u8 = 4;
const GPIO_27: u8 = 27;

fn main() -> Result<(), Box<dyn Error>> {
    let mut pin17 = Gpio::new()?.get(GPIO_17)?.into_output();
    //let mut pin0 = Gpio::new()?.get(GPIO_0)?.into_output();
    let mut pin27 = Gpio::new()?.get(GPIO_27)?.into_output();

    pin17.set_high();
    pin27.set_low();
    thread::sleep(Duration::from_millis(400));
    pin17.set_low();

    Ok(())
}