use log::info;
use std::error::Error;
use std::thread;
use std::time::Duration;

use rppal::gpio::Gpio;

const GPIO_17: u8 = 17; //In2 on shield
const GPIO_27: u8 = 27; //ENA on shield
const GP_4: u8 = 4;

// for discard arm
const DIS1: u8 = 20; //In3 on shield
const DIS2: u8 = 16; //In4 on shield

//Servo initialization stuff
const GPIO_PWM_0: u8 = 5;
const GPIO_PWM_1: u8 = 6;
const PERIOD_MS: u64 = 20;
const GPIO_SEP: u8 = 26;

// CONVEYOR
pub fn start_conveyor() -> Result<(), Box<dyn Error>> {
    let mut pin17 = Gpio::new()?.get(GPIO_17)?.into_output();
    let mut pin27 = Gpio::new()?.get(GPIO_27)?.into_output();
    let mut pin_hold = Gpio::new()?.get(GP_4)?.into_output();
    pin_hold.set_pwm(Duration::from_millis(50), Duration::from_millis(20))?;
    pin17.set_low();
    pin27.set_high();
    // thread::sleep(Duration::from_secs(15));
    // pin27.set_low();
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
        pin.set_pwm(Duration::from_millis(PERIOD_MS), Duration::from_micros(900))?;
        thread::sleep(Duration::from_millis(100));
    }
    if direction == 1 {
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(1450),
        )?;
        thread::sleep(Duration::from_millis(100));
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
            Duration::from_micros(1000),
        )?;
        thread::sleep(Duration::from_millis(100));
    }
    if direction == 1 {
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(1800),
        )?;
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}

/// Sets the sorting arms to the specified bin
pub fn sort_arm(bin: i32) -> () {
    info!("Sorting item to bin: {}", bin);
    println!("Sorting item to bin: {}", bin);
    match bin {
        0 => {
            let _ = move_sort_arm_2(1);
            thread::sleep(Duration::from_millis(200));
            let _ = move_sort_arm_1(1);
            thread::sleep(Duration::from_millis(200));
        } // Move arm 1 to left,
        1 => {
            let _ = move_sort_arm_1(0); // Move arm 1 to left
            thread::sleep(Duration::from_millis(200));
            let _ = move_sort_arm_2(1); // Move arm 2 to right
            thread::sleep(Duration::from_millis(200));
        }
        2 => {
            let _ = move_sort_arm_1(0); // Move arm 1 to left
            thread::sleep(Duration::from_millis(200));
            let _ = move_sort_arm_2(0); // Move arm 2 to left
            thread::sleep(Duration::from_millis(200));
        }
        _ => (),
    }
}

// DISCARD MOTOR
pub fn discard_item() -> Result<(), Box<dyn Error>> {
    let mut mot1 = Gpio::new()?.get(DIS1)?.into_output();
    let mut mot2 = Gpio::new()?.get(DIS2)?.into_output();
    mot1.set_high();
    mot2.set_low();
    thread::sleep(Duration::from_millis(300));

    mot1.set_low();
    mot2.set_high();
    thread::sleep(Duration::from_millis(300));
    info!("Moving separation servo up");
    println!("Moving separation servo up");
    separate_input(0);
    Ok(())
}

// INPUT SEPARATION
pub fn separate_input(direction: i32) -> Result<(), Box<dyn Error>> {
    //Initialize servo
    let mut pin = Gpio::new()?.get(GPIO_SEP)?.into_output();
    if direction == 0 {
        // up
        //info!("Moving separation servo up");
        //println!("Moving separation servo up");
        //Rotate the servo
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(2000),
        )?;
        thread::sleep(Duration::from_millis(100));
    }
    if direction == 1 {
        // down
        //info!("Moving separation servo down");
        //println!("Moving separation servo down");
        pin.set_pwm(
            Duration::from_millis(PERIOD_MS),
            Duration::from_micros(1000),
        )?;
        thread::sleep(Duration::from_millis(100));
    }
    Ok(())
}
