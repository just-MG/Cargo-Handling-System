use lcd_pcf8574::{ErrorHandling, Pcf8574};

/// Displays an error message on a 2-line LCD connected via the PCF8574 I2C expander.
///
/// # Arguments
///
/// * `error` - A reference to a `u32` representing the error code to be displayed.
///
/// # Returns
///
/// A `Result<(), Box<dyn std::error::Error>>` indicating success or failure.
///
/// # Description
///
/// This function initializes the LCD display with 2 lines and 5x8 dots. It clears the display,
/// sets the cursor to the home position, and prints the word "Error:" followed by the error code.
/// The error handling mode is set to panic on errors.
pub fn display_error(error: &u32) -> Result<(), Box<dyn std::error::Error>> {
    let bus = 1;
    let addr = 0x27;

    let mut dev = Pcf8574::new(bus, addr)?;
    dev.on_error(ErrorHandling::Panic);

    let mut display = lcd::Display::new(dev);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff,
    );

    let error_string = error.to_string();

    display.clear();
    display.home();
    display.print("Error:");
    display.position(2, 1);
    display.print(&error_string);
    display.position(2, 2);

    Ok(())
}

/// Clears the display on a 2-line LCD connected via the PCF8574 I2C expander.
///
/// # Returns
///
/// A `Result<(), Box<dyn std::error::Error>>` indicating success or failure.
///
/// # Description
///
/// This function initializes the LCD display with 2 lines and 5x8 dots. It clears the display
/// and sets the cursor to the home position. The error handling mode is set to panic on errors.
pub fn display_clear() -> Result<(), Box<dyn std::error::Error>> {
    let bus = 1;
    let addr = 0x27;

    let mut dev = Pcf8574::new(bus, addr)?;
    dev.on_error(ErrorHandling::Panic);

    let mut display = lcd::Display::new(dev);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff,
    );

    display.clear();
    display.home();

    Ok(())
}
