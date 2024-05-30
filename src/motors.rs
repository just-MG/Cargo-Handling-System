use std::error::Error;
use std::thread;
use std::time::Duration;
use log::{info};

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_17: u8 = 17; //In2 on shield
// const GPIO_0: u8 = 4; //In1 on shield
const GPIO_27: u8 = 27; //ENA on shield

// for discard arm
const DIS1: u8 = 15; //In3 on shield
const DIS2: u8 = 16; //In4 on shield
const DISENB: u8 = 18; //ENB on shield

//Servo initialization stuff
const GPIO_PWM_0: u8 = 23; //again the correct pin number is temporarirly on the raspberry pi
const GPIO_PWM_1: u8 = 21; //again the correct pin number is temporarirly on the raspberry pi
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 500;
const PULSE_MAX_US: u64 = 2500;

// CONVEYOR
pub fn start_conveyor() -> Result<(), Box<dyn Error>> {
    let mut pin17 = Gpio::new()?.get(GPIO_17)?.into_output();
    let mut pin27 = Gpio::new()?.get(GPIO_27)?.into_output();
    pin17.set_high();
    pin27.set_low();
    Ok(())
}

pub fn stop_conveyor() -> Result<(), Box<dyn Error>> {
    let mut pin17 = Gpio::new()?.get(GPIO_17)?.into_output();
    let mut pin27 = Gpio::new()?.get(GPIO_27)?.into_output();
    pin17.set_low();
    pin27.set_low();
    Ok(())
}

// SORTING ARMS
/// direction: 0 - left, 1 - right
pub fn move_sort_arm_1(direction: i32) -> Result<(), Box<dyn Error>> {
    info!("Moving motor 1 to direction: {}", direction);
    println!("Moving motor 1 to direction: {}", direction);
    //Initialize servo
    let mut pin = Gpio::new()?.get(GPIO_PWM_0)?.into_output();
    if direction == 0 {
        //Rotate the servo
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(PULSE_MAX_US),
        )?;
    }
    if direction == 1 {
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(PULSE_MIN_US),
        )?;
    } 
    Ok(())
}

/// direction: 0 - left, 1 - right
pub fn move_sort_arm_2(direction: i32) -> Result<(), Box<dyn Error>> {
    info!("Moving motor 2 to direction: {}", direction);
    println!("Moving motor 2 to direction: {}", direction);
    //Initialize servo
    let mut pin = Gpio::new()?.get(GPIO_PWM_1)?.into_output();
    if direction == 0 {
        //Rotate the servo
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(PULSE_MAX_US),
        )?;
    }
    if direction == 1 {
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(PULSE_MIN_US),
        )?;
    }
    Ok(())
}

/// Sets the sorting arms to the specified bin
pub fn sort_arm(bin: i32) -> () {
    info!("Sorting item to bin: {}", bin);
    println!("Sorting item to bin: {}", bin);
    match bin {
        0 => {move_sort_arm_1(0);}, // Move arm 1 to left,
        1 => {
            move_sort_arm_1(0); // Move arm 1 to left
            move_sort_arm_2(1); // Move arm 2 to right
        },
        2 => {
            move_sort_arm_1(0); // Move arm 1 to left
            move_sort_arm_2(0); // Move arm 2 to left
        },
        _ => (),
    }
}

// DISCARD MOTOR
pub fn discard_item() -> Result<(), Box<dyn Error>>{
    let mut mot1 = Gpio::new()?.get(DIS1)?.into_output();
    let mut mot2 = Gpio::new()?.get(DIS2)?.into_output();
    mot1.set_high();
    mot2.set_low();
    thread::sleep(Duration::from_millis(300));

    mot1.set_low();
    mot2.set_high();
    thread::sleep(Duration::from_millis(300));
    Ok(())
}
