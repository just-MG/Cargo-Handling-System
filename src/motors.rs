use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

const GPIO_2: u8 = 13; //The pin numbers are temporarirly incorrect! The correct numbers will be updated shortly (the raspberry pi has the correct numbers saved)!
const GPIO_0: u8 = 11;
const GPIO_7: u8 = 7;
// for disracr arm
const DIS1: u8 = 2; 
const DIS2: u8 = 3; 

//Servo initialization stuff
const GPIO_PWM_0: u8 = 23; //again the correct pin number is temporarirly on the raspberry pi
const GPIO_PWM_1: u8 = 21; //again the correct pin number is temporarirly on the raspberry pi
const PERIOD_MS: u64 = 20;
const PULSE_MIN_US: u64 = 1200;
const PULSE_NEUTRAL_US: u64 = 1500;
const PULSE_MAX_US: u64 = 1800;

use log::{error, info};

// let mut pin7 = Gpio::new()?.get(GPIO_7)?.into_output();

// CONVEYOR
pub fn start_conveyor() -> Result<(), Box<dyn Error>> {
    let mut pin2 = Gpio::new()?.get(GPIO_2)?.into_output();
    let mut pin0 = Gpio::new()?.get(GPIO_0)?.into_output();
    pin2.set_high();
    pin0.set_low();
    Ok(())
}

pub fn stop_conveyor() -> Result<(), Box<dyn Error>> {
    let mut pin2 = Gpio::new()?.get(GPIO_2)?.into_output();
    let mut pin0 = Gpio::new()?.get(GPIO_0)?.into_output();
    pin2.set_low();
    pin0.set_low();
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