use lcd_pcf8574::{Pcf8574, ErrorHandling};

 fn main(){
    let _ = DisplayError(1);
 }

fn display_error(error:u8) -> Result<(), Box<dyn std::error::Error>> {
    let bus = 1; 
    let addr = 0x27; 

    let mut dev = Pcf8574::new(bus, addr)?;
    dev.on_error(ErrorHandling::Panic);

    let mut display = lcd::Display::new(dev);
    display.init(lcd::FunctionLine::Line2, lcd::FunctionDots::Dots5x8);
    display.display(
        lcd::DisplayMode::DisplayOn,
        lcd::DisplayCursor::CursorOff,
        lcd::DisplayBlink::BlinkOff);

        let error_string = error.to_string();

    display.clear();
    display.home();
    display.print("Error:");
    display.position(2, 1);    
    display.print(error_string);
    display.position(2, 2);

    Ok(())
}