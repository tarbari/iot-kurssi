use rppal::gpio::Gpio;
use std::error::Error;

const LOW: u8 = 16;
const OK: u8 = 20;
const HIGH: u8 = 21;

fn main() -> Result<(), Box<dyn Error>> {
    println!(
        "Setting pins {}, {}, {} low",
        LOW.to_string(),
        OK.to_string(),
        HIGH.to_string()
    );
    let mut low = Gpio::new()?.get(LOW)?.into_output();
    let mut ok = Gpio::new()?.get(OK)?.into_output();
    let mut high = Gpio::new()?.get(HIGH)?.into_output();
    low.set_low();
    ok.set_low();
    high.set_low();
    Ok(())
}
