use rppal::gpio::OutputPin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Temperature {
    pub temperature: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Bounds {
    pub lower: i16,
    pub upper: i16,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TemperatureAndBounds {
    pub t: Temperature,
    pub b: Bounds,
}

// #[derive(Clone)]
pub struct Leds {
    pub low: OutputPin,
    pub high: OutputPin,
    pub ok: OutputPin,
}
